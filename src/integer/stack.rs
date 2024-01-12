// Copyright © 2016–2024 Trevor Spiteri

// This program is free software: you can redistribute it and/or modify it under
// the terms of the GNU Lesser General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option) any
// later version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU General Public License for more
// details.
//
// You should have received a copy of the GNU Lesser General Public License and
// a copy of the GNU General Public License along with this program. If not, see
// <https://www.gnu.org/licenses/>.

use crate::misc::NegAbs;
use crate::{Assign, Integer};
use az::{Az, Cast, WrappingCast};
use core::cell::{Ref, RefCell};
use core::ffi::c_int;
use core::fmt::{
    Binary, Debug, Display, Formatter, LowerHex, Octal, Result as FmtResult, UpperHex,
};
use core::mem;
use core::mem::MaybeUninit;
use core::ops::Deref;
use core::ptr::NonNull;
use gmp_mpfr_sys::gmp;
use gmp_mpfr_sys::gmp::{limb_t, mpz_t};

pub const LIMBS_IN_SMALL: usize = (128 / gmp::LIMB_BITS) as usize;
pub type Limbs = [MaybeUninit<limb_t>; LIMBS_IN_SMALL];

/**
A small integer that does not require any memory allocation.

This can be useful when you have a primitive integer type such as [`u64`] or
[`i8`], but need a reference to an [`Integer`].

If there are functions that take a [`u32`] or [`i32`] directly instead of an
[`Integer`] reference, using them can still be faster than using a
`StackInteger`; the functions would still need to check for the size of an
[`Integer`] obtained using `StackInteger`.

The [`borrow`][Self::borrow] method returns an object that can be
coerced to an [`Integer`], as it implements
<code>[Deref]\<[Target][Deref::Target] = [Integer]></code>.

# Examples

```rust
use rug::integer::StackInteger;
use rug::Integer;
// `a` requires a heap allocation
let mut a = Integer::from(250);
// `b` can reside on the stack
let b = StackInteger::from(-100);
a.lcm_mut(&b.borrow());
assert_eq!(a, 500);
// another computation:
a.lcm_mut(&StackInteger::from(30).borrow());
assert_eq!(a, 1500);
```

[Target]: core::ops::Deref::Target
*/
#[derive(Clone)]
pub struct StackInteger {
    inner: RefCell<mpz_t>,
    limbs: Limbs,
}

static_assert!(mem::size_of::<Limbs>() == 16);

// Safety: StackInteger cannot be Sync because it contains a RefCell.
// But StackInteger can be Send, just like RefCell.
unsafe impl Send for StackInteger {}

impl Default for StackInteger {
    #[inline]
    fn default() -> Self {
        StackInteger::new()
    }
}

impl Display for StackInteger {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&*self.borrow(), f)
    }
}

impl Debug for StackInteger {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Debug::fmt(&*self.borrow(), f)
    }
}

impl Binary for StackInteger {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Binary::fmt(&*self.borrow(), f)
    }
}

impl Octal for StackInteger {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Octal::fmt(&*self.borrow(), f)
    }
}

impl LowerHex for StackInteger {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        LowerHex::fmt(&*self.borrow(), f)
    }
}

impl UpperHex for StackInteger {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        UpperHex::fmt(&*self.borrow(), f)
    }
}

impl StackInteger {
    /// Creates a [`StackInteger`] with value 0.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::StackInteger;
    /// let i = StackInteger::new();
    /// assert_eq!(*i.borrow(), 0);
    /// ```
    #[inline]
    pub const fn new() -> Self {
        StackInteger {
            inner: RefCell::new(mpz_t {
                alloc: LIMBS_IN_SMALL as c_int,
                size: 0,
                d: NonNull::dangling(),
            }),
            limbs: small_limbs![0],
        }
    }

    /// Returns a mutable reference to an [`Integer`] for simple operations that
    /// do not need to allocate more space for the number.
    ///
    /// # Safety
    ///
    /// It is undefined behavior to perform operations that reallocate the
    /// internal data of the referenced [`Integer`] or to swap it with another
    /// number.
    ///
    /// Some GMP functions swap the allocations of their target operands;
    /// calling such functions with the mutable reference returned by this
    /// method can lead to undefined behavior.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::StackInteger;
    /// use rug::Assign;
    /// let mut i = StackInteger::from(1u64);
    /// let capacity = i.borrow().capacity();
    /// // another u64 will not require a reallocation
    /// unsafe {
    ///     i.as_nonreallocating_integer().assign(2u64);
    /// }
    /// assert_eq!(*i.borrow(), 2);
    /// assert_eq!(i.borrow().capacity(), capacity);
    /// ```
    #[inline]
    pub unsafe fn as_nonreallocating_integer(&mut self) -> &mut Integer {
        // Since we borrow self mutably, it is statically guaranteed that no borrows exist.
        let inner = self.inner.get_mut();
        // Update d to point to limbs.
        inner.d = NonNull::<[MaybeUninit<limb_t>]>::from(&self.limbs[..]).cast();
        let ptr = cast_ptr_mut!(inner, Integer);
        // Safety: since inner.d points to the limbs, it is in a consistent state.
        unsafe { &mut *ptr }
    }

