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

use crate::complex::BorrowComplex;
use crate::ext::xmpfr;
use crate::float;
use crate::float::ToMini;
use crate::{Assign, Complex};
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
use gmp_mpfr_sys::mpc::mpc_t;
use gmp_mpfr_sys::mpfr::{mpfr_t, prec_t};

const LIMBS_IN_SMALL: usize = (128 / gmp::LIMB_BITS) as usize;
type Limbs = [MaybeUninit<limb_t>; LIMBS_IN_SMALL];

/**
A small complex number that does not require any memory allocation.

This can be useful when you have real and imaginary numbers that are primitive
integers or floats and you need a reference to a [`Complex`].

The `MiniComplex` will have a precision according to the types of the
primitives used to set its real and imaginary parts. Note that if different
types are used to set the parts, the parts can have different precisions.

  * [`i8`], [`u8`]: the part will have eight bits of precision.
  * [`i16`], [`u16`]: the part will have 16 bits of precision.
  * [`i32`], [`u32`]: the part will have 32 bits of precision.
  * [`i64`], [`u64`]: the part will have 64 bits of precision.
  * [`i128`], [`u128`]: the part will have 128 bits of precision.
  * [`isize`], [`usize`]: the part will have 32 or 64 bits of precision,
    depending on the platform.
  * [`f32`]: the part will have 24 bits of precision.
  * [`f64`]: the part will have 53 bits of precision.
  * [`Special`][crate::float::Special]: the part will have the [minimum possible
    precision][crate::float::prec_min].

The [`borrow`][Self::borrow] method returns an object that can be coerced to a
[`Complex`], as it implements
<code>[Deref]\<[Target][Deref::Target] = [Complex]></code>.

# Examples

```rust
use rug::complex::MiniComplex;
use rug::Complex;
// `a` requires a heap allocation
let mut a = Complex::with_val(53, (1, 2));
// `b` can reside on the stack
let b = MiniComplex::from((-10f64, -20.5f64));
a += &*b.borrow();
assert_eq!(*a.real(), -9);
assert_eq!(*a.imag(), -18.5);
```
*/
#[derive(Clone, Copy)]
pub struct MiniComplex {
    inner: mpc_t,
    // real part is first in limbs if inner.re.d <= inner.im.d
    first_limbs: Limbs,
    last_limbs: Limbs,
}

static_assert!(mem::size_of::<Limbs>() == 16);

// SAFETY: mpc_t is thread safe as guaranteed by the MPC library.
unsafe impl Send for MiniComplex {}
unsafe impl Sync for MiniComplex {}

impl Default for MiniComplex {
    #[inline]
    fn default() -> Self {
        MiniComplex::new()
    }
}

impl Display for MiniComplex {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&*self.borrow(), f)
    }
}

impl Debug for MiniComplex {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Debug::fmt(&*self.borrow(), f)
    }
}

impl LowerExp for MiniComplex {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        LowerExp::fmt(&*self.borrow(), f)
    }
}

impl UpperExp for MiniComplex {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        UpperExp::fmt(&*self.borrow(), f)
    }
}

impl Binary for MiniComplex {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Binary::fmt(&*self.borrow(), f)
    }
}

impl Octal for MiniComplex {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Octal::fmt(&*self.borrow(), f)
    }
}

impl LowerHex for MiniComplex {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        LowerHex::fmt(&*self.borrow(), f)
    }
}

impl UpperHex for MiniComplex {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        UpperHex::fmt(&*self.borrow(), f)
    }
}

impl MiniComplex {
    /// Creates a [`MiniComplex`] with value 0 and the [minimum possible
    /// precision][crate::float::prec_min].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::complex::MiniComplex;
    /// let c = MiniComplex::new();
    /// // Borrow c as if it were Complex.
    /// assert_eq!(*c.borrow(), 0);
    /// ```
    #[inline]
    pub const fn new() -> Self {
        MiniComplex {
            inner: mpc_t {
                re: mpfr_t {
                    prec: float::prec_min() as prec_t,
                    sign: 1,
                    exp: xmpfr::EXP_ZERO,
                    d: NonNull::dangling(),
                },
                im: mpfr_t {
                    prec: float::prec_min() as prec_t,
                    sign: 1,
                    exp: xmpfr::EXP_ZERO,
                    d: NonNull::dangling(),
                },
            },
            first_limbs: small_limbs![],
            last_limbs: small_limbs![],
        }
    }

