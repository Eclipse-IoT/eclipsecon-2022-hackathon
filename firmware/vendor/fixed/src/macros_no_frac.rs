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

macro_rules! fixed_no_frac {
    (
        $Fixed:ident[$s_fixed:expr](
            $Inner:ident[$s_inner:expr], $LeEqU:tt, $UNbits:ident, $UNbits_m1:ident,
            $s_nbits:expr, $s_nbits_p1:expr, $s_nbits_m1:expr, $s_nbits_m2:expr, $s_nbits_m3:expr
        ),
        $nbytes:expr, $bytes_val:expr, $rev_bytes_val:expr, $be_bytes:expr, $le_bytes:expr,
        $IFixed:ident[$s_ifixed:expr], $UFixed:ident[$s_ufixed:expr],
        $IInner:ty, $UInner:ty, $Signedness:tt,
        $HasDouble:tt, $s_nbits_2:expr,
        $Double:ident[$s_double:expr], $DoubleInner:ty, $IDouble:ident, $IDoubleInner:ty
    ) => {
        /// The implementation of items in this block is independent
        /// of the number of fractional bits `Frac`.
        impl<Frac> $Fixed<Frac> {
            comment! {
                "Zero.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::ZERO, Fix::from_bits(0));
```
";
                pub const ZERO: $Fixed<Frac> = Self::from_bits(0);
            }

            comment! {
                "The difference between any two successive representable numbers, <i>Δ</i>.

If the number has <i>f</i>&nbsp;=&nbsp;`Frac` fractional bits, then
<i>Δ</i>&nbsp;=&nbsp;1/2<sup><i>f</i></sup>.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::DELTA, Fix::from_bits(1));
// binary 0.0001 is decimal 0.0625
assert_eq!(Fix::DELTA, 0.0625);
```
";
                pub const DELTA: $Fixed<Frac> = Self::from_bits(1);
            }

            comment! {
                "The smallest value that can be represented.

",
                if_signed_unsigned! {
                    $Signedness,
                    concat!(
                        "If the number has <i>f</i>&nbsp;=&nbsp;`Frac` fractional bits,
then the minimum is &minus;2<sup>", $s_nbits_m1, "</sup>/2<sup><i>f</i></sup>."
                    ),
                    "The minimum of unsigned numbers is 0."
                },
                "

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::MIN, Fix::from_bits(", $s_inner, "::MIN));
```
";
                pub const MIN: $Fixed<Frac> = Self::from_bits(<$Inner>::MIN);
            }

            comment! {
                "The largest value that can be represented.

If the number has <i>f</i>&nbsp;=&nbsp;`Frac` fractional bits, then the maximum is
(2<sup>",
                if_signed_unsigned!($Signedness, $s_nbits_m1, $s_nbits),
                "</sup>&nbsp;&minus;&nbsp;1)/2<sup><i>f</i></sup>.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::MAX, Fix::from_bits(", $s_inner, "::MAX));
```
";
                pub const MAX: $Fixed<Frac> = Self::from_bits(<$Inner>::MAX);
            }

            comment! {
                if_signed_unsigned!($Signedness, "[`true`]", "[`false`]"),
                "[`bool`] because the [`", $s_fixed, "`] type is ",
                if_signed_unsigned!($Signedness, "signed", "unsigned"),
                ".

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert!(", if_signed_unsigned!($Signedness, "", "!"), "Fix::IS_SIGNED);
```
";
                pub const IS_SIGNED: bool = if_signed_unsigned!($Signedness, true, false);
            }

            comment! {
                "Creates a fixed-point number that has a bitwise
representation identical to the given integer.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
// 0010.0000 = 2
assert_eq!(Fix::from_bits(0b10_0000), 2);
```
";
                #[inline]
                pub const fn from_bits(bits: $Inner) -> $Fixed<Frac> {
                    $Fixed {
                        bits,
                        phantom: PhantomData,
                    }
                }
            }

            comment! {
                "Creates an integer that has a bitwise representation
identical to the given fixed-point number.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
// 2 is 0010.0000
assert_eq!(Fix::from_num(2).to_bits(), 0b10_0000);
```
";
                #[inline]
                pub const fn to_bits(self) -> $Inner {
                    self.bits
                }
            }

            comment! {
                "Converts a fixed-point number from big endian to the target’s endianness.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let f = Fix::from_bits(", $bytes_val, ");
if cfg!(target_endian = \"big\") {
    assert_eq!(Fix::from_be(f), f);
} else {
    assert_eq!(Fix::from_be(f), f.swap_bytes());
}
```
";
                #[inline]
                pub const fn from_be(f: $Fixed<Frac>) -> $Fixed<Frac> {
                    $Fixed::from_bits(<$Inner>::from_be(f.to_bits()))
                }
            }

            comment! {
                "Converts a fixed-point number from little endian to the target’s endianness.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let f = Fix::from_bits(", $bytes_val, ");
if cfg!(target_endian = \"little\") {
    assert_eq!(Fix::from_le(f), f);
} else {
    assert_eq!(Fix::from_le(f), f.swap_bytes());
}
```
";
                #[inline]
                pub const fn from_le(f: $Fixed<Frac>) -> $Fixed<Frac> {
                    $Fixed::from_bits(<$Inner>::from_le(f.to_bits()))
                }
            }

            comment! {
                "Converts `self` to big endian from the target’s endianness.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let f = Fix::from_bits(", $bytes_val, ");
if cfg!(target_endian = \"big\") {
    assert_eq!(f.to_be(), f);
} else {
    assert_eq!(f.to_be(), f.swap_bytes());
}
```
";
                #[inline]
                #[must_use]
                pub const fn to_be(self) -> $Fixed<Frac> {
                    $Fixed::from_bits(self.to_bits().to_be())
                }
            }

            comment! {
                "Converts `self` to little endian from the target’s endianness.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let f = Fix::from_bits(", $bytes_val, ");
if cfg!(target_endian = \"little\") {
    assert_eq!(f.to_le(), f);
} else {
    assert_eq!(f.to_le(), f.swap_bytes());
}
```
";
                #[inline]
                #[must_use]
                pub const fn to_le(self) -> $Fixed<Frac> {
                    $Fixed::from_bits(self.to_bits().to_le())
                }
            }

            comment! {
                "Reverses the byte order of the fixed-point number.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let f = Fix::from_bits(", $bytes_val, ");
let swapped = Fix::from_bits(", $rev_bytes_val, ");
assert_eq!(f.swap_bytes(), swapped);
```
";
                #[inline]
                #[must_use]
                pub const fn swap_bytes(self) -> $Fixed<Frac> {
                    $Fixed::from_bits(self.to_bits().swap_bytes())
                }
            }

            comment! {
                "Creates a fixed-point number from its representation
as a byte array in big endian.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(
    Fix::from_be_bytes(", $be_bytes, "),
    Fix::from_bits(", $bytes_val, ")
);
```
";
                #[inline]
                pub const fn from_be_bytes(bytes: [u8; $nbytes]) -> $Fixed<Frac> {
                    $Fixed::from_bits(<$Inner>::from_be_bytes(bytes))
                }
            }

            comment! {
                "Creates a fixed-point number from its representation
as a byte array in little endian.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(
    Fix::from_le_bytes(", $le_bytes, "),
    Fix::from_bits(", $bytes_val, ")
);
```
";
                #[inline]
                pub const fn from_le_bytes(bytes: [u8; $nbytes]) -> $Fixed<Frac> {
                    $Fixed::from_bits(<$Inner>::from_le_bytes(bytes))
                }
            }

            comment! {
                "Creates a fixed-point number from its representation
as a byte array in native endian.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(
    if cfg!(target_endian = \"big\") {
        Fix::from_ne_bytes(", $be_bytes, ")
    } else {
        Fix::from_ne_bytes(", $le_bytes, ")
    },
    Fix::from_bits(", $bytes_val, ")
);
```
";
                #[inline]
                pub const fn from_ne_bytes(bytes: [u8; $nbytes]) -> $Fixed<Frac> {
                    $Fixed::from_bits(<$Inner>::from_ne_bytes(bytes))
                }
            }

            comment! {
                "Returns the memory representation of this fixed-point
number as a byte array in big-endian byte order.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let val = Fix::from_bits(", $bytes_val, ");
assert_eq!(
    val.to_be_bytes(),
    ", $be_bytes, "
);
```
";
                #[inline]
                pub const fn to_be_bytes(self) -> [u8; $nbytes] {
                    self.to_bits().to_be_bytes()
                }
            }

            comment! {
                "Returns the memory representation of this fixed-point
number as a byte array in little-endian byte order.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let val = Fix::from_bits(", $bytes_val, ");
assert_eq!(
    val.to_le_bytes(),
    ", $le_bytes, "
);
```
";
                #[inline]
                pub const fn to_le_bytes(self) -> [u8; $nbytes] {
                    self.to_bits().to_le_bytes()
                }
            }

            comment! {
                "Returns the memory representation of this fixed-point
number as a byte array in native byte order.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let val = Fix::from_bits(", $bytes_val, ");
assert_eq!(
    val.to_ne_bytes(),
    if cfg!(target_endian = \"big\") {
        ", $be_bytes, "
    } else {
        ", $le_bytes, "
    }
);
```
";
                #[inline]
                pub const fn to_ne_bytes(self) -> [u8; $nbytes] {
                    self.to_bits().to_ne_bytes()
                }
            }

            comment! {
                "Returns the number of ones in the binary
representation.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let f = Fix::from_bits(0b11_0010);
assert_eq!(f.count_ones(), 3);
```
";
                #[inline]
                #[doc(alias("popcount", "popcnt"))]
                pub const fn count_ones(self) -> u32 {
                    self.to_bits().count_ones()
                }
            }

            comment! {
                "Returns the number of zeros in the binary
representation.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let f = Fix::from_bits(!0b11_0010);
assert_eq!(f.count_zeros(), 3);
```
";
                #[inline]
                pub const fn count_zeros(self) -> u32 {
                    self.to_bits().count_zeros()
                }
            }

            comment! {
                "Returns the number of leading ones in the binary
representation.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let all_ones = !Fix::ZERO;
let f = all_ones - Fix::from_bits(0b10_0000);
assert_eq!(f.leading_ones(), ", $s_nbits, " - 6);
```
";
                #[inline]
                pub const fn leading_ones(self) -> u32 {
                    (!self.to_bits()).leading_zeros()
                }
            }

            comment! {
                "Returns the number of leading zeros in the binary
representation.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let f = Fix::from_bits(0b10_0000);
assert_eq!(f.leading_zeros(), ", $s_nbits, " - 6);
```
";
                #[inline]
                pub const fn leading_zeros(self) -> u32 {
                    self.to_bits().leading_zeros()
                }
            }

            comment! {
                "Returns the number of trailing ones in the binary
representation.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let f = Fix::from_bits(0b101_1111);
assert_eq!(f.trailing_ones(), 5);
```
";
                #[inline]
                pub const fn trailing_ones(self) -> u32 {
                    (!self.to_bits()).trailing_zeros()
                }
            }

            comment! {
                "Returns the number of trailing zeros in the binary
representation.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let f = Fix::from_bits(0b10_0000);
assert_eq!(f.trailing_zeros(), 5);
```
";
                #[inline]
                pub const fn trailing_zeros(self) -> u32 {
                    self.to_bits().trailing_zeros()
                }
            }

            if_unsigned! {
                $Signedness;
                comment! {
                    "Returns the number of bits required to represent the value.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::from_num(0).significant_bits(), 0);      // “____.____”
assert_eq!(Fix::from_num(0.0625).significant_bits(), 1); // “____.___1”
assert_eq!(Fix::from_num(1).significant_bits(), 5);      // “___1.0000”
assert_eq!(Fix::from_num(3).significant_bits(), 6);      // “__11.0000”
```
";
                    #[inline]
                    pub const fn significant_bits(self) -> u32 {
                        $Inner::BITS - self.leading_zeros()
                    }
                }
            }

            if_signed! {
                $Signedness;
                comment! {
                    "Returns the number of bits required to represent the value.

The number of bits required includes an initial one for negative
numbers, and an initial zero for non-negative numbers.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::from_num(-3).signed_bits(), 7);      // “_101.0000”
assert_eq!(Fix::from_num(-1).signed_bits(), 5);      // “___1.0000”
assert_eq!(Fix::from_num(-0.0625).signed_bits(), 1); // “____.___1”
assert_eq!(Fix::from_num(0).signed_bits(), 1);       // “____.___0”
assert_eq!(Fix::from_num(0.0625).signed_bits(), 2);  // “____.__01”
assert_eq!(Fix::from_num(1).signed_bits(), 6);       // “__01.0000”
assert_eq!(Fix::from_num(3).signed_bits(), 7);       // “_011.0000”
```
";
                    #[inline]
                    pub const fn signed_bits(self) -> u32 {
                        let leading = if self.is_negative() {
                            self.leading_ones()
                        } else {
                            self.leading_zeros()
                        };
                        $Inner::BITS + 1 - leading
                    }
                }
            }

            comment! {
                "Reverses the order of the bits of the fixed-point number.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let bits = ", $bytes_val, "_", $s_inner, ";
let rev_bits = bits.reverse_bits();
assert_eq!(Fix::from_bits(bits).reverse_bits(), Fix::from_bits(rev_bits));
```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn reverse_bits(self) -> $Fixed<Frac> {
                    $Fixed::from_bits(self.to_bits().reverse_bits())
                }
            }

            comment! {
                "Shifts to the left by `n` bits, wrapping the
truncated bits to the right end.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let bits: ", $s_inner, " = (0b111 << (", $s_nbits, " - 3)) | 0b1010;
let rot = 0b1010111;
assert_eq!(bits.rotate_left(3), rot);
assert_eq!(Fix::from_bits(bits).rotate_left(3), Fix::from_bits(rot));
```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn rotate_left(self, n: u32) -> $Fixed<Frac> {
                    Self::from_bits(self.to_bits().rotate_left(n))
                }
            }

            comment! {
                "Shifts to the right by `n` bits, wrapping the
truncated bits to the left end.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let bits: ", $s_inner, " = 0b1010111;
let rot = (0b111 << (", $s_nbits, " - 3)) | 0b1010;
assert_eq!(bits.rotate_right(3), rot);
assert_eq!(Fix::from_bits(bits).rotate_right(3), Fix::from_bits(rot));
```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn rotate_right(self, n: u32) -> $Fixed<Frac> {
                    Self::from_bits(self.to_bits().rotate_right(n))
                }
            }

            comment! {
                "Returns [`true`] if the number is zero.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert!(Fix::ZERO.is_zero());
assert!(!Fix::from_num(5).is_zero());
```
";
                #[inline]
                pub const fn is_zero(self) -> bool {
                    self.to_bits() == 0
                }
            }

            if_signed! {
                $Signedness;
                comment! {
                    "Returns [`true`] if the number is >&nbsp;0.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert!(Fix::from_num(5).is_positive());
assert!(!Fix::ZERO.is_positive());
assert!(!Fix::from_num(-5).is_positive());
```
";
                    #[inline]
                    pub const fn is_positive(self) -> bool {
                        self.to_bits().is_positive()
                    }
                }

                comment! {
                    "Returns [`true`] if the number is <&nbsp;0.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert!(!Fix::from_num(5).is_negative());
assert!(!Fix::ZERO.is_negative());
assert!(Fix::from_num(-5).is_negative());
```
";
                    #[inline]
                    pub const fn is_negative(self) -> bool {
                        self.to_bits().is_negative()
                    }
                }
            }

            if_unsigned! {
                $Signedness;
                comment! {
                    "Returns [`true`] if the fixed-point number is
2<sup><i>k</i></sup> for some integer <i>k</i>.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
// 3/8 is 0.0110
let three_eights = Fix::from_bits(0b0110);
// 1/2 is 0.1000
let half = Fix::from_bits(0b1000);
assert!(!three_eights.is_power_of_two());
assert!(half.is_power_of_two());
```
";
                    #[inline]
                    pub const fn is_power_of_two(self) -> bool {
                        self.to_bits().is_power_of_two()
                    }
                }
            }

            if_true! {
                $HasDouble;
                comment! {
                    "Multiplies two fixed-point numbers and returns a
wider type to retain all precision.

If `self` has <i>f</i> fractional bits and ", $s_nbits, "&nbsp;&minus;&nbsp;<i>f</i>
integer bits, and `rhs` has <i>g</i> fractional bits and ", $s_nbits,
"&nbsp;&minus;&nbsp;<i>g</i> integer bits, then the returned fixed-point number will
have <i>f</i>&nbsp;+&nbsp;<i>g</i> fractional bits and ", $s_nbits_2,
"&nbsp;&minus;&nbsp;<i>f</i>&nbsp;&minus;&nbsp;<i>g</i> integer bits.

# Examples

```rust
use fixed::{
    types::extra::{U2, U4},
    ", $s_fixed, ",
};
// decimal: 1.25 × 1.0625 = 1.328_125
// binary: 1.01 × 1.0001 = 1.010101
let a = ", $s_fixed, "::<U2>::from_num(1.25);
let b = ", $s_fixed, "::<U4>::from_num(1.0625);
assert_eq!(a.wide_mul(b), 1.328_125);
```
";
                    #[inline]
                    #[must_use = "this returns the result of the operation, without modifying the original"]
                    pub const fn wide_mul<RhsFrac>(
                        self,
                        rhs: $Fixed<RhsFrac>,
                    ) -> $Double<Sum<Frac, RhsFrac>>
                    where
                        Frac: Add<RhsFrac>,
                    {
                        let self_bits = self.to_bits() as $DoubleInner;
                        let rhs_bits = rhs.to_bits() as $DoubleInner;
                        $Double::from_bits(self_bits * rhs_bits)
                    }
                }

                if_signed! {
                    $Signedness;
                    /// Multiplies an unsigned fixed-point number and returns a
                    /// wider signed type to retain all precision.
                    ///
                    /// If `self` has <i>f</i> fractional bits and
                    #[doc = concat!($s_nbits, "&nbsp;&minus;&nbsp;<i>f</i>")]
                    /// integer bits, and `rhs` has <i>g</i> fractional bits and
                    #[doc = concat!($s_nbits, "&nbsp;&minus;&nbsp;<i>g</i>")]
                    /// integer bits, then the returned fixed-point number will
                    /// have <i>f</i>&nbsp;+&nbsp;<i>g</i> fractional bits and
                    #[doc = concat!(
                        $s_nbits_2,
                        "&nbsp;&minus;&nbsp;<i>f</i>&nbsp;&minus;&nbsp;<i>g</i>"
                    )]
                    /// integer bits.
                    ///
                    /// # Examples
                    ///
                    /// ```rust
                    /// use fixed::{
                    ///     types::extra::{U2, U4},
                    #[doc = concat!("     ", $s_fixed, ", ", $s_ufixed, ",")]
                    /// };
                    /// // decimal: -1.25 × 1.0625 = -1.328_125
                    /// // binary: -1.01 × 1.0001 = -1.010101
                    #[doc = concat!("let a = ", $s_fixed, "::<U2>::from_num(-1.25);")]
                    #[doc = concat!("let b = ", $s_ufixed, "::<U4>::from_num(1.0625);")]
                    /// assert_eq!(a.wide_mul_unsigned(b), -1.328_125);
                    /// ```
                    #[inline]
                    #[must_use]
                    pub const fn wide_mul_unsigned<RhsFrac>(
                        self,
                        rhs: $UFixed<RhsFrac>,
                    ) -> $Double<Sum<Frac, RhsFrac>>
                    where
                        Frac: Add<RhsFrac>,
                    {
                        let wide_lhs = self.to_bits() as $DoubleInner;
                        let rhs_signed = rhs.to_bits() as $Inner;
                        let wide_rhs = rhs_signed as $DoubleInner;
                        let mut wide_prod = wide_lhs * wide_rhs;
                        if rhs_signed < 0 {
                            // rhs msb treated as -2^(N - 1) instead of +2^(N - 1),
                            // so error in rhs is -2^N, and error in prod is -2^N × lhs
                            wide_prod += wide_lhs << ($nbytes * 8);
                        }
                        $Double::from_bits(wide_prod)
                    }
                }

                if_unsigned! {
                    $Signedness;
                    /// Multiplies a signed fixed-point number and returns a
                    /// wider signed type to retain all precision.
                    ///
                    /// If `self` has <i>f</i> fractional bits and
                    #[doc = concat!($s_nbits, "&nbsp;&minus;&nbsp;<i>f</i>")]
                    /// integer bits, and `rhs` has <i>g</i> fractional bits and
                    #[doc = concat!($s_nbits, "&nbsp;&minus;&nbsp;<i>g</i>")]
                    /// integer bits, then the returned fixed-point number will
                    /// have <i>f</i>&nbsp;+&nbsp;<i>g</i> fractional bits and
                    #[doc = concat!(
                        $s_nbits_2,
                        "&nbsp;&minus;&nbsp;<i>f</i>&nbsp;&minus;&nbsp;<i>g</i>"
                    )]
                    /// integer bits.
                    ///
                    /// # Examples
                    ///
                    /// ```rust
                    /// use fixed::{
                    ///     types::extra::{U2, U4},
                    #[doc = concat!("     ", $s_ifixed, ", ", $s_fixed, ",")]
                    /// };
                    /// // decimal: 1.25 × -1.0625 = -1.328_125
                    /// // binary: 1.01 × -1.0001 = -1.010101
                    #[doc = concat!("let a = ", $s_fixed, "::<U2>::from_num(1.25);")]
                    #[doc = concat!("let b = ", $s_ifixed, "::<U4>::from_num(-1.0625);")]
                    /// assert_eq!(a.wide_mul_signed(b), -1.328_125);
                    /// ```
                    #[inline]
                    #[must_use]
                    pub const fn wide_mul_signed<RhsFrac>(
                        self,
                        rhs: $IFixed<RhsFrac>,
                    ) -> $IDouble<Sum<Frac, RhsFrac>>
                    where
                        Frac: Add<RhsFrac>,
                    {
                        let lhs_signed = self.to_bits() as $IInner;
                        let wide_lhs = lhs_signed as $IDoubleInner;
                        let wide_rhs = rhs.to_bits() as $IDoubleInner;
                        let mut wide_prod = wide_lhs * wide_rhs;
                        if lhs_signed < 0 {
                            // lhs msb treated as -2^(N - 1) instead of +2^(N - 1),
                            // so error in lhs is -2^N, and error in prod is -2^N × rhs
                            wide_prod += wide_rhs << ($nbytes * 8);
                        }
                        $IDouble::from_bits(wide_prod)
                    }
                }

                comment! {
                    "Divides two fixed-point numbers and returns a
wider type to retain more precision.

If `self` has <i>f</i> fractional bits
and ", $s_nbits, "&nbsp;&minus;&nbsp;<i>f</i> integer bits, and `rhs` has
<i>g</i> fractional bits and ", $s_nbits, "&nbsp;&minus;&nbsp;<i>g</i> integer
bits, then the returned fixed-point number will
have ", $s_nbits, "&nbsp;+&nbsp;<i>f</i>&nbsp;&minus;&nbsp;<i>g</i> fractional
bits and ", $s_nbits, "&nbsp;&minus;&nbsp;<i>f</i>&nbsp;+&nbsp;<i>g</i> integer
bits.
",
                    if_signed_else_empty_str! {
                        $Signedness;
                        "
**Warning:** While most cases of overflow are avoided using this method,
dividing [`MIN`][Self::MIN] by <code>-[DELTA][Self::DELTA]</code> will still
result in panic due to overflow. The alternative [`wide_sdiv`][Self::wide_sdiv]
method avoids this by sacrificing one fractional bit in the return type.
"
                    },
                    "
# Panics

Panics if the divisor is zero",
                    if_signed_unsigned!(
                        $Signedness,
                        " or on overflow. Overflow can only occur when dividing
[`MIN`][Self::MIN] by <code>-[DELTA][Self::DELTA]</code>.",
                        ".",
                    ),
                    "

# Examples

```rust
use fixed::{
    types::extra::{U3, U5, U", $s_nbits_m2, "},
    ", $s_fixed, ", ", $s_double, ",
};
// decimal: 4.625 / 0.03125 = 148
// binary: 100.101 / 0.00001 = 10010100
let a = ", $s_fixed, "::<U3>::from_num(4.625);
let b = ", $s_fixed, "::<U5>::from_num(0.03125);
let ans: ", $s_double, "<U", $s_nbits_m2, "> = a.wide_div(b);
assert_eq!(ans, 148);
```
",
                    if_signed_else_empty_str! {
                        $Signedness;
                        "
The following panics because of overflow.

```should_panic
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let _overflow = Fix::MIN.wide_div(-Fix::DELTA);
```
",
                    };
                    #[inline]
                    #[must_use = "this returns the result of the operation, without modifying the original"]
                    pub const fn wide_div<RhsFrac>(
                        self,
                        rhs: $Fixed<RhsFrac>,
                    ) -> $Double<Diff<Sum<$UNbits, Frac>, RhsFrac>>
                    where
                        $UNbits: Add<Frac>,
                        Sum<$UNbits, Frac>: Sub<RhsFrac>,
                    {
                        let self_bits = self.to_bits() as $DoubleInner;
                        let rhs_bits = rhs.to_bits() as $DoubleInner;
                        $Double::from_bits((self_bits << $UNbits::U32) / rhs_bits)
                    }
                }

                if_signed! {
                    $Signedness;

                    /// Divides two fixed-point numbers and returns a wider type
                    /// to retain more precision.
                    ///
                    /// If `self` has <i>f</i> fractional bits and
                    #[doc = concat!($s_nbits, "&nbsp;&minus;&nbsp;<i>f</i>")]
                    /// integer bits, and `rhs` has <i>g</i> fractional bits and
                    #[doc = concat!($s_nbits, "&nbsp;&minus;&nbsp;<i>g</i>")]
                    /// integer bits, then the returned fixed-point number will have
                    #[doc = concat!(
                        $s_nbits_m1, "&nbsp;+&nbsp;<i>f</i>&nbsp;&minus;&nbsp;<i>g</i>"
                    )]
                    /// fractional bits and
                    #[doc = concat!(
                        $s_nbits_p1, "&nbsp;&minus;&nbsp;<i>f</i>&nbsp;+&nbsp;<i>g</i>"
                    )]
                    /// integer bits.
                    ///
                    /// This is similar to the [`wide_div`] method but
                    /// sacrifices one fractional bit to avoid overflow.
                    ///
                    /// # Panics
                    ///
                    /// Panics if the divisor is zero.
                    ///
                    /// # Examples
                    ///
                    /// ```rust
                    /// use fixed::{
                    #[doc = concat!("     types::extra::{U4, U5, U", $s_nbits_m2, "},")]
                    #[doc = concat!("     ", $s_fixed, ", ", $s_double, ",")]
                    /// };
                    /// // decimal: 4.625 / 0.03125 = 148
                    /// // binary: 100.101 / 0.00001 = 10010100
                    #[doc = concat!("let a = ", $s_fixed, "::<U4>::from_num(4.625);")]
                    #[doc = concat!("let b = ", $s_fixed, "::<U5>::from_num(0.03125);")]
                    #[doc = concat!(
                        "let ans: ", $s_double, "<U", $s_nbits_m2, "> = a.wide_sdiv(b);"
                    )]
                    /// assert_eq!(ans, 148);
                    /// ```
                    ///
                    /// Unlike [`wide_div`], dividing [`MIN`][Self::MIN] by
                    /// <code>-[DELTA][Self::DELTA]</code> does not overflow.
                    ///
                    /// ```rust
                    /// use fixed::{
                    #[doc = concat!("     types::extra::{U4, U", $s_nbits_m1, "},")]
                    #[doc = concat!("     ", $s_fixed, ", ", $s_double, ",")]
                    /// };
                    #[doc = concat!("type Fix = ", $s_fixed, "<U4>;")]
                    #[doc = concat!("type DFix = ", $s_double, "<U", $s_nbits_m1, ">;")]
                    /// assert_eq!(Fix::MIN.wide_sdiv(-Fix::DELTA), (DFix::MIN / 2).abs());
                    /// ```
                    ///
                    /// [`wide_div`]: Self::wide_div
                    #[inline]
                    #[must_use]
                    pub const fn wide_sdiv<RhsFrac>(
                        self,
                        rhs: $Fixed<RhsFrac>,
                    ) -> $Double<Diff<Sum<$UNbits_m1, Frac>, RhsFrac>>
                    where
                        $UNbits_m1: Add<Frac>,
                        Sum<$UNbits_m1, Frac>: Sub<RhsFrac>,
                    {
                        let self_bits = self.to_bits() as $DoubleInner;
                        let rhs_bits = rhs.to_bits() as $DoubleInner;
                        $Double::from_bits((self_bits << $UNbits_m1::U32) / rhs_bits)
                    }

                    /// Divides by an unsigned fixed-point number and returns a
                    /// wider signed type to retain more precision.
                    ///
                    /// If `self` has <i>f</i> fractional bits and
                    #[doc = concat!($s_nbits, "&nbsp;&minus;&nbsp;<i>f</i>")]
                    /// integer bits, and `rhs` has <i>g</i> fractional bits and
                    #[doc = concat!($s_nbits, "&nbsp;&minus;&nbsp;<i>g</i>")]
                    /// integer bits, then the returned fixed-point number will have
                    #[doc = concat!($s_nbits, "&nbsp;+&nbsp;<i>f</i>&nbsp;&minus;&nbsp;<i>g</i>")]
                    /// fractional bits and
                    #[doc = concat!($s_nbits, "&nbsp;&minus;&nbsp;<i>f</i>&nbsp;+&nbsp;<i>g</i>")]
                    /// integer bits.
                    ///
                    /// # Panics
                    ///
                    /// Panics if the divisor is zero.
                    ///
                    /// # Examples
                    ///
                    /// ```rust
                    /// use fixed::{
                    #[doc = concat!("     types::extra::{U3, U5, U", $s_nbits_m2, "},")]
                    #[doc = concat!("     ", $s_fixed, ", ", $s_double, ", ", $s_ufixed, ",")]
                    /// };
                    /// // decimal: -4.625 / 0.03125 = -148
                    /// // binary: -100.101 / 0.00001 = -10010100
                    #[doc = concat!("let a = ", $s_fixed, "::<U3>::from_num(-4.625);")]
                    #[doc = concat!("let b = ", $s_ufixed, "::<U5>::from_num(0.03125);")]
                    #[doc = concat!(
                        "let ans: ", $s_double, "<U", $s_nbits_m2, "> = a.wide_div_unsigned(b);"
                    )]
                    /// assert_eq!(ans, -148);
                    /// ```
                    #[inline]
                    #[must_use]
                    pub const fn wide_div_unsigned<RhsFrac>(
                        self,
                        rhs: $UFixed<RhsFrac>,
                    ) -> $Double<Diff<Sum<$UNbits, Frac>, RhsFrac>>
                    where
                        $UNbits: Add<Frac>,
                        Sum<$UNbits, Frac>: Sub<RhsFrac>,
                    {
                        let self_bits = self.to_bits() as $DoubleInner;
                        let rhs_bits = rhs.to_bits() as $DoubleInner;
                        $Double::from_bits((self_bits << $UNbits::U32) / rhs_bits)
                    }
                }

                if_unsigned! {
                    $Signedness;
                    /// Divides by a signed fixed-point number and returns a
                    /// wider signed type to retain more precision.
                    ///
                    /// If `self` has <i>f</i> fractional bits and
                    #[doc = concat!($s_nbits, "&nbsp;&minus;&nbsp;<i>f</i>")]
                    /// integer bits, and `rhs` has <i>g</i> fractional bits and
                    #[doc = concat!($s_nbits, "&nbsp;&minus;&nbsp;<i>g</i>")]
                    /// integer bits, then the returned fixed-point number will have
                    #[doc = concat!(
                        $s_nbits_m1, "&nbsp;+&nbsp;<i>f</i>&nbsp;&minus;&nbsp;<i>g</i>"
                    )]
                    /// fractional bits and
                    #[doc = concat!(
                        $s_nbits_p1, "&nbsp;&minus;&nbsp;<i>f</i>&nbsp;+&nbsp;<i>g</i>"
                    )]
                    /// integer bits.
                    ///
                    /// See also
                    #[doc = concat!(
                        "<code>[`", $s_ifixed, "`]",
                        "::[wide\\_sdiv][", $s_ifixed, "::wide_sdiv]</code>."
                    )]
                    ///
                    /// # Panics
                    ///
                    /// Panics if the divisor is zero.
                    ///
                    /// # Examples
                    ///
                    /// ```rust
                    /// use fixed::{
                    #[doc = concat!("     types::extra::{U4, U5, U", $s_nbits_m2, "},")]
                    #[doc = concat!(
                        "     ", $s_fixed, ", ", $s_ifixed, ", ", stringify!($IDouble), ","
                    )]
                    /// };
                    /// // decimal: 4.625 / -0.03125 = -148
                    /// // binary: 100.101 / -0.00001 = -10010100
                    #[doc = concat!("let a = ", $s_fixed, "::<U4>::from_num(4.625);")]
                    #[doc = concat!("let b = ", $s_ifixed, "::<U5>::from_num(-0.03125);")]
                    #[doc = concat!(
                        "let ans: ", stringify!($IDouble), "<U", $s_nbits_m2, ">",
                        " = a.wide_sdiv_signed(b);"
                    )]
                    /// assert_eq!(ans, -148);
                    /// ```
                    #[inline]
                    #[must_use]
                    pub const fn wide_sdiv_signed<RhsFrac>(
                        self,
                        rhs: $IFixed<RhsFrac>,
                    ) -> $IDouble<Diff<Sum<$UNbits_m1, Frac>, RhsFrac>>
                    where
                        $UNbits_m1: Add<Frac>,
                        Sum<$UNbits_m1, Frac>: Sub<RhsFrac>,
                    {
                        let self_bits = self.to_bits() as $IDoubleInner;
                        let rhs_bits = rhs.to_bits() as $IDoubleInner;
                        $IDouble::from_bits((self_bits << $UNbits_m1::U32) / rhs_bits)
                    }
                }
            }

            comment! {
                "Multiply and add. Returns `self` × `mul` + `add`.

",
                if_signed_else_empty_str! {
                    $Signedness;
                    "For some cases, the product `self` × `mul` would overflow
on its own, but the final result `self` × `mul` + `add` is representable; in
these cases this method returns the correct result without overflow.

",
                },
                "The `mul` parameter can have a fixed-point type like
`self` but with a different number of fractional bits.

# Panics

When debug assertions are enabled, this method panics if the result
overflows. When debug assertions are not enabled, the wrapped value
can be returned, but it is not considered a breaking change if in the
future it panics; if wrapping is required use [`wrapping_mul_add`]
instead.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(
    Fix::from_num(4).mul_add(Fix::from_num(0.5), Fix::from_num(3)),
    Fix::from_num(5)
);
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "// MAX × 1.5 - MAX = MAX / 2, which does not overflow
assert_eq!(Fix::MAX.mul_add(Fix::from_num(1.5), -Fix::MAX), Fix::MAX / 2);
"
                },
                "```