    /// Borrows the integer.
    ///
    /// The borrow lasts until the returned [`Ref`] exits scope.
    /// Multiple borrows can be taken at the same time.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::integer::StackInteger;
    /// use rug::Integer;
    /// let i = StackInteger::from(-13i32);
    /// let b = i.borrow();
    /// let abs_ref = b.abs_ref();
    /// assert_eq!(Integer::from(abs_ref), 13);
    /// ```
    #[inline]
    pub fn borrow(&self) -> impl Deref<Target = Integer> + '_ {
        // Make sure d is pointing to limbs.
        match self.inner.try_borrow_mut() {
            Ok(mut inner) => {
                // Update d to point to limbs.
                inner.d = NonNull::<[MaybeUninit<limb_t>]>::from(&self.limbs[..]).cast();
            }
            Err(_) => {
                // Since there is another borrow, d must have already been updated.
                // Keep in mind that StackInteger is !Sync.
            }
        }
        // There cannot be a mutable borrow anywhere else, so this should never fail.
        Ref::map(self.inner.borrow(), |inner| {
            let ptr = cast_ptr!(inner, Integer);
            // Safety: since inner.d points to limbs, it is in a consistent state.
            unsafe { &*ptr }
        })
    }
}

/// Types implementing this trait can be converted to [`StackInteger`].
///
/// The following are implemented when `T` implements `ToStack`:
///   * <code>[Assign][`Assign`]\<T> for [StackInteger][`StackInteger`]</code>
///   * <code>[From][`From`]\<T> for [StackInteger][`StackInteger`]</code>
///
/// This trait is sealed and cannot be implemented for more types; it is
/// implemented for [`bool`] and the unsigned integer types [`u8`], [`u16`],
/// [`u32`], [`u64`], [`u128`] and [`usize`].
pub trait ToStack: SealedToStack {}

pub trait SealedToStack: Sized {
    fn copy(self, size: &mut c_int, limbs: &mut Limbs);
    fn is_zero(&self) -> bool;
}

macro_rules! is_zero {
    () => {
        #[inline]
        fn is_zero(&self) -> bool {
            *self == 0
        }
    };
}

macro_rules! signed {
    ($($I:ty)*) => { $(
        impl ToStack for $I {}
        impl SealedToStack for $I {
            #[inline]
            fn copy(self, size: &mut c_int, limbs: &mut Limbs) {
                let (neg, abs) = self.neg_abs();
                abs.copy(size, limbs);
                if neg {
                    *size = -*size;
                }
            }

            is_zero! {}
        }
    )* };
}

macro_rules! one_limb {
    ($($U:ty)*) => { $(
        impl ToStack for $U {}
        impl SealedToStack for $U {
            #[inline]
            fn copy(self, size: &mut c_int, limbs: &mut Limbs) {
                if self == 0 {
                    *size = 0;
                } else {
                    *size = 1;
                    limbs[0] = MaybeUninit::new(self.into());
                }
            }

            is_zero! {}
        }
    )* };
}

signed! { i8 i16 i32 i64 i128 isize }

impl ToStack for bool {}

impl SealedToStack for bool {
    #[inline]
    fn copy(self, size: &mut c_int, limbs: &mut Limbs) {
        if self {
            *size = 1;
            limbs[0] = MaybeUninit::new(1);
        } else {
            *size = 0;
        }
    }

    #[inline]
    fn is_zero(&self) -> bool {
        !*self
    }
}

one_limb! { u8 u16 u32 }

#[cfg(gmp_limb_bits_64)]
one_limb! { u64 }

#[cfg(gmp_limb_bits_32)]
impl ToStack for u64 {}
#[cfg(gmp_limb_bits_32)]
impl SealedToStack for u64 {
    #[inline]
    fn copy(self, size: &mut c_int, limbs: &mut Limbs) {
        if self == 0 {
            *size = 0;
        } else if self <= 0xffff_ffff {
            *size = 1;
            limbs[0] = MaybeUninit::new(self.wrapping_cast());
        } else {
            *size = 2;
            limbs[0] = MaybeUninit::new(self.wrapping_cast());
            limbs[1] = MaybeUninit::new((self >> 32).wrapping_cast());
        }
    }

    is_zero! {}
}

impl ToStack for u128 {}

