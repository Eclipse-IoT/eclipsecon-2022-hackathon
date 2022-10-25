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

use core::{
    cmp::Ordering,
    hash::{Hash, Hasher},
    num::FpCategory,
    ops::Neg,
};

const SIGN_MASK: u128 = 1u128 << 127;
const EXP_MASK: u128 = 0x7FFF_u128 << 112;
const MANT_MASK: u128 = (1u128 << 112) - 1;
const PREC: u32 = 113;
const EXP_BITS: u32 = 15;
const EXP_BIAS: u32 = (1 << (EXP_BITS - 1)) - 1;

/// The bit representation of a *binary128* floating-point number (`f128`).
///
/// This type can be used to
///
///   * convert between fixed-point numbers and the bit representation of
///     128-bit floating-point numbers.
///   * compare fixed-point numbers and the bit representation of 128-bit
///     floating-point numbers.
///
/// # Examples
///
/// ```rust
/// use fixed::{types::I16F16, F128};
/// assert_eq!(I16F16::ONE.to_num::<F128>(), F128::ONE);
/// assert_eq!(I16F16::from_num(F128::ONE), I16F16::ONE);
///
/// // fixed-point numbers can be compared directly to F128 values
/// assert!(I16F16::from_num(1.5) > F128::ONE);
/// assert!(I16F16::from_num(0.5) < F128::ONE);
/// ```
#[derive(Clone, Copy, Default, Debug)]
pub struct F128 {
    bits: u128,
}

impl F128 {
    /// Zero.
    pub const ZERO: F128 = F128::from_bits(0);
    /// Negative zero (&minus;0).
    pub const NEG_ZERO: F128 = F128::from_bits(SIGN_MASK);
    /// One.
    pub const ONE: F128 = F128::from_bits((EXP_BIAS as u128) << (PREC - 1));
    /// Negative one (&minus;1).
    pub const NEG_ONE: F128 = F128::from_bits(SIGN_MASK | F128::ONE.to_bits());
    /// Smallest positive subnormal number, 2<sup>&minus;16494</sup>.
    pub const MIN_POSITIVE_SUB: F128 = F128::from_bits(1);
    /// Smallest positive normal number, 2<sup>&minus;16382</sup>.
    pub const MIN_POSITIVE: F128 = F128::from_bits(MANT_MASK + 1);
    /// Largest finite number,
    /// 2<sup>16384</sup>&nbsp;&minus;&nbsp;2<sup>16271</sup>.
    pub const MAX: F128 = F128::from_bits(EXP_MASK - 1);
    /// Smallest finite number; equal to &minus;[`MAX`][Self::MAX].
    pub const MIN: F128 = F128::from_bits(SIGN_MASK | F128::MAX.to_bits());
    /// Infinity (∞).
    pub const INFINITY: F128 = F128::from_bits(EXP_MASK);
    /// Negative infinity (&minus;∞).
    pub const NEG_INFINITY: F128 = F128::from_bits(SIGN_MASK | EXP_MASK);
    /// NaN.
    pub const NAN: F128 = F128::from_bits(EXP_MASK | (1u128 << (PREC - 2)));

    /// The radix or base of the internal representation.
    pub const RADIX: u32 = 2;
    /// Number of significant digits in base 2.
    pub const MANTISSA_DIGITS: u32 = PREC;

    /// The difference between 1 and the next larger representable number,
    /// 2<sup>&minus;112</sup>.
    pub const EPSILON: F128 = F128::from_bits(((EXP_BIAS - (PREC - 1)) as u128) << (PREC - 1));
    /// If <i>x</i>&nbsp;=&nbsp;`MIN_EXP`, then normal numbers
    /// ≥&nbsp;0.5&nbsp;×&nbsp;2<sup><i>x</i></sup>.
    pub const MIN_EXP: i32 = 3 - F128::MAX_EXP;
    /// If <i>x</i>&nbsp;=&nbsp;`MAX_EXP`, then normal numbers
    /// <&nbsp;1&nbsp;×&nbsp;2<sup><i>x</i></sup>.
    pub const MAX_EXP: i32 = EXP_BIAS as i32 + 1;

    /// Raw transmutation from [`u128`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::F128;
    /// let infinity_bits = 0x7FFF_u128 << 112;
    /// assert!(F128::from_bits(infinity_bits - 1).is_finite());
    /// assert!(!F128::from_bits(infinity_bits).is_finite());
    /// ```
    #[inline]
    pub const fn from_bits(bits: u128) -> F128 {
        F128 { bits }
    }

