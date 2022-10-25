// Copyright © 2018–2022 Trevor Spiteri

// This library is free software: you can redistribute it and/or
// modify it under the terms of either
//
//   * the Apache License, Version 2.0 or
//   * the MIT License
//
// at your option.
//
// You should have recieved copies of the Apache License and the MIT
// License along with the library. If not, see
// <https://www.apache.org/licenses/LICENSE-2.0> and
// <https://opensource.org/licenses/MIT>.

macro_rules! fixed_frac {
    (
        $Fixed:ident[$s_fixed:expr](
            $Inner:ident[$s_inner:expr], $LeEqU:tt, $s_nbits:expr,
            $s_nbits_m1:expr, $s_nbits_m4:expr
        ),
        $UFixed:ident, $UInner:ident, $Signedness:tt
    ) => {
        /// The implementation of items in this block depends on the
        /// number of fractional bits `Frac`.
        impl<Frac: $LeEqU> $Fixed<Frac> {
            comment! {
                "The number of integer bits.

# Examples

```rust
use fixed::{types::extra::U6, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U6>;
assert_eq!(Fix::INT_NBITS, ", $s_nbits, " - 6);
```
";
                pub const INT_NBITS: u32 = $Inner::BITS - Self::FRAC_NBITS;
            }

            comment! {
                "The number of fractional bits.

# Examples

```rust
use fixed::{types::extra::U6, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U6>;
assert_eq!(Fix::FRAC_NBITS, 6);
```
";
                pub const FRAC_NBITS: u32 = Frac::U32;
            }

            // some other useful constants for internal use:

            const INT_MASK: $Inner =
                !0 << (Self::FRAC_NBITS / 2) << (Self::FRAC_NBITS - Self::FRAC_NBITS / 2);
            const FRAC_MASK: $Inner = !Self::INT_MASK;

            // 0 when FRAC_NBITS = 0
            const INT_LSB: $Inner = Self::INT_MASK ^ (Self::INT_MASK << 1);

            // 0 when INT_NBITS = 0
            const FRAC_MSB: $Inner =
                Self::FRAC_MASK ^ ((Self::FRAC_MASK as $UInner) >> 1) as $Inner;

            fixed_from_to! { $Fixed[$s_fixed]($Inner[$s_inner], $s_nbits), $Signedness }
            fixed_round! { $Fixed[$s_fixed]($s_nbits), $Signedness }

            comment! {
                "Integer base-2 logarithm, rounded down.

# Panics

Panics if the fixed-point number is ", if_signed_unsigned!($Signedness, "≤&nbsp;0", "zero"), ".

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::from_num(4).int_log2(), 2);
assert_eq!(Fix::from_num(3.9375).int_log2(), 1);
assert_eq!(Fix::from_num(0.25).int_log2(), -2);
assert_eq!(Fix::from_num(0.1875).int_log2(), -3);
```
";
                #[inline]
                pub const fn int_log2(self) -> i32 {
                    match self.checked_int_log2() {
                        Some(s) => s,
                        None => panic!("log of non-positive number"),
                    }
                }
            }

            comment! {
                "Integer base-10 logarithm, rounded down.

# Panics

Panics if the fixed-point number is ", if_signed_unsigned!($Signedness, "≤&nbsp;0", "zero"), ".

# Examples

```rust
use fixed::{
    types::extra::{U2, U6},
    ", $s_fixed, ",
};
assert_eq!(", $s_fixed, "::<U2>::from_num(10).int_log10(), 1);
assert_eq!(", $s_fixed, "::<U2>::from_num(9.75).int_log10(), 0);
assert_eq!(", $s_fixed, "::<U6>::from_num(0.109375).int_log10(), -1);
assert_eq!(", $s_fixed, "::<U6>::from_num(0.09375).int_log10(), -2);
```
";
                #[inline]
                pub const fn int_log10(self) -> i32 {
                    match self.checked_int_log10() {
                        Some(s) => s,
                        None => panic!("log of non-positive number"),
                    }
                }
            }

            comment! {
                "Integer logarithm to the specified base, rounded down.

# Panics

Panics if the fixed-point number is ", if_signed_unsigned!($Signedness, "≤&nbsp;0", "zero"), "
or if the base is <&nbsp;2.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::from_num(4).int_log(2), 2);
assert_eq!(Fix::from_num(5.75).int_log(5), 1);
assert_eq!(Fix::from_num(0.25).int_log(5), -1);
assert_eq!(Fix::from_num(0.1875).int_log(5), -2);
```
";
                #[inline]
                pub const fn int_log(self, base: u32) -> i32 {
                    match self.checked_int_log(base) {
                        Some(s) => s,
                        None => {
                            if base < 2 {
                                panic!("log with base < 2");
                            } else {
                                panic!("log of non-positive number");
                            }
                        }
                    }
                }
            }

            comment! {
                "Checked integer base-2 logarithm, rounded down.
Returns the logarithm or [`None`] if the fixed-point number is
", if_signed_unsigned!($Signedness, "≤&nbsp;0", "zero"), ".

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::ZERO.checked_int_log2(), None);
assert_eq!(Fix::from_num(4).checked_int_log2(), Some(2));
assert_eq!(Fix::from_num(3.9375).checked_int_log2(), Some(1));
assert_eq!(Fix::from_num(0.25).checked_int_log2(), Some(-2));
assert_eq!(Fix::from_num(0.1875).checked_int_log2(), Some(-3));
```
";
                #[inline]
                pub const fn checked_int_log2(self) -> Option<i32> {
                    if self.to_bits() <= 0 {
                        None
                    } else {
                        Some(Self::INT_NBITS as i32 - 1 - self.leading_zeros() as i32)
                    }
                }
            }

            comment! {
                "Checked integer base-10 logarithm, rounded down.
Returns the logarithm or [`None`] if the fixed-point number is
", if_signed_unsigned!($Signedness, "≤&nbsp;0", "zero"), ".

# Examples

```rust
use fixed::{
    types::extra::{U2, U6},
    ", $s_fixed, ",
};
assert_eq!(", $s_fixed, "::<U2>::ZERO.checked_int_log10(), None);
assert_eq!(", $s_fixed, "::<U2>::from_num(10).checked_int_log10(), Some(1));
assert_eq!(", $s_fixed, "::<U2>::from_num(9.75).checked_int_log10(), Some(0));
assert_eq!(", $s_fixed, "::<U6>::from_num(0.109375).checked_int_log10(), Some(-1));
assert_eq!(", $s_fixed, "::<U6>::from_num(0.09375).checked_int_log10(), Some(-2));
```
";
                #[inline]
                pub const fn checked_int_log10(self) -> Option<i32> {
                    if self.to_bits() <= 0 {
                        return None;
                    }
                    // Use unsigned representation because we use all bits in fractional part.
                    let bits = self.to_bits() as $UInner;
                    let int = bits >> Self::FRAC_NBITS;
                    if int != 0 {
                        Some(log10::int_part::$UInner(int))
                    } else {
                        let frac = bits << Self::INT_NBITS;
                        Some(log10::frac_part::$UInner(frac))
                    }
                }
            }

            comment! {
                "Checked integer logarithm to the specified base, rounded down.
Returns the logarithm, or [`None`] if the fixed-point number is
", if_signed_unsigned!($Signedness, "≤&nbsp;0", "zero"), "
or if the base is <&nbsp;2.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::ZERO.checked_int_log(5), None);
assert_eq!(Fix::from_num(4).checked_int_log(2), Some(2));
assert_eq!(Fix::from_num(5.75).checked_int_log(5), Some(1));
assert_eq!(Fix::from_num(0.25).checked_int_log(5), Some(-1));
assert_eq!(Fix::from_num(0.1875).checked_int_log(5), Some(-2));
```
";
                #[inline]
                pub const fn checked_int_log(self, base: u32) -> Option<i32> {
                    if self.to_bits() <= 0 || base < 2 {
                        return None;
                    }
                    // Use unsigned representation.
                    let bits = self.to_bits() as $UInner;
                    let int = bits >> Self::FRAC_NBITS;
                    if int != 0 {
                        Some(log::int_part::$UInner(int, base))
                    } else {
                        let frac = bits << Self::INT_NBITS;
                        Some(log::frac_part::$UInner(frac, base))
                    }
                }
            }

            if_signed! {
                $Signedness;
                comment! {
                    "Returns a number representing the sign of `self`.

# Panics

When debug assertions are enabled, this method panics
  * if the value is positive and the fixed-point number has zero
    or one integer bits such that it cannot hold the value 1.
  * if the value is negative and the fixed-point number has zero
    integer bits, such that it cannot hold the value &minus;1.

When debug assertions are not enabled, the wrapped value can be
returned in those cases, but it is not considered a breaking change if
in the future it panics; using this method when 1 and &minus;1 cannot be
represented is almost certainly a bug.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::from_num(5).signum(), 1);
assert_eq!(Fix::ZERO.signum(), 0);
assert_eq!(Fix::from_num(-5).signum(), -1);
```
";
                    #[inline]
                    #[must_use]
                    pub const fn signum(self) -> $Fixed<Frac> {
                        let (ans, overflow) = self.overflowing_signum();
                        debug_assert!(!overflow, "overflow");
                        ans
                    }
                }
            }

            comment! {
                "Returns the reciprocal (inverse) of the fixed-point number, 1/`self`.

# Panics

Panics if the fixed-point number is zero.

When debug assertions are enabled, this method also panics if the
reciprocal overflows. When debug assertions are not enabled, the
wrapped value can be returned, but it is not considered a breaking
change if in the future it panics; if wrapping is required use
[`wrapping_recip`] instead.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::from_num(2).recip(), Fix::from_num(0.5));
```

[`wrapping_recip`]: Self::wrapping_recip
";
                #[inline]
                #[must_use]
                pub const fn recip(self) -> $Fixed<Frac> {
                    let (ans, overflow) = self.overflowing_recip();
                    debug_assert!(!overflow, "overflow");
                    ans
                }
            }

            comment! {
                "Euclidean division.

# Panics

Panics if the divisor is zero.

When debug assertions are enabled, this method also panics if the
division overflows. When debug assertions are not enabled, the wrapped
value can be returned, but it is not considered a breaking change if
in the future it panics; if wrapping is required use
[`wrapping_div_euclid`] instead.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::from_num(7.5).div_euclid(Fix::from_num(2)), Fix::from_num(3));
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "assert_eq!(Fix::from_num(-7.5).div_euclid(Fix::from_num(2)), Fix::from_num(-4));
",
                },
                "```

