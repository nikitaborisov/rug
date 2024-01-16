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

use crate::ext::xmpfr;
use crate::ext::xmpfr::raw_round;
use crate::float::BorrowFloat;
use crate::float::{self, Round, Special};
use crate::misc::NegAbs;
use crate::{Assign, Float};
use az::{Az, UnwrappedCast, WrappingCast};
use core::fmt::{
    Binary, Debug, Display, Formatter, LowerExp, LowerHex, Octal, Result as FmtResult, UpperExp,
    UpperHex,
};
use core::mem;
use core::mem::MaybeUninit;
use core::ops::Deref;
use core::ptr::NonNull;
use gmp_mpfr_sys::gmp;
use gmp_mpfr_sys::gmp::limb_t;
use gmp_mpfr_sys::mpfr;
use gmp_mpfr_sys::mpfr::{mpfr_t, prec_t};

const LIMBS_IN_SMALL: usize = (128 / gmp::LIMB_BITS) as usize;
type Limbs = [MaybeUninit<limb_t>; LIMBS_IN_SMALL];

/**
A small float that does not require any memory allocation.

This can be useful when you have a primitive number type but need a reference to
a [`Float`]. The `MiniFloat` will have a precision according to the type of the
primitive used to set its value.

  * [`i8`], [`u8`]: the `MiniFloat` will have eight bits of precision.
  * [`i16`], [`u16`]: the `MiniFloat` will have 16 bits of precision.
  * [`i32`], [`u32`]: the `MiniFloat` will have 32 bits of precision.
  * [`i64`], [`u64`]: the `MiniFloat` will have 64 bits of precision.
  * [`i128`], [`u128`]: the `MiniFloat` will have 128 bits of precision.
  * [`isize`], [`usize`]: the `MiniFloat` will have 32 or 64 bits of precision,
    depending on the platform.
  * [`f32`]: the `MiniFloat` will have 24 bits of precision.
  * [`f64`]: the `MiniFloat` will have 53 bits of precision.
  * [`Special`]: the `MiniFloat` will have the [minimum possible
    precision][crate::float::prec_min].

The [`borrow`][Self::borrow] method returns an object that can be coerced to a
[`Float`], as it implements
<code>[Deref]\<[Target][Deref::Target] = [Float]></code>.

# Examples

```rust
use rug::float::MiniFloat;
use rug::Float;
// `a` requires a heap allocation, has 53-bit precision
let mut a = Float::with_val(53, 250);
// `b` can reside on the stack
let b = MiniFloat::from(-100f64);
a += &*b.borrow();
assert_eq!(a, 150);
// another computation:
a *= &*b.borrow();
assert_eq!(a, -15000);
```
*/
#[derive(Clone, Copy)]
pub struct MiniFloat {
    inner: mpfr_t,
    limbs: Limbs,
}

static_assert!(mem::size_of::<Limbs>() == 16);

// SAFETY: mpfr_t is thread safe as guaranteed by the MPFR library.
unsafe impl Send for MiniFloat {}
unsafe impl Sync for MiniFloat {}

impl Default for MiniFloat {
    #[inline]
    fn default() -> Self {
        MiniFloat::new()
    }
}

impl Display for MiniFloat {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&*self.borrow(), f)
    }
}

impl Debug for MiniFloat {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Debug::fmt(&*self.borrow(), f)
    }
}

impl LowerExp for MiniFloat {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        LowerExp::fmt(&*self.borrow(), f)
    }
}

impl UpperExp for MiniFloat {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        UpperExp::fmt(&*self.borrow(), f)
    }
}

impl Binary for MiniFloat {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Binary::fmt(&*self.borrow(), f)
    }
}

impl Octal for MiniFloat {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Octal::fmt(&*self.borrow(), f)
    }
}

impl LowerHex for MiniFloat {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        LowerHex::fmt(&*self.borrow(), f)
    }
}

impl UpperHex for MiniFloat {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        UpperHex::fmt(&*self.borrow(), f)
    }
}

