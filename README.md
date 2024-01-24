<!-- Copyright © 2016–2024 Trevor Spiteri -->

<!-- Copying and distribution of this file, with or without modification, are
permitted in any medium without royalty provided the copyright notice and this
notice are preserved. This file is offered as-is, without any warranty. -->

# Arbitrary-precision numbers

Rug provides integers and floating-point numbers with arbitrary precision and
correct rounding:

  * [`Integer`] is a bignum integer with arbitrary precision,
  * [`Rational`] is a bignum rational number with arbitrary precision,
  * [`Float`] is a multi-precision floating-point number with correct rounding,
    and
  * [`Complex`] is a multi-precision complex number with correct rounding.

Rug is a high-level interface to the following [GNU] libraries:

  * [GMP] for integers and rational numbers,
  * [MPFR] for floating-point numbers, and
  * [MPC] for complex numbers.

Rug is free software: you can redistribute it and/or modify it under the terms
of the GNU Lesser General Public License as published by the Free Software
Foundation, either version 3 of the License, or (at your option) any later
version. See the full text of the [GNU LGPL] and [GNU GPL] for details.

## What’s new

### Version 1.24.0 news (unreleased)

  * [`Float`][flo-1-24] now implements
    <code>[AssignRound][assr-1-24]\<[bool][`bool`]></code>.
  * [`MiniRational`][mr-1-24] now implements
      * <code>[From][`From`]\<[MiniInteger][mi-1-24]></code>,
        <code>[Assign][ass-1-24]\<[MiniInteger][mi-1-24]></code>
  * [`MiniFloat`][mf-1-24] now implements
      * <code>[From][`From`]\<[bool][`bool`]></code>,
        <code>[Assign][ass-1-24]\<[bool][`bool`]></code>
  * [`MiniComplex`][mc-1-24] now implements
      * <code>[From][`From`]\<[MiniFloat][mf-1-24]></code>,
        <code>[Assign][ass-1-24]\<[MiniFloat][mf-1-24]></code>
      * <code>[From][`From`]\<[(][tuple][MiniFloat][mf-1-24][,][tuple] [MiniFloat][mf-1-24][)][tuple]></code>,
        <code>[Assign][ass-1-24]\<[(][tuple][MiniFloat][mf-1-24][,][tuple] [MiniFloat][mf-1-24][)][tuple]></code>
  * The following methods are now usable in const context:
      * <code>[Float][flo-1-24]::[as\_neg][flo-an-1-24]</code>,
        <code>[Float][flo-1-24]::[as\_abs][flo-aa-1-24]</code>
      * <code>[Complex][com-1-24]::[as\_neg][com-an-1-24]</code>,
        <code>[Complex][com-1-24]::[as\_conj][com-ac-1-24]</code>,
        <code>[Complex][com-1-24]::[as\_mul\_i][com-am-1-24]</code>
      * <code>[MiniInteger][mi-1-24]::[borrow][mi-b-1-24]</code>
      * <code>[MiniRational][mr-1-24]::[borrow][mr-b-1-24]</code>
      * <code>[MiniFloat][mf-1-24]::[borrow][mf-b-1-24]</code>
      * <code>[MiniComplex][mc-1-24]::[borrow][mc-b-1-24]</code>
  * The following methods were added to [`MiniInteger`][mi-1-24]:
      * [`const_from_bool`][mi-bool-1-24]
      * [`const_from_i8`][mi-i8-1-24], [`const_from_i16`][mi-i16-1-24],
        [`const_from_i32`][mi-i32-1-24], [`const_from_i64`][mi-i64-1-24],
        [`const_from_i128`][mi-i128-1-24], [`const_from_isize`][mi-isize-1-24]
      * [`const_from_u8`][mi-u8-1-24], [`const_from_u16`][mi-u16-1-24],
        [`const_from_u32`][mi-u32-1-24], [`const_from_u64`][mi-u64-1-24],
        [`const_from_u128`][mi-u128-1-24], [`const_from_usize`][mi-usize-1-24]
  * The following method was added to [`MiniRational`][mr-1-24]:
      * [`const_from_integer`][mr-cfi-1-24]
  * The following methods were added to [`MiniFloat`][mf-1-24]:
      * [`const_from_bool`][mf-bool-1-24]
      * [`const_from_i8`][mf-i8-1-24], [`const_from_i16`][mf-i16-1-24],
        [`const_from_i32`][mf-i32-1-24], [`const_from_i64`][mf-i64-1-24],
        [`const_from_i128`][mf-i128-1-24], [`const_from_isize`][mf-isize-1-24]
      * [`const_from_u8`][mf-u8-1-24], [`const_from_u16`][mf-u16-1-24],
        [`const_from_u32`][mf-u32-1-24], [`const_from_u64`][mf-u64-1-24],
        [`const_from_u128`][mf-u128-1-24], [`const_from_usize`][mf-usize-1-24]
      * [`const_from_f32`][mf-f32-1-24], [`const_from_f64`][mf-f64-1-24]
      * [`const_from_special`][mf-cfs-1-24]
  * The following methods were added to [`MiniComplex`][mc-1-24]:
      * [`const_from_real`][mc-cfr-1-24], [`const_from_parts`][mc-cfp-1-24]
  * The `std` [optional feature][feat-1-24] was added. The feature is enabled by
    default, and the crate will be compiled without the standard library
    (`no_std`) if the feature is disabled.
  * When the `num-traits` [experimental feature][feat-exp-1-24] is enabled,
    the following traits are implemented for [`Integer`][int-1-24]:
      * [`CheckedDiv`][nt-0-2-cd], [`CheckedRem`][nt-0-2-cr]
      * [`Euclid`][nt-0-2-e], [`CheckedEuclid`][nt-0-2-ce]