    /// Raw transmutation to [`u128`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::F128;
    /// assert_eq!(F128::ONE.to_bits(), 0x3FFF_u128 << 112);
    /// assert_ne!(F128::ONE.to_bits(), 1u128);
    /// ```
    #[inline]
    pub const fn to_bits(self) -> u128 {
        self.bits
    }

    /// Creates a number from a byte array in big-endian byte order.
    #[inline]
    pub const fn from_be_bytes(bytes: [u8; 16]) -> F128 {
        F128::from_bits(u128::from_be_bytes(bytes))
    }

    /// Creates a number from a byte array in little-endian byte order.
    #[inline]
    pub const fn from_le_bytes(bytes: [u8; 16]) -> F128 {
        F128::from_bits(u128::from_le_bytes(bytes))
    }

    /// Creates a number from a byte array in native-endian byte order.
    #[inline]
    pub const fn from_ne_bytes(bytes: [u8; 16]) -> F128 {
        F128::from_bits(u128::from_ne_bytes(bytes))
    }

    /// Returns the memory representation of the number as a byte array in
    /// big-endian byte order.
    #[inline]
    pub const fn to_be_bytes(self) -> [u8; 16] {
        self.to_bits().to_be_bytes()
    }

    /// Returns the memory representation of the number as a byte array in
    /// little-endian byte order.
    #[inline]
    pub const fn to_le_bytes(self) -> [u8; 16] {
        self.to_bits().to_le_bytes()
    }

    /// Returns the memory representation of the number as a byte array in
    /// native-endian byte order.
    #[inline]
    pub const fn to_ne_bytes(self) -> [u8; 16] {
        self.to_bits().to_ne_bytes()
    }

    /// Returns [`true`] if the number is NaN.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fixed::F128;
    ///
    /// assert!(F128::NAN.is_nan());
    ///
    /// assert!(!F128::ONE.is_nan());
    /// assert!(!F128::INFINITY.is_nan());
    /// assert!(!F128::NEG_INFINITY.is_nan());
    /// ```
    #[inline]
    pub const fn is_nan(self) -> bool {
        (self.to_bits() & !SIGN_MASK) > EXP_MASK
    }

    /// Returns [`true`] if the number is infinite.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fixed::F128;
    ///
    /// assert!(F128::INFINITY.is_infinite());
    /// assert!(F128::NEG_INFINITY.is_infinite());
    ///
    /// assert!(!F128::ONE.is_infinite());
    /// assert!(!F128::NAN.is_infinite());
    /// ```
    #[inline]
    pub const fn is_infinite(self) -> bool {
        (self.to_bits() & !SIGN_MASK) == EXP_MASK
    }

    /// Returns [`true`] if the number is neither infinite nor NaN.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fixed::F128;
    ///
    /// assert!(F128::ONE.is_finite());
    /// assert!(F128::MAX.is_finite());
    ///
    /// assert!(!F128::INFINITY.is_finite());
    /// assert!(!F128::NEG_INFINITY.is_finite());
    /// assert!(!F128::NAN.is_finite());
    /// ```
    #[inline]
    pub const fn is_finite(self) -> bool {
        (self.to_bits() & EXP_MASK) != EXP_MASK
    }

    /// Returns [`true`] if the number is zero.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fixed::F128;
    ///
    /// assert!(F128::ZERO.is_zero());
    /// assert!(F128::NEG_ZERO.is_zero());
    ///
    /// assert!(!F128::MIN_POSITIVE_SUB.is_zero());
    /// assert!(!F128::NAN.is_zero());
    /// ```
    #[inline]
    pub const fn is_zero(self) -> bool {
        (self.to_bits() & !SIGN_MASK) == 0
    }

    /// Returns [`true`] if the number is subnormal.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fixed::F128;
    ///
    /// assert!(F128::MIN_POSITIVE_SUB.is_subnormal());
    ///
    /// assert!(!F128::ZERO.is_subnormal());
    /// assert!(!F128::MIN_POSITIVE.is_subnormal());
    /// ```
    #[inline]
    pub const fn is_subnormal(self) -> bool {
        let abs = self.to_bits() & !SIGN_MASK;
        0 < abs && abs < F128::MIN_POSITIVE.to_bits()
    }

    /// Returns [`true`] if the number is neither zero, infinite, subnormal, or NaN.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fixed::F128;
    ///
    /// assert!(F128::MIN.is_normal());
    /// assert!(F128::MIN_POSITIVE.is_normal());
    /// assert!(F128::MAX.is_normal());
    ///
    /// assert!(!F128::ZERO.is_normal());
    /// assert!(!F128::MIN_POSITIVE_SUB.is_normal());
    /// assert!(!F128::INFINITY.is_normal());
    /// assert!(!F128::NAN.is_normal());
    /// ```
    #[inline]
    pub const fn is_normal(self) -> bool {
        let abs = self.to_bits() & !SIGN_MASK;
        F128::MIN_POSITIVE.to_bits() <= abs && abs <= F128::MAX.to_bits()
    }