[`wrapping_div_euclid`]: Self::wrapping_div_euclid
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn div_euclid(self, rhs: $Fixed<Frac>) -> $Fixed<Frac> {
                    let (ans, overflow) = self.overflowing_div_euclid(rhs);
                    debug_assert!(!overflow, "overflow");
                    ans
                }
            }

            comment! {
                "Euclidean division by an integer.

# Panics

Panics if the divisor is zero.

",
                if_signed_else_empty_str! {
                    $Signedness;
                    "When debug assertions are enabled, this method
also panics if the division overflows. Overflow can only occur when
dividing the minimum value by &minus;1. When debug assertions are not
enabled, the wrapped value can be returned, but it is not considered a
breaking change if in the future it panics; if wrapping is required
use [`wrapping_div_euclid_int`] instead.
",
                },
                "# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::from_num(7.5).div_euclid_int(2), Fix::from_num(3));
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "assert_eq!(Fix::from_num(-7.5).div_euclid_int(2), Fix::from_num(-4));
",
                },
                "```

[`wrapping_div_euclid_int`]: Self::wrapping_div_euclid_int
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn div_euclid_int(self, rhs: $Inner) -> $Fixed<Frac> {
                    let (ans, overflow) = self.overflowing_div_euclid_int(rhs);
                    debug_assert!(!overflow, "overflow");
                    ans
                }
            }

            comment! {
                "Adds `self` to the product `a`&nbsp;×&nbsp;`b`.

",
                if_signed_else_empty_str! {
                    $Signedness;
                    "For some cases, the product `a`&nbsp;×&nbsp;`b` would
overflow on its own, but the final result `self`&nbsp;+&nbsp;`a`&nbsp;×&nbsp;`b`
is representable; in these cases this method returns the correct result without
overflow.

",
                },
                "The `a` and `b` parameters can have a fixed-point type like
`self` but with a different number of fractional bits.

The [`mul_acc`] method performs the same operation as this method but mutates
`self` instead of returning the result.

# Panics

When debug assertions are enabled, this method panics if the result overflows.
When debug assertions are not enabled, the wrapped value can be returned, but it
is not considered a breaking change if in the future it panics; if wrapping is
required use [`wrapping_add_prod`] instead.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::from_num(3).add_prod(Fix::from_num(4), Fix::from_num(0.5)), 5);
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "// -MAX + MAX × 1.5 = MAX / 2, which does not overflow
assert_eq!((-Fix::MAX).add_prod(Fix::MAX, Fix::from_num(1.5)), Fix::MAX / 2);
"
                },
                "```

[`mul_acc`]: Self::mul_acc
[`wrapping_add_prod`]: Self::wrapping_add_prod
";
                #[inline]
                #[must_use]
                pub const fn add_prod<AFrac: $LeEqU, BFrac: $LeEqU>(
                    self,
                    a: $Fixed<AFrac>,
                    b: $Fixed<BFrac>,
                ) -> $Fixed<Frac> {
                    let (ans, overflow) = arith::$Inner::overflowing_mul_add(
                        a.to_bits(),
                        b.to_bits(),
                        self.to_bits(),
                        AFrac::I32 + BFrac::I32 - Frac::I32,
                    );
                    debug_assert!(!overflow, "overflow");
                    Self::from_bits(ans)
                }
            }

            comment! {
                "Multiply and accumulate. Adds (`a` × `b`) to `self`.

",
                if_signed_else_empty_str! {
                    $Signedness;
                    "For some cases, the product `a` × `b` would overflow on its
own, but the final result `self` + `a` × `b` is representable; in these cases
this method saves the correct result without overflow.

",
                },
                "The `a` and `b` parameters can have a fixed-point type like
`self` but with a different number of fractional bits.

The [`add_prod`] method performs the same operation as this method but returns
the result instead of mutating `self`.

# Panics

When debug assertions are enabled, this method panics if the result
overflows. When debug assertions are not enabled, the wrapped value
can be returned, but it is not considered a breaking change if in the
future it panics; if wrapping is required use [`wrapping_mul_acc`]
instead.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let mut acc = Fix::from_num(3);
acc.mul_acc(Fix::from_num(4), Fix::from_num(0.5));
assert_eq!(acc, 5);
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "
// MAX × 1.5 - MAX = MAX / 2, which does not overflow
acc = -Fix::MAX;
acc.mul_acc(Fix::MAX, Fix::from_num(1.5));
assert_eq!(acc, Fix::MAX / 2);
"
                },
                "```

[`add_prod`]: Self::add_prod
[`wrapping_mul_acc`]: Self::wrapping_mul_acc
";
                #[inline]
                pub fn mul_acc<AFrac: $LeEqU, BFrac: $LeEqU>(
                    &mut self,
                    a: $Fixed<AFrac>,
                    b: $Fixed<BFrac>,
                ) {
                    *self = self.add_prod(a, b);
                }
            }

            comment! {
                "Remainder for Euclidean division by an integer.

# Panics

Panics if the divisor is zero.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::from_num(7.5).rem_euclid_int(2), Fix::from_num(1.5));
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "assert_eq!(Fix::from_num(-7.5).rem_euclid_int(2), Fix::from_num(0.5));
",
                },
                "```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn rem_euclid_int(self, rhs: $Inner) -> $Fixed<Frac> {
                    let (ans, overflow) = self.overflowing_rem_euclid_int(rhs);
                    debug_assert!(!overflow, "overflow");
                    ans
                }
            }

            comment! {
                "Linear interpolation between `start` and `end`.

Returns
`start`&nbsp;+&nbsp;`self`&nbsp;×&nbsp;(`end`&nbsp;&minus;&nbsp;`start`). This
is `start` when `self`&nbsp;=&nbsp;0, `end` when `self`&nbsp;=&nbsp;1, and
linear interpolation for all other values of `self`. Linear extrapolation is
performed if `self` is not in the range 0&nbsp;≤&nbsp;<i>x</i>&nbsp;≤&nbsp;1.

# Panics

When debug assertions are enabled, this method panics if the result overflows.
When debug assertions are not enabled, the wrapped value can be returned, but it
is not considered a breaking change if in the future it panics; if wrapping is
required use [`wrapping_lerp`] instead.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let start = Fix::from_num(2);
let end = Fix::from_num(3.5);
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "assert_eq!(Fix::from_num(-1.0).lerp(start, end), 0.5);
",
                },
                "assert_eq!(Fix::from_num(0.0).lerp(start, end), 2);
assert_eq!(Fix::from_num(0.5).lerp(start, end), 2.75);
assert_eq!(Fix::from_num(1.0).lerp(start, end), 3.5);
assert_eq!(Fix::from_num(2.0).lerp(start, end), 5);
```

[`wrapping_lerp`]: Self::wrapping_lerp
";
                #[inline]
                pub const fn lerp<RangeFrac>(
                    self,
                    start: $Fixed<RangeFrac>,
                    end: $Fixed<RangeFrac>,
                ) -> $Fixed<RangeFrac> {
                    let (ans, overflow) = lerp::$Inner(
                        self.to_bits(),
                        start.to_bits(),
                        end.to_bits(),
                        Frac::U32,
                    );
                    debug_assert!(!overflow, "overflow");
                    $Fixed::from_bits(ans)
                }
            }

            if_signed! {
                $Signedness;
                comment! {
                    "Checked signum. Returns a number representing the
sign of `self`, or [`None`] on overflow.

Overflow can only occur
  * if the value is positive and the fixed-point number has zero
    or one integer bits such that it cannot hold the value 1.
  * if the value is negative and the fixed-point number has zero
    integer bits, such that it cannot hold the value &minus;1.

# Examples

```rust
use fixed::{
    types::extra::{U4, U", $s_nbits_m1, ", U", $s_nbits, "},
    ", $s_fixed, ",
};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::from_num(5).checked_signum(), Some(Fix::ONE));
assert_eq!(Fix::ZERO.checked_signum(), Some(Fix::ZERO));
assert_eq!(Fix::from_num(-5).checked_signum(), Some(Fix::NEG_ONE));

type OneIntBit = ", $s_fixed, "<U", $s_nbits_m1, ">;
type ZeroIntBits = ", $s_fixed, "<U", $s_nbits, ">;
assert_eq!(OneIntBit::from_num(0.5).checked_signum(), None);
assert_eq!(ZeroIntBits::from_num(0.25).checked_signum(), None);
assert_eq!(ZeroIntBits::from_num(-0.5).checked_signum(), None);
```
";
                    #[inline]
                    pub const fn checked_signum(self) -> Option<$Fixed<Frac>> {
                        match self.overflowing_signum() {
                            (ans, false) => Some(ans),
                            (_, true) => None,
                        }
                    }
                }
            }

            comment! {
                "Checked multiplication. Returns the product, or [`None`] on overflow.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::MAX.checked_mul(Fix::ONE), Some(Fix::MAX));
assert_eq!(Fix::MAX.checked_mul(Fix::from_num(2)), None);
```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn checked_mul(self, rhs: $Fixed<Frac>) -> Option<$Fixed<Frac>> {
                    match arith::$Inner::overflowing_mul(self.to_bits(), rhs.to_bits(), Frac::U32) {
                        (ans, false) => Some(Self::from_bits(ans)),
                        (_, true) => None,
                    }
                }
            }

            comment! {
                "Checked division. Returns the quotient, or [`None`] if
the divisor is zero or on overflow.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::MAX.checked_div(Fix::ONE), Some(Fix::MAX));
assert_eq!(Fix::MAX.checked_div(Fix::ONE / 2), None);
```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn checked_div(self, rhs: $Fixed<Frac>) -> Option<$Fixed<Frac>> {
                    if rhs.to_bits() == 0 {
                        return None;
                    }
                    match arith::$Inner::overflowing_div(self.to_bits(), rhs.to_bits(), Frac::U32) {
                        (ans, false) => Some(Self::from_bits(ans)),
                        (_, true) => None,
                    }
                }
            }

            comment! {
                "Checked reciprocal. Returns the reciprocal, or
[`None`] if `self` is zero or on overflow.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::from_num(2).checked_recip(), Some(Fix::from_num(0.5)));
assert_eq!(Fix::ZERO.checked_recip(), None);
```
";
                #[inline]
                pub const fn checked_recip(self) -> Option<$Fixed<Frac>> {
                    if self.to_bits() == 0 {
                        None
                    } else {
                        match self.overflowing_recip() {
                            (ans, false) => Some(ans),
                            (_, true) => None,
                        }
                    }
                }
            }

            comment! {
                "Checked Euclidean division. Returns the quotient, or
[`None`] if the divisor is zero or on overflow.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::from_num(7.5).checked_div_euclid(Fix::from_num(2)), Some(Fix::from_num(3)));
assert_eq!(Fix::from_num(7.5).checked_div_euclid(Fix::ZERO), None);
assert_eq!(Fix::MAX.checked_div_euclid(Fix::from_num(0.25)), None);
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "assert_eq!(Fix::from_num(-7.5).checked_div_euclid(Fix::from_num(2)), Some(Fix::from_num(-4)));
",
                },
                "```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn checked_div_euclid(self, rhs: $Fixed<Frac>) -> Option<$Fixed<Frac>> {
                    let q = match self.checked_div(rhs) {
                        Some(s) => s.round_to_zero(),
                        None => return None,
                    };
                    if_signed! {
                        $Signedness;
                        if self.unwrapped_rem(rhs).is_negative() {
                            let neg_one = match Self::TRY_NEG_ONE {
                                Some(s) => s,
                                None => return None,
                            };
                            return if rhs.is_positive() {
                                q.checked_add(neg_one)
                            } else {
                                q.checked_sub(neg_one)
                            };
                        }
                    }
                    Some(q)
                }
            }

            comment! {
                "Adds `self` to the product `a`&nbsp;×&nbsp;`b`, returning [`None`] on overflow.

",
                if_signed_else_empty_str! {
                    $Signedness;
                    "For some cases, the product `a`&nbsp;×&nbsp;`b` would
overflow on its own, but the final result `self`&nbsp;+&nbsp;`a`&nbsp;×&nbsp;`b`
is representable; in these cases this method returns the correct result without
overflow.

",
                },
                "The `a` and `b` parameters can have a fixed-point type like
`self` but with a different number of fractional bits.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(
    Fix::from_num(3).checked_add_prod(Fix::from_num(4), Fix::from_num(0.5)),
    Some(Fix::from_num(5))
);
assert_eq!(Fix::DELTA.checked_add_prod(Fix::MAX, Fix::ONE), None);
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "// -MAX + MAX × 1.5 = MAX / 2, which does not overflow
assert_eq!(
    (-Fix::MAX).checked_add_prod(Fix::MAX, Fix::from_num(1.5)),
    Some(Fix::MAX / 2)
);
"
                },
                "```
