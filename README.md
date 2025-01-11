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

### Version 1.27.0 news (unreleased)

  * The [*az* crate], which is a dependency, is now re-exported ([issue 73]).
  * The following methods were added to [`Float`][flo-1-27]:
     * [`frexp`][flo-f-1-27], [`frexp_mut`][flo-fm-1-27],
       [`frexp_ref`][flo-fr-1-27] ([issue 76])
     * [`remainder_quo31`][flo-rq-1-27], [`remainder_quo31_mut`][flo-rqm-1-27],
       [`remainder_quo31_round`][flo-rqr-1-27] ([issue 76])
     * [`remainder_quo31_from`][flo-rqf-1-27],
       [`remainder_quo31_from_round`][flo-rqfr-1-27]
     * [`remainder_quo31_ref`][flo-rqref-1-27]

[*az* crate]: https://crates.io/crates/az
[flo-1-27]: https://tspiteri.gitlab.io/rug/dev/rug/struct.Float.html
[flo-f-1-27]: https://tspiteri.gitlab.io/rug/dev/rug/struct.Float.html#method.frexp
[flo-fm-1-27]: https://tspiteri.gitlab.io/rug/dev/rug/struct.Float.html#method.frexp_mut
[flo-fr-1-27]: https://tspiteri.gitlab.io/rug/dev/rug/struct.Float.html#method.frexp_ref
[flo-rq-1-27]: https://tspiteri.gitlab.io/rug/dev/rug/struct.Float.html#method.remainder_quo31
[flo-rqf-1-27]: https://tspiteri.gitlab.io/rug/dev/rug/struct.Float.html#method.remainder_quo31_from
[flo-rqfr-1-27]: https://tspiteri.gitlab.io/rug/dev/rug/struct.Float.html#method.remainder_quo31_from_round
[flo-rqm-1-27]: https://tspiteri.gitlab.io/rug/dev/rug/struct.Float.html#method.remainder_quo31_mut
[flo-rqr-1-27]: https://tspiteri.gitlab.io/rug/dev/rug/struct.Float.html#method.remainder_quo31_round
[flo-rqref-1-27]: https://tspiteri.gitlab.io/rug/dev/rug/struct.Float.html#method.remainder_quo31_ref
[issue 73]: https://gitlab.com/tspiteri/rug/-/issues/73
[issue 76]: https://gitlab.com/tspiteri/rug/-/issues/76

### Version 1.26.1 news (2024-09-11)

  * Bug fix: implementations of [`CompleteRound`][compr-1-26] were wrongly using
    [`prec_t`][sys-mpfr-p-1-6] instead of [`u32`] for
    <code>[CompleteRound][compr-1-26]::[Prec][compr-p-1-26]</code> ([issue 72]).

[`u32`]: https://doc.rust-lang.org/nightly/core/primitive.u32.html
[compr-1-26]: https://docs.rs/rug/~1.26/rug/ops/trait.CompleteRound.html
[compr-p-1-26]: https://docs.rs/rug/~1.26/rug/ops/trait.CompleteRound.html#associatedtype.Prec
[issue 72]: https://gitlab.com/tspiteri/rug/-/issues/72
[sys-mpfr-p-1-6]: https://docs.rs/gmp-mpfr-sys/~1.6/gmp_mpfr_sys/mpfr/type.prec_t.html

### Version 1.26.0 news (2024-08-30)

  * Bug fix: [`Complete`][c-1-26] was not implemented but documented for
    incomplete values produced by the following methods ([issue 69]):
      * <code>[Integer][int-1-26]::[clamp\_ref][int-cr-1-26]</code>,
        <code>[Integer][int-1-26]::[invert\_ref][int-ir-1-26]</code>,
        <code>[Integer][int-1-26]::[pow\_mod\_ref][int-pmr-1-26]</code>
      * <code>[Integer][int-1-26]::[random\_bits][int-rb-1-26]</code>,
        <code>[Integer][int-1-26]::[random\_below\_ref][int-rbr-1-26]</code>
  * <code>[Float][flo-1-26]::[get\_significand][flo-gs-1-26]</code> is now
    usable in const context.
  * The experimental feature [`nightly-float`][feat-exp-1-26] was added ([issue
    68]).