[`wrapping_mul_add`]: Self::wrapping_mul_add
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn mul_add<MulFrac: $LeEqU>(
                    self,
                    mul: $Fixed<MulFrac>,
                    add: $Fixed<Frac>,
                ) -> $Fixed<Frac> {
                    let (ans, overflow) = arith::$Inner::overflowing_mul_add(
                        self.to_bits(),
                        mul.to_bits(),
                        add.to_bits(),
                        MulFrac::I32,
                    );
                    debug_assert!(!overflow, "overflow");
                    Self::from_bits(ans)
                }
            }

            comment! {
                "Remainder for Euclidean division.

# Panics

Panics if the divisor is zero.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::from_num(7.5).rem_euclid(Fix::from_num(2)), Fix::from_num(1.5));
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "assert_eq!(Fix::from_num(-7.5).rem_euclid(Fix::from_num(2)), Fix::from_num(0.5));
",
                },
                "```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn rem_euclid(self, rhs: $Fixed<Frac>) -> $Fixed<Frac> {
                    let rhs_bits = rhs.to_bits();
                    if_signed! {
                        $Signedness;
                        if rhs_bits == -1 {
                            return Self::ZERO;
                        }
                    }
                    Self::from_bits(self.to_bits().rem_euclid(rhs_bits))
                }
            }

            if_signed! {
                $Signedness;
                comment! {
                    "Returns the absolute value.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let five = Fix::from_num(5);
let minus_five = Fix::from_num(-5);
assert_eq!(five.abs(), five);
assert_eq!(minus_five.abs(), five);
```
";
                    #[inline]
                    #[must_use]
                    pub const fn abs(self) -> $Fixed<Frac> {
                        Self::from_bits(self.to_bits().abs())
                    }
                }

                comment! {
                    "Returns the absolute value using an unsigned type
without any wrapping or panicking.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, ", ", $s_ufixed, "};
type Fix = ", $s_fixed, "<U4>;
type UFix = ", $s_ufixed, "<U4>;
assert_eq!(Fix::from_num(-5).unsigned_abs(), UFix::from_num(5));
// min_as_unsigned has only highest bit set
let min_as_unsigned = UFix::ONE << (UFix::INT_NBITS - 1);
assert_eq!(Fix::MIN.unsigned_abs(), min_as_unsigned);
```
";
                    #[inline]
                    pub const fn unsigned_abs(self) -> $UFixed<Frac> {
                        $UFixed::from_bits(self.to_bits().unsigned_abs())
                    }
                }
            }

            comment! {
                "Returns the distance from `self` to `other`.

The distance is the absolute value of the difference.

",
                if_signed_else_empty_str! {
                    $Signedness;
                    "# Panics

When debug assertions are enabled, this method panics if the result overflows.
When debug assertions are not enabled, the wrapped value can be returned, but it
is not considered a breaking change if in the future it panics; if wrapping is
required use [`wrapping_dist`] instead.

",
                },
                "# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::ONE.dist(Fix::from_num(5)), Fix::from_num(4));
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "assert_eq!(Fix::NEG_ONE.dist(Fix::from_num(2)), Fix::from_num(3));
",
                },
                "```
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "
[`wrapping_dist`]: Self::wrapping_dist
",
                };
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn dist(self, other: $Fixed<Frac>) -> $Fixed<Frac> {
                    let s = self.to_bits();
                    let o = other.to_bits();
                    let d = if_signed_unsigned!($Signedness, (s - o).abs(), s.abs_diff(o));
                    Self::from_bits(d)
                }
            }

            if_signed! {
                $Signedness;
                comment! {
                    "Returns the distance from `self` to `other` using an
unsigned type without any wrapping or panicking.

The distance is the absolute value of the difference.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, ", ", $s_ufixed, "};
type Fix = ", $s_fixed, "<U4>;
type UFix = ", $s_ufixed, "<U4>;
assert_eq!(Fix::NEG_ONE.unsigned_dist(Fix::from_num(2)), UFix::from_num(3));
assert_eq!(Fix::MIN.unsigned_dist(Fix::MAX), UFix::MAX);
```
";
                    #[inline]
                    #[must_use = "this returns the result of the operation, without modifying the original"]
                    pub const fn unsigned_dist(self, other: $Fixed<Frac>) -> $UFixed<Frac> {
                        self.abs_diff(other)
                    }
                }
            }

            comment! {
                if_signed_unsigned!(
                    $Signedness,
                    "Returns the absolute value of the difference between `self`
and `other` using an unsigned type without any wrapping or panicking.

This method is the same as [`unsigned_dist`] for signed fixed-point numbers.

[`unsigned_dist`]: Self::unsigned_dist
",
                    "Returns the absolute value of the difference between `self` and `other`.

This method is the same as [`dist`] for unsigned fixed-point numbers.

[`dist`]: Self::dist
",
                );
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn abs_diff(self, other: $Fixed<Frac>) -> $UFixed<Frac> {
                    $UFixed::from_bits(self.to_bits().abs_diff(other.to_bits()))
                }
            }

            comment! {
                "Returns the mean of `self` and `other`.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::from_num(3).mean(Fix::from_num(4)), Fix::from_num(3.5));
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "assert_eq!(Fix::from_num(-3).mean(Fix::from_num(4)), Fix::from_num(0.5));
",
                },
                "```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn mean(self, other: $Fixed<Frac>) -> $Fixed<Frac> {
                    // a & b == common bits
                    // a ^ b == different bits
                    // a + b == 2 * (a & b) + (a ^ b)
                    // (a + b) / 2 = (a & b) + (a ^ b) / 2
                    let (a, b) = (self.to_bits(), other.to_bits());
                    $Fixed::from_bits((a & b) + ((a ^ b) >> 1))
                }
            }

            comment! {
                "Returns the smallest multiple of `other` that is ≥&nbsp;`self`",
                if_signed_else_empty_str! {
                    $Signedness;
                    " if `other` is positive, and the largest multiple of
`other` that is ≤&nbsp;`self` if `other` is negative",
                },
                ".

# Panics

Panics if `other` is zero.

When debug assertions are enabled, this method also panics if the result
overflows. When debug assertions are not enabled, the wrapped value can be
returned, but it is not considered a breaking change if in the future it panics;
if wrapping is required use [`wrapping_next_multiple_of`] instead.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(
    Fix::from_num(4).next_multiple_of(Fix::from_num(1.5)),
    Fix::from_num(4.5)
);
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "assert_eq!(
    Fix::from_num(4).next_multiple_of(Fix::from_num(-1.5)),
    Fix::from_num(3)
);
",
                },
                "```

