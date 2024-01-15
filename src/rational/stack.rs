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

#![allow(deprecated)]

use crate::ext::xmpq;
use crate::integer::ToStack;
use crate::{Assign, Rational};
use az::Cast;
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
use gmp_mpfr_sys::gmp::{limb_t, mpq_t, mpz_t};

const LIMBS_IN_SMALL: usize = (128 / gmp::LIMB_BITS) as usize;
type Limbs = [MaybeUninit<limb_t>; LIMBS_IN_SMALL];

/**
A small rational number that does not require any memory allocation.

This can be useful when you have a numerator and denominator that are primitive
integer-types such as [`i64`] or [`u8`], and you need a reference to a
[`Rational`].

Although no allocation is required, setting the value of a `StackRational` does
require some computation, as the numerator and denominator need to be
canonicalized.

The [`borrow`][Self::borrow] method returns an object that can be coerced to a
[`Rational`], as it implements
<code>[Deref]\<[Target][Deref::Target] = [Rational]></code>.

# Examples

```rust
use rug::rational::StackRational;
use rug::Rational;
// `a` requires a heap allocation
let mut a = Rational::from((100, 13));
// `b` can reside on the stack
let b = StackRational::from((-100, 21));
a /= &*b.borrow();
assert_eq!(*a.numer(), -21);
assert_eq!(*a.denom(), 13);
```
*/
#[derive(Clone)]
pub struct StackRational {
    inner: RefCell<mpq_t>,
    // numerator is first in limbs if inner.num.d <= inner.den.d
    first_limbs: Limbs,
    last_limbs: Limbs,
}

static_assert!(mem::size_of::<Limbs>() == 16);

// Safety: StackRational cannot be Sync because it contains a RefCell.
// But StackRational can be Send, just like RefCell.
unsafe impl Send for StackRational {}

impl Default for StackRational {
    #[inline]
    fn default() -> Self {
        StackRational::new()
    }
}

impl Display for StackRational {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&*self.borrow(), f)
    }
}

impl Debug for StackRational {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Debug::fmt(&*self.borrow(), f)
    }
}

impl Binary for StackRational {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Binary::fmt(&*self.borrow(), f)
    }
}

impl Octal for StackRational {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Octal::fmt(&*self.borrow(), f)
    }
}

impl LowerHex for StackRational {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        LowerHex::fmt(&*self.borrow(), f)
    }
}

impl UpperHex for StackRational {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        UpperHex::fmt(&*self.borrow(), f)
    }
}

impl StackRational {
    /// Creates a [`StackRational`] with value 0.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::rational::StackRational;
    /// let r = StackRational::new();
    /// let b = r.borrow();
    /// // Use b as if it were Rational.
    /// assert_eq!(*b.numer(), 0);
    /// assert_eq!(*b.denom(), 1);
    /// ```
    #[inline]
    pub const fn new() -> Self {
        StackRational {
            inner: RefCell::new(mpq_t {
                num: mpz_t {
                    alloc: LIMBS_IN_SMALL as c_int,
                    size: 0,
                    d: NonNull::dangling(),
                },
                den: mpz_t {
                    alloc: LIMBS_IN_SMALL as c_int,
                    size: 1,
                    d: NonNull::dangling(),
                },
            }),
            first_limbs: small_limbs![0],
            last_limbs: small_limbs![1],
        }
    }

    /// Returns a mutable reference to a [`Rational`] number for simple
    /// operations that do not need to allocate more space for the numerator or
    /// denominator.
    ///
    /// # Safety
    ///
    /// It is undefined behavior to perform operations that reallocate the
    /// internal data of the referenced [`Rational`] number or to swap it with
    /// another number, although it is allowed to swap the numerator and
    /// denominator allocations, such as in the reciprocal operation
    /// [`recip_mut`].
    ///
    /// Some GMP functions swap the allocations of their target operands;
    /// calling such functions with the mutable reference returned by this
    /// method can lead to undefined behavior.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::rational::StackRational;
    /// let mut r = StackRational::from((-15i32, 47i32));
    /// let (num_capacity, den_capacity) = {
    ///     let b = r.borrow();
    ///     (b.numer().capacity(), b.denom().capacity())
    /// };
    /// // reciprocating this will not require reallocations
    /// unsafe {
    ///     r.as_nonreallocating_rational().recip_mut();
    /// }
    /// let after = r.borrow();
    /// assert_eq!(*after, StackRational::from((-47, 15)));
    /// assert_eq!(after.numer().capacity(), num_capacity);
    /// assert_eq!(after.denom().capacity(), den_capacity);
    /// ```
    ///
    /// [`recip_mut`]: `Rational::recip_mut`
    #[inline]
    pub unsafe fn as_nonreallocating_rational(&mut self) -> &mut Rational {
        let num_is_first = self.num_is_first();
        // Since we borrow self mutably, it is statically guaranteed that no borrows exist.
        let inner = self.inner.get_mut();
        // Update num.d and den.d to point to limbs.
        let first = NonNull::<[MaybeUninit<limb_t>]>::from(&self.first_limbs[..]).cast();
        let last = NonNull::<[MaybeUninit<limb_t>]>::from(&self.last_limbs[..]).cast();
        (inner.num.d, inner.den.d) = if num_is_first {
            (first, last)
        } else {
            (last, first)
        };
        let ptr = cast_ptr_mut!(inner, Rational);
        // Safety: since inner.num.d and inner.den.d point to the limbs,
        // it is in a consistent state.
        unsafe { &mut *ptr }
    }

