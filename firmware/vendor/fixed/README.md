<!-- Copyright © 2018–2022 Trevor Spiteri -->

<!-- Copying and distribution of this file, with or without
modification, are permitted in any medium without royalty provided the
copyright notice and this notice are preserved. This file is offered
as-is, without any warranty. -->

# Fixed-point numbers

The [*fixed* crate] provides fixed-point numbers.

  * [`FixedI8`] and [`FixedU8`] are eight-bit fixed-point numbers.
  * [`FixedI16`] and [`FixedU16`] are 16-bit fixed-point numbers.
  * [`FixedI32`] and [`FixedU32`] are 32-bit fixed-point numbers.
  * [`FixedI64`] and [`FixedU64`] are 64-bit fixed-point numbers.
  * [`FixedI128`] and [`FixedU128`] are 128-bit fixed-point numbers.

An <i>n</i>-bit fixed-point number has <i>f</i>&nbsp;=&nbsp;`Frac` fractional
bits where 0&nbsp;≤&nbsp;<i>f</i>&nbsp;≤&nbsp;<i>n</i>, and
<i>n</i>&nbsp;&minus;&nbsp;<i>f</i> integer bits. For example,
<code>[FixedI32]\<[U24]></code> is a 32-bit signed fixed-point number with
<i>n</i>&nbsp;=&nbsp;32 total bits, <i>f</i>&nbsp;=&nbsp;24 fractional bits, and
<i>n</i>&nbsp;&minus;&nbsp;<i>f</i>&nbsp;=&nbsp;8 integer bits.
<code>[FixedI32]\<[U0]></code> behaves like [`i32`], and
<code>[FixedU32]\<[U0]></code> behaves like [`u32`].

The difference between any two successive representable numbers is constant
throughout the possible range for a fixed-point number:
<i>Δ</i>&nbsp;=&nbsp;1/2<sup><i>f</i></sup>. When <i>f</i>&nbsp;=&nbsp;0, like
in <code>[FixedI32]\<[U0]></code>, <i>Δ</i>&nbsp;=&nbsp;1 because representable
numbers are integers, and the difference between two successive integers is 1.
When <i>f</i>&nbsp;=&nbsp;<i>n</i>, <i>Δ</i>&nbsp;=&nbsp;1/2<sup><i>n</i></sup>
and the value lies in the range &minus;0.5&nbsp;≤&nbsp;<i>x</i>&nbsp;<&nbsp;0.5
for signed numbers like <code>[FixedI32]\<[U32]></code>, and in the range
0&nbsp;≤&nbsp;<i>x</i>&nbsp;<&nbsp;1 for unsigned numbers like
<code>[FixedU32]\<[U32]></code>.

In version 1 the [*typenum* crate] is used for the fractional bit count `Frac`;
the plan is to to have a major version 2 with const generics when the Rust
compiler’s [`generic_const_exprs` feature] is ready and stabilized. An [alpha
version] is already available.

[`generic_const_exprs` feature]: https://github.com/rust-lang/rust/issues/76560
[alpha version]: https://docs.rs/fixed/2.0.0-alpha/fixed/

The main features are

  * Representation of binary fixed-point numbers up to 128 bits wide.
  * Conversions between fixed-point numbers and numeric primitives.
  * Comparisons between fixed-point numbers and numeric primitives.
  * Parsing from strings in decimal, binary, octal and hexadecimal.
  * Display as decimal, binary, octal and hexadecimal.
  * Arithmetic and logic operations.

This crate does *not* provide decimal fixed-point numbers. For example 0.001
cannot be represented exactly, as it is 1/10<sup>3</sup>. It is binary fractions
like 1/2<sup>4</sup> (0.0625) that can be represented exactly, provided there
are enough fractional bits.

This crate does *not* provide general analytic functions.

  * No algebraic functions are provided, for example no `sqrt` or `pow`.
  * No trigonometric functions are provided, for example no `sin` or `cos`.
  * No other transcendental functions are provided, for example no `log` or
    `exp`.

These functions are not provided because different implementations can have
different trade-offs, for example trading some correctness for speed.
Implementations can be provided in other crates.

  * The [*fixed-sqrt* crate] provides the square root operation.
  * The [*cordic* crate] provides various functions implemented using the
    [CORDIC] algorithm.