[`wrapping_next_multiple_of`]: Self::wrapping_next_multiple_of
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn next_multiple_of(self, other: $Fixed<Frac>) -> $Fixed<Frac> {
                    let (ans, overflow) = self.overflowing_next_multiple_of(other);
                    debug_assert!(!overflow, "overflow");
                    ans
                }
            }

            comment! {
                "Inverse linear interpolation between `start` and `end`.

The computed value can have a fixed-point type like `self` but with a different
number of fractional bits.

Returns
(`self`&nbsp;&minus;&nbsp;`start`)&nbsp;/&nbsp;(`end`&nbsp;&minus;&nbsp;`start`).
This is 0 when `self`&nbsp;=&nbsp;`start`, and 1 when `self`&nbsp;=&nbsp;`end`.

# Panics

Panics when `start`&nbsp;=&nbsp;`end`.

When debug assertions are enabled, this method also panics if the result
overflows. When debug assertions are not enabled, the wrapped value can be
returned, but it is not considered a breaking change if in the future it panics;
if wrapping is required use [`wrapping_inv_lerp`] instead.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let start = Fix::from_num(2);
let end = Fix::from_num(3.5);
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "assert_eq!(Fix::from_num(0.5).inv_lerp::<U4>(start, end), -1);
",
                },
                "assert_eq!(Fix::from_num(2).inv_lerp::<U4>(start, end), 0);