#### Compatibility notes

  * The following methods no longer set the [MPFR NaN flag][sys-mpfr-sn-1-6] if
    a NaN is encountered:
      * <code>[Float][flo-1-24]::[as\_neg][flo-an-1-24]</code>,
        <code>[Float][flo-1-24]::[as\_abs][flo-aa-1-24]</code>
      * <code>[Complex][com-1-24]::[as\_neg][com-an-1-24]</code>,
        <code>[Complex][com-1-24]::[as\_conj][com-ac-1-24]</code>,
        <code>[Complex][com-1-24]::[as\_mul\_i][com-am-1-24]</code>
  * Functionality that depends on `std` now requires the `std` feature to be
    enabled. While the `std` feature is enabled by default, it is now disabled
    if the crate’s default features are disabled in [*Cargo.toml*]. In this
    case, `std` must be added to the list of features to reenable the
    functionality.

[`From`]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html
[`bool`]: https://doc.rust-lang.org/nightly/core/primitive.bool.html
[ass-1-24]: https://tspiteri.gitlab.io/rug/dev/rug/trait.Assign.html
[assr-1-24]:  https://tspiteri.gitlab.io/rug/dev/rug/ops/trait.AssignRound.html
[com-1-24]: https://tspiteri.gitlab.io/rug/dev/rug/struct.Complex.html
[com-ac-1-24]: https://tspiteri.gitlab.io/rug/dev/rug/struct.Complex.html#method.as_conj
[com-am-1-24]: https://tspiteri.gitlab.io/rug/dev/rug/struct.Complex.html#method.as_mul_i
[com-an-1-24]: https://tspiteri.gitlab.io/rug/dev/rug/struct.Complex.html#method.as_neg
[feat-1-24]: https://tspiteri.gitlab.io/rug/dev/rug/index.html#optional-features
[feat-exp-1-24]: https://tspiteri.gitlab.io/rug/dev/rug/index.html#experimental-optional-features
[flo-1-24]: https://tspiteri.gitlab.io/rug/dev/rug/struct.Float.html
[flo-aa-1-24]: https://tspiteri.gitlab.io/rug/dev/rug/struct.Float.html#method.as_abs
[flo-an-1-24]: https://tspiteri.gitlab.io/rug/dev/rug/struct.Float.html#method.as_neg
[int-1-24]: https://tspiteri.gitlab.io/rug/dev/rug/struct.Integer.html
[mc-1-24]: https://tspiteri.gitlab.io/rug/dev/rug/complex/struct.MiniComplex.html
[mc-b-1-24]: https://tspiteri.gitlab.io/rug/dev/rug/complex/struct.MiniComplex.html#method.borrow
[mc-cfp-1-24]: https://tspiteri.gitlab.io/rug/dev/rug/complex/struct.MiniComplex.html#method.const_from_parts
[mc-cfr-1-24]: https://tspiteri.gitlab.io/rug/dev/rug/complex/struct.MiniComplex.html#method.const_from_real
[mf-1-24]: https://tspiteri.gitlab.io/rug/dev/rug/float/struct.MiniFloat.html
[mf-b-1-24]: https://tspiteri.gitlab.io/rug/dev/rug/float/struct.MiniFloat.html#method.borrow
[mf-bool-1-24]: https://tspiteri.gitlab.io/rug/dev/rug/float/struct.MiniFloat.html#method.const_from_bool
[mf-cfs-1-24]: https://tspiteri.gitlab.io/rug/dev/rug/float/struct.MiniFloat.html#method.const_from_special
[mf-f32-1-24]: https://tspiteri.gitlab.io/rug/dev/rug/float/struct.MiniFloat.html#method.const_from_f32
[mf-f64-1-24]: https://tspiteri.gitlab.io/rug/dev/rug/float/struct.MiniFloat.html#method.const_from_f64
[mf-i128-1-24]: https://tspiteri.gitlab.io/rug/dev/rug/float/struct.MiniFloat.html#method.const_from_i128
[mf-i16-1-24]: https://tspiteri.gitlab.io/rug/dev/rug/float/struct.MiniFloat.html#method.const_from_i16
[mf-i32-1-24]: https://tspiteri.gitlab.io/rug/dev/rug/float/struct.MiniFloat.html#method.const_from_i32
[mf-i64-1-24]: https://tspiteri.gitlab.io/rug/dev/rug/float/struct.MiniFloat.html#method.const_from_i64
[mf-i8-1-24]: https://tspiteri.gitlab.io/rug/dev/rug/float/struct.MiniFloat.html#method.const_from_i8
[mf-isize-1-24]: https://tspiteri.gitlab.io/rug/dev/rug/float/struct.MiniFloat.html#method.const_from_isize
[mf-u128-1-24]: https://tspiteri.gitlab.io/rug/dev/rug/float/struct.MiniFloat.html#method.const_from_u128
[mf-u16-1-24]: https://tspiteri.gitlab.io/rug/dev/rug/float/struct.MiniFloat.html#method.const_from_u16
[mf-u32-1-24]: https://tspiteri.gitlab.io/rug/dev/rug/float/struct.MiniFloat.html#method.const_from_u32
[mf-u64-1-24]: https://tspiteri.gitlab.io/rug/dev/rug/float/struct.MiniFloat.html#method.const_from_u64
[mf-u8-1-24]: https://tspiteri.gitlab.io/rug/dev/rug/float/struct.MiniFloat.html#method.const_from_u8
[mf-usize-1-24]: https://tspiteri.gitlab.io/rug/dev/rug/float/struct.MiniFloat.html#method.const_from_usize
[mi-1-24]: https://tspiteri.gitlab.io/rug/dev/rug/integer/struct.MiniInteger.html
[mi-b-1-24]: https://tspiteri.gitlab.io/rug/dev/rug/integer/struct.MiniInteger.html#method.borrow
[mi-bool-1-24]: https://tspiteri.gitlab.io/rug/dev/rug/integer/struct.MiniInteger.html#method.const_from_bool
[mi-i128-1-24]: https://tspiteri.gitlab.io/rug/dev/rug/integer/struct.MiniInteger.html#method.const_from_i128
[mi-i16-1-24]: https://tspiteri.gitlab.io/rug/dev/rug/integer/struct.MiniInteger.html#method.const_from_i16
[mi-i32-1-24]: https://tspiteri.gitlab.io/rug/dev/rug/integer/struct.MiniInteger.html#method.const_from_i32
[mi-i64-1-24]: https://tspiteri.gitlab.io/rug/dev/rug/integer/struct.MiniInteger.html#method.const_from_i64
[mi-i8-1-24]: https://tspiteri.gitlab.io/rug/dev/rug/integer/struct.MiniInteger.html#method.const_from_i8
[mi-isize-1-24]: https://tspiteri.gitlab.io/rug/dev/rug/integer/struct.MiniInteger.html#method.const_from_isize
[mi-u128-1-24]: https://tspiteri.gitlab.io/rug/dev/rug/integer/struct.MiniInteger.html#method.const_from_u128
[mi-u16-1-24]: https://tspiteri.gitlab.io/rug/dev/rug/integer/struct.MiniInteger.html#method.const_from_u16
[mi-u32-1-24]: https://tspiteri.gitlab.io/rug/dev/rug/integer/struct.MiniInteger.html#method.const_from_u32
[mi-u64-1-24]: https://tspiteri.gitlab.io/rug/dev/rug/integer/struct.MiniInteger.html#method.const_from_u64
[mi-u8-1-24]: https://tspiteri.gitlab.io/rug/dev/rug/integer/struct.MiniInteger.html#method.const_from_u8
[mi-usize-1-24]: https://tspiteri.gitlab.io/rug/dev/rug/integer/struct.MiniInteger.html#method.const_from_usize
[mr-1-24]: https://tspiteri.gitlab.io/rug/dev/rug/rational/struct.MiniRational.html
[mr-b-1-24]: https://tspiteri.gitlab.io/rug/dev/rug/rational/struct.MiniRational.html#method.borrow
[mr-cfi-1-24]: https://tspiteri.gitlab.io/rug/dev/rug/rational/struct.MiniRational.html#method.const_from_integer
[nt-0-2-cd]: https://docs.rs/num-traits/~0.2/num_traits/ops/checked/trait.CheckedDiv.html
[nt-0-2-ce]: https://docs.rs/num-traits/~0.2/num_traits/ops/euclid/trait.CheckedEuclid.html
[nt-0-2-cr]: https://docs.rs/num-traits/~0.2/num_traits/ops/checked/trait.CheckedRem.html
[nt-0-2-e]: https://docs.rs/num-traits/~0.2/num_traits/ops/euclid/trait.Euclid.html
[sys-mpfr-sn-1-6]: https://docs.rs/gmp-mpfr-sys/~1.6/gmp_mpfr_sys/mpfr/fn.set_nanflag.html
[tuple]: https://doc.rust-lang.org/nightly/core/primitive.tuple.html

