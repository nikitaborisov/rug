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

### Version 1.23.0 news (unreleased)

  * [`StackInteger`][sti-1-23] and [`StackRational`][str-1-23] were added to
    replace [`SmallInteger`][smi-1-23] and [`SmallRational`][smr-1-23], which
    are now deprecated.

[smi-1-23]: https://tspiteri.gitlab.io/rug/dev/rug/integer/struct.SmallInteger.html
[smr-1-23]: https://tspiteri.gitlab.io/rug/dev/rug/rational/struct.SmallRational.html
[sti-1-23]: https://tspiteri.gitlab.io/rug/dev/rug/integer/struct.StackInteger.html
[str-1-23]: https://tspiteri.gitlab.io/rug/dev/rug/rational/struct.StackRational.html

### Version 1.22.0 news (2023-09-11)

  * Bug fix: implementations of [`PartialOrd`] and [`PartialEq`] between
    [`Rational`][rat-1-22] numbers and tuples of two primitive integers were
    breaking the transitivity property of the traits, so the implementations
    were removed ([issue 58]). See the compatibility note below.
  * Direct [`PartialOrd`] and [`PartialEq`] comparisons are now implemented
      * between [`Integer`][int-1-22] and [`SmallInteger`][smi-1-22],
      * between [`Rational`][rat-1-22] and [`SmallRational`][smr-1-22], and
      * between [`Float`][flo-1-22] and [`SmallFloat`][smf-1-22].
  * Direct [`PartialEq`] comparisons are now implemented between
    [`Complex`][com-1-22] and [`SmallComplex`][smc-1-22].
  * [`SmallInteger`][smi-1-22] can now be assigned or converted directly to
    [`Integer`][int-1-22] using [`Assign`][ass-1-22] and [`From`].
  * [`SmallRational`][smr-1-22] can now be assigned or converted directly to
    [`Rational`][rat-1-22] using [`Assign`][ass-1-22] and [`From`].
  * [`SmallFloat`][smf-1-22] can now be assigned directly to [`Float`][flo-1-22]
    using [`AssignRound`][assr-1-22] and [`Assign`][ass-1-22].
  * [`SmallComplex`][smc-1-22] can now be assigned directly to
    [`Complex`][com-1-22] using [`AssignRound`][assr-1-22] and
    [`Assign`][ass-1-22].
  * [`Debug`] is now implemented for [`SmallInteger`][smi-1-22],
    [`SmallRational`][smr-1-22], [`SmallFloat`][smf-1-22] and
    [`SmallComplex`][smc-1-22].
  * For the [`Complex`][com-1-22] struct, the method [`eq0`][com-e-1-22] was
    renamed to [`is_zero`][com-iz-1-22]. The old method name is deprecated.

#### Compatibility note

The implementations of [`PartialOrd`] and [`PartialEq`] between
[`Rational`][rat-1-22] numbers and tuples of two primitive integers were
breaking the transitivity property of the two traits in question. Since the
implementations necessarily break the trait guarantees, having the traits
implemented is considered a bug (see [issue 58] for more details). These buggy
implementations were removed. To fix code that used these comparisons without
adding extra allocations, [`SmallRational`][smr-1-22] can be used. For example
`rational == (1, 3)` can be replaced with
`rational == SmallRational::from((1, 3))`.

[`Debug`]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html
[`From`]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html
[`PartialEq`]: https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html
[`PartialOrd`]: https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html
[ass-1-22]: https://docs.rs/rug/~1.22/rug/trait.Assign.html
[assr-1-22]: https://docs.rs/rug/~1.22/rug/ops/trait.AssignRound.html
[com-1-22]: https://docs.rs/rug/~1.22/rug/struct.Complex.html
[com-e-1-22]: https://docs.rs/rug/~1.22/rug/struct.Complex.html#method.eq0
[com-iz-1-22]: https://docs.rs/rug/~1.22/rug/struct.Complex.html#method.is_zero
[flo-1-22]: https://docs.rs/rug/~1.22/rug/struct.Float.html
[int-1-22]: https://docs.rs/rug/~1.22/rug/struct.Integer.html
[issue 58]: https://gitlab.com/tspiteri/rug/-/issues/58
[rat-1-22]: https://docs.rs/rug/~1.22/rug/struct.Rational.html
[smc-1-22]: https://docs.rs/rug/~1.22/rug/complex/struct.SmallComplex.html
[smf-1-22]: https://docs.rs/rug/~1.22/rug/float/struct.SmallFloat.html
[smi-1-22]: https://docs.rs/rug/~1.22/rug/integer/struct.SmallInteger.html
[smr-1-22]: https://docs.rs/rug/~1.22/rug/rational/struct.SmallRational.html

### Version 1.21.0 news (2023-08-28)

  * The following methods were added to [`Integer`][int-1-21]:
      * [`is_zero`][int-iz-1-21], [`is_positive`][int-ip-1-21],
        [`is_negative`][int-in-1-21]
      * [`modulo`][int-m-1-21], [`modulo_mut`][int-mm-1-21],
        [`modulo_from`][int-mf-1-21], [`modulo_ref`][int-mr-1-21]
  * The following methods were added to [`Rational`][rat-1-21]:
      * [`is_zero`][rat-iz-1-21], [`is_positive`][rat-ip-1-21],
        [`is_negative`][rat-in-1-21]
  * Bug fix: panic instead of raising floating-point exception when dividing by
    [`Rational`][rat-1-21] zero ([issue 53]).
  * [`Complete`][c-1-21] was implemented for incomplete values produced by
    [`DivRounding`][dr-1-21] and [`RemRounding`][rr-1-21] implementations
    ([merge request 4]).

