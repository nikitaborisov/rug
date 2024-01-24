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

#![allow(dead_code)]

use az::{Az, UnwrappedAs, WrappingCast};
use core::ffi::c_char;
use core::mem;
use core::mem::MaybeUninit;
use core::ptr;
use core::slice;
use core::str;
use libc::size_t;

pub trait NegAbs {
    type Abs;
    fn neg_abs(self) -> (bool, Self::Abs);
}

macro_rules! neg_abs {
    ($I:ty; $U:ty) => {
        impl NegAbs for $I {
            type Abs = $U;
            #[inline]
            fn neg_abs(self) -> (bool, $U) {
                if self < 0 {
                    (true, self.wrapping_neg().wrapping_cast())
                } else {
                    (false, self.wrapping_cast())
                }
            }
        }

        impl NegAbs for $U {
            type Abs = $U;
            #[inline]
            fn neg_abs(self) -> (bool, $U) {
                (false, self)
            }
        }
    };
}

neg_abs! { i8; u8 }
neg_abs! { i16; u16 }
neg_abs! { i32; u32 }
neg_abs! { i64; u64 }
neg_abs! { i128; u128 }
neg_abs! { isize; usize }

pub fn trunc_f64_to_f32(f: f64) -> f32 {
    // f as f32 might round away from zero, so we need to clear
    // the least significant bits of f.
    //   * If f is a nan, we do NOT want to clear any mantissa bits,
    //     as this may change f into +/- infinity.
    //   * If f is +/- infinity, the bits are already zero, so the
    //     masking has no effect.
    //   * If f is subnormal, f as f32 will be zero anyway.
    //
    // When f is normal but would be subnormal as f32, we need to clear more
    // bits. Let x be exponent minus minimum f32 normal exponent, that is x =
    // biased f64 exponent - 1023 + 126. Then
    //   * If x >= 0, then truncate 53 - 24 bits.
    //   * If x <= -24, then truncate at least 53 bits, but there are 52
    //     non-implicit bits, so return 0.
    //   * If -23 <= x <= -1, then truncate 53 - 24 - x bits.
    if f.is_nan() {
        f as f32
    } else {
        let u = f.to_bits();
        let biased_exp = (u >> 52).az::<u32>() & 0x7FF;
        let trunc_count = if biased_exp >= 1023 - 126 {
            // normally f64 has 29 more significant bits than f32
            29
        } else if biased_exp <= 1023 - 126 - 24 {
            // Do not try to keep sign bit, as that is not consistent with
            // gmp::mpz_get_d
            return 0.0;
        } else {
            // 1023 - 126 - 23 <= biased_exp <= 1023 - 126 - 1
            // 52 >= trunc_count >= 30
            29 + 1023 - 126 - biased_exp
        };
        // f64 has 29 more significant bits than f32.
        let trunc_u = u & (!0 << trunc_count);
        let trunc_f = f64::from_bits(trunc_u);
        trunc_f as f32
    }
}

fn lcase(byte: u8) -> u8 {
    match byte {
        b'A'..=b'Z' => byte - b'A' + b'a',
        _ => byte,
    }
}

pub fn trim_start(bytes: &[u8]) -> &[u8] {
    for (start, &b) in bytes.iter().enumerate() {
        match b {
            b' ' | b'\t' | b'\n' | 0x0b | 0x0c | 0x0d => {}
            _ => return &bytes[start..],
        }
    }
    &[]
}

pub fn trim_end(bytes: &[u8]) -> &[u8] {
    for (end, &b) in bytes.iter().enumerate().rev() {
        match b {
            b' ' | b'\t' | b'\n' | 0x0b | 0x0c | 0x0d => {}
            _ => return &bytes[..=end],
        }
    }
    &[]
}

// If bytes starts with a match to one of patterns, return bytes with
// the match skipped. Only bytes is converted to lcase.
pub fn skip_lcase_match<'a>(bytes: &'a [u8], patterns: &[&[u8]]) -> Option<&'a [u8]> {
    'next_pattern: for pattern in patterns {
        if bytes.len() < pattern.len() {
            continue 'next_pattern;
        }
        for (&b, &p) in bytes.iter().zip(pattern.iter()) {
            if lcase(b) != p {
                continue 'next_pattern;
            }
        }
        return Some(&bytes[pattern.len()..]);
    }
    None
}

// If bytes starts with '(' and has a matching ')', returns the
// contents and the remainder.
pub fn matched_brackets(bytes: &[u8]) -> Option<(&[u8], &[u8])> {
    let mut iter = bytes.iter().enumerate();
    match iter.next() {
        Some((_, &b'(')) => {}
        _ => return None,
    }
    let mut level = 1;
    for (i, &b) in iter {
        match b {
            b'(' => level += 1,
            b')' => {
                level -= 1;
                if level == 0 {
                    return Some((&bytes[1..i], &bytes[i + 1..]));
                }
            }
            _ => {}
        }
    }
    None
}