[c-1-26]: https://docs.rs/rug/~1.26/rug/trait.Complete.html
[feat-exp-1-26]: https://docs.rs/rug/~1.26/rug/index.html#experimental-optional-features
[flo-1-26]: https://docs.rs/rug/~1.26/rug/struct.Float.html
[flo-gs-1-26]: https://docs.rs/rug/~1.26/rug/struct.Float.html#method.get_significand
[int-1-26]: https://docs.rs/rug/~1.26/rug/struct.Integer.html
[int-cr-1-26]: https://docs.rs/rug/~1.26/rug/struct.Integer.html#method.clamp_ref
[int-ir-1-26]: https://docs.rs/rug/~1.26/rug/struct.Integer.html#method.invert_ref
[int-pmr-1-26]: https://docs.rs/rug/~1.26/rug/struct.Integer.html#method.pow_mod_ref
[int-rb-1-26]: https://docs.rs/rug/~1.26/rug/struct.Integer.html#method.random_bits
[int-rbr-1-26]: https://docs.rs/rug/~1.26/rug/struct.Integer.html#method.random_below_ref
[issue 68]: https://gitlab.com/tspiteri/rug/-/issues/68
[issue 69]: https://gitlab.com/tspiteri/rug/-/issues/69

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
rug = "1.26"
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
version = "1.26"
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
 2. `nightly-float`, disabled by default. This requires the nightly compiler,
    and implements some operations with the experimental [`f16`] and [`f128`]
    primitives. (The plan is to always implement the operations and remove this
    experimental feature once the primitives are stabilized.)

[*Cargo.toml*]: https://doc.rust-lang.org/cargo/guide/dependencies.html
[*Incomplete-computation values*]: https://docs.rs/rug/~1.26/rug/index.html#incomplete-computation-values
[*RELEASES.md*]: https://gitlab.com/tspiteri/rug/blob/master/RELEASES.md
[*num-integer* crate]: https://crates.io/crates/num-integer
[*num-traits* crate]: https://crates.io/crates/num-traits
[GMP]: https://gmplib.org/
[GNU GPL]: https://www.gnu.org/licenses/gpl-3.0.html
[GNU LGPL]: https://www.gnu.org/licenses/lgpl-3.0.en.html
[GNU]: https://www.gnu.org/
[MPC]: https://www.multiprecision.org/mpc/
[MPFR]: https://www.mpfr.org/
[`Assign::assign`]: https://docs.rs/rug/~1.26/rug/trait.Assign.html#tymethod.assign
[`Assign`]: https://docs.rs/rug/~1.26/rug/trait.Assign.html
[`Complex`]: https://docs.rs/rug/~1.26/rug/struct.Complex.html
[`Error`]: https://doc.rust-lang.org/nightly/std/error/trait.Error.html
[`Float`]: https://docs.rs/rug/~1.26/rug/struct.Float.html
[`Integer`]: https://docs.rs/rug/~1.26/rug/struct.Integer.html
[`RandState`]: https://docs.rs/rug/~1.26/rug/rand/struct.RandState.html
[`Rational`]: https://docs.rs/rug/~1.26/rug/struct.Rational.html
[`String`]: https://doc.rust-lang.org/nightly/std/string/struct.String.html
[`f128`]: https://doc.rust-lang.org/nightly/std/primitive.f128.html
[`f16`]: https://doc.rust-lang.org/nightly/std/primitive.f16.html
[`new`]: https://docs.rs/rug/~1.26/rug/struct.Integer.html#method.new
[`ops`]: https://docs.rs/rug/~1.26/rug/ops/index.html
[`parse_radix`]: https://docs.rs/rug/~1.26/rug/struct.Integer.html#method.parse_radix
[`parse`]: https://docs.rs/rug/~1.26/rug/struct.Integer.html#method.parse
[assignment]: https://doc.rust-lang.org/reference/expressions/operator-expr.html#assignment-expressions
[operators]: https://docs.rs/rug/~1.26/rug/index.html#operators
[primitive types]: https://docs.rs/rug/~1.26/rug/index.html#using-with-primitive-types
[rug crate]: https://crates.io/crates/rug
[serde crate]: https://crates.io/crates/serde
[sys crate]: https://crates.io/crates/gmp-mpfr-sys
[sys gnu]: https://docs.rs/gmp-mpfr-sys/~1.6/gmp_mpfr_sys/index.html#building-on-gnulinux
[sys mac]: https://docs.rs/gmp-mpfr-sys/~1.6/gmp_mpfr_sys/index.html#building-on-macos
[sys win]: https://docs.rs/gmp-mpfr-sys/~1.6/gmp_mpfr_sys/index.html#building-on-windows
[sys]: https://docs.rs/gmp-mpfr-sys/~1.6/gmp_mpfr_sys/index.html