assert_eq!(Fix::from_num(2.75).inv_lerp::<U4>(start, end), 0.5);
assert_eq!(Fix::from_num(3.5).inv_lerp::<U4>(start, end), 1);
assert_eq!(Fix::from_num(5).inv_lerp::<U4>(start, end), 2);
```

[`wrapping_inv_lerp`]: Self::wrapping_inv_lerp
";
                #[inline]
                pub const fn inv_lerp<RetFrac: $LeEqU>(
                    self,
                    start: $Fixed<Frac>,
                    end: $Fixed<Frac>,
                ) -> $Fixed<RetFrac> {
                    let (ans, overflow) = inv_lerp::$Inner(
                        self.to_bits(),
                        start.to_bits(),
                        end.to_bits(),
                        RetFrac::U32,
                    );
                    debug_assert!(!overflow, "overflow");
                    $Fixed::from_bits(ans)
                }
            }

            if_unsigned! {
                $Signedness;
                comment! {
                    "Returns the highest one in the binary
representation, or zero if `self` is zero.

If `self`&nbsp;>&nbsp;0, the highest one is equal to the largest power of two
that is ≤&nbsp;`self`.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::from_bits(0b11_0010).highest_one(), Fix::from_bits(0b10_0000));
assert_eq!(Fix::from_num(0.3).highest_one(), Fix::from_num(0.25));
assert_eq!(Fix::from_num(4).highest_one(), Fix::from_num(4));
assert_eq!(Fix::from_num(6.5).highest_one(), Fix::from_num(4));
assert_eq!(Fix::ZERO.highest_one(), Fix::ZERO);
```
";
                    #[inline]
                    #[must_use]
                    pub const fn highest_one(self) -> $Fixed<Frac> {
                        const ONE: $Inner = 1;
                        let bits = self.to_bits();
                        if bits == 0 {
                            self
                        } else {
                            $Fixed::from_bits(ONE << (ONE.leading_zeros() - bits.leading_zeros()))
                        }
                    }
                }

                comment! {
                    "Returns the smallest power of two that is ≥&nbsp;`self`.

# Panics

When debug assertions are enabled, panics if the next power of two is
too large to represent. When debug assertions are not enabled, zero
can be returned, but it is not considered a breaking change if in the
future it panics; if this is not desirable use
[`checked_next_power_of_two`] instead.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::from_bits(0b11_0010).next_power_of_two(), Fix::from_bits(0b100_0000));
assert_eq!(Fix::from_num(0.3).next_power_of_two(), Fix::from_num(0.5));
assert_eq!(Fix::from_num(4).next_power_of_two(), Fix::from_num(4));
assert_eq!(Fix::from_num(6.5).next_power_of_two(), Fix::from_num(8));
```

[`checked_next_power_of_two`]: Self::checked_next_power_of_two
";
                    #[inline]
                    #[must_use]
                    pub const fn next_power_of_two(self) -> $Fixed<Frac> {
                        Self::from_bits(self.to_bits().next_power_of_two())
                    }
                }
            }

            if_signed! {
                $Signedness;
                /// Addition with an unsigned fixed-point number.
                ///
                /// # Panics
                ///
                /// When debug assertions are enabled, this method panics if the
                /// result overflows. When debug assertions are not enabled, the
                /// wrapped value can be returned, but it is not considered a
                /// breaking change if in the future it panics; if wrapping is
                /// required use [`wrapping_add_unsigned`] instead.
                ///
                /// # Examples
                ///
                /// ```rust
                #[doc = concat!("use fixed::{types::extra::U4, ", $s_fixed, ", ", $s_ufixed, "};")]
                #[doc = concat!("type Fix = ", $s_fixed, "<U4>;")]
                #[doc = concat!("type UFix = ", $s_ufixed, "<U4>;")]
                /// assert_eq!(Fix::from_num(-5).add_unsigned(UFix::from_num(3)), -2);
                /// ```
                ///
                /// [`wrapping_add_unsigned`]: Self::wrapping_add_unsigned
                #[inline]
                #[must_use]
                pub const fn add_unsigned(self, rhs: $UFixed<Frac>) -> $Fixed<Frac> {
                    let (ans, overflow) = self.overflowing_add_unsigned(rhs);
                    debug_assert!(!overflow, "overflow");
                    ans
                }

                /// Subtraction with an unsigned fixed-point number.
                ///
                /// # Panics
                ///
                /// When debug assertions are enabled, this method panics if the
                /// result overflows. When debug assertions are not enabled, the
                /// wrapped value can be returned, but it is not considered a
                /// breaking change if in the future it panics; if wrapping is
                /// required use [`wrapping_sub_unsigned`] instead.
                ///
                /// # Examples
                ///
                /// ```rust
                #[doc = concat!("use fixed::{types::extra::U4, ", $s_fixed, ", ", $s_ufixed, "};")]
                #[doc = concat!("type Fix = ", $s_fixed, "<U4>;")]
                #[doc = concat!("type UFix = ", $s_ufixed, "<U4>;")]
                /// assert_eq!(Fix::from_num(3).sub_unsigned(UFix::from_num(5)), -2);
                /// ```
                ///
                /// [`wrapping_sub_unsigned`]: Self::wrapping_sub_unsigned
                #[inline]
                #[must_use]
                pub const fn sub_unsigned(self, rhs: $UFixed<Frac>) -> $Fixed<Frac> {
                    let (ans, overflow) = self.overflowing_sub_unsigned(rhs);
                    debug_assert!(!overflow, "overflow");
                    ans
                }
            }

            if_unsigned! {
                $Signedness;
                /// Addition with a signed fixed-point number.
                ///
                /// # Panics
                ///
                /// When debug assertions are enabled, this method panics if the
                /// result overflows. When debug assertions are not enabled, the
                /// wrapped value can be returned, but it is not considered a
                /// breaking change if in the future it panics; if wrapping is
                /// required use [`wrapping_add_signed`] instead.
                ///
                /// # Examples
                ///
                /// ```rust
                #[doc = concat!("use fixed::{types::extra::U4, ", $s_ifixed, ", ", $s_fixed, "};")]
                #[doc = concat!("type Fix = ", $s_fixed, "<U4>;")]
                #[doc = concat!("type IFix = ", $s_ifixed, "<U4>;")]
                /// assert_eq!(Fix::from_num(5).add_signed(IFix::from_num(-3)), 2);
                /// ```
                ///
                /// [`wrapping_add_signed`]: Self::wrapping_add_signed
                #[inline]
                #[must_use]
                pub const fn add_signed(self, rhs: $IFixed<Frac>) -> $Fixed<Frac> {
                    let (ans, overflow) = self.overflowing_add_signed(rhs);
                    debug_assert!(!overflow, "overflow");
                    ans
                }

                /// Subtraction with a signed fixed-point number.
                ///
                /// # Panics
                ///
                /// When debug assertions are enabled, this method panics if the
                /// result overflows. When debug assertions are not enabled, the
                /// wrapped value can be returned, but it is not considered a
                /// breaking change if in the future it panics; if wrapping is
                /// required use [`wrapping_sub_signed`] instead.
                ///
                /// # Examples
                ///
                /// ```rust
                #[doc = concat!("use fixed::{types::extra::U4, ", $s_ifixed, ", ", $s_fixed, "};")]
                #[doc = concat!("type Fix = ", $s_fixed, "<U4>;")]
                #[doc = concat!("type IFix = ", $s_ifixed, "<U4>;")]
                /// assert_eq!(Fix::from_num(5).sub_signed(IFix::from_num(-3)), 8);
                /// ```
                ///
                /// [`wrapping_sub_signed`]: Self::wrapping_sub_signed
                #[inline]
                #[must_use]
                pub const fn sub_signed(self, rhs: $IFixed<Frac>) -> $Fixed<Frac> {
                    let (ans, overflow) = self.overflowing_sub_signed(rhs);
                    debug_assert!(!overflow, "overflow");
                    ans
                }
            }

            comment! {
                "Bitwise NOT. Usable in constant context.

This is equivalent to the `!` operator and
<code>[Not][core::ops::Not]::[not][core::ops::Not::not]</code>, but
can also be used in constant context. Unless required in constant
context, use the operator or trait instead.

# Planned deprecation

This method will be deprecated when the `!` operator and the
[`Not`][core::ops::Not] trait are usable in constant context.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
const A: Fix = Fix::from_bits(0x3E);
const NOT_A: Fix = A.const_not();
assert_eq!(NOT_A, !A);
```
";
                #[inline]
                #[must_use]
                pub const fn const_not(self) -> $Fixed<Frac> {
                    Self::from_bits(!self.to_bits())
                }
            }

            comment! {
                "Bitwise AND. Usable in constant context.

This is equivalent to the `&` operator and
<code>[BitAnd][core::ops::BitAnd]::[bitand][core::ops::BitAnd::bitand]</code>,
but can also be used in constant context. Unless required in constant
context, use the operator or trait instead.

# Planned deprecation

This method will be deprecated when the `&` operator and the
[`BitAnd`][core::ops::BitAnd] trait are usable in constant context.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
const A: Fix = Fix::from_bits(0x3E);
const B: Fix = Fix::from_bits(0x55);
const A_BITAND_B: Fix = A.const_bitand(B);
assert_eq!(A_BITAND_B, A & B);
```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn const_bitand(self, rhs: $Fixed<Frac>) -> $Fixed<Frac> {
                    Self::from_bits(self.to_bits() & rhs.to_bits())
                }
            }

            comment! {
                "Bitwise OR. Usable in constant context.

This is equivalent to the `|` operator and
<code>[BitOr][core::ops::BitOr]::[bitor][core::ops::BitOr::bitor]</code>,
but can also be used in constant context. Unless required in constant
context, use the operator or trait instead.

# Planned deprecation

This method will be deprecated when the `|` operator and the
[`BitOr`][core::ops::BitOr] trait are usable in constant context.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
const A: Fix = Fix::from_bits(0x3E);
const B: Fix = Fix::from_bits(0x55);
const A_BITOR_B: Fix = A.const_bitor(B);
assert_eq!(A_BITOR_B, A | B);
```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn const_bitor(self, rhs: $Fixed<Frac>) -> $Fixed<Frac> {
                    Self::from_bits(self.to_bits() | rhs.to_bits())
                }
            }

            comment! {
                "Bitwise XOR. Usable in constant context.

This is equivalent to the `^` operator and
<code>[BitXor][core::ops::BitXor]::[bitxor][core::ops::BitXor::bitxor]</code>,
but can also be used in constant context. Unless required in constant
context, use the operator or trait instead.

# Planned deprecation

This method will be deprecated when the `^` operator and the
[`BitXor`][core::ops::BitXor] trait are usable in constant context.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
const A: Fix = Fix::from_bits(0x3E);
const B: Fix = Fix::from_bits(0x55);
const A_BITXOR_B: Fix = A.const_bitxor(B);
assert_eq!(A_BITXOR_B, A ^ B);
```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn const_bitxor(self, rhs: $Fixed<Frac>) -> $Fixed<Frac> {
                    Self::from_bits(self.to_bits() ^ rhs.to_bits())
                }
            }

            comment! {
                "Checked negation. Returns the negated value, or [`None`] on overflow.

",
                if_signed_unsigned!(
                    $Signedness,
                    "Overflow can only occur when negating the minimum value.",
                    "Only zero can be negated without overflow.",
                ),
                "

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
",
                if_signed_unsigned!(
                    $Signedness,
                    "assert_eq!(Fix::from_num(5).checked_neg(), Some(Fix::from_num(-5)));
assert_eq!(Fix::MIN.checked_neg(), None);",
                    "assert_eq!(Fix::ZERO.checked_neg(), Some(Fix::ZERO));
assert_eq!(Fix::from_num(5).checked_neg(), None);",
                ),
                "
```
";
                #[inline]
                pub const fn checked_neg(self) -> Option<$Fixed<Frac>> {
                    match self.to_bits().checked_neg() {
                        None => None,
                        Some(bits) => Some(Self::from_bits(bits)),
                    }
                }
            }

            comment! {
                "Checked addition. Returns the sum, or [`None`] on overflow.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!((Fix::MAX - Fix::ONE).checked_add(Fix::ONE), Some(Fix::MAX));
assert_eq!(Fix::MAX.checked_add(Fix::ONE), None);
```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn checked_add(self, rhs: $Fixed<Frac>) -> Option<$Fixed<Frac>> {
                    match self.to_bits().checked_add(rhs.to_bits()) {
                        None => None,
                        Some(bits) => Some(Self::from_bits(bits)),
                    }
                }
            }

            comment! {
                "Checked subtraction. Returns the difference, or [`None`] on overflow.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!((Fix::MIN + Fix::ONE).checked_sub(Fix::ONE), Some(Fix::MIN));
assert_eq!(Fix::MIN.checked_sub(Fix::ONE), None);
```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn checked_sub(self, rhs: $Fixed<Frac>) -> Option<$Fixed<Frac>> {
                    match self.to_bits().checked_sub(rhs.to_bits()) {
                        None => None,
                        Some(bits) => Some(Self::from_bits(bits)),
                    }
                }
            }

            comment! {
                "Checked remainder. Returns the remainder, or [`None`] if
the divisor is zero.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::from_num(1.5).checked_rem(Fix::ONE), Some(Fix::from_num(0.5)));
assert_eq!(Fix::from_num(1.5).checked_rem(Fix::ZERO), None);
```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn checked_rem(self, rhs: $Fixed<Frac>) -> Option<$Fixed<Frac>> {
                    let rhs_bits = rhs.to_bits();
                    if_signed! {
                        $Signedness;
                        if rhs_bits == -1 {
                            return Some(Self::ZERO);
                        }
                    }
                    match self.to_bits().checked_rem(rhs_bits) {
                        None => None,
                        Some(bits) => Some(Self::from_bits(bits)),
                    }
                }
            }

            comment! {
                "Checked multiply and add.
Returns `self` × `mul` + `add`, or [`None`] on overflow.

",
                if_signed_else_empty_str! {
                    $Signedness;
                    "For some cases, the product `self` × `mul` would overflow
on its own, but the final result `self` × `mul` + `add` is representable; in
these cases this method returns the correct result without overflow.

",
                },
                "The `mul` parameter can have a fixed-point type like
`self` but with a different number of fractional bits.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(
    Fix::from_num(4).checked_mul_add(Fix::from_num(0.5), Fix::from_num(3)),
    Some(Fix::from_num(5))
);
assert_eq!(Fix::MAX.checked_mul_add(Fix::ONE, Fix::ZERO), Some(Fix::MAX));
assert_eq!(Fix::MAX.checked_mul_add(Fix::ONE, Fix::DELTA), None);
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "// MAX × 1.5 - MAX = MAX / 2, which does not overflow
assert_eq!(Fix::MAX.checked_mul_add(Fix::from_num(1.5), -Fix::MAX), Some(Fix::MAX / 2));
"
                },
                "```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn checked_mul_add<MulFrac: $LeEqU>(
                    self,
                    mul: $Fixed<MulFrac>,
                    add: $Fixed<Frac>,
                ) -> Option<$Fixed<Frac>> {
                    match arith::$Inner::overflowing_mul_add(
                        self.to_bits(),
                        mul.to_bits(),
                        add.to_bits(),
                        MulFrac::I32,
                    ) {
                        (ans, false) => Some(Self::from_bits(ans)),
                        (_, true) => None,
                    }
                }
            }

            comment! {
                "Checked multiplication by an integer. Returns the
product, or [`None`] on overflow.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::MAX.checked_mul_int(1), Some(Fix::MAX));
assert_eq!(Fix::MAX.checked_mul_int(2), None);
```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn checked_mul_int(self, rhs: $Inner) -> Option<$Fixed<Frac>> {
                    match self.to_bits().checked_mul(rhs) {
                        None => None,
                        Some(bits) => Some(Self::from_bits(bits)),
                    }
                }
            }

            comment! {
                "Checked division by an integer. Returns the quotient, or
[`None`] if the divisor is zero",
                if_signed_unsigned!(
                    $Signedness,
                    " or if the division results in overflow.",
                    ".",
                ),
                "

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::MAX.checked_div_int(1), Some(Fix::MAX));
assert_eq!(Fix::ONE.checked_div_int(0), None);
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "assert_eq!(Fix::MIN.checked_div_int(-1), None);
",
                },
                "```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn checked_div_int(self, rhs: $Inner) -> Option<$Fixed<Frac>> {
                    match self.to_bits().checked_div(rhs) {
                        None => None,
                        Some(bits) => Some(Self::from_bits(bits)),
                    }
                }
            }

            comment! {
                "Checked remainder for Euclidean division. Returns the
remainder, or [`None`] if the divisor is zero.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let num = Fix::from_num(7.5);
assert_eq!(num.checked_rem_euclid(Fix::from_num(2)), Some(Fix::from_num(1.5)));
assert_eq!(num.checked_rem_euclid(Fix::ZERO), None);
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "assert_eq!((-num).checked_rem_euclid(Fix::from_num(2)), Some(Fix::from_num(0.5)));
",
                },
                "```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn checked_rem_euclid(self, rhs: $Fixed<Frac>) -> Option<$Fixed<Frac>> {
                    let rhs_bits = rhs.to_bits();
                    if_signed! {
                        $Signedness;
                        if rhs_bits == -1 {
                            return Some(Self::ZERO);
                        }
                    }
                    match self.to_bits().checked_rem_euclid(rhs_bits) {
                        None => None,
                        Some(bits) => Some(Self::from_bits(bits)),
                    }
                }
            }

            comment! {
                "Checked shift left. Returns the shifted number,
or [`None`] if `rhs`&nbsp;≥&nbsp;", $s_nbits, ".

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!((Fix::ONE / 2).checked_shl(3), Some(Fix::from_num(4)));
assert_eq!((Fix::ONE / 2).checked_shl(", $s_nbits, "), None);
```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn checked_shl(self, rhs: u32) -> Option<$Fixed<Frac>> {
                    match self.to_bits().checked_shl(rhs) {
                        None => None,
                        Some(bits) => Some(Self::from_bits(bits)),
                    }
                }
            }

            comment! {
                "Checked shift right. Returns the shifted number,
or [`None`] if `rhs`&nbsp;≥&nbsp;", $s_nbits, ".

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::from_num(4).checked_shr(3), Some(Fix::ONE / 2));
assert_eq!(Fix::from_num(4).checked_shr(", $s_nbits, "), None);
```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn checked_shr(self, rhs: u32) -> Option<$Fixed<Frac>> {
                    match self.to_bits().checked_shr(rhs) {
                        None => None,
                        Some(bits) => Some(Self::from_bits(bits)),
                    }
                }
            }

            if_signed! {
                $Signedness;
                comment! {
                    "Checked absolute value. Returns the absolute value, or [`None`] on overflow.

Overflow can only occur when trying to find the absolute value of the minimum value.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::from_num(-5).checked_abs(), Some(Fix::from_num(5)));
assert_eq!(Fix::MIN.checked_abs(), None);
```
";
                    #[inline]
                    pub const fn checked_abs(self) -> Option<$Fixed<Frac>> {
                        match self.to_bits().checked_abs() {
                            None => None,
                            Some(bits) => Some(Self::from_bits(bits)),
                        }
                    }
                }
            }

            comment! {
                "Checked distance. Returns the distance from `self` to `other`",
                if_signed_else_empty_str! { $Signedness; ", or [`None`] on overflow" },
                ".

The distance is the absolute value of the difference.

",
                if_unsigned_else_empty_str! {
                    $Signedness;
                    "Can never overflow for unsigned types.

",
                },
                "# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::ONE.checked_dist(Fix::from_num(5)), Some(Fix::from_num(4)));
",
                if_signed_unsigned!(
                    $Signedness,
                    "assert_eq!(Fix::MIN.checked_dist(Fix::ZERO), None);",
                    "assert_eq!(Fix::ZERO.checked_dist(Fix::MAX), Some(Fix::MAX));",
                ),
                "
```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn checked_dist(self, other: $Fixed<Frac>) -> Option<$Fixed<Frac>> {
                    if_signed! {
                        $Signedness;
                        if self.to_bits() < other.to_bits() {
                            other.checked_sub(self)
                        } else {
                            self.checked_sub(other)
                        }
                    }
                    if_unsigned! {
                        $Signedness;
                        Some(self.dist(other))
                    }
                }
            }

            comment! {
                "Checked next multiple of `other`. Returns the next multiple, or
[`None`] if `other` is zero or on overflow.

The next multiple is the smallest multiple of `other` that is ≥&nbsp;`self`",
                if_signed_else_empty_str! {
                    $Signedness;
                    " if `other` is positive, and the largest multiple of
`other` that is ≤&nbsp;`self` if `other` is negative",
                },
                ".

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(
    Fix::from_num(4).checked_next_multiple_of(Fix::from_num(1.5)),
    Some(Fix::from_num(4.5))
);
assert_eq!(Fix::from_num(4).checked_next_multiple_of(Fix::ZERO), None);
assert_eq!(Fix::MAX.checked_next_multiple_of(Fix::from_num(2)), None);
```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn checked_next_multiple_of(
                    self,
                    other: $Fixed<Frac>,
                ) -> Option<$Fixed<Frac>> {
                    if other.to_bits() == 0 {
                        None
                    } else {
                        match self.overflowing_next_multiple_of(other) {
                            (ans, false) => Some(ans),
                            (_, true) => None,
                        }
                    }
                }
            }

            comment! {
                "Checked inverse linear interpolation between `start` and `end`.
Returns [`None`] on overflow or when `start`&nbsp;=&nbsp;`end`.

The computed value can have a fixed-point type like `self` but with a different
number of fractional bits.

Returns
(`self`&nbsp;&minus;&nbsp;`start`)&nbsp;/&nbsp;(`end`&nbsp;&minus;&nbsp;`start`).
This is 0 when `self`&nbsp;=&nbsp;`start`, and 1 when `self`&nbsp;=&nbsp;`end`.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let two = Fix::from_num(2);
let four = Fix::from_num(4);
assert_eq!(Fix::from_num(3).checked_inv_lerp::<U4>(two, four), Some(Fix::from_num(0.5)));
assert_eq!(Fix::from_num(2).checked_inv_lerp::<U4>(two, two), None);
assert_eq!(Fix::MAX.checked_inv_lerp::<U4>(Fix::ZERO, Fix::from_num(0.5)), None);
```
";
                #[inline]
                pub const fn checked_inv_lerp<RetFrac: $LeEqU>(
                    self,
                    start: $Fixed<Frac>,
                    end: $Fixed<Frac>,
                ) -> Option<$Fixed<RetFrac>> {
                    let start = start.to_bits();
                    let end = end.to_bits();
                    if start == end {
                        return None;
                    }
                    match inv_lerp::$Inner(self.to_bits(), start, end, RetFrac::U32) {
                        (bits, false) => Some($Fixed::from_bits(bits)),
                        (_, true) => None,
                    }
                }
            }

            if_unsigned! {
                $Signedness;
                comment! {
                    "Returns the smallest power of two that is ≥&nbsp;`self`, or
[`None`] if the next power of two is too large to represent.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
// 3/8 is 0.0110
let three_eights = Fix::from_bits(0b0110);
// 1/2 is 0.1000
let half = Fix::from_bits(0b1000);
assert_eq!(three_eights.checked_next_power_of_two(), Some(half));
assert!(Fix::MAX.checked_next_power_of_two().is_none());
```
";
                    #[inline]
                    pub const fn checked_next_power_of_two(self) -> Option<$Fixed<Frac>> {
                        match self.to_bits().checked_next_power_of_two() {
                            Some(bits) => Some(Self::from_bits(bits)),
                            None => None,
                        }
                    }
                }
            }

            if_signed! {
                $Signedness;
                /// Checked addition with an unsigned fixed-point number.
                /// Returns the sum, or [`None`] on overflow.
                ///
                /// # Examples
                ///
                /// ```rust
                #[doc = concat!("use fixed::{types::extra::U4, ", $s_fixed, ", ", $s_ufixed, "};")]
                #[doc = concat!("type Fix = ", $s_fixed, "<U4>;")]
                #[doc = concat!("type UFix = ", $s_ufixed, "<U4>;")]
                /// assert_eq!(
                ///     Fix::from_num(-5).checked_add_unsigned(UFix::from_num(3)),
                ///     Some(Fix::from_num(-2))
                /// );
                /// assert_eq!(Fix::MAX.checked_add_unsigned(UFix::DELTA), None);
                /// ```
                #[inline]
                #[must_use]
                pub const fn checked_add_unsigned(
                    self,
                    rhs: $UFixed<Frac>,
                ) -> Option<$Fixed<Frac>> {
                    match self.overflowing_add_unsigned(rhs) {
                        (ans, false) => Some(ans),
                        (_, true) => None,
                    }
                }

                /// Checked subtraction with an unsigned fixed-point number.
                /// Returns the difference, or [`None`] on overflow.
                ///
                /// # Examples
                ///
                /// ```rust
                #[doc = concat!("use fixed::{types::extra::U4, ", $s_fixed, ", ", $s_ufixed, "};")]
                #[doc = concat!("type Fix = ", $s_fixed, "<U4>;")]
                #[doc = concat!("type UFix = ", $s_ufixed, "<U4>;")]
                /// assert_eq!(
                ///     Fix::from_num(3).checked_sub_unsigned(UFix::from_num(5)),
                ///     Some(Fix::from_num(-2))
                /// );
                /// assert_eq!(Fix::MIN.checked_sub_unsigned(UFix::DELTA), None);
                /// ```
                #[inline]
                #[must_use]
                pub const fn checked_sub_unsigned(
                    self,
                    rhs: $UFixed<Frac>,
                ) -> Option<$Fixed<Frac>> {
                    match self.overflowing_sub_unsigned(rhs) {
                        (ans, false) => Some(ans),
                        (_, true) => None,
                    }
                }
            }

            if_unsigned! {
                $Signedness;
                /// Checked addition with a signed fixed-point number.
                /// Returns the sum, or [`None`] on overflow.
                ///
                /// # Examples
                ///
                /// ```rust
                #[doc = concat!("use fixed::{types::extra::U4, ", $s_ifixed, ", ", $s_fixed, "};")]
                #[doc = concat!("type Fix = ", $s_fixed, "<U4>;")]
                #[doc = concat!("type IFix = ", $s_ifixed, "<U4>;")]
                /// assert_eq!(
                ///     Fix::from_num(5).checked_add_signed(IFix::from_num(-3)),
                ///     Some(Fix::from_num(2))
                /// );
                /// assert_eq!(Fix::from_num(2).checked_add_signed(IFix::from_num(-3)), None);
                /// ```
                #[inline]
                #[must_use]
                pub const fn checked_add_signed(self, rhs: $IFixed<Frac>) -> Option<$Fixed<Frac>> {
                    match self.overflowing_add_signed(rhs) {
                        (ans, false) => Some(ans),
                        (_, true) => None,
                    }
                }

                /// Checked subtraction with a signed fixed-point number.
                /// Returns the difference, or [`None`] on overflow.
                ///
                /// # Examples
                ///
                /// ```rust
                #[doc = concat!("use fixed::{types::extra::U4, ", $s_ifixed, ", ", $s_fixed, "};")]
                #[doc = concat!("type Fix = ", $s_fixed, "<U4>;")]
                #[doc = concat!("type IFix = ", $s_ifixed, "<U4>;")]
                /// assert_eq!(
                ///     Fix::from_num(5).checked_sub_signed(IFix::from_num(-3)),
                ///     Some(Fix::from_num(8))
                /// );
                /// assert_eq!(Fix::from_num(2).checked_sub_signed(IFix::from_num(3)), None);
                /// ```
                #[inline]
                #[must_use]
                pub const fn checked_sub_signed(self, rhs: $IFixed<Frac>) -> Option<$Fixed<Frac>> {
                    match self.overflowing_sub_signed(rhs) {
                        (ans, false) => Some(ans),
                        (_, true) => None,
                    }
                }
            }

            comment! {
                "Saturating negation. Returns the negated value, saturating on overflow.

",
                if_signed_unsigned!(
                    $Signedness,
                    "Overflow can only occur when negating the minimum value.",
                    "This method always returns zero.",
                ),
                "

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
",
                if_signed_unsigned!(
                    $Signedness,
                    "assert_eq!(Fix::from_num(5).saturating_neg(), Fix::from_num(-5));
assert_eq!(Fix::MIN.saturating_neg(), Fix::MAX);",
                    "assert_eq!(Fix::ZERO.saturating_neg(), Fix::from_num(0));
assert_eq!(Fix::from_num(5).saturating_neg(), Fix::ZERO);",
                ),
                "
```
";
                #[inline]
                #[must_use]
                pub const fn saturating_neg(self) -> $Fixed<Frac> {
                    if_signed_unsigned!(
                        $Signedness,
                        {
                            match self.overflowing_neg() {
                                (val, false) => val,
                                (_, true) => Self::MAX,
                            }
                        },
                        Self::ZERO,
                    )
                }
            }

            comment! {
                "Saturating addition. Returns the sum, saturating on overflow.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::from_num(3).saturating_add(Fix::from_num(2)), Fix::from_num(5));
assert_eq!(Fix::MAX.saturating_add(Fix::ONE), Fix::MAX);
```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn saturating_add(self, rhs: $Fixed<Frac>) -> $Fixed<Frac> {
                    match self.overflowing_add(rhs) {
                        (val, false) => val,
                        (_, true) => if_signed_unsigned!(
                            $Signedness,
                            if self.is_negative() { Self::MIN } else { Self::MAX },
                            Self::MAX,
                        ),
                    }
                }
            }

            comment! {
                "Saturating subtraction. Returns the difference, saturating on overflow.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
",
                if_signed_unsigned!(
                    $Signedness,
                    "assert_eq!(Fix::ONE.saturating_sub(Fix::from_num(3)), Fix::from_num(-2));
assert_eq!(Fix::MIN.saturating_sub(Fix::ONE), Fix::MIN);",
                    "assert_eq!(Fix::from_num(5).saturating_sub(Fix::from_num(3)), Fix::from_num(2));
assert_eq!(Fix::ZERO.saturating_sub(Fix::ONE), Fix::ZERO);",
                ),
                "
```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn saturating_sub(self, rhs: $Fixed<Frac>) -> $Fixed<Frac> {
                    match self.overflowing_sub(rhs) {
                        (val, false) => val,
                        (_, true) => if_signed_unsigned!(
                            $Signedness,
                            if self.to_bits() < rhs.to_bits() {
                                Self::MIN
                            } else {
                                Self::MAX
                            },
                            Self::MIN,
                        ),
                    }
                }
            }

            comment! {
                "Saturating multiply and add.
Returns `self` × `mul` + `add`, saturating on overflow.

",
                if_signed_else_empty_str! {
                    $Signedness;
                    "For some cases, the product `self` × `mul` would overflow
on its own, but the final result `self` × `mul` + `add` is representable; in
these cases this method returns the correct result without overflow.

",
                },
                "The `mul` parameter can have a fixed-point type like
`self` but with a different number of fractional bits.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(
    Fix::from_num(4).saturating_mul_add(Fix::from_num(0.5), Fix::from_num(3)),
    Fix::from_num(5)
);
let half_max = Fix::MAX / 2;
assert_eq!(half_max.saturating_mul_add(Fix::from_num(3), half_max), Fix::MAX);
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "assert_eq!(half_max.saturating_mul_add(Fix::from_num(-5), half_max), Fix::MIN);
// MAX × 1.5 - MAX = MAX / 2, which does not overflow
assert_eq!(Fix::MAX.saturating_mul_add(Fix::from_num(1.5), -Fix::MAX), half_max);
"
                },
                "```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn saturating_mul_add<MulFrac: $LeEqU>(
                    self,
                    mul: $Fixed<MulFrac>,
                    add: $Fixed<Frac>,
                ) -> $Fixed<Frac> {
                    match arith::$Inner::overflowing_mul_add(
                        self.to_bits(),
                        mul.to_bits(),
                        add.to_bits(),
                        MulFrac::I32,
                    ) {
                        (ans, false) => Self::from_bits(ans),
                        (_, true) => {
                            let negative = if_signed_unsigned!(
                                $Signedness,
                                self.is_negative() != mul.is_negative(),
                                false,
                            );
                            if negative {
                                Self::MIN
                            } else {
                                Self::MAX
                            }
                        }
                    }
                }
            }

            comment! {
                "Saturating multiplication by an integer. Returns the product, saturating on overflow.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::from_num(3).saturating_mul_int(2), Fix::from_num(6));
assert_eq!(Fix::MAX.saturating_mul_int(2), Fix::MAX);
```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn saturating_mul_int(self, rhs: $Inner) -> $Fixed<Frac> {
                    match self.overflowing_mul_int(rhs) {
                        (val, false) => val,
                        (_, true) => if_signed_unsigned!(
                            $Signedness,
                            if self.is_negative() != rhs.is_negative() {
                                Self::MIN
                            } else {
                                Self::MAX
                            },
                            Self::MAX,
                        ),
                    }
                }
            }

            if_signed! {
                $Signedness;
                comment! {
                    "Saturating absolute value. Returns the absolute value, saturating on overflow.

Overflow can only occur when trying to find the absolute value of the minimum value.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::from_num(-5).saturating_abs(), Fix::from_num(5));
assert_eq!(Fix::MIN.saturating_abs(), Fix::MAX);
```
";
                    #[inline]
                    #[must_use]
                    pub const fn saturating_abs(self) -> $Fixed<Frac> {
                        match self.overflowing_abs() {
                            (val, false) => val,
                            (_, true) => Self::MAX,
                        }
                    }
                }
            }

            comment! {
                "Saturating distance. Returns the distance from `self` to `other`",
                if_signed_else_empty_str! { $Signedness; ", saturating on overflow" },
                ".

The distance is the absolute value of the difference.

",
                if_unsigned_else_empty_str! {
                    $Signedness;
                    "Can never overflow for unsigned types.

",
                },
                "# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::ONE.saturating_dist(Fix::from_num(5)), Fix::from_num(4));
",
                if_signed_unsigned!(
                    $Signedness,
                    "assert_eq!(Fix::MIN.saturating_dist(Fix::MAX), Fix::MAX);",
                    "assert_eq!(Fix::ZERO.saturating_dist(Fix::MAX), Fix::MAX);",
                ),
                "
```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn saturating_dist(self, other: $Fixed<Frac>) -> $Fixed<Frac> {
                    if_signed! {
                        $Signedness;
                        match self.checked_dist(other) {
                            None => $Fixed::MAX,
                            Some(dist) => dist,
                        }
                    }
                    if_unsigned! {
                        $Signedness;
                        self.dist(other)
                    }
                }
            }

            comment! {
                "Saturating next multiple of `other`.

The next multiple is the smallest multiple of `other` that is ≥&nbsp;`self`",
                if_signed_else_empty_str! {
                    $Signedness;
                    " if `other` is positive, and the largest multiple of
`other` that is ≤&nbsp;`self` if `other` is negative",
                },
                ".

# Panics

Panics if `other` is zero.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(
    Fix::from_num(4).saturating_next_multiple_of(Fix::from_num(1.5)),
    Fix::from_num(4.5)
);
assert_eq!(Fix::MAX.saturating_next_multiple_of(Fix::from_num(2)), Fix::MAX);
```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn saturating_next_multiple_of(
                    self,
                    other: $Fixed<Frac>
                ) -> $Fixed<Frac> {
                    match self.overflowing_next_multiple_of(other) {
                        (ans, false) => ans,
                        (_, true) => {
                            if_signed_unsigned!(
                                $Signedness,
                                if other.to_bits() < 0 {
                                    $Fixed::MIN
                                } else {
                                    $Fixed::MAX
                                },
                                $Fixed::MAX,
                            )
                        }
                    }
                }
            }

            comment! {
                "Inverse linear interpolation between `start` and `end`,
saturating on overflow.

The computed value can have a fixed-point type like `self` but with a different
number of fractional bits.

Returns
(`self`&nbsp;&minus;&nbsp;`start`)&nbsp;/&nbsp;(`end`&nbsp;&minus;&nbsp;`start`).
This is 0 when `self`&nbsp;=&nbsp;`start`, and 1 when `self`&nbsp;=&nbsp;`end`.

# Panics

Panics when `start`&nbsp;=&nbsp;`end`.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let two = Fix::from_num(2);
let four = Fix::from_num(4);
assert_eq!(Fix::from_num(3).saturating_inv_lerp::<U4>(two, four), 0.5);
assert_eq!(Fix::MAX.saturating_inv_lerp::<U4>(Fix::ZERO, Fix::from_num(0.5)), Fix::MAX);
assert_eq!(Fix::MAX.saturating_inv_lerp::<U4>(Fix::from_num(0.5), Fix::ZERO), Fix::MIN);
```
";
                #[inline]
                pub const fn saturating_inv_lerp<RetFrac: $LeEqU>(
                    self,
                    start: $Fixed<Frac>,
                    end: $Fixed<Frac>,
                ) -> $Fixed<RetFrac> {
                    let self_bits = self.to_bits();
                    let start = start.to_bits();
                    let end = end.to_bits();
                    match inv_lerp::$Inner(self_bits, start, end, RetFrac::U32) {
                        (bits, false) => $Fixed::from_bits(bits),
                        (_, true) => if_signed_unsigned!(
                            $Signedness,
                            if (self_bits < start) == (end < start) {
                                $Fixed::MAX
                            } else {
                                $Fixed::MIN
                            },
                            if end < start {
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
                /// Saturating addition with an unsigned fixed-point number.
                /// Returns the sum, saturating on overflow.
                ///
                /// # Examples
                ///
                /// ```rust
                #[doc = concat!("use fixed::{types::extra::U4, ", $s_fixed, ", ", $s_ufixed, "};")]
                #[doc = concat!("type Fix = ", $s_fixed, "<U4>;")]
                #[doc = concat!("type UFix = ", $s_ufixed, "<U4>;")]
                /// assert_eq!(Fix::from_num(-5).saturating_add_unsigned(UFix::from_num(3)), -2);
                /// assert_eq!(Fix::from_num(-5).saturating_add_unsigned(UFix::MAX), Fix::MAX);
                /// ```
                #[inline]
                #[must_use]
                pub const fn saturating_add_unsigned(self, rhs: $UFixed<Frac>) -> $Fixed<Frac> {
                    match self.overflowing_add_unsigned(rhs) {
                        (ans, false) => ans,
                        (_, true) => $Fixed::MAX,
                    }
                }

                /// Saturating subtraction with an unsigned fixed-point number.
                /// Returns the difference, saturating on overflow.
                ///
                /// # Examples
                ///
                /// ```rust
                #[doc = concat!("use fixed::{types::extra::U4, ", $s_fixed, ", ", $s_ufixed, "};")]
                #[doc = concat!("type Fix = ", $s_fixed, "<U4>;")]
                #[doc = concat!("type UFix = ", $s_ufixed, "<U4>;")]
                /// assert_eq!(Fix::from_num(3).saturating_sub_unsigned(UFix::from_num(5)), -2);
                /// assert_eq!(Fix::from_num(5).saturating_sub_unsigned(UFix::MAX), Fix::MIN);
                /// ```
                #[inline]
                #[must_use]
                pub const fn saturating_sub_unsigned(self, rhs: $UFixed<Frac>) -> $Fixed<Frac> {
                    match self.overflowing_sub_unsigned(rhs) {
                        (ans, false) => ans,
                        (_, true) => $Fixed::MIN,
                    }
                }
            }

            if_unsigned! {
                $Signedness;
                /// Saturating addition with a signed fixed-point number.
                /// Returns the sum, saturating on overflow.
                ///
                /// # Examples
                ///
                /// ```rust
                #[doc = concat!("use fixed::{types::extra::U4, ", $s_ifixed, ", ", $s_fixed, "};")]
                #[doc = concat!("type Fix = ", $s_fixed, "<U4>;")]
                #[doc = concat!("type IFix = ", $s_ifixed, "<U4>;")]
                /// assert_eq!(Fix::from_num(5).saturating_add_signed(IFix::from_num(-3)), 2);
                /// assert_eq!(Fix::from_num(2).saturating_add_signed(IFix::from_num(-3)), 0);
                /// assert_eq!(Fix::MAX.saturating_add_signed(IFix::MAX), Fix::MAX);
                /// ```
                #[inline]
                #[must_use]
                pub const fn saturating_add_signed(self, rhs: $IFixed<Frac>) -> $Fixed<Frac> {
                    match self.overflowing_add_signed(rhs) {
                        (ans, false) => ans,
                        (_, true) => {
                            if rhs.is_negative() {
                                $Fixed::ZERO
                            } else {
                                $Fixed::MAX
                            }
                        }
                    }
                }

                /// Saturating subtraction with a signed fixed-point number.
                /// Returns the difference, saturating on overflow.
                ///
                /// # Examples
                ///
                /// ```rust
                #[doc = concat!("use fixed::{types::extra::U4, ", $s_ifixed, ", ", $s_fixed, "};")]
                #[doc = concat!("type Fix = ", $s_fixed, "<U4>;")]
                #[doc = concat!("type IFix = ", $s_ifixed, "<U4>;")]
                /// assert_eq!(Fix::from_num(5).saturating_sub_signed(IFix::from_num(-3)), 8);
                /// assert_eq!(Fix::ONE.saturating_sub_signed(IFix::MAX), Fix::ZERO);
                /// assert_eq!(Fix::MAX.saturating_sub_signed(IFix::MIN), Fix::MAX);
                /// ```
                #[inline]
                #[must_use]
                pub const fn saturating_sub_signed(self, rhs: $IFixed<Frac>) -> $Fixed<Frac> {
                    match self.overflowing_sub_signed(rhs) {
                        (ans, false) => ans,
                        (_, true) => {
                            if rhs.is_negative() {
                                $Fixed::MAX
                            } else {
                                $Fixed::ZERO
                            }
                        }
                    }
                }
            }

            comment! {
                "Wrapping negation. Returns the negated value, wrapping on overflow.

",
                if_signed_unsigned!(
                    $Signedness,
                    "Overflow can only occur when negating the minimum value.",
                    "Only zero can be negated without overflow.",
                ),
                "

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
",
                if_signed_unsigned!(
                    $Signedness,
                    "assert_eq!(Fix::from_num(5).wrapping_neg(), Fix::from_num(-5));
assert_eq!(Fix::MIN.wrapping_neg(), Fix::MIN);",
                    "assert_eq!(Fix::ZERO.wrapping_neg(), Fix::from_num(0));
assert_eq!(Fix::from_num(5).wrapping_neg(), Fix::wrapping_from_num(-5));
let neg_five_bits = !Fix::from_num(5).to_bits() + 1;
assert_eq!(Fix::from_num(5).wrapping_neg(), Fix::from_bits(neg_five_bits));",
                ),
                "
```
";
                #[inline]
                #[must_use]
                pub const fn wrapping_neg(self) -> $Fixed<Frac> {
                    Self::from_bits(self.to_bits().wrapping_neg())
                }
            }

            comment! {
                "Wrapping addition. Returns the sum, wrapping on overflow.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let one_minus_delta = Fix::ONE - Fix::DELTA;
assert_eq!(Fix::from_num(3).wrapping_add(Fix::from_num(2)), Fix::from_num(5));
assert_eq!(Fix::MAX.wrapping_add(Fix::ONE), ",
                if_signed_else_empty_str! { $Signedness; "Fix::MIN + " },
                "one_minus_delta);
```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn wrapping_add(self, rhs: $Fixed<Frac>) -> $Fixed<Frac> {
                    Self::from_bits(self.to_bits().wrapping_add(rhs.to_bits()))
                }
            }

            comment! {
                "Wrapping subtraction. Returns the difference, wrapping on overflow.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let one_minus_delta = Fix::ONE - Fix::DELTA;
",
                if_signed_unsigned!(
                    $Signedness,
                    "assert_eq!(Fix::from_num(3).wrapping_sub(Fix::from_num(5)), Fix::from_num(-2));
assert_eq!(Fix::MIN",
                    "assert_eq!(Fix::from_num(5).wrapping_sub(Fix::from_num(3)), Fix::from_num(2));
assert_eq!(Fix::ZERO",
                ),
                ".wrapping_sub(Fix::ONE), Fix::MAX - one_minus_delta);
```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn wrapping_sub(self, rhs: $Fixed<Frac>) -> $Fixed<Frac> {
                    Self::from_bits(self.to_bits().wrapping_sub(rhs.to_bits()))
                }
            }

            comment! {
                "Wrapping multiply and add.
Returns `self` × `mul` + `add`, wrapping on overflow.

The `mul` parameter can have a fixed-point type like
`self` but with a different number of fractional bits.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(
    Fix::from_num(4).wrapping_mul_add(Fix::from_num(0.5), Fix::from_num(3)),
    Fix::from_num(5)
);
assert_eq!(Fix::MAX.wrapping_mul_add(Fix::ONE, Fix::from_num(0)), Fix::MAX);
assert_eq!(Fix::MAX.wrapping_mul_add(Fix::ONE, Fix::from_bits(1)), Fix::MIN);
let wrapped = Fix::MAX.wrapping_mul_int(4);
assert_eq!(Fix::MAX.wrapping_mul_add(Fix::from_num(3), Fix::MAX), wrapped);
```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn wrapping_mul_add<MulFrac: $LeEqU>(
                    self,
                    mul: $Fixed<MulFrac>,
                    add: $Fixed<Frac>,
                ) -> $Fixed<Frac> {
                    let (ans, _) = arith::$Inner::overflowing_mul_add(
                        self.to_bits(),
                        mul.to_bits(),
                        add.to_bits(),
                        MulFrac::I32,
                    );
                    Self::from_bits(ans)
                }
            }

            comment! {
                "Wrapping multiplication by an integer. Returns the product, wrapping on overflow.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::from_num(3).wrapping_mul_int(2), Fix::from_num(6));
let wrapped = Fix::from_bits(!0 << 2);
assert_eq!(Fix::MAX.wrapping_mul_int(4), wrapped);
```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn wrapping_mul_int(self, rhs: $Inner) -> $Fixed<Frac> {
                    Self::from_bits(self.to_bits().wrapping_mul(rhs))
                }
            }

            comment! {
                "Wrapping division by an integer. Returns the quotient",
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
// 1.5 is binary 1.1
let one_point_5 = Fix::from_bits(0b11 << (4 - 1));
assert_eq!(Fix::from_num(3).wrapping_div_int(2), one_point_5);
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "assert_eq!(Fix::MIN.wrapping_div_int(-1), Fix::MIN);
",
                },
                "```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn wrapping_div_int(self, rhs: $Inner) -> $Fixed<Frac> {
                    Self::from_bits(self.to_bits().wrapping_div(rhs))
                }
            }

            comment! {
                "Wrapping shift left. Wraps `rhs` if `rhs`&nbsp;≥&nbsp;", $s_nbits, ",
then shifts and returns the number.

Unlike most other methods which wrap the result, this method (as well as
[`wrapping_shr`]) wraps the input operand `rhs`.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!((Fix::ONE / 2).wrapping_shl(3), Fix::from_num(4));
assert_eq!((Fix::ONE / 2).wrapping_shl(3 + ", $s_nbits, "), Fix::from_num(4));
```

[`wrapping_shr`]: Self::wrapping_shr
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn wrapping_shl(self, rhs: u32) -> $Fixed<Frac> {
                    Self::from_bits(self.to_bits().wrapping_shl(rhs))
                }
            }

            comment! {
                "Wrapping shift right. Wraps `rhs` if `rhs`&nbsp;≥&nbsp;", $s_nbits, ",
then shifts and returns the number.

Unlike most other methods which wrap the result, this method (as well as
[`wrapping_shl`]) wraps the input operand `rhs`.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!((Fix::from_num(4)).wrapping_shr(3), Fix::ONE / 2);
assert_eq!((Fix::from_num(4)).wrapping_shr(3 + ", $s_nbits, "), Fix::ONE / 2);
```

[`wrapping_shl`]: Self::wrapping_shl
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn wrapping_shr(self, rhs: u32) -> $Fixed<Frac> {
                    Self::from_bits(self.to_bits().wrapping_shr(rhs))
                }
            }

            if_signed! {
                $Signedness;
                comment! {
                    "Wrapping absolute value. Returns the absolute value, wrapping on overflow.

Overflow can only occur when trying to find the absolute value of the minimum value.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::from_num(-5).wrapping_abs(), Fix::from_num(5));
assert_eq!(Fix::MIN.wrapping_abs(), Fix::MIN);
```
";
                    #[inline]
                    #[must_use]
                    pub const fn wrapping_abs(self) -> $Fixed<Frac> {
                        Self::from_bits(self.to_bits().wrapping_abs())
                    }
                }
            }

            comment! {
                "Wrapping distance. Returns the distance from `self` to `other`",
                if_signed_else_empty_str! { $Signedness; ", wrapping on overflow" },
                ".

The distance is the absolute value of the difference.

",
                if_unsigned_else_empty_str! {
                    $Signedness;
                    "Can never overflow for unsigned types.

",
                },
                "# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::ONE.wrapping_dist(Fix::from_num(5)), Fix::from_num(4));
",
                if_signed_unsigned!(
                    $Signedness,
                    "assert_eq!(Fix::MIN.wrapping_dist(Fix::MAX), -Fix::DELTA);",
                    "assert_eq!(Fix::ZERO.wrapping_dist(Fix::MAX), Fix::MAX);",
                ),
                "
```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn wrapping_dist(self, other: $Fixed<Frac>) -> $Fixed<Frac> {
                    if_signed_unsigned!(
                        $Signedness,
                        self.overflowing_dist(other).0,
                        self.dist(other),
                    )
                }
            }

            comment! {
                "Wrapping next multiple of `other`.

The next multiple is the smallest multiple of `other` that is ≥&nbsp;`self`",
                if_signed_else_empty_str! {
                    $Signedness;
                    " if `other` is positive, and the largest multiple of
`other` that is ≤&nbsp;`self` if `other` is negative",
                },
                ".

# Panics

Panics if `other` is zero.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(
    Fix::from_num(4).wrapping_next_multiple_of(Fix::from_num(1.5)),
    Fix::from_num(4.5)
);
let max_minus_delta = Fix::MAX - Fix::DELTA;
assert_eq!(
    Fix::MAX.wrapping_next_multiple_of(max_minus_delta),
    max_minus_delta.wrapping_mul_int(2)
);
```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn wrapping_next_multiple_of(
                    self,
                    other: $Fixed<Frac>
                ) -> $Fixed<Frac> {
                    let (ans, _) = self.overflowing_next_multiple_of(other);
                    ans
                }
            }

            comment! {
                "Inverse linear interpolation between `start` and `end`,
wrapping on overflow.

The computed value can have a fixed-point type like `self` but with a different
number of fractional bits.

Returns
(`self`&nbsp;&minus;&nbsp;`start`)&nbsp;/&nbsp;(`end`&nbsp;&minus;&nbsp;`start`).
This is 0 when `self`&nbsp;=&nbsp;`start`, and 1 when `self`&nbsp;=&nbsp;`end`.

# Panics

Panics when `start`&nbsp;=&nbsp;`end`.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let two = Fix::from_num(2);
let four = Fix::from_num(4);
assert_eq!(Fix::from_num(3).wrapping_inv_lerp::<U4>(two, four), 0.5);
assert_eq!(
    Fix::MAX.wrapping_inv_lerp::<U4>(Fix::ZERO, Fix::from_num(0.5)),
    Fix::MAX.wrapping_mul_int(2)
);
```
";
                #[inline]
                pub const fn wrapping_inv_lerp<RetFrac: $LeEqU>(
                    self,
                    start: $Fixed<Frac>,
                    end: $Fixed<Frac>,
                ) -> $Fixed<RetFrac> {
                    let (bits, _) = inv_lerp::$Inner(
                        self.to_bits(),
                        start.to_bits(),
                        end.to_bits(),
                        RetFrac::U32,
                    );
                    $Fixed::from_bits(bits)
                }
            }

            if_unsigned! {
                $Signedness;
                comment! {
                    "Returns the smallest power of two that is ≥&nbsp;`self`,
wrapping to 0 if the next power of two is too large to represent.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
// 3/8 is 0.0110
let three_eights = Fix::from_bits(0b0110);
// 1/2 is 0.1000
let half = Fix::from_bits(0b1000);
assert_eq!(three_eights.wrapping_next_power_of_two(), half);
assert_eq!(Fix::MAX.wrapping_next_power_of_two(), 0);
```
";
                    #[inline]
                    #[must_use]
                    pub const fn wrapping_next_power_of_two(self) -> $Fixed<Frac> {
                        match self.checked_next_power_of_two() {
                            Some(x) => x,
                            None => Self::ZERO,
                        }
                    }
                }
            }

            if_signed! {
                $Signedness;
                /// Wrapping addition with an unsigned fixed-point number.
                /// Returns the sum, wrapping on overflow.
                ///
                /// # Examples
                ///
                /// ```rust
                #[doc = concat!("use fixed::{types::extra::U4, ", $s_fixed, ", ", $s_ufixed, "};")]
                #[doc = concat!("type Fix = ", $s_fixed, "<U4>;")]
                #[doc = concat!("type UFix = ", $s_ufixed, "<U4>;")]
                /// assert_eq!(Fix::from_num(-5).wrapping_add_unsigned(UFix::from_num(3)), -2);
                /// assert_eq!(Fix::ZERO.wrapping_add_unsigned(UFix::MAX), -Fix::DELTA);
                /// ```
                #[inline]
                #[must_use]
                pub const fn wrapping_add_unsigned(self, rhs: $UFixed<Frac>) -> $Fixed<Frac> {
                    let (ans, _) = self.overflowing_add_unsigned(rhs);
                    ans
                }

                /// Wrapping subtraction with an unsigned fixed-point number.
                /// Returns the difference, wrapping on overflow.
                ///
                /// # Examples
                ///
                /// ```rust
                #[doc = concat!("use fixed::{types::extra::U4, ", $s_fixed, ", ", $s_ufixed, "};")]
                #[doc = concat!("type Fix = ", $s_fixed, "<U4>;")]
                #[doc = concat!("type UFix = ", $s_ufixed, "<U4>;")]
                /// assert_eq!(Fix::from_num(3).wrapping_sub_unsigned(UFix::from_num(5)), -2);
                /// assert_eq!(Fix::ZERO.wrapping_sub_unsigned(UFix::MAX), Fix::DELTA);
                /// ```
                #[inline]
                #[must_use]
                pub const fn wrapping_sub_unsigned(self, rhs: $UFixed<Frac>) -> $Fixed<Frac> {
                    let (ans, _) = self.overflowing_sub_unsigned(rhs);
                    ans
                }
            }

            if_unsigned! {
                $Signedness;
                /// Wrapping addition with a signed fixed-point number.
                /// Returns the sum, wrapping on overflow.
                ///
                /// # Examples
                ///
                /// ```rust
                #[doc = concat!("use fixed::{types::extra::U4, ", $s_ifixed, ", ", $s_fixed, "};")]
                #[doc = concat!("type Fix = ", $s_fixed, "<U4>;")]
                #[doc = concat!("type IFix = ", $s_ifixed, "<U4>;")]
                /// assert_eq!(Fix::from_num(5).wrapping_add_signed(IFix::from_num(-3)), 2);
                /// assert_eq!(Fix::ZERO.wrapping_add_signed(-IFix::DELTA), Fix::MAX);
                /// ```
                #[inline]
                #[must_use]
                pub const fn wrapping_add_signed(self, rhs: $IFixed<Frac>) -> $Fixed<Frac> {
                    let (ans, _) = self.overflowing_add_signed(rhs);
                    ans
                }

                /// Wrapping subtraction with a signed fixed-point number.
                /// Returns the difference, wrapping on overflow.
                ///
                /// # Examples
                ///
                /// ```rust
                #[doc = concat!("use fixed::{types::extra::U4, ", $s_ifixed, ", ", $s_fixed, "};")]
                #[doc = concat!("type Fix = ", $s_fixed, "<U4>;")]
                #[doc = concat!("type IFix = ", $s_ifixed, "<U4>;")]
                /// assert_eq!(Fix::from_num(5).wrapping_sub_signed(IFix::from_num(-3)), 8);
                /// assert_eq!(Fix::ZERO.wrapping_sub_signed(IFix::DELTA), Fix::MAX);
                /// ```
                #[inline]
                #[must_use]
                pub const fn wrapping_sub_signed(self, rhs: $IFixed<Frac>) -> $Fixed<Frac> {
                    let (ans, _) = self.overflowing_sub_signed(rhs);
                    ans
                }
            }

            comment! {
                "Unwrapped negation. Returns the negated value, panicking on overflow.

",
                if_signed_unsigned!(
                    $Signedness,
                    "Overflow can only occur when negating the minimum value.",
                    "Only zero can be negated without overflow.",
                ),
                "

# Panics

Panics if the result does not fit.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
",
                if_signed_unsigned!(
                    $Signedness,
                    concat!(
                        "assert_eq!(Fix::from_num(5).unwrapped_neg(), Fix::from_num(-5));
```

The following panics because of overflow.

```should_panic
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let _overflow = Fix::MIN.unwrapped_neg();",
                    ),
                    concat!(
                        "assert_eq!(Fix::ZERO.unwrapped_neg(), Fix::ZERO);
```

The following panics because of overflow.

```should_panic
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let _overflow = Fix::from_num(5).unwrapped_neg();",
                    ),
                ),
                "
```
";
                #[inline]
                #[track_caller]
                #[must_use]
                pub const fn unwrapped_neg(self) -> $Fixed<Frac> {
                    match self.checked_neg() {
                        Some(s) => s,
                        None => panic!("overflow"),
                    }
                }
            }

            comment! {
                "Unwrapped addition. Returns the sum, panicking on overflow.

# Panics

Panics if the result does not fit.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::from_num(3).unwrapped_add(Fix::from_num(2)), Fix::from_num(5));
```

The following panics because of overflow.

```should_panic
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let _overflow = Fix::MAX.unwrapped_add(Fix::DELTA);
```
";
                #[inline]
                #[track_caller]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn unwrapped_add(self, rhs: $Fixed<Frac>) -> $Fixed<Frac> {
                    match self.checked_add(rhs) {
                        Some(s) => s,
                        None => panic!("overflow"),
                    }
                }
            }

            comment! {
                "Unwrapped subtraction. Returns the difference, panicking on overflow.

# Panics

Panics if the result does not fit.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
",
                if_signed_unsigned!(
                    $Signedness,
                    "assert_eq!(Fix::from_num(3).unwrapped_sub(Fix::from_num(5)), Fix::from_num(-2));
",
                    "assert_eq!(Fix::from_num(5).unwrapped_sub(Fix::from_num(3)), Fix::from_num(2));
",
                ),
                "```

The following panics because of overflow.

```should_panic
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let _overflow = Fix::MIN.unwrapped_sub(Fix::DELTA);
```
";
                #[inline]
                #[track_caller]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn unwrapped_sub(self, rhs: $Fixed<Frac>) -> $Fixed<Frac> {
                    match self.checked_sub(rhs) {
                        Some(s) => s,
                        None => panic!("overflow"),
                    }
                }
            }

            comment! {
                "Unwrapped remainder. Returns the remainder, panicking if the divisor is zero.

# Panics

Panics if the divisor is zero.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::from_num(1.5).unwrapped_rem(Fix::ONE), Fix::from_num(0.5));
```

The following panics because the divisor is zero.

```should_panic
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let _divisor_is_zero = Fix::from_num(1.5).unwrapped_rem(Fix::ZERO);
```
";
                #[inline]
                #[track_caller]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn unwrapped_rem(self, rhs: $Fixed<Frac>) -> $Fixed<Frac> {
                    let rhs_bits = rhs.to_bits();
                    if_signed! {
                        $Signedness;
                        if rhs_bits == -1 {
                            return Self::ZERO;
                        }
                    }
                    Self::from_bits(self.to_bits() % rhs_bits)
                }
            }

            comment! {
                "Unwrapped multiply and add.
Returns `self` × `mul` + `add`, panicking on overflow.

",
                if_signed_else_empty_str! {
                    $Signedness;
                    "For some cases, the product `self` × `mul` would overflow
on its own, but the final result `self` × `mul` + `add` is representable; in
these cases this method returns the correct result without overflow.

",
                },
                "The `mul` parameter can have a fixed-point type like
`self` but with a different number of fractional bits.

# Panics

Panics if the result does not fit.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(
    Fix::from_num(4).unwrapped_mul_add(Fix::from_num(0.5), Fix::from_num(3)),
    Fix::from_num(5)
);
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "// MAX × 1.5 - MAX = MAX / 2, which does not overflow
assert_eq!(Fix::MAX.unwrapped_mul_add(Fix::from_num(1.5), -Fix::MAX), Fix::MAX / 2);
"
                },
                "```

The following panics because of overflow.

```should_panic
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let _overflow = Fix::MAX.unwrapped_mul_add(Fix::ONE, Fix::DELTA);
```
";
                #[inline]
                #[track_caller]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn unwrapped_mul_add<MulFrac: $LeEqU>(
                    self,
                    mul: $Fixed<MulFrac>,
                    add: $Fixed<Frac>,
                ) -> $Fixed<Frac> {
                    match self.checked_mul_add(mul, add) {
                        Some(s) => s,
                        None => panic!("overflow"),
                    }
                }
            }

            comment! {
                "Unwrapped multiplication by an integer. Returns the product, panicking on overflow.

# Panics

Panics if the result does not fit.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::from_num(3).unwrapped_mul_int(2), Fix::from_num(6));
```

The following panics because of overflow.

```should_panic
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let _overflow = Fix::MAX.unwrapped_mul_int(4);
```
";
                #[inline]
                #[track_caller]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn unwrapped_mul_int(self, rhs: $Inner) -> $Fixed<Frac> {
                    match self.checked_mul_int(rhs) {
                        Some(s) => s,
                        None => panic!("overflow"),
                    }
                }
            }

            comment! {
                "Unwrapped division by an integer. Returns the quotient",
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
// 1.5 is binary 1.1
let one_point_5 = Fix::from_bits(0b11 << (4 - 1));
assert_eq!(Fix::from_num(3).unwrapped_div_int(2), one_point_5);
```