### Version 1.23.0 news (2024-01-18)

  * [`MiniInteger`][mi-1-23], [`MiniRational`][mr-1-23], [`MiniFloat`][mf-1-23]
    and [`MiniComplex`][mc-1-23] were added to replace
    [`SmallInteger`][smi-1-23], [`SmallRational`][smr-1-23],
    [`SmallFloat`][smf-1-23] and [`SmallComplex`][smc-1-23], which are now
    deprecated.
      * The new structs do *not* implement [`Deref`] directly, as this causes
        issues with the current Rust memory models ([issue 52]).
      * Borrowing requires the use of the [`borrow`][mi-b-1-23] methods, which
        return an object that implements [`Deref`].
      * The new [`borrow_excl`][mi-be-1-23] methods were also added; they
        require exclusive access in order to save on some housekeeping and to
        return references directly without the need of [`Deref`].
      * The new structs implement [`Copy`] and [`Sync`].

[`Copy`]: https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html
[`Deref`]: https://doc.rust-lang.org/nightly/core/ops/trait.Deref.html
[`Sync`]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html
[issue 52]: https://gitlab.com/tspiteri/rug/-/issues/52
[mc-1-23]: https://docs.rs/rug/~1.23/rug/complex/struct.MiniComplex.html
[mf-1-23]: https://docs.rs/rug/~1.23/rug/float/struct.MiniFloat.html
[mi-1-23]: https://docs.rs/rug/~1.23/rug/integer/struct.MiniInteger.html
[mi-b-1-23]: https://docs.rs/rug/~1.23/rug/integer/struct.MiniInteger.html#method.borrow
[mi-be-1-23]: https://docs.rs/rug/~1.23/rug/integer/struct.MiniInteger.html#method.borrow_excl
[mr-1-23]: https://docs.rs/rug/~1.23/rug/rational/struct.MiniRational.html
[smc-1-23]: https://docs.rs/rug/~1.23/rug/complex/struct.SmallComplex.html
[smf-1-23]: https://docs.rs/rug/~1.23/rug/float/struct.SmallFloat.html
[smi-1-23]: https://docs.rs/rug/~1.23/rug/integer/struct.SmallInteger.html
[smr-1-23]: https://docs.rs/rug/~1.23/rug/rational/struct.SmallRational.html