";
                #[inline]
                #[must_use]
                pub const fn checked_add_prod<AFrac: $LeEqU, BFrac: $LeEqU>(
                    self,
                    a: $Fixed<AFrac>,
                    b: $Fixed<BFrac>,
                ) -> Option<$Fixed<Frac>> {
                    let (ans, overflow) = arith::$Inner::overflowing_mul_add(
                        a.to_bits(),
                        b.to_bits(),
                        self.to_bits(),
                        AFrac::I32 + BFrac::I32 - Frac::I32,
                    );
                    if overflow {
                        return None;
                    }
                    Some(Self::from_bits(ans))
                }
            }

            comment! {
                r#"Checked multiply and accumulate. Adds (`a` × `b`) to `self`,
or returns [`None`] on overflow.

Like all other checked methods, this method wraps the successful return value in
an [`Option`]. Since the unchecked [`mul_acc`] method does not return a value,
which is equivalent to returning [`()`][unit], this method wraps [`()`][unit]
into <code>[Some]\([()][unit])</code> on success.

When overflow occurs, `self` is not modified and retains its previous value.

"#,
                if_signed_else_empty_str! {
                    $Signedness;
                    "For some cases, the product `a` × `b` would overflow on its
own, but the final result `self` + `a` × `b` is representable; in these cases
this method saves the correct result without overflow.

",
                },
                "The `a` and `b` parameters can have a fixed-point type like
`self` but with a different number of fractional bits.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let mut acc = Fix::from_num(3);
let check = acc.checked_mul_acc(Fix::from_num(4), Fix::from_num(0.5));
assert_eq!(check, Some(()));
assert_eq!(acc, 5);

acc = Fix::DELTA;
let check = acc.checked_mul_acc(Fix::MAX, Fix::ONE);
assert_eq!(check, None);
// acc is unchanged on error
assert_eq!(acc, Fix::DELTA);
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "
// MAX × 1.5 - MAX = MAX / 2, which does not overflow
acc = -Fix::MAX;
let check = acc.checked_mul_acc(Fix::MAX, Fix::from_num(1.5));
assert_eq!(check, Some(()));
assert_eq!(acc, Fix::MAX / 2);
"
                },
                "```

[`mul_acc`]: Self::mul_acc
";
                #[inline]
                #[must_use = "this `Option` may be a `None` variant indicating overflow, which should be handled"]
                pub fn checked_mul_acc<AFrac: $LeEqU, BFrac: $LeEqU>(
                    &mut self,
                    a: $Fixed<AFrac>,
                    b: $Fixed<BFrac>,
                ) -> Option<()> {
                    match self.checked_add_prod(a, b) {
                        Some(s) => {
                            *self = s;
                            Some(())
                        }
                        None => None,
                    }
                }
            }

            comment! {
                "Checked fixed-point remainder for division by an integer.
Returns the remainder, or [`None`] if the divisor is zero.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::from_num(3.75).checked_rem_int(2), Some(Fix::from_num(1.75)));
assert_eq!(Fix::from_num(3.75).checked_rem_int(0), None);
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "assert_eq!(Fix::from_num(-3.75).checked_rem_int(2), Some(Fix::from_num(-1.75)));
",
                },
                "```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn checked_rem_int(self, rhs: $Inner) -> Option<$Fixed<Frac>> {
                    // Overflow converting rhs to $Fixed<Frac> means that either
                    //   * |rhs| > |self|, and so remainder is self, or
                    //   * self is signed min with at least one integer bit,
                    //     and the value of rhs is -self, so remainder is 0.
                    if rhs == 0 {
                        return None;
                    }
                    let can_shift = if_signed_unsigned!(
                        $Signedness,
                        if rhs.is_negative() {
                            rhs.leading_ones() - 1
                        } else {
                            rhs.leading_zeros() - 1
                        },
                        rhs.leading_zeros(),
                    );
                    if Self::FRAC_NBITS <= can_shift {
                        let fixed_rhs = Self::from_bits(rhs << Self::FRAC_NBITS);
                        self.checked_rem(fixed_rhs)
                    } else {
                        if_signed_unsigned!(
                            $Signedness,
                            if self.to_bits() == $Inner::MIN
                                && (Self::INT_NBITS > 0 && rhs == 1 << (Self::INT_NBITS - 1))
                            {
                                Some(Self::ZERO)
                            } else {
                                Some(self)
                            },
                            Some(self),
                        )
                    }
                }
            }

            comment! {
                "Checked Euclidean division by an integer. Returns the
quotient, or [`None`] if the divisor is zero",
                if_signed_else_empty_str! {
                    $Signedness;
                    " or if the division results in overflow",
                },
                ".

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::from_num(7.5).checked_div_euclid_int(2), Some(Fix::from_num(3)));
assert_eq!(Fix::from_num(7.5).checked_div_euclid_int(0), None);
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "assert_eq!(Fix::MIN.checked_div_euclid_int(-1), None);
",
                },
                "```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn checked_div_euclid_int(self, rhs: $Inner) -> Option<$Fixed<Frac>> {
                    let q = match self.checked_div_int(rhs) {
                        Some(s) => s.round_to_zero(),
                        None => return None,
                    };
                    if_signed! {
                        $Signedness;
                        if self.unwrapped_rem_int(rhs).is_negative() {
                            let neg_one = match Self::TRY_NEG_ONE {
                                Some(s) => s,
                                None => return None,
                            };
                            return if rhs.is_positive() {
                                q.checked_add(neg_one)
                            } else {
                                q.checked_sub(neg_one)
                            };
                        }
                    }
                    Some(q)
                }
            }

            comment! {
                "Checked remainder for Euclidean division by an integer.
Returns the remainder, or [`None`] if the divisor is zero",
                if_signed_else_empty_str! {
                    $Signedness;
                    " or if the remainder results in overflow",
                },
                ".

# Examples

```rust
use fixed::{types::extra::U", $s_nbits_m4, ", ", $s_fixed, "};
type Fix = ", $s_fixed, "<U", $s_nbits_m4, ">;
assert_eq!(Fix::from_num(7.5).checked_rem_euclid_int(2), Some(Fix::from_num(1.5)));
assert_eq!(Fix::from_num(7.5).checked_rem_euclid_int(0), None);
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "assert_eq!(Fix::from_num(-7.5).checked_rem_euclid_int(2), Some(Fix::from_num(0.5)));
// -8 ≤ Fix < 8, so the answer 12.5 overflows
assert_eq!(Fix::from_num(-7.5).checked_rem_euclid_int(20), None);
",
                },
                "```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn checked_rem_euclid_int(self, rhs: $Inner) -> Option<$Fixed<Frac>> {
                    if_signed! {
                        $Signedness;
                        let rem = match self.checked_rem_int(rhs){
                            Some(s) => s,
                            None => return None,
                        };
                        if !rem.is_negative() {
                            return Some(rem);
                        }
                        // Work in unsigned.
                        // Answer required is |rhs| - |rem|, but rhs is int, rem is fixed.
                        // INT_NBITS == 0 is a special case, as fraction can be negative.
                        if Self::INT_NBITS == 0 {
                            // -0.5 <= rem < 0, so euclidean remainder is in the range
                            // 0.5 <= answer < 1, which does not fit.
                            return None;
                        }
                        let rhs_abs = rhs.wrapping_abs() as $UInner;
                        let remb = rem.to_bits();
                        let remb_abs = remb.wrapping_neg() as $UInner;
                        let rem_int_abs = remb_abs >> Self::FRAC_NBITS;
                        let rem_frac = remb & Self::FRAC_MASK;
                        let ans_int = rhs_abs - rem_int_abs - if rem_frac > 0 { 1 } else { 0 };
                        let ansb_abs = if ans_int == 0 {
                            0
                        } else if Self::FRAC_NBITS <= ans_int.leading_zeros() {
                            ans_int << Self::FRAC_NBITS
                        } else {
                            return None
                        };
                        let ansb = ansb_abs as $Inner;
                        if ansb.is_negative() {
                            return None;
                        }
                        Some(Self::from_bits(ansb | rem_frac))
                    }
                    if_unsigned! {
                        $Signedness;
                        self.checked_rem_int(rhs)
                    }
                }
            }

            comment! {
                "Checked linear interpolation between `start` and `end`. Returns
[`None`] on overflow.

The interpolted value is
`start`&nbsp;+&nbsp;`self`&nbsp;×&nbsp;(`end`&nbsp;&minus;&nbsp;`start`). This
is `start` when `self`&nbsp;=&nbsp;0, `end` when `self`&nbsp;=&nbsp;1, and
linear interpolation for all other values of `self`. Linear extrapolation is
performed if `self` is not in the range 0&nbsp;≤&nbsp;<i>x</i>&nbsp;≤&nbsp;1.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::from_num(0.5).checked_lerp(Fix::ZERO, Fix::MAX), Some(Fix::MAX / 2));
assert_eq!(Fix::from_num(1.5).checked_lerp(Fix::ZERO, Fix::MAX), None);
```
";
                #[inline]
                pub const fn checked_lerp<RangeFrac>(
                    self,
                    start: $Fixed<RangeFrac>,
                    end: $Fixed<RangeFrac>,
                ) -> Option<$Fixed<RangeFrac>> {
                    match lerp::$Inner(self.to_bits(), start.to_bits(), end.to_bits(), Frac::U32) {
                        (bits, false) => Some($Fixed::from_bits(bits)),
                        (_, true) => None,
                    }
                }
            }

            if_signed! {
                $Signedness;
                comment! {
                    "Saturating signum. Returns a number representing
the sign of `self`, saturating on overflow.

Overflow can only occur
  * if the value is positive and the fixed-point number has zero
    or one integer bits such that it cannot hold the value 1.
  * if the value is negative and the fixed-point number has zero
    integer bits, such that it cannot hold the value &minus;1.

# Examples

```rust
use fixed::{
    types::extra::{U4, U", $s_nbits_m1, ", U", $s_nbits, "},
    ", $s_fixed, ",
};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::from_num(5).saturating_signum(), 1);
assert_eq!(Fix::ZERO.saturating_signum(), 0);
assert_eq!(Fix::from_num(-5).saturating_signum(), -1);