The conversions supported cover the following cases.

  * Infallible lossless conversions between fixed-point numbers and numeric
    primitives are provided using [`From`] and [`Into`]. These never fail
    (infallible) and do not lose any bits (lossless).
  * Infallible lossy conversions between fixed-point numbers and numeric
    primitives are provided using the [`LossyFrom`] and [`LossyInto`] traits.
    The source can have more fractional bits than the destination.
  * Checked lossless conversions between fixed-point numbers and numeric
    primitives are provided using the [`LosslessTryFrom`] and
    [`LosslessTryInto`] traits. The source cannot have more fractional bits than
    the destination.
  * Checked conversions between fixed-point numbers and numeric primitives are
    provided using the [`FromFixed`] and [`ToFixed`] traits, or using the
    [`from_num`] and [`to_num`] methods and [their checked
    versions][`checked_from_num`].
  * Additionally, [`az`] casts are implemented for conversion between
    fixed-point numbers and numeric primitives.
  * Fixed-point numbers can be parsed from decimal strings using [`FromStr`],
    and from binary, octal and hexadecimal strings using the
    [`from_str_binary`], [`from_str_octal`] and [`from_str_hex`] methods. The
    result is rounded to the nearest, with ties rounded to even.
  * Fixed-point numbers can be converted to strings using [`Display`],
    [`Binary`], [`Octal`], [`LowerHex`] and [`UpperHex`]. The output is rounded
    to the nearest, with ties rounded to even.
  * All fixed-point numbers are plain old data, so [`bytemuck`] bit casting
    conversions can be used.

## What’s new

### Version 1.19.0 news (2022-08-29)

  * Bug fix: comparison of and conversion from subnormal floating-point numbers
    to fixed-point numbers were off by a factor of 2.
  * The following associated constants were added to the [`FixedBits`][fb-1-19]
    trait:
      * [`MIN`][fb-mi-1-19], [`MAX`][fb-ma-1-19]
      * [`IS_SIGNED`][fb-is-1-19], [`BITS`][fb-b-1-19]
  * [`FixedEquiv`][fe-1-19] is now a supertrait of the [`FixedBits`][fb-1-19]
    trait.
  * [`FixedBits`][fb-1-19] now has more supertraits from the [*num-traits*
    crate] if the [`num-traits`][feat-exp-1-19] experimental feature is enabled.

[fb-1-19]: https://docs.rs/fixed/~1.19/fixed/traits/trait.FixedBits.html
[fb-b-1-19]: https://docs.rs/fixed/~1.19/fixed/traits/trait.FixedBits.html#associatedconstant.BITS
[fb-is-1-19]: https://docs.rs/fixed/~1.19/fixed/traits/trait.FixedBits.html#associatedconstant.IS_SIGNED
[fb-ma-1-19]: https://docs.rs/fixed/~1.19/fixed/traits/trait.FixedBits.html#associatedconstant.MAX
[fb-mi-1-19]: https://docs.rs/fixed/~1.19/fixed/traits/trait.FixedBits.html#associatedconstant.MIN
[fe-1-19]: https://docs.rs/fixed/~1.19/fixed/traits/trait.FixedEquiv.html
[feat-exp-1-19]: https://docs.rs/fixed/~1.19/fixed/index.html#experimental-optional-features

### Version 1.18.0 news (2022-08-19)

  * Bug fix: checked division methods were panicking when dividing
    [`MIN`][f-m-1-18] by <code>-[DELTA][f-d-1-18]</code> for fixed-point numbers
    with zero integer bits, that is when all bits are fractional bits ([issue
    51]).
  * The following methods were added to all fixed-point numbers, to the
    [`Fixed`][tf-1-18] trait, and to the [`Wrapping`][w-1-18] and
    [`Unwrapped`][u-1-18] wrappers:
      * [`int_log`][f-il-1-18], [`checked_int_log`][f-cil-1-18]
  * The [`F128`][f128-1-18] struct was added to replace the
    [`F128Bits`][f128b-1-18] struct which is now deprecated. [`F128`][f128-1-18]
    has standard floating-point ordering and various classification methods and
    associated constants.
  * The [`from_str_dec`][u-fsd-1-18] method was added to the
    [`Unwrapped`][u-1-18] wrapper.
  * The [`Contiguous`][bm-c-1] trait from the [*bytemuck* crate] was implemented
    for all fixed-point numbers, added as a supertrait to the [`Fixed`][tf-1-18]
    trait, and implemented for the [`Wrapping`][w-1-18] and
    [`Unwrapped`][u-1-18] wrappers.