### Other releases

Details on other releases can be found in [*RELEASES.md*].

## Quick example

```rust
use rug::{Assign, Integer};
let mut int = Integer::new();
assert_eq!(int, 0);
int.assign(14);
assert_eq!(int, 14);

let decimal = "98_765_432_109_876_543_210";
int.assign(Integer::parse(decimal).unwrap());
assert!(int > 100_000_000);

let hex_160 = "ffff0000ffff0000ffff0000ffff0000ffff0000";
int.assign(Integer::parse_radix(hex_160, 16).unwrap());
assert_eq!(int.significant_bits(), 160);
int = (int >> 128) - 1;
assert_eq!(int, 0xfffe_ffff_u32);
```

  * <code>[Integer][`Integer`]::[new][`new`]</code> creates a new [`Integer`]
    intialized to zero.
  * To assign values to Rug types, we use the [`Assign`] trait and its method
    [`Assign::assign`]. We do not use the [assignment operator `=`][assignment]
    as that would drop the left-hand-side operand and replace it with a
    right-hand-side operand of the same type, which is not what we want here.
  * Arbitrary precision numbers can hold numbers that are too large to fit in a
    primitive type. To assign such a number to the large types, we use strings
    rather than primitives; in the example this is done using
    <code>[Integer][`Integer`]::[parse][`parse`]</code> and
    <code>[Integer][`Integer`]::[parse_radix][`parse_radix`]</code>.
  * We can compare Rug types to primitive types or to other Rug types using the
    normal comparison operators, for example `int > 100_000_000`.
  * Most arithmetic operations are supported with Rug types and primitive types
    on either side of the operator, for example `int >> 128`.