impl SealedToStack for u128 {
    #[cfg(gmp_limb_bits_64)]
    #[inline]
    fn copy(self, size: &mut c_int, limbs: &mut Limbs) {
        if self == 0 {
            *size = 0;
        } else if self <= 0xffff_ffff_ffff_ffff {
            *size = 1;
            limbs[0] = MaybeUninit::new(self.wrapping_cast());
        } else {
            *size = 2;
            limbs[0] = MaybeUninit::new(self.wrapping_cast());
            limbs[1] = MaybeUninit::new((self >> 64).wrapping_cast());
        }
    }

    #[cfg(gmp_limb_bits_32)]
    #[inline]
    fn copy(self, size: &mut c_int, limbs: &mut Limbs) {
        if self == 0 {
            *size = 0;
        } else if self <= 0xffff_ffff {
            *size = 1;
            limbs[0] = MaybeUninit::new(self.wrapping_cast());
        } else if self <= 0xffff_ffff_ffff_ffff {
            *size = 2;
            limbs[0] = MaybeUninit::new(self.wrapping_cast());
            limbs[1] = MaybeUninit::new((self >> 32).wrapping_cast());
        } else if self <= 0xffff_ffff_ffff_ffff_ffff_ffff {
            *size = 3;
            limbs[0] = MaybeUninit::new(self.wrapping_cast());
            limbs[1] = MaybeUninit::new((self >> 32).wrapping_cast());
            limbs[2] = MaybeUninit::new((self >> 64).wrapping_cast());
        } else {
            *size = 4;
            limbs[0] = MaybeUninit::new(self.wrapping_cast());
            limbs[1] = MaybeUninit::new((self >> 32).wrapping_cast());
            limbs[2] = MaybeUninit::new((self >> 64).wrapping_cast());
            limbs[3] = MaybeUninit::new((self >> 96).wrapping_cast());
        }
    }

    is_zero! {}
}

impl ToStack for usize {}
impl SealedToStack for usize {
    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn copy(self, size: &mut c_int, limbs: &mut Limbs) {
        self.az::<u32>().copy(size, limbs);
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn copy(self, size: &mut c_int, limbs: &mut Limbs) {
        self.az::<u64>().copy(size, limbs);
    }

    is_zero! {}
}

impl<T: ToStack> Assign<T> for StackInteger {
    #[inline]
    fn assign(&mut self, src: T) {
        src.copy(&mut self.inner.get_mut().size, &mut self.limbs);
    }
}

impl<T: ToStack> From<T> for StackInteger {
    #[inline]
    fn from(src: T) -> Self {
        let mut size = 0;
        let mut limbs = small_limbs![0];
        src.copy(&mut size, &mut limbs);
        StackInteger {
            inner: RefCell::new(mpz_t {
                alloc: LIMBS_IN_SMALL.cast(),
                size,
                d: NonNull::dangling(),
            }),
            limbs,
        }
    }
}

impl Assign<&Self> for StackInteger {
    #[inline]
    fn assign(&mut self, other: &Self) {
        self.clone_from(other);
    }
}

impl Assign for StackInteger {
    #[inline]
    fn assign(&mut self, other: Self) {
        drop(mem::replace(self, other));
    }
}

#[cfg(test)]
mod tests {
    use crate::integer::StackInteger;
    use crate::Assign;

    #[test]
    fn check_assign() {
        let mut i = StackInteger::from(-1i32);
        assert_eq!(*i.borrow(), -1);
        let other = StackInteger::from(2i32);
        i.assign(&other);
        assert_eq!(*i.borrow(), 2);
        i.assign(6u8);
        assert_eq!(*i.borrow(), 6);
        i.assign(-6i8);
        assert_eq!(*i.borrow(), -6);
        i.assign(other);
        assert_eq!(*i.borrow(), 2);
        i.assign(6u16);
        assert_eq!(*i.borrow(), 6);
        i.assign(-6i16);
        assert_eq!(*i.borrow(), -6);
        i.assign(6u32);
        assert_eq!(*i.borrow(), 6);
        i.assign(-6i32);
        assert_eq!(*i.borrow(), -6);
        i.assign(0xf_0000_0006u64);
        assert_eq!(*i.borrow(), 0xf_0000_0006u64);
        i.assign(-0xf_0000_0006i64);
        assert_eq!(*i.borrow(), -0xf_0000_0006i64);
        i.assign(6u128 << 64 | 7u128);
        assert_eq!(*i.borrow(), 6u128 << 64 | 7u128);
        i.assign(-6i128 << 64 | 7i128);
        assert_eq!(*i.borrow(), -6i128 << 64 | 7i128);
        i.assign(6usize);
        assert_eq!(*i.borrow(), 6);
        i.assign(-6isize);
        assert_eq!(*i.borrow(), -6);
        i.assign(0u32);
        assert_eq!(*i.borrow(), 0);
    }
}