type OneIntBit = ", $s_fixed, "<U", $s_nbits_m1, ">;
type ZeroIntBits = ", $s_fixed, "<U", $s_nbits, ">;
assert_eq!(OneIntBit::from_num(0.5).saturating_signum(), OneIntBit::MAX);
assert_eq!(ZeroIntBits::from_num(0.25).saturating_signum(), ZeroIntBits::MAX);
assert_eq!(ZeroIntBits::from_num(-0.5).saturating_signum(), ZeroIntBits::MIN);
```
";
                    #[inline]
                    #[must_use]
                    pub const fn saturating_signum(self) -> $Fixed<Frac> {
                        match self.overflowing_signum() {
                            (ans, false) => ans,
                            (_, true) => {
                                if self.is_negative() {
                                    $Fixed::MIN
                                } else {
                                    $Fixed::MAX
                                }
                            }
                        }
                    }
                }
            }

            comment! {
                "Saturating multiplication. Returns the product, saturating on overflow.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::from_num(3).saturating_mul(Fix::from_num(2)), Fix::from_num(6));
assert_eq!(Fix::MAX.saturating_mul(Fix::from_num(2)), Fix::MAX);
```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn saturating_mul(self, rhs: $Fixed<Frac>) -> $Fixed<Frac> {
                    match arith::$Inner::overflowing_mul(self.to_bits(), rhs.to_bits(), Frac::U32) {
                        (ans, false) => Self::from_bits(ans),
                        (_, true) => {
                            if_signed_unsigned!(
                                $Signedness,
                                if self.is_negative() != rhs.is_negative() {
                                    Self::MIN
                                } else {
                                    Self::MAX
                                },
                                Self::MAX,
                            )
                        }
                    }
                }
            }

            comment! {
                "Saturating division. Returns the quotient, saturating on overflow.

# Panics

Panics if the divisor is zero.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let one_half = Fix::ONE / 2;
assert_eq!(Fix::ONE.saturating_div(Fix::from_num(2)), one_half);
assert_eq!(Fix::MAX.saturating_div(one_half), Fix::MAX);
```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn saturating_div(self, rhs: $Fixed<Frac>) -> $Fixed<Frac> {
                    match arith::$Inner::overflowing_div(self.to_bits(), rhs.to_bits(), Frac::U32) {
                        (ans, false) => Self::from_bits(ans),
                        (_, true) => {
                            if_signed_unsigned!(
                                $Signedness,
                                if self.is_negative() != rhs.is_negative() {
                                    Self::MIN
                                } else {
                                    Self::MAX
                                },
                                Self::MAX,
                            )
                        }
                    }
                }
            }

            comment! {
                "Saturating reciprocal. Returns the reciprocal,
saturating on overflow.

# Panics

Panics if the fixed-point number is zero.

# Examples

```rust
use fixed::{types::extra::U", $s_nbits_m1, ", ", $s_fixed, "};
// only one integer bit
type Fix = ", $s_fixed, "<U", $s_nbits_m1, ">;
assert_eq!(Fix::from_num(0.25).saturating_recip(), Fix::MAX);
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "assert_eq!(Fix::from_num(-0.25).saturating_recip(), Fix::MIN);
",
                },
                "```
";
                #[inline]
                #[must_use]
                pub const fn saturating_recip(self) -> $Fixed<Frac> {
                    match self.overflowing_recip() {
                        (ans, false) => ans,
                        (_, true) => {
                            if_signed_unsigned!(
                                $Signedness,
                                if self.is_negative() {
                                    Self::MIN
                                } else {
                                    Self::MAX
                                },
                                Self::MAX,
                            )
                        }
                    }
                }
            }

            comment! {
                "Saturating Euclidean division. Returns the quotient,
saturating on overflow.

# Panics

Panics if the divisor is zero.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::from_num(7.5).saturating_div_euclid(Fix::from_num(2)), Fix::from_num(3));
assert_eq!(Fix::MAX.saturating_div_euclid(Fix::from_num(0.25)), Fix::MAX);
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "assert_eq!(Fix::from_num(-7.5).saturating_div_euclid(Fix::from_num(2)), Fix::from_num(-4));
assert_eq!(Fix::MIN.saturating_div_euclid(Fix::from_num(0.25)), Fix::MIN);
",
                },
                "```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn saturating_div_euclid(self, rhs: $Fixed<Frac>) -> $Fixed<Frac> {
                    match self.overflowing_div_euclid(rhs) {
                        (val, false) => val,
                        (_, true) => {
                            if_signed_unsigned!(
                                $Signedness,
                                if self.is_negative() != rhs.is_negative() {
                                    Self::MIN
                                } else {
                                    Self::MAX
                                },
                                Self::MAX,
                            )
                        }
                    }
                }
            }

            comment! {
                "Saturating Euclidean division by an integer. Returns the quotient",
                if_signed_unsigned!(
                    $Signedness,
                    ", saturating on overflow.

Overflow can only occur when dividing the minimum value by &minus;1.",
                    ".

Can never overflow for unsigned values.",
                ),
                "

# Panics

Panics if the divisor is zero.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::from_num(7.5).saturating_div_euclid_int(2), Fix::from_num(3));
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "assert_eq!(Fix::from_num(-7.5).saturating_div_euclid_int(2), Fix::from_num(-4));
assert_eq!(Fix::MIN.saturating_div_euclid_int(-1), Fix::MAX);
",
                },
                "```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn saturating_div_euclid_int(self, rhs: $Inner) -> $Fixed<Frac> {
                    // dividing by integer can never result in something < MIN
                    match self.overflowing_div_euclid_int(rhs) {
                        (val, false) => val,
                        (_, true) => $Fixed::MAX,
                    }
                }
            }

            comment! {
                "Adds `self` to the product `a`&nbsp;×&nbsp;`b`, saturating on overflow.

",
                if_signed_else_empty_str! {
                    $Signedness;
                    "For some cases, the product `a`&nbsp;×&nbsp;`b` would
overflow on its own, but the final result `self`&nbsp;+&nbsp;`a`&nbsp;×&nbsp;`b`
is representable; in these cases this method returns the correct result without
overflow.

",
                },
                "The `a` and `b` parameters can have a fixed-point type like
`self` but with a different number of fractional bits.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(
    Fix::from_num(3).saturating_add_prod(Fix::from_num(4), Fix::from_num(0.5)),
    5
);
assert_eq!(Fix::ONE.saturating_add_prod(Fix::MAX, Fix::from_num(3)), Fix::MAX);
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "// -MAX + MAX × 1.5 = MAX / 2, which does not overflow
assert_eq!(
    (-Fix::MAX).saturating_add_prod(Fix::MAX, Fix::from_num(1.5)),
    Fix::MAX / 2
);
"
                },
                "```
";
                #[inline]
                #[must_use]
                pub const fn saturating_add_prod<AFrac: $LeEqU, BFrac: $LeEqU>(
                    self,
                    a: $Fixed<AFrac>,
                    b: $Fixed<BFrac>,
                ) -> $Fixed<Frac> {
                    let (ans, overflow) = arith::$Inner::overflowing_mul_add(
                        a.to_bits(),
                        b.to_bits(),
                        self.to_bits(),
                        AFrac::I32 + BFrac::I32 - Frac::I32,
                    );
                    if overflow {
                        if_signed_unsigned!(
                            $Signedness,
                            if a.is_negative() != b.is_negative() {
                                Self::MIN
                            } else {
                                Self::MAX
                            },
                            Self::MAX,
                        )
                    } else {
                        Self::from_bits(ans)
                    }
                }
            }

            comment! {
                "Saturating multiply and accumulate. Adds (`a` × `b`) to `self`,
saturating on overflow.

",
                if_signed_else_empty_str! {
                    $Signedness;
                    "For some cases, the product `a` × `b` would overflow on its
own, but the final result `self` + `a` × `b` is representable; in these cases
this method saves the correct result without overflow.

",
                },
                "The `a` and `b` parameters can have a fixed-point type like
`self` but with a different number of fractional bits.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let mut acc = Fix::from_num(3);
acc.saturating_mul_acc(Fix::from_num(4), Fix::from_num(0.5));
assert_eq!(acc, 5);

acc = Fix::MAX / 2;
acc.saturating_mul_acc(Fix::MAX / 2, Fix::from_num(3));
assert_eq!(acc, Fix::MAX);
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "
// MAX × 1.5 - MAX = MAX / 2, which does not overflow
acc = -Fix::MAX;
acc.saturating_mul_acc(Fix::MAX, Fix::from_num(1.5));
assert_eq!(acc, Fix::MAX / 2);
"
                },
                "```