    /// Returns a mutable reference to a [`Complex`] number for simple
    /// operations that do not need to change the precision of the real or
    /// imaginary part.
    ///
    /// # Safety
    ///
    /// It is undefined behavior to modify the precision of the referenced
    /// [`Complex`] number or to swap it with another number.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::complex::MiniComplex;
    /// let mut c = MiniComplex::from((1.0f32, 3.0f32));
    /// // rotation does not change the precision
    /// unsafe {
    ///     c.as_nonreallocating_complex().mul_i_mut(false);
    /// }
    /// assert_eq!(*c.borrow(), (-3.0, 1.0));
    /// ```
    #[inline]
    pub unsafe fn as_nonreallocating_complex(&mut self) -> &mut Complex {
        // Update re.d and im.d to point to limbs.
        let first = NonNull::<[MaybeUninit<limb_t>]>::from(&self.first_limbs[..]).cast();
        let last = NonNull::<[MaybeUninit<limb_t>]>::from(&self.last_limbs[..]).cast();
        let (re_d, im_d) = if self.re_is_first() {
            (first, last)
        } else {
            (last, first)
        };
        self.inner.re.d = re_d;
        self.inner.im.d = im_d;
        let ptr = cast_ptr_mut!(&mut self.inner, Complex);
        // SAFETY: since inner.re.d and inner.im.d point to the limbs, it is
        // in a consistent state.
        unsafe { &mut *ptr }
    }

    /// Borrows the complex number.
    ///
    /// The returned object implements
    /// <code>[Deref]\<[Target][Deref::Target] = [Complex]></code>.
    ///
    /// The borrow lasts until the returned object exits scope. Multiple borrows
    /// can be taken at the same time.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rug::complex::MiniComplex;
    /// use rug::Complex;
    /// let c = MiniComplex::from((-13f64, 5.5f64));
    /// let b = c.borrow();
    /// let conj = Complex::with_val(53, b.conj_ref());
    /// assert_eq!(*conj.real(), -13);
    /// assert_eq!(*conj.imag(), -5.5);
    /// ```
    #[inline]
    pub fn borrow(&self) -> impl Deref<Target = Complex> + '_ {
        let first = NonNull::<[MaybeUninit<limb_t>]>::from(&self.first_limbs[..]).cast();
        let last = NonNull::<[MaybeUninit<limb_t>]>::from(&self.last_limbs[..]).cast();
        let (re_d, im_d) = if self.re_is_first() {
            (first, last)
        } else {
            (last, first)
        };
        // SAFETY: Since re_d and im_d point to the limbs, the mpc_t is in a
        // consistent state. Also, the lifetime of the BorrowComplex is the
        // lifetime of self, which covers the limbs.
        unsafe {
            BorrowComplex::from_raw(mpc_t {
                re: mpfr_t {
                    prec: self.inner.re.prec,
                    sign: self.inner.re.sign,
                    exp: self.inner.re.exp,
                    d: re_d,
                },
                im: mpfr_t {
                    prec: self.inner.im.prec,
                    sign: self.inner.im.sign,
                    exp: self.inner.im.exp,
                    d: im_d,
                },
            })
        }
    }

    #[inline]
    fn re_is_first(&self) -> bool {
        self.inner.re.d <= self.inner.im.d
    }
}

impl<Re: ToMini> Assign<Re> for MiniComplex {
    fn assign(&mut self, src: Re) {
        unsafe {
            src.copy(&mut self.inner.re, &mut self.first_limbs);
            xmpfr::custom_zero(
                &mut self.inner.im,
                cast_ptr_mut!(self.last_limbs.as_mut_ptr(), limb_t),
                self.inner.re.prec,
            );
        }
    }
}

impl<Re: ToMini> From<Re> for MiniComplex {
    fn from(src: Re) -> Self {
        let mut inner = mpc_t {
            re: mpfr_t {
                prec: 0,
                sign: 0,
                exp: 0,
                d: NonNull::dangling(),
            },
            im: mpfr_t {
                prec: 0,
                sign: 0,
                exp: 0,
                d: NonNull::dangling(),
            },
        };
        let mut re_limbs = small_limbs![];
        let mut im_limbs = small_limbs![];
        unsafe {
            src.copy(&mut inner.re, &mut re_limbs);
            xmpfr::custom_zero(
                &mut inner.im,
                cast_ptr_mut!(im_limbs.as_mut_ptr(), limb_t),
                inner.re.prec,
            );
        }
        // order of limbs is important as inner.num.d != inner.den.d
        if re_limbs.as_ptr() <= im_limbs.as_ptr() {
            MiniComplex {
                inner,
                first_limbs: re_limbs,
                last_limbs: im_limbs,
            }
        } else {
            MiniComplex {
                inner,
                first_limbs: im_limbs,
                last_limbs: re_limbs,
            }
        }
    }
}