[*bytemuck* crate]: https://crates.io/crates/bytemuck
[bm-c-1]: https://docs.rs/bytemuck/^1/bytemuck/trait.Contiguous.html
[f-cil-1-18]: https://docs.rs/fixed/~1.18/fixed/struct.FixedI32.html#method.checked_int_log
[f-d-1-18]: https://docs.rs/fixed/~1.18/fixed/struct.FixedI32.html#associatedconstant.DELTA
[f-il-1-18]: https://docs.rs/fixed/~1.18/fixed/struct.FixedI32.html#method.int_log
[f-m-1-18]: https://docs.rs/fixed/~1.18/fixed/struct.FixedI32.html#associatedconstant.MIN
[f128-1-18]: https://docs.rs/fixed/~1.18/fixed/struct.F128.html
[f128b-1-18]: https://docs.rs/fixed/~1.18/fixed/struct.F128Bits.html
[issue 51]: https://gitlab.com/tspiteri/fixed/-/issues/51
[tf-1-18]: https://docs.rs/fixed/~1.18/fixed/traits/trait.Fixed.html
[u-1-18]: https://docs.rs/fixed/~1.18/fixed/struct.Unwrapped.html
[u-fsd-1-18]: https://docs.rs/fixed/~1.18/fixed/struct.Unwrapped.html#method.from_str_dec
[w-1-18]: https://docs.rs/fixed/~1.18/fixed/struct.Wrapping.html

### Other releases

Details on other releases can be found in [*RELEASES.md*].

[*RELEASES.md*]: https://gitlab.com/tspiteri/fixed/blob/master/RELEASES.md

## Quick examples

```rust
use fixed::types::I20F12;

// 19/3 = 6 1/3
let six_and_third = I20F12::from_num(19) / 3;
// four decimal digits for 12 binary digits
assert_eq!(six_and_third.to_string(), "6.3333");
// find the ceil and convert to i32
assert_eq!(six_and_third.ceil().to_num::<i32>(), 7);
// we can also compare directly to integers
assert_eq!(six_and_third.ceil(), 7);
```

The type [`I20F12`] is a 32-bit fixed-point signed number with 20 integer bits
and 12 fractional bits. It is an alias to <code>[FixedI32]\<[U12]></code>. The
unsigned counterpart would be [`U20F12`]. Aliases are provided for all
combinations of integer and fractional bits adding up to a total of eight, 16,
32, 64 or 128 bits.

```rust
use fixed::types::{I4F4, I4F12};

// -8 ≤ I4F4 < 8 with steps of 1/16 (~0.06)
let a = I4F4::from_num(1);
// multiplication and division by integers are possible
let ans1 = a / 5 * 17;
// 1 / 5 × 17 = 3 2/5 (3.4), but we get 3 3/16 (~3.2)
assert_eq!(ans1, I4F4::from_bits((3 << 4) + 3));
assert_eq!(ans1.to_string(), "3.2");

// -8 ≤ I4F12 < 8 with steps of 1/4096 (~0.0002)
let wider_a = I4F12::from(a);
let wider_ans = wider_a / 5 * 17;
let ans2 = I4F4::from_num(wider_ans);
// now the answer is the much closer 3 6/16 (~3.4)
assert_eq!(ans2, I4F4::from_bits((3 << 4) + 6));
assert_eq!(ans2.to_string(), "3.4");
```

The second example shows some precision and conversion issues. The low precision
of `a` means that `a / 5` is 3⁄16 instead of 1⁄5, leading to an inaccurate
result `ans1` = 3 3⁄16 (~3.2). With a higher precision, we get `wider_a / 5`
equal to 819⁄4096, leading to a more accurate intermediate result `wider_ans` =
3 1635⁄4096. When we convert back to four fractional bits, we get `ans2` = 3
6⁄16 (~3.4).

Note that we can convert from [`I4F4`] to [`I4F12`] using [`From`], as the
target type has the same number of integer bits and a larger number of
fractional bits. Converting from [`I4F12`] to [`I4F4`] cannot use [`From`] as we
have less fractional bits, so we use [`from_num`] instead.

## Writing fixed-point constants and values literally

The parsing methods are available as `const` functions.

```rust
use fixed::types::I16F16;

const TWELVE_POINT_75: I16F16 = I16F16::unwrapped_from_str("12.75");
// 1.1 binary is 1.5 decimal
const ONE_POINT_5: I16F16 = I16F16::unwrapped_from_str_binary("1.1");
// 12.75 + 1.5 = 14.25
let sum = TWELVE_POINT_75 + ONE_POINT_5;
assert_eq!(sum, 14.25);
```