";
                #[inline]
                pub fn saturating_mul_acc<AFrac: $LeEqU, BFrac: $LeEqU>(
                    &mut self,
                    a: $Fixed<AFrac>,
                    b: $Fixed<BFrac>,
                ) {
                    *self = self.saturating_add_prod(a, b);
                }
            }

            comment! {
                "Saturating remainder for Euclidean division by an integer. Returns the remainder",
                if_signed_unsigned!(
                    $Signedness,
                    ", saturating on overflow.",
                    ".

Can never overflow for unsigned values.",
                ),
                "

# Panics

Panics if the divisor is zero.

# Examples

```rust
use fixed::{types::extra::U", $s_nbits_m4, ", ", $s_fixed, "};
type Fix = ", $s_fixed, "<U", $s_nbits_m4, ">;
assert_eq!(Fix::from_num(7.5).saturating_rem_euclid_int(2), Fix::from_num(1.5));
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "assert_eq!(Fix::from_num(-7.5).saturating_rem_euclid_int(2), Fix::from_num(0.5));
// -8 ≤ Fix < 8, so the answer 12.5 saturates
assert_eq!(Fix::from_num(-7.5).saturating_rem_euclid_int(20), Fix::MAX);
",
                },
                "```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn saturating_rem_euclid_int(self, rhs: $Inner) -> $Fixed<Frac> {
                    match self.overflowing_rem_euclid_int(rhs) {
                        (val, false) => val,
                        (_, true) => $Fixed::MAX,
                    }
                }
            }

            comment! {
                "Linear interpolation between `start` and `end`, saturating on
overflow.

The interpolated value is
`start`&nbsp;+&nbsp;`self`&nbsp;×&nbsp;(`end`&nbsp;&minus;&nbsp;`start`). This
is `start` when `self`&nbsp;=&nbsp;0, `end` when `self`&nbsp;=&nbsp;1, and
linear interpolation for all other values of `self`. Linear extrapolation is
performed if `self` is not in the range 0&nbsp;≤&nbsp;<i>x</i>&nbsp;≤&nbsp;1.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::from_num(0.5).saturating_lerp(Fix::ZERO, Fix::MAX), Fix::MAX / 2);
assert_eq!(Fix::from_num(1.5).saturating_lerp(Fix::ZERO, Fix::MAX), Fix::MAX);
",
                if_signed_unsigned! (
                    $Signedness,
                    "assert_eq!(Fix::from_num(-2.0).saturating_lerp(Fix::ZERO, Fix::MAX), Fix::MIN);
assert_eq!(Fix::from_num(3.0).saturating_lerp(Fix::MAX, Fix::ZERO), Fix::MIN);
",
                    "assert_eq!(Fix::from_num(3.0).saturating_lerp(Fix::MAX, Fix::ZERO), Fix::ZERO);
",
                ),
                "```
";
                #[inline]
                pub const fn saturating_lerp<RangeFrac>(
                    self,
                    start: $Fixed<RangeFrac>,
                    end: $Fixed<RangeFrac>,
                ) -> $Fixed<RangeFrac> {
                    match lerp::$Inner(self.to_bits(), start.to_bits(), end.to_bits(), Frac::U32) {
                        (bits, false) => $Fixed::from_bits(bits),
                        (_, true) => if_signed_unsigned!(
                            $Signedness,
                            if self.is_negative() == (end.to_bits() < start.to_bits()) {
                                $Fixed::MAX
                            } else {
                                $Fixed::MIN
                            },
                            if end.to_bits() < start.to_bits() {
                                $Fixed::MIN
                            } else {
                                $Fixed::MAX
                            },
                        ),
                    }
                }
            }

            if_signed! {
                $Signedness;
                comment! {
                    "Wrapping signum. Returns a number representing
the sign of `self`, wrapping on overflow.

Overflow can only occur
  * if the value is positive and the fixed-point number has zero
    or one integer bits such that it cannot hold the value 1.
  * if the value is negative and the fixed-point number has zero
    integer bits, such that it cannot hold the value &minus;1.

# Examples

```rust
use fixed::{
    types::extra::{U4, U", $s_nbits_m1, ", U", $s_nbits, "},
    ", $s_fixed, ",
};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::from_num(5).wrapping_signum(), 1);
assert_eq!(Fix::ZERO.wrapping_signum(), 0);
assert_eq!(Fix::from_num(-5).wrapping_signum(), -1);

type OneIntBit = ", $s_fixed, "<U", $s_nbits_m1, ">;
type ZeroIntBits = ", $s_fixed, "<U", $s_nbits, ">;
assert_eq!(OneIntBit::from_num(0.5).wrapping_signum(), -1);
assert_eq!(ZeroIntBits::from_num(0.25).wrapping_signum(), 0);
assert_eq!(ZeroIntBits::from_num(-0.5).wrapping_signum(), 0);
```
";
                    #[inline]
                    #[must_use]
                    pub const fn wrapping_signum(self) -> $Fixed<Frac> {
                        self.overflowing_signum().0
                    }
                }
            }

            comment! {
                "Wrapping multiplication. Returns the product, wrapping on overflow.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::from_num(3).wrapping_mul(Fix::from_num(2)), Fix::from_num(6));
let wrapped = Fix::from_bits(!0 << 2);
assert_eq!(Fix::MAX.wrapping_mul(Fix::from_num(4)), wrapped);
```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn wrapping_mul(self, rhs: $Fixed<Frac>) -> $Fixed<Frac> {
                    let (ans, _) =
                        arith::$Inner::overflowing_mul(self.to_bits(), rhs.to_bits(), Frac::U32);
                    Self::from_bits(ans)
                }
            }

            comment! {
                "Wrapping division. Returns the quotient, wrapping on overflow.

# Panics

Panics if the divisor is zero.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let one_point_5 = Fix::from_bits(0b11 << (4 - 1));
assert_eq!(Fix::from_num(3).wrapping_div(Fix::from_num(2)), one_point_5);
let quarter = Fix::ONE / 4;
let wrapped = Fix::from_bits(!0 << 2);
assert_eq!(Fix::MAX.wrapping_div(quarter), wrapped);
```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn wrapping_div(self, rhs: $Fixed<Frac>) -> $Fixed<Frac> {
                    let (ans, _) =
                        arith::$Inner::overflowing_div(self.to_bits(), rhs.to_bits(), Frac::U32);
                    Self::from_bits(ans)
                }
            }

            comment! {
                "Wrapping reciprocal. Returns the reciprocal,
wrapping on overflow.

# Panics

Panics if the fixed-point number is zero.

# Examples

```rust
use fixed::{types::extra::U", $s_nbits_m1, ", ", $s_fixed, "};
// only one integer bit
type Fix = ", $s_fixed, "<U", $s_nbits_m1, ">;
assert_eq!(Fix::from_num(0.25).wrapping_recip(), Fix::ZERO);
```
";
                #[inline]
                #[must_use]
                pub const fn wrapping_recip(self) -> $Fixed<Frac> {
                    let (ans, _) = self.overflowing_recip();
                    ans
                }
            }

            comment! {
                "Wrapping Euclidean division. Returns the quotient, wrapping on overflow.

# Panics

Panics if the divisor is zero.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::from_num(7.5).wrapping_div_euclid(Fix::from_num(2)), Fix::from_num(3));
let wrapped = Fix::MAX.wrapping_mul_int(4).round_to_zero();
assert_eq!(Fix::MAX.wrapping_div_euclid(Fix::from_num(0.25)), wrapped);
```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn wrapping_div_euclid(self, rhs: $Fixed<Frac>) -> $Fixed<Frac> {
                    self.overflowing_div_euclid(rhs).0
                }
            }

            comment! {
                "Wrapping Euclidean division by an integer. Returns the quotient",
                if_signed_unsigned!(
                    $Signedness,
                    ", wrapping on overflow.

Overflow can only occur when dividing the minimum value by &minus;1.",
                    ".

Can never overflow for unsigned values.",
                ),
                "

# Panics

Panics if the divisor is zero.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::from_num(7.5).wrapping_div_euclid_int(2), Fix::from_num(3));
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "assert_eq!(Fix::from_num(-7.5).wrapping_div_euclid_int(2), Fix::from_num(-4));
let wrapped = Fix::MIN.round_to_zero();
assert_eq!(Fix::MIN.wrapping_div_euclid_int(-1), wrapped);
",
                },
                "```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn wrapping_div_euclid_int(self, rhs: $Inner) -> $Fixed<Frac> {
                    self.overflowing_div_euclid_int(rhs).0
                }
            }

            comment! {
                "Adds `self` to the product `a`&nbsp;×&nbsp;`b`, wrapping on overflow.

The `a` and `b` parameters can have a fixed-point type like `self` but with a
different number of fractional bits.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(
    Fix::from_num(3).wrapping_add_prod(Fix::from_num(4), Fix::from_num(0.5)),
    5
);
assert_eq!(
    Fix::MAX.wrapping_add_prod(Fix::MAX, Fix::from_num(3)),
    Fix::MAX.wrapping_mul_int(4)
);
```
";
                #[inline]
                #[must_use]
                pub const fn wrapping_add_prod<AFrac: $LeEqU, BFrac: $LeEqU>(
                    self,
                    a: $Fixed<AFrac>,
                    b: $Fixed<BFrac>,
                ) -> $Fixed<Frac> {
                    let (ans, _) = arith::$Inner::overflowing_mul_add(
                        a.to_bits(),
                        b.to_bits(),
                        self.to_bits(),
                        AFrac::I32 + BFrac::I32 - Frac::I32,
                    );
                    Self::from_bits(ans)
                }
            }

            comment! {
                "Wrapping multiply and accumulate. Adds (`a` × `b`) to `self`,
wrapping on overflow.

The `a` and `b` parameters can have a fixed-point type like
`self` but with a different number of fractional bits.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let mut acc = Fix::from_num(3);
acc.wrapping_mul_acc(Fix::from_num(4), Fix::from_num(0.5));
assert_eq!(acc, 5);

acc = Fix::MAX;
acc.wrapping_mul_acc(Fix::MAX, Fix::from_num(3));
assert_eq!(acc, Fix::MAX.wrapping_mul_int(4));
```
";
                #[inline]
                pub fn wrapping_mul_acc<AFrac: $LeEqU, BFrac: $LeEqU>(
                    &mut self,
                    a: $Fixed<AFrac>,
                    b: $Fixed<BFrac>,
                ) {
                    *self = self.wrapping_add_prod(a, b);
                }
            }

            comment! {
                "Wrapping remainder for Euclidean division by an integer. Returns the remainder",
                if_signed_unsigned!(
                    $Signedness,
                    ", wrapping on overflow.

Note that while remainder for Euclidean division cannot be negative,
the wrapped value can be negative.",
                    ".

Can never overflow for unsigned values.",
                ),
                "

# Panics

Panics if the divisor is zero.

# Examples

```rust
use fixed::{types::extra::U", $s_nbits_m4, ", ", $s_fixed, "};
type Fix = ", $s_fixed, "<U", $s_nbits_m4, ">;
assert_eq!(Fix::from_num(7.5).wrapping_rem_euclid_int(2), Fix::from_num(1.5));
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "assert_eq!(Fix::from_num(-7.5).wrapping_rem_euclid_int(2), Fix::from_num(0.5));
// -8 ≤ Fix < 8, so the answer 12.5 wraps to -3.5
assert_eq!(Fix::from_num(-7.5).wrapping_rem_euclid_int(20), Fix::from_num(-3.5));
",
                },
                "```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn wrapping_rem_euclid_int(self, rhs: $Inner) -> $Fixed<Frac> {
                    self.overflowing_rem_euclid_int(rhs).0
                }
            }

            comment! {
                "Linear interpolation between `start` and `end`, wrapping on
overflow.

The interpolated value is
`start`&nbsp;+&nbsp;`self`&nbsp;×&nbsp;(`end`&nbsp;&minus;&nbsp;`start`). This
is `start` when `self`&nbsp;=&nbsp;0, `end` when `self`&nbsp;=&nbsp;1, and
linear interpolation for all other values of `self`. Linear extrapolation is
performed if `self` is not in the range 0&nbsp;≤&nbsp;<i>x</i>&nbsp;≤&nbsp;1.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::from_num(0.5).wrapping_lerp(Fix::ZERO, Fix::MAX), Fix::MAX / 2);
assert_eq!(
    Fix::from_num(1.5).wrapping_lerp(Fix::ZERO, Fix::MAX),
    Fix::MAX.wrapping_add(Fix::MAX / 2)
);
```
";
                #[inline]
                pub const fn wrapping_lerp<RangeFrac>(
                    self,
                    start: $Fixed<RangeFrac>,
                    end: $Fixed<RangeFrac>,
                ) -> $Fixed<RangeFrac> {
                    let (bits, _) =
                        lerp::$Inner(self.to_bits(), start.to_bits(), end.to_bits(), Frac::U32);
                    $Fixed::from_bits(bits)
                }
            }

            if_signed! {
                $Signedness;
                comment! {
                    "Unwrapped signum. Returns a number representing
the sign of `self`, panicking on overflow.

Overflow can only occur
  * if the value is positive and the fixed-point number has zero
    or one integer bits such that it cannot hold the value 1.
  * if the value is negative and the fixed-point number has zero
    integer bits, such that it cannot hold the value &minus;1.

# Panics

Panics if the result does not fit.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::from_num(5).unwrapped_signum(), 1);
assert_eq!(Fix::ZERO.unwrapped_signum(), 0);
assert_eq!(Fix::from_num(-5).unwrapped_signum(), -1);
```