impl<Re: ToMini, Im: ToMini> Assign<(Re, Im)> for MiniComplex {
    fn assign(&mut self, src: (Re, Im)) {
        unsafe {
            src.0.copy(&mut self.inner.re, &mut self.first_limbs);
            src.1.copy(&mut self.inner.im, &mut self.last_limbs);
        }
    }
}

impl<Re: ToMini, Im: ToMini> From<(Re, Im)> for MiniComplex {
    fn from(src: (Re, Im)) -> Self {
        let mut inner = mpc_t {
            re: mpfr_t {
                prec: 0,
                sign: 0,
                exp: 0,
                d: NonNull::dangling(),
            },
            im: mpfr_t {
                prec: 0,
                sign: 0,
                exp: 0,
                d: NonNull::dangling(),
            },
        };
        let mut re_limbs = small_limbs![];
        let mut im_limbs = small_limbs![];
        unsafe {
            src.0.copy(&mut inner.re, &mut re_limbs);
            src.1.copy(&mut inner.im, &mut im_limbs);
        }
        // order of limbs is important as inner.num.d != inner.den.d
        if re_limbs.as_ptr() <= im_limbs.as_ptr() {
            MiniComplex {
                inner,
                first_limbs: re_limbs,
                last_limbs: im_limbs,
            }
        } else {
            MiniComplex {
                inner,
                first_limbs: im_limbs,
                last_limbs: re_limbs,
            }
        }
    }
}

impl Assign<&Self> for MiniComplex {
    #[inline]
    fn assign(&mut self, other: &Self) {
        self.clone_from(other);
    }
}

impl Assign for MiniComplex {
    #[inline]
    fn assign(&mut self, other: Self) {
        *self = other;
    }
}

#[cfg(test)]
mod tests {
    use crate::complex::MiniComplex;
    use crate::float;
    use crate::float::FreeCache;
    use crate::Assign;

    #[test]
    fn check_assign() {
        let mut c = MiniComplex::from((1.0, 2.0));
        assert_eq!(*c.borrow(), (1.0, 2.0));
        c.assign(3.0);
        assert_eq!(*c.borrow(), (3.0, 0.0));
        let other = MiniComplex::from((4.0, 5.0));
        c.assign(&other);
        assert_eq!(*c.borrow(), (4.0, 5.0));
        c.assign((6.0, 7.0));
        assert_eq!(*c.borrow(), (6.0, 7.0));
        c.assign(other);
        assert_eq!(*c.borrow(), (4.0, 5.0));

        float::free_cache(FreeCache::All);
    }

    fn swapped_parts(small: &MiniComplex) -> bool {
        unsafe {
            let borrow = small.borrow();
            let re = (*borrow.real().as_raw()).d;
            let im = (*borrow.imag().as_raw()).d;
            re > im
        }
    }

    #[test]
    fn check_swapped_parts() {
        let mut c = MiniComplex::from((1, 2));
        assert_eq!(*c.borrow(), (1, 2));
        assert_eq!(*c.clone().borrow(), c);
        let mut orig_swapped_parts = swapped_parts(&c);
        unsafe {
            c.as_nonreallocating_complex().mul_i_mut(false);
        }
        assert_eq!(*c.borrow(), (-2, 1));
        assert_eq!(*c.clone().borrow(), c);
        assert!(swapped_parts(&c) != orig_swapped_parts);

        c.assign(12);
        assert_eq!(*c.borrow(), 12);
        assert_eq!(*c.clone().borrow(), c);
        orig_swapped_parts = swapped_parts(&c);
        unsafe {
            c.as_nonreallocating_complex().mul_i_mut(false);
        }
        assert_eq!(*c.borrow(), (0, 12));
        assert_eq!(*c.clone().borrow(), c);
        assert!(swapped_parts(&c) != orig_swapped_parts);

        c.assign((4, 5));
        assert_eq!(*c.borrow(), (4, 5));
        assert_eq!(*c.clone().borrow(), c);
        orig_swapped_parts = swapped_parts(&c);
        unsafe {
            c.as_nonreallocating_complex().mul_i_mut(false);
        }
        assert_eq!(*c.borrow(), (-5, 4));
        assert_eq!(*c.clone().borrow(), c);
        assert!(swapped_parts(&c) != orig_swapped_parts);
    }
}