    /// Borrows the rational number.
    ///
    /// The returned object implements
    /// <code>[Deref]\<[Target][Deref::Target] = [Rational]></code>.
    ///
    /// The borrow lasts until the returned object exits scope. Multiple borrows
    /// can be taken at the same time.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::rational::StackRational;
    /// use rug::Rational;
    /// let r = StackRational::from((-13i32, 5i32));
    /// let b = r.borrow();
    /// let abs_ref = Rational::from(b.abs_ref());
    /// assert_eq!(*abs_ref.numer(), 13);
    /// assert_eq!(*abs_ref.denom(), 5);
    /// ```
    #[inline]
    pub fn borrow(&self) -> impl Deref<Target = Rational> + '_ {
        let num_is_first = self.num_is_first();
        // Make sure num.d and den.d are pointing to limbs.
        match self.inner.try_borrow_mut() {
            Ok(mut inner) => {
                // Update num.d and den.d to point to limbs.
                let first = NonNull::<[MaybeUninit<limb_t>]>::from(&self.first_limbs[..]).cast();
                let last = NonNull::<[MaybeUninit<limb_t>]>::from(&self.last_limbs[..]).cast();
                (inner.num.d, inner.den.d) = if num_is_first {
                    (first, last)
                } else {
                    (last, first)
                };
            }
            Err(_) => {
                // Since there is another borrow, d must have already been updated.
                // Keep in mind that StackRational is !Sync.
            }
        }
        // There cannot be a mutable borrow anywhere else, so
        // self.inner.borrow() cannot fail owing to self.inner being borrowed
        // mutably. It can still fail if the reference count overflows, but that
        // is an extreme case of more than isize::MAX borrows, so there is no
        // need to document the panic.
        Ref::map(self.inner.borrow(), |inner| {
            let ptr = cast_ptr!(inner, Rational);
            // Safety: since inner.num.d and inner.den.d point to limbs, it is
            // in a consistent state.
            unsafe { &*ptr }
        })
    }

    /// Creates a [`StackRational`] from a numerator and denominator, assuming
    /// they are in canonical form.
    ///
    /// # Safety
    ///
    /// This method leads to undefined behavior if `den` is zero or if `num` and
    /// `den` have common factors.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::rational::StackRational;
    /// let from_unsafe = unsafe { StackRational::from_canonical(-13, 10) };
    /// // from_safe is canonicalized to the same form as from_unsafe
    /// let from_safe = StackRational::from((130, -100));
    /// let unsafe_borrow = from_unsafe.borrow();
    /// let safe_borrow = from_safe.borrow();
    /// assert_eq!(unsafe_borrow.numer(), safe_borrow.numer());
    /// assert_eq!(unsafe_borrow.denom(), safe_borrow.denom());
    /// ```
    pub unsafe fn from_canonical<Num: ToStack, Den: ToStack>(num: Num, den: Den) -> Self {
        let mut num_size = 0;
        let mut den_size = 0;
        let mut num_limbs: Limbs = small_limbs![0];
        let mut den_limbs: Limbs = small_limbs![0];
        num.copy(&mut num_size, &mut num_limbs);
        den.copy(&mut den_size, &mut den_limbs);
        // since inner.num.d == inner.den.d, first_limbs are num_limbs
        StackRational {
            inner: RefCell::new(mpq_t {
                num: mpz_t {
                    alloc: LIMBS_IN_SMALL.cast(),
                    size: num_size,
                    d: NonNull::dangling(),
                },
                den: mpz_t {
                    alloc: LIMBS_IN_SMALL.cast(),
                    size: den_size,
                    d: NonNull::dangling(),
                },
            }),
            first_limbs: num_limbs,
            last_limbs: den_limbs,
        }
    }

    /// Assigns a numerator and denominator to a [`StackRational`], assuming
    /// they are in canonical form.
    ///
    /// # Safety
    ///
    /// This method leads to undefined behavior if `den` is zero or negative, or
    /// if `num` and `den` have common factors.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::rational::StackRational;
    /// use rug::Assign;
    /// let mut a = StackRational::new();
    /// unsafe {
    ///     a.assign_canonical(-13, 10);
    /// }
    /// // b is canonicalized to the same form as a
    /// let mut b = StackRational::new();
    /// b.assign((130, -100));
    /// let a_borrow = a.borrow();
    /// let b_borrow = b.borrow();
    /// assert_eq!(a_borrow.numer(), b_borrow.numer());
    /// assert_eq!(a_borrow.denom(), b_borrow.denom());
    /// ```
    pub unsafe fn assign_canonical<Num: ToStack, Den: ToStack>(&mut self, num: Num, den: Den) {
        let (num_limbs, den_limbs) = if self.num_is_first() {
            (&mut self.first_limbs, &mut self.last_limbs)
        } else {
            (&mut self.last_limbs, &mut self.first_limbs)
        };
        num.copy(&mut self.inner.get_mut().num.size, num_limbs);
        den.copy(&mut self.inner.get_mut().den.size, den_limbs);
    }

    #[inline]
    // Safety: self is not Sync, so reading d does not cause a data race.
    fn num_is_first(&self) -> bool {
        // Safety: the reference is only used within the match, and no mutable
        // borrowing takes place.
        unsafe {
            match self.inner.try_borrow_unguarded() {
                Ok(q) => q.num.d <= q.den.d,
                Err(_) => unreachable!(),
            }
        }
    }
}