## Using with primitive types

With Rust primitive types, arithmetic operators usually operate on two values of
the same type, for example `12i32 + 5i32`. Unlike primitive types, conversion to
and from Rug types can be expensive, so the arithmetic operators are overloaded
to work on many combinations of Rug types and primitives. More details are
available in the [documentation][primitive types].

## Operators

Operators are overloaded to work on Rug types alone or on a combination of Rug
types and Rust primitives. When at least one operand is an owned value of a Rug
type, the operation will consume that value and return a value of the Rug type.
For example

```rust
use rug::Integer;
let a = Integer::from(10);
let b = 5 - a;
assert_eq!(b, 5 - 10);
```

Here `a` is consumed by the subtraction, and `b` is an owned [`Integer`].

If on the other hand there are no owned Rug types and there are references
instead, the returned value is not the final value, but an
incomplete-computation value. For example

```rust
use rug::Integer;
let (a, b) = (Integer::from(10), Integer::from(20));
let incomplete = &a - &b;
// This would fail to compile: assert_eq!(incomplete, -10);
let sub = Integer::from(incomplete);
assert_eq!(sub, -10);
```

Here `a` and `b` are not consumed, and `incomplete` is not the final value. It
still needs to be converted or assigned into an [`Integer`]. This is covered in
more detail in the documentation’s [*Incomplete-computation values*] section.

More details on operators are available in the [documentation][operators].

## Using Rug

Rug is available on [crates.io][rug crate]. To use Rug in your crate, add it as
a dependency inside [*Cargo.toml*]:

```toml
[dependencies]
rug = "1.23"
```

Rug requires rustc version 1.65.0 or later.

Rug also depends on the [GMP], [MPFR] and [MPC] libraries through the low-level
FFI bindings in the [gmp-mpfr-sys crate][sys crate], which needs some setup to
build; the [gmp-mpfr-sys documentation][sys] has some details on usage under
[GNU/Linux][sys gnu], [macOS][sys mac] and [Windows][sys win].

## Optional features

The Rug crate has six optional features:

 1. `integer`, enabled by default. Required for the [`Integer`] type and its
    supporting features.
 2. `rational`, enabled by default. Required for the [`Rational`] number type
    and its supporting features. This feature requires the `integer` feature.
 3. `float`, enabled by default. Required for the [`Float`] type and its
    supporting features.
 4. `complex`, enabled by default. Required for the [`Complex`] number type and
    its supporting features. This feature requires the `float` feature.
 5. `rand`, enabled by default. Required for the [`RandState`] type and its
    supporting features. This feature requires the `integer` feature.
 6. `std`, enabled by default. This is for features that are not possible under
    `no_std`, such as methods that return [`String`] or the implementation of
    the [`Error`] trait.
 7. `serde`, disabled by default. This provides serialization support for the
    [`Integer`], [`Rational`], [`Float`] and [`Complex`] number types, providing
    that they are enabled. This feature requires the `std` feature and the
    [serde crate].