The following panics because of overflow.

```should_panic
use fixed::{types::extra::U", $s_nbits_m1, ", ", $s_fixed, "};
type OneIntBit = ", $s_fixed, "<U", $s_nbits_m1, ">;
let _overflow = OneIntBit::from_num(0.5).unwrapped_signum();
```
";
                    #[inline]
                    #[track_caller]
                    #[must_use]
                    pub const fn unwrapped_signum(self) -> $Fixed<Frac> {
                        let (ans, overflow) = self.overflowing_signum();
                        assert!(!overflow, "overflow");
                        ans
                    }
                }
            }

            comment! {
                "Unwrapped multiplication. Returns the product, panicking on overflow.

# Panics

Panics if the result does not fit.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::from_num(3).unwrapped_mul(Fix::from_num(2)), Fix::from_num(6));
```

The following panics because of overflow.

```should_panic
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let _overflow = Fix::MAX.unwrapped_mul(Fix::from_num(4));
```
";
                #[inline]
                #[track_caller]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn unwrapped_mul(self, rhs: $Fixed<Frac>) -> $Fixed<Frac> {
                    match self.checked_mul(rhs) {
                        Some(s) => s,
                        None => panic!("overflow"),
                    }
                }
            }

            comment! {
                "Unwrapped division. Returns the quotient, panicking on overflow.

# Panics

Panics if the divisor is zero or if the division results in overflow.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let one_point_5 = Fix::from_bits(0b11 << (4 - 1));
assert_eq!(Fix::from_num(3).unwrapped_div(Fix::from_num(2)), one_point_5);
```

The following panics because of overflow.

```should_panic
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let quarter = Fix::ONE / 4;
let _overflow = Fix::MAX.unwrapped_div(quarter);
```
";
                #[inline]
                #[track_caller]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn unwrapped_div(self, rhs: $Fixed<Frac>) -> $Fixed<Frac> {
                    match self.overflowing_div(rhs) {
                        (_, true) => panic!("overflow"),
                        (ans, false) => ans,
                    }
                }
            }

            comment! {
                "Unwrapped reciprocal. Returns the reciprocal,
panicking on overflow.

# Panics

Panics if the fixed-point number is zero or on overflow.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::from_num(0.25).unwrapped_recip(), Fix::from_num(4));
```
";
                #[inline]
                #[track_caller]
                #[must_use]
                pub const fn unwrapped_recip(self) -> $Fixed<Frac> {
                    match self.overflowing_recip() {
                        (_, true) => panic!("overflow"),
                        (ans, false) => ans,
                    }
                }
            }

            comment! {
                "Unwrapped Euclidean division. Returns the quotient, panicking on overflow.

# Panics

Panics if the divisor is zero or if the division results in overflow.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::from_num(7.5).unwrapped_div_euclid(Fix::from_num(2)), Fix::from_num(3));
```

The following panics because of overflow.

```should_panic
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let _overflow = Fix::MAX.unwrapped_div_euclid(Fix::from_num(0.25));
```
";
                #[inline]
                #[track_caller]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn unwrapped_div_euclid(self, rhs: $Fixed<Frac>) -> $Fixed<Frac> {
                    match self.overflowing_div_euclid(rhs) {
                        (_, true) => panic!("overflow"),
                        (ans, false) => ans,
                    }
                }
            }

            comment! {
                "Adds `self` to the product `a`&nbsp;×&nbsp;`b`, panicking on overflow.

",
                if_signed_else_empty_str! {
                    $Signedness;
                    "For some cases, the product `a`&nbsp;×&nbsp;`b` would
overflow on its own, but the final result `self`&nbsp;+&nbsp;`a`&nbsp;×&nbsp;`b`
is representable; in these cases this method returns the correct result without
overflow.

",
                },
                "The `a` and `b` parameters can have a fixed-point type like
`self` but with a different number of fractional bits.

# Panics

Panics if the result does not fit.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(
    Fix::from_num(3).unwrapped_add_prod(Fix::from_num(4), Fix::from_num(0.5)),
    5
);
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "// -MAX + MAX × 1.5 = MAX / 2, which does not overflow
assert_eq!(
    (-Fix::MAX).unwrapped_add_prod(Fix::MAX, Fix::from_num(1.5)),
    Fix::MAX / 2
);
"
                },
                "```

The following panics because of overflow.

```should_panic
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let _overflow = Fix::DELTA.unwrapped_add_prod(Fix::MAX, Fix::ONE);
```
";
                #[inline]
                #[must_use]
                #[track_caller]
                pub const fn unwrapped_add_prod<AFrac: $LeEqU, BFrac: $LeEqU>(
                    self,
                    a: $Fixed<AFrac>,
                    b: $Fixed<BFrac>,
                ) -> $Fixed<Frac> {
                    let (ans, overflow) = arith::$Inner::overflowing_mul_add(
                        a.to_bits(),
                        b.to_bits(),
                        self.to_bits(),
                        AFrac::I32 + BFrac::I32 - Frac::I32,
                    );
                    assert!(!overflow, "overflow");
                    Self::from_bits(ans)
                }
            }

            comment! {
                "Unwrapped multiply and accumulate. Adds (`a` × `b`) to `self`,
panicking on overflow.

",
                if_signed_else_empty_str! {
                    $Signedness;
                    "For some cases, the product `a` × `b` would overflow on its
own, but the final result `self` + `a` × `b` is representable; in these cases
this method saves the correct result without overflow.

",
                },
                "The `a` and `b` parameters can have a fixed-point type like
`self` but with a different number of fractional bits.

# Panics

Panics if the result does not fit.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let mut acc = Fix::from_num(3);
acc.unwrapped_mul_acc(Fix::from_num(4), Fix::from_num(0.5));
assert_eq!(acc, 5);
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "
// MAX × 1.5 - MAX = MAX / 2, which does not overflow
acc = -Fix::MAX;
acc.unwrapped_mul_acc(Fix::MAX, Fix::from_num(1.5));
assert_eq!(acc, Fix::MAX / 2);
"
                },
                "```

The following panics because of overflow.

```should_panic
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let mut acc = Fix::DELTA;
acc.unwrapped_mul_acc(Fix::MAX, Fix::ONE);
```
";
                #[inline]
                #[track_caller]
                pub fn unwrapped_mul_acc<AFrac: $LeEqU, BFrac: $LeEqU>(
                    &mut self,
                    a: $Fixed<AFrac>,
                    b: $Fixed<BFrac>,
                ) {
                    *self = self.unwrapped_add_prod(a, b);
                }
            }

            comment! {
                "Unwrapped fixed-point remainder for division by an integer.
Returns the remainder, panicking if the divisor is zero.

# Panics

Panics if the divisor is zero.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::from_num(3.75).unwrapped_rem_int(2), Fix::from_num(1.75));
```

The following panics because the divisor is zero.

```should_panic
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let _divisor_is_zero = Fix::from_num(3.75).unwrapped_rem_int(0);
```
";
                #[inline]
                #[track_caller]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn unwrapped_rem_int(self, rhs: $Inner) -> $Fixed<Frac> {
                    match self.checked_rem_int(rhs) {
                        Some(ans) => ans,
                        None => panic!("division by zero"),
                    }
                }
            }

            comment! {
                "Unwrapped Euclidean division by an integer. Returns the quotient",
                if_signed_unsigned!(
                    $Signedness,
                    ", panicking on overflow.

Overflow can only occur when dividing the minimum value by &minus;1.",
                    ".

Can never overflow for unsigned values.",
                ),
                "

# Panics

Panics if the divisor is zero",
                if_signed_else_empty_str! {
                    $Signedness;
                    " or if the division results in overflow",
                },
                ".

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::from_num(7.5).unwrapped_div_euclid_int(2), Fix::from_num(3));
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "assert_eq!(Fix::from_num(-7.5).unwrapped_div_euclid_int(2), Fix::from_num(-4));
```