impl MiniFloat {
    /// Creates a [`MiniFloat`] with value 0 and the [minimum possible
    /// precision][crate::float::prec_min].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::float::MiniFloat;
    /// let f = MiniFloat::new();
    /// // Borrow f as if it were Float.
    /// assert_eq!(*f.borrow(), 0);
    /// ```
    #[inline]
    pub const fn new() -> Self {
        MiniFloat {
            inner: mpfr_t {
                prec: float::prec_min() as prec_t,
                sign: 1,
                exp: xmpfr::EXP_ZERO,
                d: NonNull::dangling(),
            },
            limbs: small_limbs![],
        }
    }

    /// Returns a mutable reference to a [`Float`] for simple operations that do
    /// not need to change the precision of the number.
    ///
    /// # Safety
    ///
    /// It is undefined behavior modify the precision of the referenced
    /// [`Float`] or to swap it with another number.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::float::MiniFloat;
    /// let mut f = MiniFloat::from(1.0f32);
    /// // addition does not change the precision
    /// unsafe {
    ///     *f.as_nonreallocating_float() += 2.0;
    /// }
    /// assert_eq!(*f.borrow(), 3.0);
    /// ```
    #[inline]
    pub unsafe fn as_nonreallocating_float(&mut self) -> &mut Float {
        // Update d to point to limbs.
        self.inner.d = NonNull::<[MaybeUninit<limb_t>]>::from(&self.limbs[..]).cast();
        let ptr = cast_ptr_mut!(&mut self.inner, Float);
        // SAFETY: since inner.d points to the limbs, it is in a consistent state.
        unsafe { &mut *ptr }
    }

    /// Borrows the floating-point number.
    ///
    /// The returned object implements
    /// <code>[Deref]\<[Target][Deref::Target] = [Float]></code>.
    ///
    /// The borrow lasts until the returned object exits scope. Multiple borrows
    /// can be taken at the same time.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::float::MiniFloat;
    /// use rug::Float;
    /// let f = MiniFloat::from(-13i32);
    /// let b = f.borrow();
    /// let abs_ref = b.abs_ref();
    /// assert_eq!(Float::with_val(53, abs_ref), 13);
    /// ```
    #[inline]
    pub fn borrow(&self) -> impl Deref<Target = Float> + '_ {
        // SAFETY: Since d points to the limbs, the mpfr_t is in a consistent
        // state. Also, the lifetime of the BorrowFloat is the lifetime of self,
        // which covers the limbs.
        unsafe {
            BorrowFloat::from_raw(mpfr_t {
                prec: self.inner.prec,
                sign: self.inner.sign,
                exp: self.inner.exp,
                d: NonNull::<[MaybeUninit<limb_t>]>::from(&self.limbs[..]).cast(),
            })
        }
    }

    /// Borrows the floating-point number exclusively.
    ///
    /// This is similar to the [`borrow`][Self::borrow] method, but it requires
    /// exclusive access to the underlying [`MiniFloat`]; the returned reference
    /// can however be shared. The exclusive access is required to reduce the
    /// amount of housekeeping necessary, providing a more efficient operation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::float::MiniFloat;
    /// use rug::Float;
    /// let mut f = MiniFloat::from(-13i32);
    /// let b = f.borrow_excl();
    /// let abs_ref = b.abs_ref();
    /// assert_eq!(Float::with_val(53, abs_ref), 13);
    /// ```
    #[inline]
    pub fn borrow_excl(&mut self) -> &Float {
        // SAFETY: since the return is a const reference, there will be no reallocation
        unsafe { &*self.as_nonreallocating_float() }
    }
}

/// Types implementing this trait can be converted to [`MiniFloat`].
///
/// The following are implemented when `T` implements `ToMini`:
///   * <code>[Assign]\<T> for [MiniFloat]</code>
///   * <code>[From]\<T> for [MiniFloat]</code>
///
/// This trait is sealed and cannot be implemented for more types; it is
/// implemented for the integer types [`i8`], [`i16`], [`i32`], [`i64`],
/// [`i128`], [`isize`], [`u8`], [`u16`], [`u32`], [`u64`], [`u128`] and
/// [`usize`], and for the floating-point types [`f32`] and [`f64`].
pub trait ToMini: SealedToMini {}