    /// Returns the floating point category of the number.
    ///
    /// If only one property is going to be tested, it is generally faster to
    /// use the specific predicate instead.
    ///
    /// # Example
    ///
    /// ```rust
    /// use core::num::FpCategory;
    /// use fixed::F128;
    ///
    /// assert_eq!(F128::ZERO.classify(), FpCategory::Zero);
    /// assert_eq!(F128::MIN_POSITIVE_SUB.classify(), FpCategory::Subnormal);
    /// assert_eq!(F128::MIN_POSITIVE.classify(), FpCategory::Normal);
    /// assert_eq!(F128::INFINITY.classify(), FpCategory::Infinite);
    /// assert_eq!(F128::NAN.classify(), FpCategory::Nan);
    /// ```
    #[inline]
    pub const fn classify(self) -> FpCategory {
        let exp = self.to_bits() & EXP_MASK;
        let mant = self.to_bits() & MANT_MASK;
        if exp == 0 {
            if mant == 0 {
                FpCategory::Zero
            } else {
                FpCategory::Subnormal
            }
        } else if exp == EXP_MASK {
            if mant == 0 {
                FpCategory::Infinite
            } else {
                FpCategory::Nan
            }
        } else {
            FpCategory::Normal
        }
    }

    /// Returns the absolute value of the number.
    ///
    /// The only difference possible between the input value and the returned
    /// value is in the sign bit, which is always cleared in the return value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fixed::F128;
    ///
    /// // -0 == +0, but -0 bits != +0 bits
    /// assert_eq!(F128::NEG_ZERO, F128::ZERO);
    /// assert_ne!(F128::NEG_ZERO.to_bits(), F128::ZERO.to_bits());
    /// assert_eq!(F128::NEG_ZERO.abs().to_bits(), F128::ZERO.to_bits());
    ///
    /// assert_eq!(F128::NEG_INFINITY.abs(), F128::INFINITY);
    /// assert_eq!(F128::MIN.abs(), F128::MAX);
    ///
    /// assert!(F128::NAN.abs().is_nan());
    /// ```
    #[inline]
    pub const fn abs(self) -> F128 {
        F128::from_bits(self.to_bits() & !SIGN_MASK)
    }

    /// Returns a number that represents the sign of the input value.
    ///
    ///   * 1 if the number is positive, +0, or +∞
    ///   * &minus;1 if the number is negative, &minus;0, or &minus;∞
    ///   * NaN if the number is NaN
    ///
    /// # Example
    ///
    /// ```rust
    /// use fixed::F128;
    ///
    /// assert_eq!(F128::ONE.signum(), F128::ONE);
    /// assert_eq!(F128::INFINITY.signum(), F128::ONE);
    /// assert_eq!(F128::NEG_ZERO.signum(), F128::NEG_ONE);
    /// assert_eq!(F128::MIN.signum(), F128::NEG_ONE);
    ///
    /// assert!(F128::NAN.signum().is_nan());
    /// ```
    #[inline]
    pub const fn signum(self) -> F128 {
        if self.is_nan() {
            self
        } else if self.is_sign_positive() {
            F128::ONE
        } else {
            F128::NEG_ONE
        }
    }

    /// Returns a number composed of the magnitude of `self` and the sign of `sign`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fixed::F128;
    ///
    /// assert_eq!(F128::ONE.copysign(F128::NEG_ZERO), F128::NEG_ONE);
    /// assert_eq!(F128::ONE.copysign(F128::ZERO), F128::ONE);
    /// assert_eq!(F128::NEG_ONE.copysign(F128::NEG_INFINITY), F128::NEG_ONE);
    /// assert_eq!(F128::NEG_ONE.copysign(F128::INFINITY), F128::ONE);
    ///
    /// assert!(F128::NAN.copysign(F128::ONE).is_nan());
    /// assert!(F128::NAN.copysign(F128::ONE).is_sign_positive());
    /// assert!(F128::NAN.copysign(F128::NEG_ONE).is_sign_negative());
    /// ```
    #[inline]
    pub const fn copysign(self, sign: F128) -> F128 {
        F128::from_bits((self.to_bits() & !SIGN_MASK) | (sign.to_bits() & SIGN_MASK))
    }