The following panics because of overflow.

```should_panic
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let _overflow = Fix::MIN.unwrapped_div_euclid_int(-1);
",
                },
                "```
";
                #[inline]
                #[track_caller]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn unwrapped_div_euclid_int(self, rhs: $Inner) -> $Fixed<Frac> {
                    match self.overflowing_div_euclid_int(rhs) {
                        (_, true) => panic!("overflow"),
                        (ans, false) => ans,
                    }
                }
            }

            comment! {
                "Unwrapped remainder for Euclidean division by an integer. Returns the remainder",
                if_signed_unsigned!(
                    $Signedness,
                    ", panicking on overflow.

Note that while remainder for Euclidean division cannot be negative,
the wrapped value can be negative.",
                    ".

Can never overflow for unsigned values.",
                ),
                "

# Panics

Panics if the divisor is zero",
                if_signed_else_empty_str! {
                    $Signedness;
                    " or if the division results in overflow",
                },
                ".

# Examples

```rust
use fixed::{types::extra::U", $s_nbits_m4, ", ", $s_fixed, "};
type Fix = ", $s_fixed, "<U", $s_nbits_m4, ">;
assert_eq!(Fix::from_num(7.5).unwrapped_rem_euclid_int(2), Fix::from_num(1.5));
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "assert_eq!(Fix::from_num(-7.5).unwrapped_rem_euclid_int(2), Fix::from_num(0.5));
```

The following panics because of overflow.

```should_panic
use fixed::{types::extra::U", $s_nbits_m4, ", ", $s_fixed, "};
type Fix = ", $s_fixed, "<U", $s_nbits_m4, ">;
// -8 ≤ Fix < 8, so the answer 12.5 overflows
let _overflow = Fix::from_num(-7.5).unwrapped_rem_euclid_int(20);
",
                },
                "```
";
                #[inline]
                #[track_caller]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn unwrapped_rem_euclid_int(self, rhs: $Inner) -> $Fixed<Frac> {
                    match self.overflowing_rem_euclid_int(rhs) {
                        (_, true) => panic!("overflow"),
                        (ans, false) => ans,
                    }
                }
            }

            comment! {
                "Linear interpolation between `start` and `end`, panicking on
overflow.

The interpolated value is
`start`&nbsp;+&nbsp;`self`&nbsp;×&nbsp;(`end`&nbsp;&minus;&nbsp;`start`). This
is `start` when `self`&nbsp;=&nbsp;0, `end` when `self`&nbsp;=&nbsp;1, and
linear interpolation for all other values of `self`. Linear extrapolation is
performed if `self` is not in the range 0&nbsp;≤&nbsp;<i>x</i>&nbsp;≤&nbsp;1.

# Panics

Panics if the result overflows.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::from_num(0.5).unwrapped_lerp(Fix::ZERO, Fix::MAX), Fix::MAX / 2);
```

The following panics because of overflow.

```should_panic
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let _overflow = Fix::from_num(1.5).unwrapped_lerp(Fix::ZERO, Fix::MAX);
```
";
                #[inline]
                pub const fn unwrapped_lerp<RangeFrac>(
                    self,
                    start: $Fixed<RangeFrac>,
                    end: $Fixed<RangeFrac>,
                ) -> $Fixed<RangeFrac> {
                    match lerp::$Inner(self.to_bits(), start.to_bits(), end.to_bits(), Frac::U32) {
                        (bits, false) => $Fixed::from_bits(bits),
                        (_, true) => panic!("overflow"),
                    }
                }
            }

            if_signed! {
                $Signedness;
                comment! {
                    "Overflowing signum.

Returns a [tuple] of the signum and a [`bool`] indicating whether an
overflow has occurred. On overflow, the wrapped value is returned.

Overflow can only occur
  * if the value is positive and the fixed-point number has zero
    or one integer bits such that it cannot hold the value 1.
  * if the value is negative and the fixed-point number has zero
    integer bits, such that it cannot hold the value &minus;1.

# Examples

```rust
use fixed::{
    types::extra::{U4, U", $s_nbits_m1, ", U", $s_nbits, "},
    ", $s_fixed, ",
};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::from_num(5).overflowing_signum(), (Fix::ONE, false));
assert_eq!(Fix::ZERO.overflowing_signum(), (Fix::ZERO, false));
assert_eq!(Fix::from_num(-5).overflowing_signum(), (Fix::NEG_ONE, false));

type OneIntBit = ", $s_fixed, "<U", $s_nbits_m1, ">;
type ZeroIntBits = ", $s_fixed, "<U", $s_nbits, ">;
assert_eq!(OneIntBit::from_num(0.5).overflowing_signum(), (OneIntBit::NEG_ONE, true));
assert_eq!(ZeroIntBits::from_num(0.25).overflowing_signum(), (ZeroIntBits::ZERO, true));
assert_eq!(ZeroIntBits::from_num(-0.5).overflowing_signum(), (ZeroIntBits::ZERO, true));
```
";
                    #[inline]
                    pub const fn overflowing_signum(self) -> ($Fixed<Frac>, bool) {
                        if self.to_bits() == 0 {
                            ($Fixed::ZERO, false)
                        } else if Frac::U32 == $Inner::BITS {
                            ($Fixed::ZERO, true)
                        } else if self.to_bits() < 0 {
                            ($Fixed::from_bits(-1 << Frac::U32), false)
                        } else {
                            ($Fixed::from_bits(1 << Frac::U32), Frac::U32 == $Inner::BITS - 1)
                        }
                    }
                }
            }

            comment! {
                "Overflowing multiplication.

Returns a [tuple] of the product and a [`bool`] indicating whether an
overflow has occurred. On overflow, the wrapped value is returned.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::from_num(3).overflowing_mul(Fix::from_num(2)), (Fix::from_num(6), false));
let wrapped = Fix::from_bits(!0 << 2);
assert_eq!(Fix::MAX.overflowing_mul(Fix::from_num(4)), (wrapped, true));
```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn overflowing_mul(self, rhs: $Fixed<Frac>) -> ($Fixed<Frac>, bool) {
                    let (ans, overflow) =
                        arith::$Inner::overflowing_mul(self.to_bits(), rhs.to_bits(), Frac::U32);
                    (Self::from_bits(ans), overflow)
                }
            }

            comment! {
                "Overflowing division.

Returns a [tuple] of the quotient and a [`bool`] indicating whether an
overflow has occurred. On overflow, the wrapped value is returned.

# Panics

Panics if the divisor is zero.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let one_point_5 = Fix::from_bits(0b11 << (4 - 1));
assert_eq!(Fix::from_num(3).overflowing_div(Fix::from_num(2)), (one_point_5, false));
let quarter = Fix::ONE / 4;
let wrapped = Fix::from_bits(!0 << 2);
assert_eq!(Fix::MAX.overflowing_div(quarter), (wrapped, true));
```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn overflowing_div(self, rhs: $Fixed<Frac>) -> ($Fixed<Frac>, bool) {
                    let (ans, overflow) =
                        arith::$Inner::overflowing_div(self.to_bits(), rhs.to_bits(), Frac::U32);
                    (Self::from_bits(ans), overflow)
                }
            }

            comment! {
                "Overflowing reciprocal.

Returns a [tuple] of the reciprocal and a [`bool`] indicating whether
an overflow has occurred. On overflow, the wrapped value is returned.

# Panics

Panics if the fixed-point number is zero.

# Examples

```rust
use fixed::{
    types::extra::{U4, U", $s_nbits_m1, "},
    ", $s_fixed, ",
};
type Fix = ", $s_fixed, "<U4>;
// only one integer bit
type Small = ", $s_fixed, "<U", $s_nbits_m1, ">;
assert_eq!(Fix::from_num(0.25).overflowing_recip(), (Fix::from_num(4), false));
assert_eq!(Small::from_num(0.25).overflowing_recip(), (Small::ZERO, true));
```
";
                #[inline]
                pub const fn overflowing_recip(self) -> ($Fixed<Frac>, bool) {
                    if let Some(one) = Self::TRY_ONE {
                        return one.overflowing_div(self);
                    }
                    if_signed! {
                        $Signedness;
                        let (neg, abs) = int_helper::$Inner::neg_abs(self.to_bits());
                        let uns_abs = $UFixed::<Frac>::from_bits(abs);
                        let (uns_wrapped, overflow1) = uns_abs.overflowing_recip();
                        let wrapped = $Fixed::<Frac>::from_bits(uns_wrapped.to_bits() as $Inner);
                        let overflow2 = wrapped.is_negative();
                        if wrapped.to_bits() == $Inner::MIN && neg {
                            return (wrapped, overflow1);
                        }
                        if neg {
                            // if we do not have overflow yet, we will not overflow now
                            (wrapped.wrapping_neg(), overflow1 | overflow2)
                        } else {
                            (wrapped, overflow1 | overflow2)
                        }
                    }
                    if_unsigned! {
                        $Signedness;
                        // 0 < x < 1: 1/x = 1 + (1 - x) / x, wrapped to (1 - x) / x
                        // x.wrapping_neg() = 1 - x

                        // x = 0: we still get division by zero

                        (self.wrapping_neg().wrapping_div(self), true)
                    }
                }
            }

            comment! {
                "Overflowing Euclidean division.

Returns a [tuple] of the quotient and a [`bool`] indicating whether an
overflow has occurred. On overflow, the wrapped value is returned.

# Panics

Panics if the divisor is zero.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let check = Fix::from_num(3);
assert_eq!(Fix::from_num(7.5).overflowing_div_euclid(Fix::from_num(2)), (check, false));
let wrapped = Fix::MAX.wrapping_mul_int(4).round_to_zero();
assert_eq!(Fix::MAX.overflowing_div_euclid(Fix::from_num(0.25)), (wrapped, true));
```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn overflowing_div_euclid(
                    self,
                    rhs: $Fixed<Frac>,
                ) -> ($Fixed<Frac>, bool) {
                    let (mut q, overflow) = self.overflowing_div(rhs);
                    q = q.round_to_zero();
                    if_signed! {
                        $Signedness;
                        if self.unwrapped_rem(rhs).is_negative() {
                            let neg_one = match Self::TRY_NEG_ONE {
                                Some(s) => s,
                                None => return (q, true),
                            };
                            let (q, overflow2) = if rhs.is_positive() {
                                q.overflowing_add(neg_one)
                            } else {
                                q.overflowing_sub(neg_one)
                            };
                            return (q, overflow | overflow2);
                        }
                    }
                    (q, overflow)
                }
            }

            comment! {
                "Overflowing Euclidean division by an integer.

Returns a [tuple] of the quotient and ",
                if_signed_unsigned!(
                    $Signedness,
                    "a [`bool`] indicating whether an overflow has
occurred. On overflow, the wrapped value is returned. Overflow can
only occur when dividing the minimum value by &minus;1.",
                    "[`false`], as the division can never overflow for unsigned values.",
                ),
                "

# Panics

Panics if the divisor is zero.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::from_num(7.5).overflowing_div_euclid_int(2), (Fix::from_num(3), false));
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "assert_eq!(Fix::from_num(-7.5).overflowing_div_euclid_int(2), (Fix::from_num(-4), false));
let wrapped = Fix::MIN.round_to_zero();
assert_eq!(Fix::MIN.overflowing_div_euclid_int(-1), (wrapped, true));
",
                },
                "```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn overflowing_div_euclid_int(self, rhs: $Inner) -> ($Fixed<Frac>, bool) {
                    let (mut q, overflow) = self.overflowing_div_int(rhs);
                    q = q.round_to_zero();
                    if_signed! {
                        $Signedness;
                        if self.unwrapped_rem_int(rhs).is_negative() {
                            let neg_one = match Self::TRY_NEG_ONE {
                                Some(s) => s,
                                None => return (q, true),
                            };
                            let (q, overflow2) = if rhs.is_positive() {
                                q.overflowing_add(neg_one)
                            } else {
                                q.overflowing_sub(neg_one)
                            };
                            return (q, overflow | overflow2);
                        }
                    }
                    (q, overflow)
                }
            }

            comment! {
                "Adds `self` to the product `a`&nbsp;×&nbsp;`b`.

Returns a [tuple] of the result and a [`bool`] indicating whether an overflow
has occurred. On overflow, the wrapped value is returned.

",
                if_signed_else_empty_str! {
                    $Signedness;
                    "For some cases, the product `a`&nbsp;×&nbsp;`b` would
overflow on its own, but the final result `self`&nbsp;+&nbsp;`a`&nbsp;×&nbsp;`b`
is representable; in these cases this method returns the correct result without
overflow.

",
                },
                "The `a` and `b` parameters can have a fixed-point type like
`self` but with a different number of fractional bits.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(
    Fix::from_num(3).overflowing_add_prod(Fix::from_num(4), Fix::from_num(0.5)),
    (Fix::from_num(5), false)
);
assert_eq!(
    Fix::MAX.overflowing_add_prod(Fix::MAX, Fix::from_num(3)),
    (Fix::MAX.wrapping_mul_int(4), true)
);
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "// -MAX + MAX × 1.5 = MAX / 2, which does not overflow
assert_eq!(
    (-Fix::MAX).overflowing_add_prod(Fix::MAX, Fix::from_num(1.5)),
    (Fix::MAX / 2, false)
);
"
                },
                "```