pub trait SealedToMini: Copy {
    unsafe fn copy(self, inner: *mut mpfr_t, limbs: &mut Limbs);
}

macro_rules! unsafe_signed {
    ($($I:ty)*) => { $(
        impl ToMini for $I {}
        impl SealedToMini for $I {
            #[inline]
            unsafe fn copy(self, inner: *mut mpfr_t, limbs: &mut Limbs) {
                let (neg, abs) = self.neg_abs();
                unsafe{
                    abs.copy(inner, limbs);
                    if neg {
                        (*inner).sign = -1;
                    }
                }
            }
        }
    )* };
}

macro_rules! unsafe_unsigned_32 {
    ($U:ty, $bits:expr) => {
        impl ToMini for $U {}
        impl SealedToMini for $U {
            #[inline]
            unsafe fn copy(self, inner: *mut mpfr_t, limbs: &mut Limbs) {
                let limbs_ptr = cast_ptr_mut!(limbs.as_mut_ptr(), limb_t);
                if self == 0 {
                    unsafe {
                        xmpfr::custom_zero(inner, limbs_ptr, $bits);
                    }
                } else {
                    let leading = self.leading_zeros();
                    let limb_leading = leading + gmp::LIMB_BITS.az::<u32>() - $bits;
                    limbs[0] = MaybeUninit::new(limb_t::from(self) << limb_leading);
                    let exp = ($bits - leading).unwrapped_cast();
                    unsafe {
                        xmpfr::custom_regular(inner, limbs_ptr, exp, $bits);
                    }
                }
            }
        }
    };
}

unsafe_signed! { i8 i16 i32 i64 i128 isize }

unsafe_unsigned_32! { u8, 8 }
unsafe_unsigned_32! { u16, 16 }
unsafe_unsigned_32! { u32, 32 }

impl ToMini for u64 {}
impl SealedToMini for u64 {
    #[inline]
    unsafe fn copy(self, inner: *mut mpfr_t, limbs: &mut Limbs) {
        let limbs_ptr = cast_ptr_mut!(limbs.as_mut_ptr(), limb_t);
        if self == 0 {
            unsafe {
                xmpfr::custom_zero(inner, limbs_ptr, 64);
            }
        } else {
            let leading = self.leading_zeros();
            let sval = self << leading;
            #[cfg(gmp_limb_bits_64)]
            {
                limbs[0] = MaybeUninit::new(sval);
            }
            #[cfg(gmp_limb_bits_32)]
            {
                limbs[0] = MaybeUninit::new(sval.wrapping_cast());
                limbs[1] = MaybeUninit::new((sval >> 32).wrapping_cast());
            }
            let exp = (64 - leading).unwrapped_cast();
            unsafe {
                xmpfr::custom_regular(inner, limbs_ptr, exp, 64);
            }
        }
    }
}

impl ToMini for u128 {}
impl SealedToMini for u128 {
    #[inline]
    unsafe fn copy(self, inner: *mut mpfr_t, limbs: &mut Limbs) {
        let limbs_ptr = cast_ptr_mut!(limbs.as_mut_ptr(), limb_t);
        if self == 0 {
            unsafe {
                xmpfr::custom_zero(inner, limbs_ptr, 128);
            }
        } else {
            let leading = self.leading_zeros();
            let sval = self << leading;
            #[cfg(gmp_limb_bits_64)]
            {
                limbs[0] = MaybeUninit::new(sval.wrapping_cast());
                limbs[1] = MaybeUninit::new((sval >> 64).wrapping_cast());
            }
            #[cfg(gmp_limb_bits_32)]
            {
                limbs[0] = MaybeUninit::new(sval.wrapping_cast());
                limbs[1] = MaybeUninit::new((sval >> 32).wrapping_cast());
                limbs[2] = MaybeUninit::new((sval >> 64).wrapping_cast());
                limbs[3] = MaybeUninit::new((sval >> 96).wrapping_cast());
            }
            let exp = (128 - leading).unwrapped_cast();
            unsafe {
                xmpfr::custom_regular(inner, limbs_ptr, exp, 128);
            }
        }
    }
}

