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

// the value must be positive for all public functions

pub mod int_part {
    // 1 <= val <= MAX (255)
    // 0 <= log <= 2
    #[inline]
    pub const fn u8(val: u8) -> i32 {
        debug_assert!(val >= 1);
        // | from |  to |  a |  b | a&b |
        // |    1 |   9 | 10 | 01 |  00 |
        // |   10 |  99 | 11 | 01 |  01 |
        // |  100 | MAX | 11 | 10 |  10 |
        let val = val as i32;
        let a = (0b11 << 8) - 10 + val;
        let b = (0b10 << 8) - 100 + val;
        (a & b) >> 8
    }

    // 1 <= val <= MAX (65_535)
    // 0 <= log <= 4
    #[inline]
    pub const fn u16(val: u16) -> i32 {
        debug_assert!(val >= 1);
        less_than_5(val as u32)
    }

    // 1 <= val <= MAX
    // 0 <= log <= 9
    pub const fn u32(mut val: u32) -> i32 {
        debug_assert!(val >= 1);
        let mut log = 0;
        if val >= 100_000 {
            val /= 100_000;
            log += 5;
        }
        log + less_than_5(val)
    }

    // 1 <= val <= MAX
    // 0 <= log <= 19
    pub const fn u64(mut val: u64) -> i32 {
        debug_assert!(val >= 1);
        let mut log = 0;
        if val >= 10_000_000_000 {
            val /= 10_000_000_000;
            log += 10;
        }
        if val >= 100_000 {
            val /= 100_000;
            log += 5;
        }
        debug_assert!(val <= u32::MAX as u64);
        log + less_than_5(val as u32)
    }

    // 1 <= val <= MAX
    // 0 <= log <= 38
    pub const fn u128(mut val: u128) -> i32 {
        debug_assert!(val >= 1);
        let mut log = 0;
        if val >= 100_000_000_000_000_000_000_000_000_000_000 {
            val /= 100_000_000_000_000_000_000_000_000_000_000;
            debug_assert!(val <= u32::MAX as u128);
            return 32 + u32(val as u32);
        }
        if val >= 10_000_000_000_000_000 {
            val /= 10_000_000_000_000_000;
            log += 16;
        }
        debug_assert!(val <= u64::MAX as u128);
        log + u64(val as u64)
    }

    // 0 < val < 100_000
    // 0 <= log <= 4
    const fn less_than_5(val: u32) -> i32 {
        debug_assert!(val < 100_000);
        // |  from |    to |   a |   b | a&b |   c |   d | c&d | a&b ^ c&d |
        // |     1 |     9 | 010 | 011 | 010 | 110 | 011 | 010 |       000 |
        // |    10 |    99 | 011 | 011 | 011 | 110 | 011 | 010 |       001 |
        // |   100 |   999 | 011 | 100 | 000 | 110 | 011 | 010 |       010 |
        // |  1000 |  9999 | 011 | 100 | 000 | 111 | 011 | 011 |       011 |
        // | 10000 | 99999 | 011 | 100 | 000 | 111 | 100 | 100 |       100 |
        let val = val as i32;
        let a = (0b011 << 17) - 10 + val;
        let b = (0b100 << 17) - 100 + val;
        let c = (0b111 << 17) - 1000 + val;
        let d = (0b100 << 17) - 10000 + val;
        ((a & b) ^ (c & d)) >> 17
    }
}

pub mod frac_part {
    // MAX / 1000 (0) < val <= MAX (255)
    // -3 <= log <= -1
    #[inline]
    pub const fn u8(val: u8) -> i32 {
        if val > 25 {
            -1
        } else if val > 2 {
            -2
        } else {
            debug_assert!(val > 0);
            -3
        }
    }

    // MAX / 100_000 (0) < val <= MAX (65_535)
    // -5 <= log <= -1
    #[inline]
    pub const fn u16(val: u16) -> i32 {
        if val > 6553 {
            -1
        } else if val > 655 {
            -2
        } else if val > 65 {
            -3
        } else if val > 6 {
            -4
        } else {
            debug_assert!(val > 0);
            -5
        }
    }