impl<Num: ToStack> Assign<Num> for StackRational {
    #[inline]
    fn assign(&mut self, src: Num) {
        let (num_limbs, den_limbs) = if self.num_is_first() {
            (&mut self.first_limbs, &mut self.last_limbs)
        } else {
            (&mut self.last_limbs, &mut self.first_limbs)
        };
        src.copy(&mut self.inner.get_mut().num.size, num_limbs);
        self.inner.get_mut().den.size = 1;
        den_limbs[0] = MaybeUninit::new(1);
    }
}

impl<Num: ToStack> From<Num> for StackRational {
    fn from(src: Num) -> Self {
        let mut num_size = 0;
        let mut num_limbs = small_limbs![0];
        src.copy(&mut num_size, &mut num_limbs);
        // since inner.num.d == inner.den.d, first_limbs are num_limbs
        StackRational {
            inner: RefCell::new(mpq_t {
                num: mpz_t {
                    alloc: LIMBS_IN_SMALL.cast(),
                    size: num_size,
                    d: NonNull::dangling(),
                },
                den: mpz_t {
                    alloc: LIMBS_IN_SMALL.cast(),
                    size: 1,
                    d: NonNull::dangling(),
                },
            }),
            first_limbs: num_limbs,
            last_limbs: small_limbs![1],
        }
    }
}

impl<Num: ToStack, Den: ToStack> Assign<(Num, Den)> for StackRational {
    fn assign(&mut self, src: (Num, Den)) {
        assert!(!src.1.is_zero(), "division by zero");
        {
            let (num_limbs, den_limbs) = if self.num_is_first() {
                (&mut self.first_limbs, &mut self.last_limbs)
            } else {
                (&mut self.last_limbs, &mut self.first_limbs)
            };
            src.0.copy(&mut self.inner.get_mut().num.size, num_limbs);
            src.1.copy(&mut self.inner.get_mut().den.size, den_limbs);
        }
        // Safety: canonicalization will never need to make a number larger.
        xmpq::canonicalize(unsafe { self.as_nonreallocating_rational() });
    }
}