impl ToMini for usize {}
impl SealedToMini for usize {
    #[inline]
    unsafe fn copy(self, inner: *mut mpfr_t, limbs: &mut Limbs) {
        #[cfg(target_pointer_width = "32")]
        {
            let val = self.az::<u32>();
            unsafe {
                val.copy(inner, limbs);
            }
        }
        #[cfg(target_pointer_width = "64")]
        {
            let val = self.az::<u64>();
            unsafe {
                val.copy(inner, limbs);
            }
        }
    }
}

impl ToMini for f32 {}
impl SealedToMini for f32 {
    #[inline]
    unsafe fn copy(self, inner: *mut mpfr_t, limbs: &mut Limbs) {
        let limbs_ptr = cast_ptr_mut!(limbs.as_mut_ptr(), limb_t);
        let val = self.into();
        let rnd = raw_round(Round::Nearest);
        unsafe {
            xmpfr::custom_zero(inner, limbs_ptr, 24);
            mpfr::set_d(inner, val, rnd);
        }
        // retain sign in case of NaN
        if self.is_sign_negative() {
            unsafe {
                (*inner).sign = -1;
            }
        }
    }
}

impl ToMini for f64 {}
impl SealedToMini for f64 {
    #[inline]
    unsafe fn copy(self, inner: *mut mpfr_t, limbs: &mut Limbs) {
        let limbs_ptr = cast_ptr_mut!(limbs.as_mut_ptr(), limb_t);
        let rnd = raw_round(Round::Nearest);
        unsafe {
            xmpfr::custom_zero(inner, limbs_ptr, 53);
            mpfr::set_d(inner, self, rnd);
        }
        // retain sign in case of NaN
        if self.is_sign_negative() {
            unsafe {
                (*inner).sign = -1;
            }
        }
    }
}

impl ToMini for Special {}
impl SealedToMini for Special {
    #[inline]
    unsafe fn copy(self, inner: *mut mpfr_t, limbs: &mut Limbs) {
        let limbs_ptr = cast_ptr_mut!(limbs.as_mut_ptr(), limb_t);
        let prec = float::prec_min().az();
        unsafe {
            xmpfr::custom_special(inner, limbs_ptr, self, prec);
        }
    }
}

impl<T: ToMini> Assign<T> for MiniFloat {
    #[inline]
    fn assign(&mut self, src: T) {
        unsafe {
            src.copy(&mut self.inner, &mut self.limbs);
        }
    }
}

impl<T: ToMini> From<T> for MiniFloat {
    #[inline]
    fn from(src: T) -> Self {
        let mut inner = mpfr_t {
            prec: 0,
            sign: 0,
            exp: 0,
            d: NonNull::dangling(),
        };
        let mut limbs = small_limbs![];
        unsafe {
            src.copy(&mut inner, &mut limbs);
        }
        MiniFloat { inner, limbs }
    }
}

impl Assign<&Self> for MiniFloat {
    #[inline]
    fn assign(&mut self, other: &Self) {
        self.clone_from(other);
    }
}

impl Assign for MiniFloat {
    #[inline]
    fn assign(&mut self, other: Self) {
        *self = other;
    }
}

#[inline]
pub(crate) unsafe fn unchecked_get_unshifted_u8(small: &MiniFloat) -> u8 {
    debug_assert!(small.borrow().prec() >= 8);
    debug_assert!(small.borrow().is_normal());
    (unsafe { small.limbs[0].assume_init() } >> (gmp::LIMB_BITS - 8)).wrapping_cast()
}

#[inline]
pub(crate) unsafe fn unchecked_get_unshifted_u16(small: &MiniFloat) -> u16 {
    debug_assert!(small.borrow().prec() >= 16);
    debug_assert!(small.borrow().is_normal());
    (unsafe { small.limbs[0].assume_init() } >> (gmp::LIMB_BITS - 16)).wrapping_cast()
}

