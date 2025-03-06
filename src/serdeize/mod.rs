// Copyright © 2016–2025 Trevor Spiteri

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

#[cfg(all(feature = "serde", any(feature = "integer", feature = "float")))]
pub mod serde;

#[allow(dead_code)]
pub enum PrecReq {
    Zero,
    One,
    Two,
}

#[allow(dead_code)]
pub enum PrecVal {
    Zero,
    One(u32),
    Two((u32, u32)),
}

#[allow(dead_code)]
pub struct Data {
    pub prec: PrecVal,
    pub radix: i32,
    pub value: String,
}

#[allow(dead_code)]
pub fn check_range<T>(name: &'static str, val: T, min: T, max: T) -> Result<(), String>
where
    T: Copy + core::fmt::Display + Ord,
{
    if val < min {
        Err(format!("{name} {val} less than minimum {min}"))
    } else if val > max {
        Err(format!("{name} {val} greater than maximum {max}"))
    } else {
        Ok(())
    }
}

#[cfg(test)]
pub mod test {
    #[cfg(all(feature = "serde", any(feature = "integer", feature = "float")))]
    pub use super::serde::test::*;
}