pub fn find_outside_brackets(bytes: &[u8], pattern: u8) -> Option<usize> {
    let mut level = 0;
    for (i, &b) in bytes.iter().enumerate() {
        match b {
            b'(' => level += 1,
            b')' if level > 0 => level -= 1,
            _ if level == 0 && b == pattern => return Some(i),
            _ => {}
        }
    }
    None
}

pub fn find_space_outside_brackets(bytes: &[u8]) -> Option<usize> {
    let mut level = 0;
    for (i, &b) in bytes.iter().enumerate() {
        match b {
            b'(' => level += 1,
            b')' if level > 0 => level -= 1,
            b' ' | b'\t' | b'\n' | 0x0b | 0x0c | 0x0d if level == 0 => {
                return Some(i);
            }
            _ => {}
        }
    }
    None
}

pub enum StringLike {
    String(String),
    Malloc {
        ptr: *mut c_char,
        cap: size_t,
        len: size_t,
    },
}

impl StringLike {
    pub fn new_string() -> Self {
        StringLike::String(String::new())
    }

    pub fn unwrap_string(mut self) -> String {
        match &mut self {
            StringLike::String(s) => mem::replace(s, String::new()),
            StringLike::Malloc { .. } => unreachable!("unexpected variant"),
        }
    }

    pub fn new_malloc() -> Self {
        StringLike::Malloc {
            ptr: ptr::null_mut(),
            cap: 0,
            len: 0,
        }
    }

    pub fn push_str(&mut self, s: &str) {
        if let StringLike::String(st) = self {
            st.push_str(s);
            return;
        }
        self.reserve(s.len());
        let StringLike::Malloc { ptr, cap: _, len } = self else {
            unreachable!();
        };
        unsafe {
            ptr.cast::<u8>()
                .offset((*len).unwrapped_as())
                .copy_from_nonoverlapping(s.as_ptr(), s.len());
            self.increase_len(s.len());
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            StringLike::String(s) => s,
            StringLike::Malloc { ptr, cap: _, len } => unsafe {
                let s = slice::from_raw_parts(ptr.cast::<u8>(), (*len).unwrapped_as());
                str::from_utf8_unchecked(s)
            },
        }
    }

    pub fn reserve(&mut self, additional: usize) {
        match self {
            StringLike::String(s) => {
                s.reserve(additional);
            }
            StringLike::Malloc { ptr, cap, len } => {
                let new_cap = len
                    .checked_add(additional.unwrapped_as::<size_t>())
                    .expect("overflow");
                if new_cap > *cap {
                    let new_ptr = unsafe { libc::realloc(ptr.cast(), new_cap) };
                    *ptr = new_ptr.cast::<c_char>();
                    *cap = new_cap;
                }
            }
        }
    }

    // SAFETY: must keep
    pub unsafe fn reserved_space(&mut self) -> &mut [MaybeUninit<u8>] {
        match self {
            StringLike::String(s) => {
                let mu_ptr = s.as_mut_ptr().cast::<MaybeUninit<u8>>();
                unsafe {
                    slice::from_raw_parts_mut(
                        mu_ptr.offset(s.len().unwrapped_as()),
                        s.capacity() - s.len(),
                    )
                }
            }
            StringLike::Malloc { ptr, cap, len } => {
                let mu_ptr = (*ptr).cast::<MaybeUninit<u8>>();
                unsafe {
                    slice::from_raw_parts_mut(
                        mu_ptr.offset((*len).unwrapped_as()),
                        (*cap - *len).unwrapped_as(),
                    )
                }
            }
        }
    }

    // SAFETY: there should be enough capacity to increase length by increment
    pub unsafe fn increase_len(&mut self, increment: usize) {
        match self {
            StringLike::String(s) => unsafe {
                let new_len = s.len().checked_add(increment).expect("overflow");
                s.as_mut_vec().set_len(new_len);
            },
            StringLike::Malloc {
                ptr: _,
                cap: _,
                len,
            } => {
                let new_len = len
                    .checked_add(increment.unwrapped_as::<size_t>())
                    .expect("overflow");
                *len = new_len;
            }
        }
    }
}

impl Drop for StringLike {
    fn drop(&mut self) {
        match self {
            StringLike::String(_) => {}
            StringLike::Malloc { ptr, .. } => unsafe {
                if *ptr != ptr::null_mut() {
                    libc::free(ptr.cast());
                }
            },
        }
    }
}