The first six optional features are enabled by default; to use features
selectively, you can add the dependency like this to [*Cargo.toml*]:

```toml
[dependencies.rug]
version = "1.23"
default-features = false
features = ["integer", "float", "std"]
```

Here only the `integer`, `float` and `rand` features are enabled. If none of the
features are selected, the [gmp-mpfr-sys crate][sys crate] is not required and
thus not enabled. In that case, only the [`Assign`] trait and the traits that
are in the [`ops`] module are provided by the crate.

## Experimental optional features

It is not considered a breaking change if the following experimental features
are removed. The removal of experimental features would however require a minor
version bump. Similarly, on a minor version bump, optional dependencies can be
updated to an incompatible newer version.

 1. `num-traits`, disabled by default. This implements some traits from the
    [*num-traits* crate] and the [*num-integer* crate]. (The plan is to promote
    this to an optional feature once the [*num-traits* crate] and the
    [*num-integer* crate] reach version 1.0.0.)

[*Cargo.toml*]: https://doc.rust-lang.org/cargo/guide/dependencies.html
[*Incomplete-computation values*]: https://docs.rs/rug/~1.23/rug/index.html#incomplete-computation-values
[*RELEASES.md*]: https://gitlab.com/tspiteri/rug/blob/master/RELEASES.md
[*num-integer* crate]: https://crates.io/crates/num-integer
[*num-traits* crate]: https://crates.io/crates/num-traits
[GMP]: https://gmplib.org/
[GNU GPL]: https://www.gnu.org/licenses/gpl-3.0.html
[GNU LGPL]: https://www.gnu.org/licenses/lgpl-3.0.en.html
[GNU]: https://www.gnu.org/
[MPC]: https://www.multiprecision.org/mpc/
[MPFR]: https://www.mpfr.org/
[`Assign::assign`]: https://docs.rs/rug/~1.23/rug/trait.Assign.html#tymethod.assign
[`Assign`]: https://docs.rs/rug/~1.23/rug/trait.Assign.html
[`Complex`]: https://docs.rs/rug/~1.23/rug/struct.Complex.html
[`Error`]: https://doc.rust-lang.org/nightly/std/error/trait.Error.html
[`Float`]: https://docs.rs/rug/~1.23/rug/struct.Float.html
[`Integer`]: https://docs.rs/rug/~1.23/rug/struct.Integer.html
[`RandState`]: https://docs.rs/rug/~1.23/rug/rand/struct.RandState.html
[`Rational`]: https://docs.rs/rug/~1.23/rug/struct.Rational.html
[`String`]: https://doc.rust-lang.org/nightly/std/string/struct.String.html
[`new`]: https://docs.rs/rug/~1.23/rug/struct.Integer.html#method.new
[`ops`]: https://docs.rs/rug/~1.23/rug/ops/index.html
[`parse_radix`]: https://docs.rs/rug/~1.23/rug/struct.Integer.html#method.parse_radix
[`parse`]: https://docs.rs/rug/~1.23/rug/struct.Integer.html#method.parse
[assignment]: https://doc.rust-lang.org/reference/expressions/operator-expr.html#assignment-expressions
[operators]: https://docs.rs/rug/~1.23/rug/index.html#operators
[primitive types]: https://docs.rs/rug/~1.23/rug/index.html#using-with-primitive-types
[rug crate]: https://crates.io/crates/rug
[serde crate]: https://crates.io/crates/serde
[sys crate]: https://crates.io/crates/gmp-mpfr-sys
[sys gnu]: https://docs.rs/gmp-mpfr-sys/~1.6/gmp_mpfr_sys/index.html#building-on-gnulinux
[sys mac]: https://docs.rs/gmp-mpfr-sys/~1.6/gmp_mpfr_sys/index.html#building-on-macos
[sys win]: https://docs.rs/gmp-mpfr-sys/~1.6/gmp_mpfr_sys/index.html#building-on-windows
[sys]: https://docs.rs/gmp-mpfr-sys/~1.6/gmp_mpfr_sys/index.html