#[inline]
pub(crate) unsafe fn unchecked_get_unshifted_u32(small: &MiniFloat) -> u32 {
    debug_assert!(small.borrow().prec() >= 32);
    debug_assert!(small.borrow().is_normal());
    #[cfg(gmp_limb_bits_32)]
    {
        unsafe { small.limbs[0].assume_init() }
    }
    #[cfg(gmp_limb_bits_64)]
    {
        (unsafe { small.limbs[0].assume_init() } >> 32).wrapping_cast()
    }
}

#[inline]
pub(crate) unsafe fn unchecked_get_unshifted_u64(small: &MiniFloat) -> u64 {
    debug_assert!(small.borrow().prec() >= 64);
    debug_assert!(small.borrow().is_normal());
    #[cfg(gmp_limb_bits_32)]
    {
        u64::from(unsafe { small.limbs[0].assume_init() })
            | (u64::from(unsafe { small.limbs[1].assume_init() }) << 32)
    }
    #[cfg(gmp_limb_bits_64)]
    {
        unsafe { small.limbs[0].assume_init() }
    }
}

#[inline]
pub(crate) unsafe fn unchecked_get_unshifted_u128(small: &MiniFloat) -> u128 {
    debug_assert!(small.borrow().prec() >= 128);
    debug_assert!(small.borrow().is_normal());
    #[cfg(gmp_limb_bits_32)]
    {
        u128::from(unsafe { small.limbs[0].assume_init() })
            | (u128::from(unsafe { small.limbs[1].assume_init() }) << 32)
            | (u128::from(unsafe { small.limbs[2].assume_init() }) << 64)
            | (u128::from(unsafe { small.limbs[3].assume_init() }) << 96)
    }
    #[cfg(gmp_limb_bits_64)]
    {
        u128::from(unsafe { small.limbs[0].assume_init() })
            | (u128::from(unsafe { small.limbs[1].assume_init() }) << 64)
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use crate::float;
    use crate::float::{FreeCache, MiniFloat, Special};
    use crate::Assign;

    #[test]
    fn check_assign() {
        let mut f = MiniFloat::from(-1.0f32);
        assert_eq!(*f.borrow(), -1.0);
        f.assign(-2.0f64);
        assert_eq!(*f.borrow(), -2.0);
        let other = MiniFloat::from(4u8);
        f.assign(&other);
        assert_eq!(*f.borrow(), 4);
        f.assign(5i8);
        assert_eq!(*f.borrow(), 5);
        f.assign(other);
        assert_eq!(*f.borrow(), 4);
        f.assign(6u16);
        assert_eq!(*f.borrow(), 6);
        f.assign(-6i16);
        assert_eq!(*f.borrow(), -6);
        f.assign(6u32);
        assert_eq!(*f.borrow(), 6);
        f.assign(-6i32);
        assert_eq!(*f.borrow(), -6);
        f.assign(6u64);
        assert_eq!(*f.borrow(), 6);
        f.assign(-6i64);
        assert_eq!(*f.borrow(), -6);
        f.assign(6u128);
        assert_eq!(*f.borrow(), 6);
        f.assign(-6i128);
        assert_eq!(*f.borrow(), -6);
        f.assign(6usize);
        assert_eq!(*f.borrow(), 6);
        f.assign(-6isize);
        assert_eq!(*f.borrow(), -6);
        f.assign(0u32);
        assert_eq!(*f.borrow(), 0);
        f.assign(Special::Infinity);
        assert!(f.borrow().is_infinite() && f.borrow().is_sign_positive());
        f.assign(Special::NegZero);
        assert!(f.borrow().is_zero() && f.borrow().is_sign_negative());
        f.assign(Special::NegInfinity);
        assert!(f.borrow().is_infinite() && f.borrow().is_sign_negative());
        f.assign(Special::Zero);
        assert!(f.borrow().is_zero() && f.borrow().is_sign_positive());
        f.assign(Special::Nan);
        assert!(f.borrow().is_nan());

        float::free_cache(FreeCache::All);
    }
}