    // 0 < val <= MAX
    // -10 <= log <= -1
    pub const fn u32(mut val: u32) -> i32 {
        const MAX: u32 = u32::MAX;
        if val <= MAX / 100_000_000 {
            val *= 100_000_000;
            // At this point, we have shifted out 8 digits, and we can only shift out 2 more.
            // We can only check up to -2 more because -10 <= log <= -8.
            if val > MAX / 10 {
                -9
            } else {
                debug_assert!(val > MAX / 100);
                -10
            }
        } else {
            greater_equal_m8_u32(val)
        }
    }

    // 0 < val <= MAX
    // -20 <= log <= -1
    pub const fn u64(mut val: u64) -> i32 {
        const MAX: u64 = u64::MAX;
        let mut log = 0;
        if val <= MAX / 10_000_000_000_000_000 {
            // After this, we can only check up to -4 more because -20 <= log <= -16.
            // That is, we can skip the checks against MAX / 100_000_000 and MAX / 10_000.
            val *= 10_000_000_000_000_000;
            log += -16;
        } else {
            if val <= MAX / 100_000_000 {
                val *= 100_000_000;
                log += -8;
            }
            if val <= MAX / 10_000 {
                val *= 10_000;
                log += -4;
            }
        }
        log + if val > MAX / 10 {
            -1
        } else if val > MAX / 100 {
            -2
        } else if val > MAX / 1000 {
            -3
        } else {
            debug_assert!(val > MAX / 10_000);
            -4
        }
    }

    // 0 < val <= MAX
    // -39 <= log <= -1
    pub const fn u128(mut val: u128) -> i32 {
        const MAX: u128 = u128::MAX;
        let mut log = 0;
        if val <= MAX / 100_000_000_000_000_000_000_000_000_000_000 {
            val *= 100_000_000_000_000_000_000_000_000_000_000;
            // At this point we have shifted out 32 digits, and we can only shift out 7 more.
            // We can
            //   * use val >> 96 because we have shifted out 32 decimal digits (106 bits)
            //   * only check up to -8 more because -39 <= log <= -32
            return -32 + greater_equal_m8_u32((val >> 96) as u32);
        }
        if val <= MAX / 10_000_000_000_000_000 {
            val *= 10_000_000_000_000_000;
            log += -16;
        }
        if val <= MAX / 100_000_000 {
            val *= 100_000_000;
            log += -8;
        }
        if log == -24 {
            // At this point we have shifted out 24 digits, and we can only shift out 15 more.
            // We can
            //   * use val >> 64 because we have shifted out 24 decimal digits (79 bits)
            //   * only check up to -8 more because -32 <= log <= -24
            return -24 + greater_equal_m8_u64((val >> 64) as u64);
        }
        // We have *not* shifted out enough decimal digits, so we must *not* convert to u32 or u64.
        if val <= MAX / 10_000 {
            val *= 10_000;
            log += -4;
        }
        log + if val > MAX / 10 {
            -1
        } else if val > MAX / 100 {
            -2
        } else if val > MAX / 1000 {
            -3
        } else {
            debug_assert!(val > MAX / 10_000);
            -4
        }
    }

    // MAX / 100_000_000 < val <= MAX
    // -8 <= log <= -1
    const fn greater_equal_m8_u32(mut val: u32) -> i32 {
        const MAX: u32 = u32::MAX;
        debug_assert!(val > MAX / 100_000_000);
        let mut log = 0;
        if val <= MAX / 10_000 {
            val *= 10_000;
            log += -4;
        }
        log + if val > MAX / 10 {
            -1
        } else if val > MAX / 100 {
            -2
        } else if val > MAX / 1000 {
            -3
        } else {
            debug_assert!(val > MAX / 10_000);
            -4
        }
    }

    // MAX / 100_000_000 < val <= MAX
    // -8 <= log <= 1
    const fn greater_equal_m8_u64(mut val: u64) -> i32 {
        const MAX: u64 = u64::MAX;
        debug_assert!(val > MAX / 100_000_000);
        let mut log = 0;
        if val <= MAX / 10_000 {
            val *= 10_000;
            log += -4;
        }
        log + if val > MAX / 10 {
            -1
        } else if val > MAX / 100 {
            -2
        } else if val > MAX / 1000 {
            -3
        } else {
            debug_assert!(val > MAX / 10_000);
            -4
        }
    }
}

// check log10() and log(10) in tandem
#[cfg(test)]
mod tests {
    use crate::{log, log10};