The [*fixed-macro* crate] is an alternative which provides a convenient macro to
write down fixed-point constants literally in the code. It supports underscores
as separators, scientific notation, and binary/octal/hexadecimal integers, but
it does not support binary/octal/hexadecimal fractions as they cannot be parsed
by the Rust compiler.

```rust
use fixed::types::I16F16;
use fixed_macro::fixed;

// 0.1275e2 is 12.75
const NUM1: I16F16 = fixed!(0.127_5e2: I16F16);
// 11 binary is 3 decimal
let num2 = NUM1 + fixed!(0b11: I16F16);
// 12.75 + 3 = 15.75
assert_eq!(num2, 15.75);
```

## Using the *fixed* crate

The *fixed* crate is available on [crates.io][*fixed* crate]. To use it in your
crate, add it as a dependency inside [*Cargo.toml*]:

```toml
[dependencies]
fixed = "1.19"
```

The *fixed* crate requires rustc version 1.61.0 or later.

## Optional features

The *fixed* crate has these optional feature:

 1. `arbitrary`, disabled by default. This provides the generation of arbitrary
    fixed-point numbers from raw, unstructured data. This feature requires the
    [*arbitrary* crate].
 2. `serde`, disabled by default. This provides serialization support for the
    fixed-point types. This feature requires the [*serde* crate].
 3. `std`, disabled by default. This is for features that are not possible under
    `no_std`: currently the implementation of the [`Error`] trait for
    [`ParseFixedError`].
 4. `serde-str`, disabled by default. Fixed-point numbers are serialized as
    strings showing the value when using human-readable formats. This feature
    requires the `serde` and the `std` optional features. **Warning:** numbers
    serialized when this feature is enabled cannot be deserialized when this
    feature is disabled, and vice versa.

To enable features, you can add the dependency like this to [*Cargo.toml*]:

```toml
[dependencies.fixed]
version = "1.19"
features = ["serde"]
```

## Experimental optional features

It is not considered a breaking change if the following experimental features
are removed. The removal of experimental features would however require a minor
version bump. Similarly, on a minor version bump, optional dependencies can be
updated to an incompatible newer version.

 1. `borsh`, disabled by default. This implements serialization and
    deserialization using the [*borsh* crate]. (The plan is to promote this to
    an optional feature once the [*borsh* crate] reaches version 1.0.0.)
 2. `num-traits`, disabled by default. This implements some traits from the
    [*num-traits* crate]. (The plan is to promote this to an optional feature
    once the [*num-traits* crate] reaches version 1.0.0.)

## Deprecated optional features

The following optional features are deprecated and will be removed in the next
major version of the crate.

 1. `az`, has no effect. Previously required for the [`az`] cast traits. Now
    these cast traits are always provided.
 2. `f16`, has no effect. Previously required for conversion to/from [`f16`] and
    [`bf16`]. Now these conversions are always provided.

## License

This crate is free software: you can redistribute it and/or modify it under the
terms of either

  * the [Apache License, Version 2.0][LICENSE-APACHE] or
  * the [MIT License][LICENSE-MIT]

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache License, Version 2.0,
shall be dual licensed as above, without any additional terms or conditions.