The following panics because the divisor is zero.

```should_panic
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let _divisor_is_zero = Fix::from_num(3).unwrapped_div_int(0);
```
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "
The following panics because of overflow.

```should_panic
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let _overflow = Fix::MIN.unwrapped_div_int(-1);
```
",
                };
                #[inline]
                #[track_caller]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn unwrapped_div_int(self, rhs: $Inner) -> $Fixed<Frac> {
                    Self::from_bits(self.to_bits() / rhs)
                }
            }


            comment! {
                "Unwrapped remainder for Euclidean division. Returns the
remainder, panicking if the divisor is zero.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let num = Fix::from_num(7.5);
assert_eq!(num.unwrapped_rem_euclid(Fix::from_num(2)), Fix::from_num(1.5));
```

The following panics because the divisor is zero.

```should_panic
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let _divisor_is_zero = Fix::from_num(3).unwrapped_rem_euclid(Fix::ZERO);
```
";
                #[inline]
                #[track_caller]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn unwrapped_rem_euclid(self, rhs: $Fixed<Frac>) -> $Fixed<Frac> {
                    self.rem_euclid(rhs)
                }
            }

            comment! {
                "Unwrapped shift left. Panics if `rhs`&nbsp;≥&nbsp;", $s_nbits, ".

# Panics

Panics if `rhs`&nbsp;≥&nbsp;", $s_nbits, ".

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!((Fix::ONE / 2).unwrapped_shl(3), Fix::from_num(4));
```

The following panics because of overflow.

```should_panic
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let _overflow = Fix::ONE.unwrapped_shl(", $s_nbits, ");
```
";
                #[inline]
                #[track_caller]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn unwrapped_shl(self, rhs: u32) -> $Fixed<Frac> {
                    match self.checked_shl(rhs) {
                        Some(s) => s,
                        None => panic!("overflow"),
                    }
                }
            }

            comment! {
                "Unwrapped shift right. Panics if `rhs`&nbsp;≥&nbsp;", $s_nbits, ".

# Panics

Panics if `rhs`&nbsp;≥&nbsp;", $s_nbits, ".

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!((Fix::from_num(4)).unwrapped_shr(3), Fix::ONE / 2);
```

The following panics because of overflow.

```should_panic
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let _overflow = Fix::ONE.unwrapped_shr(", $s_nbits, ");
```
";
                #[inline]
                #[track_caller]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn unwrapped_shr(self, rhs: u32) -> $Fixed<Frac> {
                    match self.checked_shr(rhs) {
                        Some(s) => s,
                        None => panic!("overflow"),
                    }
                }
            }

            if_signed! {
                $Signedness;
                comment! {
                    "Unwrapped absolute value. Returns the absolute value, panicking on overflow.

Overflow can only occur when trying to find the absolute value of the minimum value.

# Panics

Panics if the result does not fit.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::from_num(-5).unwrapped_abs(), Fix::from_num(5));
```

The following panics because of overflow.

```should_panic
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let _overflow = Fix::MIN.unwrapped_abs();
```
";
                    #[inline]
                    #[track_caller]
                    #[must_use]
                    pub const fn unwrapped_abs(self) -> $Fixed<Frac> {
                        match self.checked_abs() {
                            Some(s) => s,
                            None => panic!("overflow"),
                        }
                    }
                }
            }

            comment! {
                "Unwrapped distance. Returns the distance from `self` to `other`",
                if_signed_else_empty_str! { $Signedness; ", panicking on overflow" },
                ".

The distance is the absolute value of the difference.

",
                if_signed_unsigned!(
                    $Signedness,
                    "# Panics

Panics if the result does not fit.",
                    "Can never overflow for unsigned types.",
                ),
                "

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::ONE.unwrapped_dist(Fix::from_num(5)), Fix::from_num(4));
",
                if_unsigned_else_empty_str! {
                    $Signedness;
                    "assert_eq!(Fix::ZERO.unwrapped_dist(Fix::MAX), Fix::MAX);
"
                },
                "```
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "
The following panics because of overflow.