    macro_rules! check_loop {
        ($T:ident) => {
            for i in 0..=log10::int_part::$T(<$T>::MAX) {
                let p = (10 as $T).pow(i as u32);
                if i > 0 {
                    assert_eq!(log10::int_part::$T(p - 1), i - 1);
                    assert_eq!(log::int_part::$T(p - 1, 10), i - 1);
                }
                assert_eq!(log10::int_part::$T(p), i);
                assert_eq!(log::int_part::$T(p, 10), i);
                assert_eq!(log10::int_part::$T(p + 1), i);
                assert_eq!(log::int_part::$T(p + 1, 10), i);
            }

            for i in 0..-log10::frac_part::$T(1) {
                let p = <$T>::MAX / (10 as $T).pow(i as u32);
                if p > 1 {
                    assert_eq!(log10::frac_part::$T(p - 1), -1 - i);
                    assert_eq!(log::frac_part::$T(p - 1, 10), -1 - i);
                }
                assert_eq!(log10::frac_part::$T(p), -1 - i);
                assert_eq!(log::frac_part::$T(p, 10), -1 - i);
                if i > 0 {
                    assert_eq!(log10::frac_part::$T(p + 1), -i);
                    assert_eq!(log::frac_part::$T(p + 1, 10), -i);
                }
            }
        };
    }

    #[test]
    fn log10_u8() {
        assert_eq!(log10::int_part::u8(1), 0);
        assert_eq!(log::int_part::u8(1, 10), 0);
        assert_eq!(log10::int_part::u8(u8::MAX), 2);
        assert_eq!(log::int_part::u8(u8::MAX, 10), 2);
        assert_eq!(log10::frac_part::u8(1), -3);
        assert_eq!(log::frac_part::u8(1, 10), -3);
        assert_eq!(log10::frac_part::u8(u8::MAX), -1);
        assert_eq!(log::frac_part::u8(u8::MAX, 10), -1);

        check_loop! { u8 }
    }

    #[test]
    fn log10_u16() {
        assert_eq!(log10::int_part::u16(1), 0);
        assert_eq!(log::int_part::u16(1, 10), 0);
        assert_eq!(log10::int_part::u16(u16::MAX), 4);
        assert_eq!(log::int_part::u16(u16::MAX, 10), 4);
        assert_eq!(log10::frac_part::u16(1), -5);
        assert_eq!(log::frac_part::u16(1, 10), -5);
        assert_eq!(log10::frac_part::u16(u16::MAX), -1);
        assert_eq!(log::frac_part::u16(u16::MAX, 10), -1);

        check_loop! { u16 }
    }

    #[test]
    fn log10_u32() {
        assert_eq!(log10::int_part::u32(1), 0);
        assert_eq!(log::int_part::u32(1, 10), 0);
        assert_eq!(log10::int_part::u32(u32::MAX), 9);
        assert_eq!(log::int_part::u32(u32::MAX, 10), 9);
        assert_eq!(log10::frac_part::u32(1), -10);
        assert_eq!(log::frac_part::u32(1, 10), -10);
        assert_eq!(log10::frac_part::u32(u32::MAX), -1);
        assert_eq!(log::frac_part::u32(u32::MAX, 10), -1);

        check_loop! { u32 }
    }

    #[test]
    fn log10_u64() {
        assert_eq!(log10::int_part::u64(1), 0);
        assert_eq!(log::int_part::u64(1, 10), 0);
        assert_eq!(log10::int_part::u64(u64::MAX), 19);
        assert_eq!(log::int_part::u64(u64::MAX, 10), 19);
        assert_eq!(log10::frac_part::u64(1), -20);
        assert_eq!(log::frac_part::u64(1, 10), -20);
        assert_eq!(log10::frac_part::u64(u64::MAX), -1);
        assert_eq!(log::frac_part::u64(u64::MAX, 10), -1);

        check_loop! { u64 }
    }

    #[test]
    fn log10_u128() {
        assert_eq!(log10::int_part::u128(1), 0);
        assert_eq!(log::int_part::u128(1, 10), 0);
        assert_eq!(log10::int_part::u128(u128::MAX), 38);
        assert_eq!(log::int_part::u128(u128::MAX, 10), 38);
        assert_eq!(log10::frac_part::u128(1), -39);
        assert_eq!(log::frac_part::u128(1, 10), -39);
        assert_eq!(log10::frac_part::u128(u128::MAX), -1);
        assert_eq!(log::frac_part::u128(u128::MAX, 10), -1);

        check_loop! { u128 }
    }
}