";
                #[inline]
                #[must_use]
                pub const fn overflowing_add_prod<AFrac: $LeEqU, BFrac: $LeEqU>(
                    self,
                    a: $Fixed<AFrac>,
                    b: $Fixed<BFrac>,
                ) -> ($Fixed<Frac>, bool) {
                    let (ans, overflow) = arith::$Inner::overflowing_mul_add(
                        a.to_bits(),
                        b.to_bits(),
                        self.to_bits(),
                        AFrac::I32 + BFrac::I32 - Frac::I32,
                    );
                    (Self::from_bits(ans), overflow)
                }
            }

            comment! {
                "Overflowing multiply and accumulate. Adds (`a` × `b`) to `self`,
wrapping and returning [`true`] if overflow occurs.

",
                if_signed_else_empty_str! {
                    $Signedness;
                    "For some cases, the product `a` × `b` would overflow on its
own, but the final result `self` + `a` × `b` is representable; in these cases
this method saves the correct result without overflow.

",
                },
                "The `a` and `b` parameters can have a fixed-point type like
`self` but with a different number of fractional bits.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let mut acc = Fix::from_num(3);
assert!(!acc.overflowing_mul_acc(Fix::from_num(4), Fix::from_num(0.5)));
assert_eq!(acc, 5);

acc = Fix::MAX;
assert!(acc.overflowing_mul_acc(Fix::MAX, Fix::from_num(3)));
assert_eq!(acc, Fix::MAX.wrapping_mul_int(4));
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "
// MAX × 1.5 - MAX = MAX / 2, which does not overflow
acc = -Fix::MAX;
assert!(!acc.overflowing_mul_acc(Fix::MAX, Fix::from_num(1.5)));
assert_eq!(acc, Fix::MAX / 2);
"
                },
                "```
";
                #[inline]
                #[must_use = "this returns whether overflow occurs; use `wrapping_mul_acc` if the flag is not needed"]
                pub fn overflowing_mul_acc<AFrac: $LeEqU, BFrac: $LeEqU>(
                    &mut self,
                    a: $Fixed<AFrac>,
                    b: $Fixed<BFrac>,
                ) -> bool {
                    let (ans, overflow) = self.overflowing_add_prod(a, b);
                    *self = ans;
                    overflow
                }
            }

            comment! {
                "Remainder for Euclidean division by an integer.

Returns a [tuple] of the remainder and ",
                if_signed_unsigned!(
                    $Signedness,
                    "a [`bool`] indicating whether an overflow has
occurred. On overflow, the wrapped value is returned.

Note that while remainder for Euclidean division cannot be negative,
the wrapped value can be negative.",
                    "[`false`], as this can never overflow for unsigned values.",
                ),
                "

# Panics

Panics if the divisor is zero.

# Examples

```rust
use fixed::{types::extra::U", $s_nbits_m4, ", ", $s_fixed, "};
type Fix = ", $s_fixed, "<U", $s_nbits_m4, ">;
assert_eq!(Fix::from_num(7.5).overflowing_rem_euclid_int(2), (Fix::from_num(1.5), false));
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "assert_eq!(Fix::from_num(-7.5).overflowing_rem_euclid_int(2), (Fix::from_num(0.5), false));
// -8 ≤ Fix < 8, so the answer 12.5 wraps to -3.5
assert_eq!(Fix::from_num(-7.5).overflowing_rem_euclid_int(20), (Fix::from_num(-3.5), true));
",
                },
                "```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn overflowing_rem_euclid_int(self, rhs: $Inner) -> ($Fixed<Frac>, bool) {
                    if_signed! {
                        $Signedness;
                        let rem = self.unwrapped_rem_int(rhs);
                        if !rem.is_negative() {
                            return (rem, false);
                        }
                        // Work in unsigned.
                        // Answer required is |rhs| - |rem|, but rhs is int, rem is fixed.
                        // INT_NBITS == 0 is a special case, as fraction can be negative.
                        if Self::INT_NBITS == 0 {
                            // -0.5 <= rem < 0, so euclidean remainder is in the range
                            // 0.5 <= answer < 1, which does not fit.
                            return (rem, true);
                        }
                        let rhs_abs = rhs.wrapping_abs() as $UInner;
                        let remb = rem.to_bits();
                        let remb_abs = remb.wrapping_neg() as $UInner;
                        let rem_int_abs = remb_abs >> Self::FRAC_NBITS;
                        let rem_frac = remb & Self::FRAC_MASK;
                        let ans_int = rhs_abs - rem_int_abs - if rem_frac > 0 { 1 } else { 0 };
                        let (ansb_abs, overflow1) = if ans_int == 0 {
                            (0, false)
                        } else if Self::FRAC_NBITS <= ans_int.leading_zeros() {
                            (ans_int << Self::FRAC_NBITS, false)
                        } else if Self::FRAC_NBITS == $Inner::BITS {
                            (0, true)
                        } else {
                            (ans_int << Self::FRAC_NBITS, true)
                        };
                        let ansb = ansb_abs as $Inner;
                        let overflow2 = ansb.is_negative();
                        (Self::from_bits(ansb | rem_frac), overflow1 | overflow2)
                    }
                    if_unsigned! {
                        $Signedness;
                        (self.unwrapped_rem_int(rhs), false)
                    }
                }
            }

            comment! {
                "Overflowing linear interpolation between `start` and `end`.

Returns a [tuple] of the result and a [`bool`] indicationg whether an overflow
has occurred. On overflow, the wrapped value is returned.

The interpolated value is
`start`&nbsp;+&nbsp;`self`&nbsp;×&nbsp;(`end`&nbsp;&minus;&nbsp;`start`). This
is `start` when `self`&nbsp;=&nbsp;0, `end` when `self`&nbsp;=&nbsp;1, and
linear interpolation for all other values of `self`. Linear extrapolation is
performed if `self` is not in the range 0&nbsp;≤&nbsp;<i>x</i>&nbsp;≤&nbsp;1.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(
    Fix::from_num(0.5).overflowing_lerp(Fix::ZERO, Fix::MAX),
    (Fix::MAX / 2, false)
);
assert_eq!(
    Fix::from_num(1.5).overflowing_lerp(Fix::ZERO, Fix::MAX),
    (Fix::MAX.wrapping_add(Fix::MAX / 2), true)
);
```
";
                #[inline]
                pub const fn overflowing_lerp<RangeFrac>(
                    self,
                    start: $Fixed<RangeFrac>,
                    end: $Fixed<RangeFrac>,
                ) -> ($Fixed<RangeFrac>, bool) {
                    match lerp::$Inner(self.to_bits(), start.to_bits(), end.to_bits(), Frac::U32) {
                        (bits, overflow) => ($Fixed::from_bits(bits), overflow),
                    }
                }
            }

            const TRY_ONE: Option<Self> =
                if Self::FRAC_NBITS < $Inner::BITS - if_signed_unsigned!($Signedness, 1, 0) {
                    Some(Self::DELTA.unwrapped_shl(Self::FRAC_NBITS))
                } else {
                    None
                };

            if_signed! {
                $Signedness;
                const TRY_NEG_ONE: Option<Self> = if Self::FRAC_NBITS < $Inner::BITS {
                    Some(Self::DELTA.unwrapped_neg().unwrapped_shl(Self::FRAC_NBITS))
                } else {
                    None
                };
            }
        }
    };
}