```should_panic
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let _overflow = Fix::MIN.unwrapped_dist(Fix::ZERO);
```
"
                };
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn unwrapped_dist(self, other: $Fixed<Frac>) -> $Fixed<Frac> {
                    if_signed_unsigned!(
                        $Signedness,
                        match self.checked_dist(other) {
                            Some(s) => s,
                            None => panic!("overflow"),
                        },
                        self.dist(other),
                    )
                }
            }

            comment! {
                "Returns the next multiple of `other`, panicking on overflow.

The next multiple is the smallest multiple of `other` that is ≥&nbsp;`self`",
                if_signed_else_empty_str! {
                    $Signedness;
                    " if `other` is positive, and the largest multiple of
`other` that is ≤&nbsp;`self` if `other` is negative",
                },
                ".

# Panics

Panics if `other` is zero or on overflow.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(
    Fix::from_num(4).unwrapped_next_multiple_of(Fix::from_num(1.5)),
    Fix::from_num(4.5)
);
```

The following panics because of overflow.

```rust,should_panic
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let _overflow = Fix::MAX.unwrapped_next_multiple_of(Fix::from_num(2));
```
";
                #[inline]
                #[track_caller]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn unwrapped_next_multiple_of(
                    self,
                    other: $Fixed<Frac>
                ) -> $Fixed<Frac> {
                    let (ans, overflow) = self.overflowing_next_multiple_of(other);
                    assert!(!overflow, "overflow");
                    ans
                }
            }

            comment! {
                "Inverse linear interpolation between `start` and `end`,
panicking on overflow.

The computed value can have a fixed-point type like `self` but with a different
number of fractional bits.

Returns
(`self`&nbsp;&minus;&nbsp;`start`)&nbsp;/&nbsp;(`end`&nbsp;&minus;&nbsp;`start`).
This is 0 when `self`&nbsp;=&nbsp;`start`, and 1 when `self`&nbsp;=&nbsp;`end`.

# Panics

Panics when `start`&nbsp;=&nbsp;`end` or when the results overflows.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let two = Fix::from_num(2);
let four = Fix::from_num(4);
assert_eq!(Fix::from_num(3).unwrapped_inv_lerp::<U4>(two, four), 0.5);
```

The following panics because `start`&nbsp;=&nbsp;`end`.

```should_panic
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let two = Fix::from_num(2);
let _zero_range = two.unwrapped_inv_lerp::<U4>(two, two);
```

The following panics because of overflow.

```should_panic
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let _overflow = Fix::MAX.unwrapped_inv_lerp::<U4>(Fix::ZERO, Fix::from_num(0.5));
```
";
                #[inline]
                pub const fn unwrapped_inv_lerp<RetFrac: $LeEqU>(
                    self,
                    start: $Fixed<Frac>,
                    end: $Fixed<Frac>,
                ) -> $Fixed<RetFrac> {
                    let (bits, overflow) = inv_lerp::$Inner(
                        self.to_bits(),
                        start.to_bits(),
                        end.to_bits(),
                        RetFrac::U32,
                    );
                    assert!(!overflow, "overflow");
                    $Fixed::from_bits(bits)
                }
            }

            if_unsigned! {
                $Signedness;
                comment! {
                    "Returns the smallest power of two that is ≥&nbsp;`self`,
panicking if the next power of two is too large to represent.

# Panics

Panics if the result does not fit.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
// 3/8 is 0.0110
let three_eights = Fix::from_bits(0b0110);
// 1/2 is 0.1000
let half = Fix::from_bits(0b1000);
assert_eq!(three_eights.unwrapped_next_power_of_two(), half);
```

The following panics because of overflow.

```should_panic
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let _overflow = Fix::MAX.unwrapped_next_power_of_two();
```
";
                    #[inline]
                    #[track_caller]
                    #[must_use]
                    pub const fn unwrapped_next_power_of_two(self) -> $Fixed<Frac> {
                        match self.checked_next_power_of_two() {
                            Some(s) => s,
                            None => panic!("overflow"),
                        }
                    }
                }
            }

            if_signed! {
                $Signedness;
                /// Unwrapped addition with an unsigned fixed-point number.
                /// Returns the sum, panicking on overflow.
                ///
                /// # Panics
                ///
                /// Panics if the result does not fit.
                ///
                /// # Examples
                ///
                /// ```rust
                #[doc = concat!("use fixed::{types::extra::U4, ", $s_fixed, ", ", $s_ufixed, "};")]
                #[doc = concat!("type Fix = ", $s_fixed, "<U4>;")]
                #[doc = concat!("type UFix = ", $s_ufixed, "<U4>;")]
                /// assert_eq!(Fix::from_num(-5).unwrapped_add_unsigned(UFix::from_num(3)), -2);
                /// ```
                ///
                /// The following panics because of overflow.
                ///
                /// ```rust,should_panic
                #[doc = concat!("use fixed::{types::extra::U4, ", $s_fixed, ", ", $s_ufixed, "};")]
                #[doc = concat!("type Fix = ", $s_fixed, "<U4>;")]
                #[doc = concat!("type UFix = ", $s_ufixed, "<U4>;")]
                /// let _overflow = Fix::MAX.unwrapped_add_unsigned(UFix::DELTA);
                /// ```
                #[inline]
                #[must_use]
                pub const fn unwrapped_add_unsigned(self, rhs: $UFixed<Frac>) -> $Fixed<Frac> {
                    let (ans, overflow) = self.overflowing_add_unsigned(rhs);
                    assert!(!overflow, "overflow");
                    ans
                }

                /// Unwrapped subtraction with an unsigned fixed-point number.
                /// Returns the difference, panicking on overflow.
                ///
                /// # Panics
                ///
                /// Panics if the result does not fit.
                ///
                /// # Examples
                ///
                /// ```rust
                #[doc = concat!("use fixed::{types::extra::U4, ", $s_fixed, ", ", $s_ufixed, "};")]
                #[doc = concat!("type Fix = ", $s_fixed, "<U4>;")]
                #[doc = concat!("type UFix = ", $s_ufixed, "<U4>;")]
                /// assert_eq!(Fix::from_num(3).unwrapped_sub_unsigned(UFix::from_num(5)), -2);
                /// ```
                ///
                /// The following panics because of overflow.
                ///
                /// ```rust,should_panic
                #[doc = concat!("use fixed::{types::extra::U4, ", $s_fixed, ", ", $s_ufixed, "};")]
                #[doc = concat!("type Fix = ", $s_fixed, "<U4>;")]
                #[doc = concat!("type UFix = ", $s_ufixed, "<U4>;")]
                /// let _overflow = Fix::MIN.unwrapped_sub_unsigned(UFix::DELTA);
                /// ```
                #[inline]
                #[must_use]
                pub const fn unwrapped_sub_unsigned(self, rhs: $UFixed<Frac>) -> $Fixed<Frac> {
                    let (ans, overflow) = self.overflowing_sub_unsigned(rhs);
                    assert!(!overflow, "overflow");
                    ans
                }
            }

            if_unsigned! {
                $Signedness;
                /// Unwrapped addition with a signed fixed-point number.
                /// Returns the sum, panicking on overflow.
                ///
                /// # Panics
                ///
                /// Panics if the result does not fit.
                ///
                /// # Examples
                ///
                /// ```rust
                #[doc = concat!("use fixed::{types::extra::U4, ", $s_ifixed, ", ", $s_fixed, "};")]
                #[doc = concat!("type Fix = ", $s_fixed, "<U4>;")]
                #[doc = concat!("type IFix = ", $s_ifixed, "<U4>;")]
                /// assert_eq!(Fix::from_num(5).unwrapped_add_signed(IFix::from_num(-3)), 2);
                /// ```
                ///
                /// The following panics because of overflow.
                ///
                /// ```rust,should_panic
                #[doc = concat!("use fixed::{types::extra::U4, ", $s_ifixed, ", ", $s_fixed, "};")]
                #[doc = concat!("type Fix = ", $s_fixed, "<U4>;")]
                #[doc = concat!("type IFix = ", $s_ifixed, "<U4>;")]
                /// let _overflow = Fix::from_num(2).unwrapped_add_signed(IFix::from_num(-3));
                /// ```
                #[inline]
                #[must_use]
                pub const fn unwrapped_add_signed(self, rhs: $IFixed<Frac>) -> $Fixed<Frac> {
                    let (ans, overflow) = self.overflowing_add_signed(rhs);
                    assert!(!overflow, "overflow");
                    ans
                }

                /// Unwrapped subtraction with a signed fixed-point number.
                /// Returns the difference, panicking on overflow.
                ///
                /// # Panics
                ///
                /// Panics if the result does not fit.
                ///
                /// # Examples
                ///
                /// ```rust
                #[doc = concat!("use fixed::{types::extra::U4, ", $s_ifixed, ", ", $s_fixed, "};")]
                #[doc = concat!("type Fix = ", $s_fixed, "<U4>;")]
                #[doc = concat!("type IFix = ", $s_ifixed, "<U4>;")]
                /// assert_eq!(Fix::from_num(5).unwrapped_sub_signed(IFix::from_num(-3)), 8);
                /// ```
                ///
                /// The following panics because of overflow.
                ///
                /// ```rust,should_panic
                #[doc = concat!("use fixed::{types::extra::U4, ", $s_ifixed, ", ", $s_fixed, "};")]
                #[doc = concat!("type Fix = ", $s_fixed, "<U4>;")]
                #[doc = concat!("type IFix = ", $s_ifixed, "<U4>;")]
                /// let _overflow = Fix::from_num(2).unwrapped_sub_signed(IFix::from_num(3));
                /// ```
                #[inline]
                #[must_use]
                pub const fn unwrapped_sub_signed(self, rhs: $IFixed<Frac>) -> $Fixed<Frac> {
                    let (ans, overflow) = self.overflowing_sub_signed(rhs);
                    assert!(!overflow, "overflow");
                    ans
                }
            }

            comment! {
                "Overflowing negation.

Returns a [tuple] of the negated value and a [`bool`] indicating whether
an overflow has occurred. On overflow, the wrapped value is returned.

",
                if_signed_unsigned!(
                    $Signedness,
                    "Overflow can only occur when negating the minimum value.",
                    "Only zero can be negated without overflow.",
                ),
                "

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
",
                if_signed_unsigned!(
                    $Signedness,
                    "assert_eq!(Fix::from_num(5).overflowing_neg(), (Fix::from_num(-5), false));
assert_eq!(Fix::MIN.overflowing_neg(), (Fix::MIN, true));",
                    "assert_eq!(Fix::ZERO.overflowing_neg(), (Fix::ZERO, false));
assert_eq!(Fix::from_num(5).overflowing_neg(), Fix::overflowing_from_num(-5));
let neg_five_bits = !Fix::from_num(5).to_bits() + 1;
assert_eq!(Fix::from_num(5).overflowing_neg(), (Fix::from_bits(neg_five_bits), true));",
                ),
                "
```
";
                #[inline]
                pub const fn overflowing_neg(self) -> ($Fixed<Frac>, bool) {
                    let (ans, o) = self.to_bits().overflowing_neg();
                    (Self::from_bits(ans), o)
                }
            }

            comment! {
                "Overflowing addition.

Returns a [tuple] of the sum and a [`bool`] indicating whether an
overflow has occurred. On overflow, the wrapped value is returned.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let one_minus_delta = Fix::ONE - Fix::DELTA;
assert_eq!(Fix::from_num(3).overflowing_add(Fix::from_num(2)), (Fix::from_num(5), false));
assert_eq!(Fix::MAX.overflowing_add(Fix::ONE), (",
                if_signed_else_empty_str! { $Signedness; "Fix::MIN + " },
                "one_minus_delta, true));
```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn overflowing_add(self, rhs: $Fixed<Frac>) -> ($Fixed<Frac>, bool) {
                    let (ans, o) = self.to_bits().overflowing_add(rhs.to_bits());
                    (Self::from_bits(ans), o)
                }
            }

            comment! {
                "Overflowing subtraction.

Returns a [tuple] of the difference and a [`bool`] indicating whether an
overflow has occurred. On overflow, the wrapped value is returned.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let one_minus_delta = Fix::ONE - Fix::DELTA;
",
                if_signed_unsigned!(
                    $Signedness,
                    "assert_eq!(Fix::from_num(3).overflowing_sub(Fix::from_num(5)), (Fix::from_num(-2), false));
assert_eq!(Fix::MIN",
                    "assert_eq!(Fix::from_num(5).overflowing_sub(Fix::from_num(3)), (Fix::from_num(2), false));
assert_eq!(Fix::ZERO",
                ),
                ".overflowing_sub(Fix::ONE), (Fix::MAX - one_minus_delta, true));
```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn overflowing_sub(self, rhs: $Fixed<Frac>) -> ($Fixed<Frac>, bool) {
                    let (ans, o) = self.to_bits().overflowing_sub(rhs.to_bits());
                    (Self::from_bits(ans), o)
                }
            }

            comment! {
                "Overflowing multiply and add.

Returns a [tuple] of `self` × `mul` + `add` and a [`bool`] indicating
whether an overflow has occurred. On overflow, the wrapped value is
returned.

",
                if_signed_else_empty_str! {
                    $Signedness;
                    "For some cases, the product `self` × `mul` would overflow
on its own, but the final result `self` × `mul` + `add` is representable; in
these cases this method returns the correct result without overflow.

",
                },
                "The `mul` parameter can have a fixed-point type like
`self` but with a different number of fractional bits.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(
    Fix::MAX.overflowing_mul_add(Fix::ONE, Fix::ZERO),
    (Fix::MAX, false)
);
assert_eq!(
    Fix::MAX.overflowing_mul_add(Fix::ONE, Fix::DELTA),
    (Fix::MIN, true)
);
assert_eq!(
    Fix::MAX.overflowing_mul_add(Fix::from_num(3), Fix::MAX),
    Fix::MAX.overflowing_mul_int(4)
);
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "// MAX × 1.5 - MAX = MAX / 2, which does not overflow
assert_eq!(
    Fix::MAX.overflowing_mul_add(Fix::from_num(1.5), -Fix::MAX),
    (Fix::MAX / 2, false)
);
"
                },
                "```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn overflowing_mul_add<MulFrac: $LeEqU>(
                    self,
                    mul: $Fixed<MulFrac>,
                    add: $Fixed<Frac>,
                ) -> ($Fixed<Frac>, bool) {
                    let (ans, overflow) = arith::$Inner::overflowing_mul_add(
                        self.to_bits(),
                        mul.to_bits(),
                        add.to_bits(),
                        MulFrac::I32,
                    );
                    (Self::from_bits(ans), overflow)
                }
            }

            comment! {
                "Overflowing multiplication by an integer.

Returns a [tuple] of the product and a [`bool`] indicating whether an
overflow has occurred. On overflow, the wrapped value is returned.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::from_num(3).overflowing_mul_int(2), (Fix::from_num(6), false));
let wrapped = Fix::from_bits(!0 << 2);
assert_eq!(Fix::MAX.overflowing_mul_int(4), (wrapped, true));
```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn overflowing_mul_int(self, rhs: $Inner) -> ($Fixed<Frac>, bool) {
                    let (ans, o) = self.to_bits().overflowing_mul(rhs);
                    (Self::from_bits(ans), o)
                }
            }

            comment! {
                "Overflowing division by an integer.

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
// 1.5 is binary 1.1
let one_point_5 = Fix::from_bits(0b11 << (4 - 1));
assert_eq!(Fix::from_num(3).overflowing_div_int(2), (one_point_5, false));
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "assert_eq!(Fix::MIN.overflowing_div_int(-1), (Fix::MIN, true));
",
                },
                "```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn overflowing_div_int(self, rhs: $Inner) -> ($Fixed<Frac>, bool) {
                    let (ans, o) = self.to_bits().overflowing_div(rhs);
                    (Self::from_bits(ans), o)
                }
            }

            comment! {
                "Overflowing shift left.

Returns a [tuple] of the shifted value and a [`bool`] indicating whether
an overflow has occurred. Overflow occurs when `rhs`&nbsp;≥&nbsp;", $s_nbits, ".
On overflow `rhs` is wrapped before the shift operation.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!((Fix::ONE / 2).overflowing_shl(3), (Fix::from_num(4), false));
assert_eq!((Fix::ONE / 2).overflowing_shl(3 + ", $s_nbits, "), (Fix::from_num(4), true));
```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn overflowing_shl(self, rhs: u32) -> ($Fixed<Frac>, bool) {
                    let (ans, o) = self.to_bits().overflowing_shl(rhs);
                    (Self::from_bits(ans), o)
                }
            }

            comment! {
                "Overflowing shift right.

Returns a [tuple] of the shifted value and a [`bool`] indicating whether
an overflow has occurred. Overflow occurs when `rhs`&nbsp;≥&nbsp;", $s_nbits, ".
On overflow `rhs` is wrapped before the shift operation.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!((Fix::from_num(4)).overflowing_shr(3), (Fix::ONE / 2, false));
assert_eq!((Fix::from_num(4)).overflowing_shr(3 + ", $s_nbits, "), (Fix::ONE / 2, true));
```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn overflowing_shr(self, rhs: u32) -> ($Fixed<Frac>, bool) {
                    let (ans, o) = self.to_bits().overflowing_shr(rhs);
                    (Self::from_bits(ans), o)
                }
            }

            if_signed! {
                $Signedness;
                comment! {
                    "Overflowing absolute value.

Returns a [tuple] of the absolute value and a [`bool`] indicating
whether an overflow has occurred. On overflow, the wrapped value is
returned.

Overflow can only occur when trying to find the absolute value of the minimum value.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(Fix::from_num(-5).overflowing_abs(), (Fix::from_num(5), false));
assert_eq!(Fix::MIN.overflowing_abs(), (Fix::MIN, true));
```
";
                    #[inline]
                    pub const fn overflowing_abs(self) -> ($Fixed<Frac>, bool) {
                        let (ans, o) = self.to_bits().overflowing_abs();
                        (Self::from_bits(ans), o)
                    }
                }
            }

            comment! {
                "Overflowing distance.

Returns a [tuple] of the distance from `self` to `other` and ",
                if_signed_unsigned!(
                    $Signedness,
                    "a [`bool`] indicating whether an overflow has
occurred. On overflow, the wrapped value is returned.",
                    "[`false`], as overflow can never happen for unsigned types.",
                ),
                "

The distance is the absolute value of the difference.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(
    Fix::ONE.overflowing_dist(Fix::from_num(5)),
    (Fix::from_num(4), false)
);
",
                if_signed_unsigned!(
                    $Signedness,
                    "assert_eq!(
    Fix::MIN.overflowing_dist(Fix::MAX),
    (-Fix::DELTA, true)
);",
                    "assert_eq!(
    Fix::ZERO.overflowing_dist(Fix::MAX),
    (Fix::MAX, false)
);",
                ),
                "
```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn overflowing_dist(self, other: $Fixed<Frac>,) -> ($Fixed<Frac>, bool) {
                    if_signed! {
                        $Signedness;
                        if self.to_bits() < other.to_bits() {
                            other.overflowing_sub(self)
                        } else {
                            self.overflowing_sub(other)
                        }
                    }
                    if_unsigned! {
                        $Signedness;
                        (self.dist(other), false)
                    }
                }
            }

            comment! {
                "Overflowing next multiple of `other`.

Returns a [tuple] of the next multiple and a [`bool`] indicating whether an
overflow has occurred. On overflow, the wrapped value is returned.

The next multiple is the smallest multiple of `other` that is ≥&nbsp;`self`",
                if_signed_else_empty_str! {
                    $Signedness;
                    " if `other` is positive, and the largest multiple of
`other` that is ≤&nbsp;`self` if `other` is negative",
                },
                ".

# Panics

Panics if `other` is zero.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
assert_eq!(
    Fix::from_num(4).overflowing_next_multiple_of(Fix::from_num(1.5)),
    (Fix::from_num(4.5), false)
);
let max_minus_delta = Fix::MAX - Fix::DELTA;
assert_eq!(
    Fix::MAX.overflowing_next_multiple_of(max_minus_delta),
    (max_minus_delta.wrapping_mul_int(2), true)
);
```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn overflowing_next_multiple_of(
                    self,
                    other: $Fixed<Frac>
                ) -> ($Fixed<Frac>, bool) {
                    let slf = self.to_bits();
                    let other = other.to_bits();

                    if_signed! {
                        $Signedness;

                        // check for overflowing division
                        if other == -1 {
                            return (self, false);
                        }

                        // panics if other == 0
                        let rem = slf % other;

                        let m = if rem.is_negative() != other.is_negative() {
                            // cannot overflow as they have opposite signs
                            rem + other
                        } else {
                            rem
                        };
                        if m == 0 {
                            (self, false)
                        } else {
                            // other - m cannot overflow because they have the same sign
                            self.overflowing_add($Fixed::from_bits(other - m))
                        }
                    }

                    if_unsigned! {
                        $Signedness;

                        // panics if other == 0
                        let rem = slf % other;

                        if rem == 0 {
                            (self, false)
                        } else {
                            // other - rem cannot overflow because rem is smaller
                            self.overflowing_add($Fixed::from_bits(other - rem))
                        }
                    }
                }
            }

            comment! {
                "Overflowing inverse linear interpolation between `start` and `end`.

Returns a [tuple] of the result and a [`bool`] indicationg whether an overflow
has occurred. On overflow, the wrapped value is returned.

The computed value can have a fixed-point type like `self` but with a different
number of fractional bits.

Computes
(`self`&nbsp;&minus;&nbsp;`start`)&nbsp;/&nbsp;(`end`&nbsp;&minus;&nbsp;`start`).
This is 0 when `self`&nbsp;=&nbsp;`start`, and 1 when `self`&nbsp;=&nbsp;`end`.

# Panics

Panics when `start`&nbsp;=&nbsp;`end`.

# Examples

```rust
use fixed::{types::extra::U4, ", $s_fixed, "};
type Fix = ", $s_fixed, "<U4>;
let two = Fix::from_num(2);
let four = Fix::from_num(4);
assert_eq!(
    Fix::from_num(3).overflowing_inv_lerp::<U4>(two, four),
    (Fix::from_num(0.5), false)
);
assert_eq!(
    Fix::MAX.overflowing_inv_lerp::<U4>(Fix::ZERO, Fix::from_num(0.5)),
    (Fix::MAX.wrapping_mul_int(2), true)
);
```
";
                #[inline]
                pub const fn overflowing_inv_lerp<RetFrac: $LeEqU>(
                    self,
                    start: $Fixed<Frac>,
                    end: $Fixed<Frac>,
                ) -> ($Fixed<RetFrac>, bool) {
                    let (bits, overflow) = inv_lerp::$Inner(
                        self.to_bits(),
                        start.to_bits(),
                        end.to_bits(),
                        RetFrac::U32,
                    );
                    ($Fixed::from_bits(bits), overflow)
                }
            }

            if_signed! {
                $Signedness;
                /// Overflowing addition with an unsigned fixed-point number.
                ///
                /// Returns a [tuple] of the sum and a [`bool`] indicating
                /// whether an overflow has occurred. On overflow, the wrapped
                /// value is returned.
                ///
                /// # Examples
                ///
                /// ```rust
                #[doc = concat!("use fixed::{types::extra::U4, ", $s_fixed, ", ", $s_ufixed, "};")]
                #[doc = concat!("type Fix = ", $s_fixed, "<U4>;")]
                #[doc = concat!("type UFix = ", $s_ufixed, "<U4>;")]
                /// assert_eq!(
                ///     Fix::from_num(-5).overflowing_add_unsigned(UFix::from_num(3)),
                ///     (Fix::from_num(-2), false)
                /// );
                /// assert_eq!(
                ///     Fix::ZERO.overflowing_add_unsigned(UFix::MAX),
                ///     (-Fix::DELTA, true)
                /// );
                /// ```
                #[inline]
                #[must_use]
                pub const fn overflowing_add_unsigned(
                    self,
                    rhs: $UFixed<Frac>,
                ) -> ($Fixed<Frac>, bool) {
                    let signed_rhs = rhs.to_bits() as $Inner;
                    let overflow1 = signed_rhs < 0;
                    let (bits, overflow2) = self.to_bits().overflowing_add(signed_rhs);
                    // if both overflow1 and overflow2, then they cancel each other out
                    ($Fixed::from_bits(bits), overflow1 != overflow2)
                }

                /// Overflowing subtraction with an unsigned fixed-point number.
                ///
                /// Returns a [tuple] of the difference and a [`bool`]
                /// indicating whether an overflow has occurred. On overflow,
                /// the wrapped value is returned.
                ///
                /// # Examples
                ///
                /// ```rust
                #[doc = concat!("use fixed::{types::extra::U4, ", $s_fixed, ", ", $s_ufixed, "};")]
                #[doc = concat!("type Fix = ", $s_fixed, "<U4>;")]
                #[doc = concat!("type UFix = ", $s_ufixed, "<U4>;")]
                /// assert_eq!(
                ///     Fix::from_num(3).overflowing_sub_unsigned(UFix::from_num(5)),
                ///     (Fix::from_num(-2), false)
                /// );
                /// assert_eq!(
                ///     Fix::ZERO.overflowing_sub_unsigned(UFix::MAX),
                ///     (Fix::DELTA, true)
                /// );
                /// ```
                #[inline]
                #[must_use]
                pub const fn overflowing_sub_unsigned(
                    self,
                    rhs: $UFixed<Frac>,
                ) -> ($Fixed<Frac>, bool) {
                    let signed_rhs = rhs.to_bits() as $Inner;
                    let overflow1 = signed_rhs < 0;
                    let (bits, overflow2) = self.to_bits().overflowing_sub(signed_rhs);
                    // if both overflow1 and overflow2, then they cancel each other out
                    ($Fixed::from_bits(bits), overflow1 != overflow2)
                }
            }

            if_unsigned! {
                $Signedness;
                /// Overflowing addition with a signed fixed-point number.
                ///
                /// Returns a [tuple] of the sum and a [`bool`] indicating
                /// whether an overflow has occurred. On overflow, the wrapped
                /// value is returned.
                ///
                /// # Examples
                ///
                /// ```rust
                #[doc = concat!("use fixed::{types::extra::U4, ", $s_ifixed, ", ", $s_fixed, "};")]
                #[doc = concat!("type Fix = ", $s_fixed, "<U4>;")]
                #[doc = concat!("type IFix = ", $s_ifixed, "<U4>;")]
                /// assert_eq!(
                ///     Fix::from_num(5).overflowing_add_signed(IFix::from_num(-3)),
                ///     (Fix::from_num(2), false)
                /// );
                /// assert_eq!(
                ///     Fix::ZERO.overflowing_add_signed(-IFix::DELTA),
                ///     (Fix::MAX, true)
                /// );
                /// ```
                #[inline]
                #[must_use]
                pub const fn overflowing_add_signed(
                    self,
                    rhs: $IFixed<Frac>,
                ) -> ($Fixed<Frac>, bool) {
                    let unsigned_rhs = rhs.to_bits() as $Inner;
                    let overflow1 = rhs.is_negative();
                    let (bits, overflow2) = self.to_bits().overflowing_add(unsigned_rhs);
                    // if both overflow1 and overflow2, then they cancel each other out
                    ($Fixed::from_bits(bits), overflow1 != overflow2)
                }

                /// Overflowing subtraction with a signed fixed-point number.
                ///
                /// Returns a [tuple] of the difference and a [`bool`]
                /// indicating whether an overflow has occurred. On overflow,
                /// the wrapped value is returned.
                ///
                /// # Examples
                ///
                /// ```rust
                #[doc = concat!("use fixed::{types::extra::U4, ", $s_ifixed, ", ", $s_fixed, "};")]
                #[doc = concat!("type Fix = ", $s_fixed, "<U4>;")]
                #[doc = concat!("type IFix = ", $s_ifixed, "<U4>;")]
                /// assert_eq!(
                ///     Fix::from_num(5).overflowing_sub_signed(IFix::from_num(-3)),
                ///     (Fix::from_num(8), false)
                /// );
                /// assert_eq!(
                ///     Fix::ZERO.overflowing_sub_signed(IFix::DELTA),
                ///     (Fix::MAX, true)
                /// );
                /// ```
                #[inline]
                #[must_use]
                pub const fn overflowing_sub_signed(
                    self,
                    rhs: $IFixed<Frac>,
                ) -> ($Fixed<Frac>, bool) {
                    let unsigned_rhs = rhs.to_bits() as $Inner;
                    let overflow1 = rhs.is_negative();
                    let (bits, overflow2) = self.to_bits().overflowing_sub(unsigned_rhs);
                    // if both overflow1 and overflow2, then they cancel each other out
                    ($Fixed::from_bits(bits), overflow1 != overflow2)
                }
            }
        }
    };
}