[*Cargo.toml*]: https://doc.rust-lang.org/cargo/guide/dependencies.html
[*arbitrary* crate]: https://crates.io/crates/arbitrary
[*borsh* crate]: https://crates.io/crates/borsh
[*cordic* crate]: https://crates.io/crates/cordic
[*fixed* crate]: https://crates.io/crates/fixed
[*fixed-macro* crate]: https://crates.io/crates/fixed-macro
[*fixed-sqrt* crate]: https://crates.io/crates/fixed-sqrt
[*half* crate]: https://crates.io/crates/half
[*num-traits* crate]: https://crates.io/crates/num-traits
[*serde* crate]: https://crates.io/crates/serde
[*typenum* crate]: https://crates.io/crates/typenum
[CORDIC]: https://en.wikipedia.org/wiki/CORDIC
[FixedI32]: https://docs.rs/fixed/~1.19/fixed/struct.FixedI32.html
[FixedU32]: https://docs.rs/fixed/~1.19/fixed/struct.FixedU32.html
[LICENSE-APACHE]: https://www.apache.org/licenses/LICENSE-2.0
[LICENSE-MIT]: https://opensource.org/licenses/MIT
[U0]: https://docs.rs/fixed/~1.19/fixed/types/extra/type.U0.html
[U12]: https://docs.rs/fixed/~1.19/fixed/types/extra/type.U12.html
[U24]: https://docs.rs/fixed/~1.19/fixed/types/extra/type.U24.html
[U32]: https://docs.rs/fixed/~1.19/fixed/types/extra/type.U32.html
[`Binary`]: https://doc.rust-lang.org/nightly/core/fmt/trait.Binary.html
[`Display`]: https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html
[`Error`]: https://doc.rust-lang.org/nightly/std/error/trait.Error.html
[`FixedI128`]: https://docs.rs/fixed/~1.19/fixed/struct.FixedI128.html
[`FixedI16`]: https://docs.rs/fixed/~1.19/fixed/struct.FixedI16.html
[`FixedI32`]: https://docs.rs/fixed/~1.19/fixed/struct.FixedI32.html
[`FixedI64`]: https://docs.rs/fixed/~1.19/fixed/struct.FixedI64.html
[`FixedI8`]: https://docs.rs/fixed/~1.19/fixed/struct.FixedI8.html
[`FixedU128`]: https://docs.rs/fixed/~1.19/fixed/struct.FixedU128.html
[`FixedU16`]: https://docs.rs/fixed/~1.19/fixed/struct.FixedU16.html
[`FixedU32`]: https://docs.rs/fixed/~1.19/fixed/struct.FixedU32.html
[`FixedU64`]: https://docs.rs/fixed/~1.19/fixed/struct.FixedU64.html
[`FixedU8`]: https://docs.rs/fixed/~1.19/fixed/struct.FixedU8.html
[`FromFixed`]: https://docs.rs/fixed/~1.19/fixed/traits/trait.FromFixed.html
[`FromStr`]: https://doc.rust-lang.org/nightly/core/str/trait.FromStr.html
[`From`]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html
[`I20F12`]: https://docs.rs/fixed/~1.19/fixed/types/type.I20F12.html
[`I4F12`]: https://docs.rs/fixed/~1.19/fixed/types/type.I4F12.html
[`I4F4`]: https://docs.rs/fixed/~1.19/fixed/types/type.I4F4.html
[`Into`]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html
[`LosslessTryFrom`]: https://docs.rs/fixed/~1.19/fixed/traits/trait.LosslessTryFrom.html
[`LosslessTryInto`]: https://docs.rs/fixed/~1.19/fixed/traits/trait.LosslessTryInto.html
[`LossyFrom`]: https://docs.rs/fixed/~1.19/fixed/traits/trait.LossyFrom.html
[`LossyInto`]: https://docs.rs/fixed/~1.19/fixed/traits/trait.LossyInto.html
[`LowerHex`]: https://doc.rust-lang.org/nightly/core/fmt/trait.LowerHex.html
[`Octal`]: https://doc.rust-lang.org/nightly/core/fmt/trait.Octal.html
[`ParseFixedError`]: https://docs.rs/fixed/~1.19/fixed/struct.ParseFixedError.html
[`ToFixed`]: https://docs.rs/fixed/~1.19/fixed/traits/trait.ToFixed.html
[`U20F12`]: https://docs.rs/fixed/~1.19/fixed/types/type.U20F12.html
[`UpperHex`]: https://doc.rust-lang.org/nightly/core/fmt/trait.UpperHex.html
[`az`]: https://docs.rs/az/^1/az/index.html
[`bf16`]: https://docs.rs/half/^2/half/struct.bf16.html
[`bytemuck`]: https://docs.rs/bytemuck/^1/bytemuck/index.html
[`checked_from_num`]: https://docs.rs/fixed/~1.19/fixed/struct.FixedI32.html#method.checked_from_num
[`f16`]: https://docs.rs/half/^2/half/struct.f16.html
[`from_num`]: https://docs.rs/fixed/~1.19/fixed/struct.FixedI32.html#method.from_num
[`from_str_binary`]: https://docs.rs/fixed/~1.19/fixed/struct.FixedI32.html#method.from_str_binary
[`from_str_hex`]: https://docs.rs/fixed/~1.19/fixed/struct.FixedI32.html#method.from_str_hex
[`from_str_octal`]: https://docs.rs/fixed/~1.19/fixed/struct.FixedI32.html#method.from_str_octal
[`i32`]: https://doc.rust-lang.org/nightly/core/primitive.i32.html
[`to_num`]: https://docs.rs/fixed/~1.19/fixed/struct.FixedI32.html#method.to_num
[`u32`]: https://doc.rust-lang.org/nightly/core/primitive.u32.html