[c-1-21]: https://docs.rs/rug/~1.21/rug/trait.Complete.html
[dr-1-21]: https://docs.rs/rug/~1.21/rug/ops/trait.DivRounding.html
[int-1-21]: https://docs.rs/rug/~1.21/rug/struct.Integer.html
[int-in-1-21]: https://docs.rs/rug/~1.21/rug/struct.Integer.html#method.is_negative
[int-ip-1-21]: https://docs.rs/rug/~1.21/rug/struct.Integer.html#method.is_positive
[int-iz-1-21]: https://docs.rs/rug/~1.21/rug/struct.Integer.html#method.is_zero
[int-m-1-21]: https://docs.rs/rug/~1.21/rug/struct.Integer.html#method.modulo
[int-mf-1-21]: https://docs.rs/rug/~1.21/rug/struct.Integer.html#method.modulo_from
[int-mm-1-21]: https://docs.rs/rug/~1.21/rug/struct.Integer.html#method.modulo_mut
[int-mr-1-21]: https://docs.rs/rug/~1.21/rug/struct.Integer.html#method.modulo_ref
[issue 53]: https://gitlab.com/tspiteri/rug/-/issues/53
[merge request 4]: https://gitlab.com/tspiteri/rug/-/merge_requests/4
[rat-1-21]: https://docs.rs/rug/~1.21/rug/struct.Rational.html
[rat-in-1-21]: https://docs.rs/rug/~1.21/rug/struct.Rational.html#method.is_negative
[rat-ip-1-21]: https://docs.rs/rug/~1.21/rug/struct.Rational.html#method.is_positive
[rat-iz-1-21]: https://docs.rs/rug/~1.21/rug/struct.Rational.html#method.is_zero
[rr-1-21]: https://docs.rs/rug/~1.21/rug/ops/trait.RemRounding.html

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
rug = "1.22"
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
 6. `serde`, disabled by default. This provides serialization support for the
    [`Integer`], [`Rational`], [`Float`] and [`Complex`] number types, providing
    that they are enabled. This feature requires the [serde crate].

The first five optional features are enabled by default; to use features
selectively, you can add the dependency like this to [*Cargo.toml*]:

```toml
[dependencies.rug]
version = "1.22"
default-features = false
features = ["integer", "float", "rand"]
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
    [*num-traits* crate] and the [*num-integer* crate].

[*Cargo.toml*]: https://doc.rust-lang.org/cargo/guide/dependencies.html
[*Incomplete-computation values*]: https://docs.rs/rug/~1.22/rug/index.html#incomplete-computation-values
[*RELEASES.md*]: https://gitlab.com/tspiteri/rug/blob/master/RELEASES.md
[*num-integer* crate]: https://crates.io/crates/num-integer
[*num-traits* crate]: https://crates.io/crates/num-traits
[GMP]: https://gmplib.org/
[GNU GPL]: https://www.gnu.org/licenses/gpl-3.0.html
[GNU LGPL]: https://www.gnu.org/licenses/lgpl-3.0.en.html
[GNU]: https://www.gnu.org/
[MPC]: https://www.multiprecision.org/mpc/
[MPFR]: https://www.mpfr.org/
[`Assign::assign`]: https://docs.rs/rug/~1.22/rug/trait.Assign.html#tymethod.assign
[`Assign`]: https://docs.rs/rug/~1.22/rug/trait.Assign.html
[`Complex`]: https://docs.rs/rug/~1.22/rug/struct.Complex.html
[`Float`]: https://docs.rs/rug/~1.22/rug/struct.Float.html
[`Integer`]: https://docs.rs/rug/~1.22/rug/struct.Integer.html
[`RandState`]: https://docs.rs/rug/~1.22/rug/rand/struct.RandState.html
[`Rational`]: https://docs.rs/rug/~1.22/rug/struct.Rational.html
[`new`]: https://docs.rs/rug/~1.22/rug/struct.Integer.html#method.new
[`ops`]: https://docs.rs/rug/~1.22/rug/ops/index.html
[`parse_radix`]: https://docs.rs/rug/~1.22/rug/struct.Integer.html#method.parse_radix
[`parse`]: https://docs.rs/rug/~1.22/rug/struct.Integer.html#method.parse
[assignment]: https://doc.rust-lang.org/reference/expressions/operator-expr.html#assignment-expressions
[operators]: https://docs.rs/rug/~1.22/rug/index.html#operators
[primitive types]: https://docs.rs/rug/~1.22/rug/index.html#using-with-primitive-types
[rug crate]: https://crates.io/crates/rug
[serde crate]: https://crates.io/crates/serde
[sys crate]: https://crates.io/crates/gmp-mpfr-sys
[sys gnu]: https://docs.rs/gmp-mpfr-sys/~1.6/gmp_mpfr_sys/index.html#building-on-gnulinux
[sys mac]: https://docs.rs/gmp-mpfr-sys/~1.6/gmp_mpfr_sys/index.html#building-on-macos
[sys win]: https://docs.rs/gmp-mpfr-sys/~1.6/gmp_mpfr_sys/index.html#building-on-windows
[sys]: https://docs.rs/gmp-mpfr-sys/~1.6/gmp_mpfr_sys/index.html