impl<Num: ToStack, Den: ToStack> From<(Num, Den)> for StackRational {
    fn from(src: (Num, Den)) -> Self {
        assert!(!src.1.is_zero(), "division by zero");
        let mut inner = mpq_t {
            num: mpz_t {
                alloc: LIMBS_IN_SMALL.cast(),
                size: 0,
                d: NonNull::dangling(),
            },
            den: mpz_t {
                alloc: LIMBS_IN_SMALL.cast(),
                size: 0,
                d: NonNull::dangling(),
            },
        };
        let mut num_limbs: Limbs = small_limbs![0];
        let mut den_limbs: Limbs = small_limbs![0];
        src.0.copy(&mut inner.num.size, &mut num_limbs);
        src.1.copy(&mut inner.den.size, &mut den_limbs);
        inner.num.d = NonNull::<[MaybeUninit<limb_t>]>::from(&mut num_limbs[..]).cast();
        inner.den.d = NonNull::<[MaybeUninit<limb_t>]>::from(&mut den_limbs[..]).cast();
        unsafe {
            gmp::mpq_canonicalize(&mut inner);
        }
        // order of limbs is important as inner.num.d != inner.den.d
        if num_limbs.as_ptr() <= den_limbs.as_ptr() {
            StackRational {
                inner: RefCell::new(inner),
                first_limbs: num_limbs,
                last_limbs: den_limbs,
            }
        } else {
            StackRational {
                inner: RefCell::new(inner),
                first_limbs: den_limbs,
                last_limbs: num_limbs,
            }
        }
    }
}

impl Assign<&Self> for StackRational {
    #[inline]
    fn assign(&mut self, other: &Self) {
        self.clone_from(other);
    }
}

impl Assign for StackRational {
    #[inline]
    fn assign(&mut self, other: Self) {
        drop(mem::replace(self, other));
    }
}

#[cfg(test)]
mod tests {
    use crate::rational::StackRational;
    use crate::Assign;

    #[test]
    fn check_assign() {
        let mut r = StackRational::from((1, 2));
        assert_eq!(*r.borrow(), StackRational::from((1, 2)));
        r.assign(3);
        assert_eq!(*r.borrow(), 3);
        let other = StackRational::from((4, 5));
        r.assign(&other);
        assert_eq!(*r.borrow(), StackRational::from((4, 5)));
        r.assign((6, 7));
        assert_eq!(*r.borrow(), StackRational::from((6, 7)));
        r.assign(other);
        assert_eq!(*r.borrow(), StackRational::from((4, 5)));
    }

    fn swapped_parts(small: &StackRational) -> bool {
        unsafe {
            let borrow = small.borrow();
            let num = (*borrow.numer().as_raw()).d;
            let den = (*borrow.denom().as_raw()).d;
            num > den
        }
    }

    #[test]
    fn check_swapped_parts() {
        let mut r = StackRational::from((2, 3));
        assert_eq!(*r.borrow(), StackRational::from((2, 3)));
        assert_eq!(*r.clone().borrow(), r);
        let mut orig_swapped_parts = swapped_parts(&r);
        unsafe {
            r.as_nonreallocating_rational().recip_mut();
        }
        assert_eq!(*r.borrow(), StackRational::from((3, 2)));
        assert_eq!(*r.clone().borrow(), r);
        assert!(swapped_parts(&r) != orig_swapped_parts);

        unsafe {
            r.assign_canonical(5, 7);
        }
        assert_eq!(*r.borrow(), StackRational::from((5, 7)));
        assert_eq!(*r.clone().borrow(), r);
        orig_swapped_parts = swapped_parts(&r);
        unsafe {
            r.as_nonreallocating_rational().recip_mut();
        }
        assert_eq!(*r.borrow(), StackRational::from((7, 5)));
        assert_eq!(*r.clone().borrow(), r);
        assert!(swapped_parts(&r) != orig_swapped_parts);

        r.assign(2);
        assert_eq!(*r.borrow(), 2);
        assert_eq!(*r.clone().borrow(), r);
        orig_swapped_parts = swapped_parts(&r);
        unsafe {
            r.as_nonreallocating_rational().recip_mut();
        }
        assert_eq!(*r.borrow(), StackRational::from((1, 2)));
        assert_eq!(*r.clone().borrow(), r);
        assert!(swapped_parts(&r) != orig_swapped_parts);

        r.assign((3, -5));
        assert_eq!(*r.borrow(), StackRational::from((-3, 5)));
        assert_eq!(*r.clone().borrow(), r);
        orig_swapped_parts = swapped_parts(&r);
        unsafe {
            r.as_nonreallocating_rational().recip_mut();
        }
        assert_eq!(*r.borrow(), StackRational::from((-5, 3)));
        assert_eq!(*r.clone().borrow(), r);
        assert!(swapped_parts(&r) != orig_swapped_parts);
    }
}