    /// Returns [`true`] if the number has a positive sign, including +0, +∞,
    /// and NaN without a negative sign bit.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fixed::F128;
    ///
    /// assert!(F128::ZERO.is_sign_positive());
    /// assert!(F128::MAX.is_sign_positive());
    /// assert!(F128::INFINITY.is_sign_positive());
    ///
    /// assert!(!F128::NEG_ZERO.is_sign_positive());
    /// assert!(!F128::MIN.is_sign_positive());
    /// assert!(!F128::NEG_INFINITY.is_sign_positive());
    /// ```
    #[inline]
    pub const fn is_sign_positive(self) -> bool {
        (self.to_bits() & SIGN_MASK) == 0
    }

    /// Returns [`true`] if the number has a negative sign, including &minus;0,
    /// &minus;∞, and NaN with a negative sign bit.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fixed::F128;
    ///
    /// assert!(F128::NEG_ZERO.is_sign_negative());
    /// assert!(F128::MIN.is_sign_negative());
    /// assert!(F128::NEG_INFINITY.is_sign_negative());
    ///
    /// assert!(!F128::ZERO.is_sign_negative());
    /// assert!(!F128::MAX.is_sign_negative());
    /// assert!(!F128::INFINITY.is_sign_negative());
    /// ```
    #[inline]
    pub const fn is_sign_negative(self) -> bool {
        (self.to_bits() & SIGN_MASK) != 0
    }

    /// Returns the ordering between `self` and `other`.
    ///
    /// Unlike the [`PartialOrd`] implementation, this method always returns an
    /// order in the following sequence:
    ///
    ///   * NaN with the sign bit set
    ///   * &minus;∞
    ///   * negative normal numbers
    ///   * negative subnormal numbers
    ///   * &minus;0
    ///   * +0
    ///   * positive subnormal numbers
    ///   * positive normal numbers
    ///   * +∞
    ///   * NaN with the sign bit cleared
    ///
    /// # Example
    ///
    /// ```rust
    /// use core::cmp::Ordering;
    /// use fixed::F128;
    ///
    /// let neg_nan = F128::NAN.copysign(F128::NEG_ONE);
    /// let pos_nan = F128::NAN.copysign(F128::ONE);
    /// let neg_inf = F128::NEG_INFINITY;
    /// let pos_inf = F128::INFINITY;
    /// let neg_zero = F128::NEG_ZERO;
    /// let pos_zero = F128::ZERO;
    ///
    /// assert_eq!(neg_nan.total_cmp(&neg_inf), Ordering::Less);
    /// assert_eq!(pos_nan.total_cmp(&pos_inf), Ordering::Greater);
    /// assert_eq!(neg_zero.total_cmp(&pos_zero), Ordering::Less);
    /// ```
    #[inline]
    pub const fn total_cmp(&self, other: &F128) -> Ordering {
        let a = self.to_bits();
        let b = other.to_bits();
        match (self.is_sign_negative(), other.is_sign_negative()) {
            (false, false) => cmp_bits(a, b),
            (true, true) => cmp_bits(b, a),
            (false, true) => Ordering::Greater,
            (true, false) => Ordering::Less,
        }
    }
}

const fn cmp_bits(a: u128, b: u128) -> Ordering {
    if a < b {
        Ordering::Less
    } else if a > b {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

impl PartialEq for F128 {
    #[inline]
    fn eq(&self, other: &F128) -> bool {
        if self.is_nan() || other.is_nan() {
            return false;
        }
        let a = self.to_bits();
        let b = other.to_bits();
        // handle zero
        if ((a | b) & !SIGN_MASK) == 0 {
            return true;
        }
        a == b
    }
}

impl PartialOrd for F128 {
    #[inline]
    fn partial_cmp(&self, other: &F128) -> Option<Ordering> {
        if self.is_nan() || other.is_nan() {
            return None;
        }
        let a = self.to_bits();
        let b = other.to_bits();
        // handle zero
        if ((a | b) & !SIGN_MASK) == 0 {
            return Some(Ordering::Equal);
        }
        match (self.is_sign_negative(), other.is_sign_negative()) {
            (false, false) => a.partial_cmp(&b),
            (true, true) => b.partial_cmp(&a),
            (false, true) => Some(Ordering::Greater),
            (true, false) => Some(Ordering::Less),
        }
    }
}

impl Hash for F128 {
    #[inline]
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        let mut bits = self.to_bits();
        if bits == F128::NEG_ZERO.to_bits() {
            bits = 0;
        }
        bits.hash(state);
    }
}

impl Neg for F128 {
    type Output = F128;
    #[inline]
    fn neg(self) -> F128 {
        F128::from_bits(self.to_bits() ^ SIGN_MASK)
    }
}
