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

#[cfg(not(debug_assertions))]
use core::hint;

use core::num::{NonZeroI128, NonZeroU128};

#[derive(Clone, Copy, Debug)]
pub struct U256 {
    pub lo: u128,
    pub hi: u128,
}

#[derive(Clone, Copy, Debug)]
pub struct I256 {
    pub lo: u128,
    pub hi: i128,
}

#[inline]
pub const fn u256_wrapping_as_i256(a: U256) -> I256 {
    I256 {
        lo: a.lo,
        hi: a.hi as i128,
    }
}

#[inline]
pub const fn wrapping_add_u256_u128(a: U256, b: u128) -> U256 {
    let (lo, carry) = a.lo.overflowing_add(b);
    let hi = a.hi.wrapping_add(carry as u128);
    U256 { lo, hi }
}

#[inline]
pub const fn overflowing_add_u128_u256(a: u128, b: U256) -> (u128, bool) {
    let (lo, carry) = a.overflowing_add(b.lo);
    (lo, carry | (b.hi != 0))
}

#[inline]
pub const fn overflowing_sub_u128_u256(a: u128, b: U256) -> (u128, bool) {
    let (lo, borrow) = a.overflowing_sub(b.lo);
    (lo, borrow | (b.hi != 0))
}

#[inline]
pub const fn wrapping_neg_u256(a: U256) -> U256 {
    let (lo, carry) = (!a.lo).overflowing_add(1);
    let hi = (!a.hi).wrapping_add(carry as u128);
    U256 { lo, hi }
}

#[inline]
pub const fn overflowing_add_i256_i128(a: I256, b: i128) -> (I256, bool) {
    let b = I256 {
        lo: b as u128,
        hi: b >> 127,
    };
    let (lo, carry) = a.lo.overflowing_add(b.lo);
    // b.hi is in {-1, 0}, and carry is in {0, 1}, so we can add them wrappingly
    let b_hi_plus_carry = b.hi.wrapping_add(carry as i128);
    let (hi, overflow) = a.hi.overflowing_add(b_hi_plus_carry);
    (I256 { lo, hi }, overflow)
}

#[inline]
const fn u128_lo_hi(u: u128) -> (u128, u128) {
    (u & !(!0 << 64), u >> 64)
}

#[inline]
const fn i128_lo_hi(i: i128) -> (i128, i128) {
    (i & !(!0 << 64), i >> 64)
}

#[inline]
pub const fn wide_mul_u128(lhs: u128, rhs: u128) -> U256 {
    let (ll, lh) = u128_lo_hi(lhs);
    let (rl, rh) = u128_lo_hi(rhs);
    // 0 <= ll_rl <= 2^128 - 2^65 + 1; ll_rl unit is 1
    let ll_rl = ll.wrapping_mul(rl);
    // 0 <= lh_rl <= 2^128 - 2^65 + 1; lh_rl unit is 2^64
    let lh_rl = lh.wrapping_mul(rl);
    // 0 <= ll_rh <= 2^128 - 2^65 + 1; ll_rh unit is 2^64
    let ll_rh = ll.wrapping_mul(rh);
    // 0 <= lh_rh <= 2^128 - 2^65 + 1; lh_rh unit is 2^128
    let lh_rh = lh.wrapping_mul(rh);

    // 0 <= col0 <= 2^64 - 1
    // 0 <= col64a <= 2^64 - 2
    let (col0, col64a) = u128_lo_hi(ll_rl);

    // 0 <= col64b <= 2^128 - 2^64 - 1
    let col64b = col64a.wrapping_add(lh_rl);

    // 0 <= col64c <= 2^128 - 1
    // 0 <= col192 <= 1
    let (col64c, col192) = col64b.overflowing_add(ll_rh);
    let col192 = if col192 { 1u128 } else { 0u128 };

    // 0 <= col64 <= 2^64 - 1
    // 0 <= col128 <= 2^64 - 1
    let (col64, col128) = u128_lo_hi(col64c);

    // Since both col0 and col64 fit in 64 bits, ans0 sum will never overflow.
    let ans0 = col0.wrapping_add(col64 << 64);
    // Since lhs * rhs fits in 256 bits, ans128 sum will never overflow.
    let ans128 = lh_rh.wrapping_add(col128).wrapping_add(col192 << 64);
    U256 {
        lo: ans0,
        hi: ans128,
    }
}

#[inline]
pub const fn wide_mul_i128(lhs: i128, rhs: i128) -> I256 {
    let (ll, lh) = i128_lo_hi(lhs);
    let (rl, rh) = i128_lo_hi(rhs);
    // 0 <= ll_rl <= 2^128 - 2^65 + 1; ll_rl unit is 1; must be unsigned to hold all range!
    let ll_rl = (ll as u128).wrapping_mul(rl as u128);
    // -2^127 + 2^63 <= lh_rl <= 2^127 - 2^64 - 2^63 + 1; lh_rl unit is 2^64
    let lh_rl = lh.wrapping_mul(rl);
    // -2^127 + 2^63 <= ll_rh <= 2^127 - 2^64 - 2^63 + 1; ll_rh unit is 2^64
    let ll_rh = ll.wrapping_mul(rh);
    // -2^126 + 2^63 <= lh_rh <= 2^126; lh_rh unit is 2^128
    let lh_rh = lh.wrapping_mul(rh);

    // 0 <= col0 <= 2^64 - 1
    // 0 <= col64a <= 2^64 - 2
    let (col0, col64a) = u128_lo_hi(ll_rl);

    // -2^127 + 2^63 <= col64b <= 2^127 - 2^63 - 1
    let col64b = (col64a as i128).wrapping_add(lh_rl);

    // -2^127 <= col64c <= 2^127 - 1
    // -1 <= col192 <= 1
    let (col64c, col192) = col64b.overflowing_add(ll_rh);
    let col192 = if col192 {
        // col64b and ll_rh have the same sign, and col64c has the opposite sign.
        if col64b < 0 {
            -1i128
        } else {
            1i128
        }
    } else {
        0i128
    };

    // 0 <= col64 <= 2^64 - 1
    // -2^63 <= col128 <= 2^63 - 1
    let (col64, col128) = i128_lo_hi(col64c);

    // Since both col0 and col64 fit in 64 bits, ans0 sum will never overflow.
    let ans0 = col0.wrapping_add((col64 as u128) << 64);
    // Since lhs * rhs fits in 256 bits, ans128 sum will never overflow.
    let ans128 = lh_rh.wrapping_add(col128).wrapping_add(col192 << 64);
    I256 {
        lo: ans0,
        hi: ans128,
    }
}

#[inline]
pub const fn shl_u256_max_128(a: U256, sh: u32) -> U256 {
    if sh == 0 {
        a
    } else if sh == 128 {
        U256 { lo: a.hi, hi: 0 }
    } else {
        U256 {
            lo: (a.lo >> sh) | (a.hi << (128 - sh)),
            hi: a.hi >> sh,
        }
    }
}

#[inline]
pub const fn shl_i256_max_128(a: I256, sh: u32) -> I256 {
    if sh == 0 {
        a
    } else if sh == 128 {
        I256 {
            lo: a.hi as u128,
            hi: a.hi >> 127,
        }
    } else {
        I256 {
            lo: (a.lo >> sh) | (a.hi << (128 - sh)) as u128,
            hi: a.hi >> sh,
        }
    }
}

/// # Safety
///
/// d must have msb set.
#[inline]
const unsafe fn div_half_u128(r: u128, d: u128, next_half: u128) -> (u128, u128) {
    let (dl, dh) = u128_lo_hi(d);
    // SAFETY: we know d has the most significant bit set because we normalized
    if dh == 0 {
        #[cfg(debug_assertions)]
        {
            unreachable!();
        }
        #[cfg(not(debug_assertions))]
        unsafe {
            hint::unreachable_unchecked();
        }
    }
    let (mut q, rr) = (r / dh, r % dh);
    let m = q * dl;
    let mut r = next_half + (rr << 64);
    if r < m {
        q -= 1;
        let (new_r, overflow) = r.overflowing_add(d);
        r = if !overflow && new_r < m {
            q -= 1;
            new_r.wrapping_add(d)
        } else {
            new_r
        };
    }
    r = r.wrapping_sub(m);
    (r, q)
}

#[inline]
pub const fn div_rem_u256_u128(mut n: U256, d: NonZeroU128) -> (U256, u128) {
    let zeros = d.leading_zeros();
    let (mut r, d) = if zeros == 0 {
        (0, d.get())
    } else {
        let n2 = n.hi >> (128 - zeros);
        n.hi = n.hi << zeros | n.lo >> (128 - zeros);
        n.lo <<= zeros;
        (n2, d.get() << zeros)
    };

    // SAFETY: we know that d has msb set because it is not zero and it was
    // shifted to the right by the number of leading zeros.
    let (nhl, nhh) = u128_lo_hi(n.hi);
    let (new_r, qhh) = unsafe { div_half_u128(r, d, nhh) };
    r = new_r;
    let (new_r, qhl) = unsafe { div_half_u128(r, d, nhl) };
    r = new_r;
    let (nll, nlh) = u128_lo_hi(n.lo);
    let (new_r, qlh) = unsafe { div_half_u128(r, d, nlh) };
    r = new_r;
    let (new_r, qll) = unsafe { div_half_u128(r, d, nll) };
    r = new_r;
    let q = U256 {
        lo: qll + (qlh << 64),
        hi: qhl + (qhh << 64),
    };
    r >>= zeros;
    (q, r)
}

// must not result in overflow
#[inline]
pub const fn div_rem_i256_i128_no_overflow(n: I256, d: NonZeroI128) -> (I256, i128) {
    let (n_neg, n_abs) = if n.hi < 0 {
        let (nl, overflow) = n.lo.overflowing_neg();
        let nh = n.hi.wrapping_neg().wrapping_sub(overflow as i128) as u128;
        (true, U256 { lo: nl, hi: nh })
    } else {
        let nl = n.lo;
        let nh = n.hi as u128;
        (false, U256 { lo: nl, hi: nh })
    };
    let (d_neg, d_abs) = if d.get() < 0 {
        // SAFETY: d and -d are not zero
        let ud = unsafe { NonZeroU128::new_unchecked(d.get().wrapping_neg() as u128) };
        (true, ud)
    } else {
        // SAFETY: d is not zero
        let ud = unsafe { NonZeroU128::new_unchecked(d.get() as u128) };
        (false, ud)
    };

    let (q_abs, r_abs) = div_rem_u256_u128(n_abs, d_abs);

    let q = if n_neg != d_neg {
        let (ql, overflow) = q_abs.lo.overflowing_neg();
        let qh = q_abs.hi.wrapping_neg().wrapping_sub(overflow as u128) as i128;
        I256 { lo: ql, hi: qh }
    } else {
        let ql = q_abs.lo;
        let qh = q_abs.hi as i128;
        I256 { lo: ql, hi: qh }
    };
    let r = if n_neg {
        r_abs.wrapping_neg() as i128
    } else {
        r_abs as i128
    };
    (q, r)
}

#[inline]
pub const fn overflowing_shl_u256_into_u128(a: U256, sh: u32) -> (u128, bool) {
    if sh == 128 {
        (a.hi, false)
    } else if sh == 0 {
        (a.lo, a.hi != 0)
    } else {
        let lo = a.lo >> sh;
        let hi = a.hi << (128 - sh);
        (lo | hi, a.hi >> sh != 0)
    }
}

#[inline]
pub const fn overflowing_shl_i256_into_i128(a: I256, sh: u32) -> (i128, bool) {
    if sh == 128 {
        (a.hi, false)
    } else if sh == 0 {
        let ans = a.lo as i128;
        (ans, a.hi != ans >> 127)
    } else {
        let lo = (a.lo >> sh) as i128;
        let hi = a.hi << (128 - sh);
        let ans = lo | hi;
        (ans, a.hi >> sh != ans >> 127)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::num::{NonZeroI128, NonZeroU128};

    fn check_udiv_rem(num: U256, den: u128) {
        let (quot, rem) = div_rem_u256_u128(num, NonZeroU128::new(den).unwrap());
        assert!(rem <= den);

        let ql_d = wide_mul_u128(quot.lo, den);
        let qh_d = wide_mul_u128(quot.hi, den);
        assert!(qh_d.hi == 0);
        let prod_lo = ql_d.lo;
        let prod_hi = ql_d.hi.checked_add(qh_d.lo).unwrap();
        let (sum_lo, carry) = prod_lo.overflowing_add(rem);
        let sum_hi = prod_hi.checked_add(u128::from(carry)).unwrap();
        assert!(sum_lo == num.lo && sum_hi == num.hi);
    }

    fn check_idiv_rem_signs(num: I256, den: i128) {
        let (quot, rem) = div_rem_i256_i128_no_overflow(num, NonZeroI128::new(den).unwrap());
        assert!(rem.unsigned_abs() <= den.unsigned_abs());

        if num.hi < 0 {
            assert!(rem <= 0);
        } else {
            assert!(rem >= 0);
        }

        if (num.hi < 0) != (den < 0) {
            assert!(quot.hi <= 0);
        } else {
            assert!(quot.hi >= 0);
        }
    }

    #[test]
    fn test_udiv_rem() {
        for d in 1u8..=255 {
            for n1 in (0u8..=255).step_by(15) {
                for n0 in (0u8..=255).step_by(15) {
                    let d = u128::from(d) << 120 | 1;
                    let n1 = u128::from(n1) << 120 | 1;
                    let n0 = u128::from(n0) << 120 | 1;
                    check_udiv_rem(U256 { lo: n0, hi: n1 }, d);
                }
            }
        }
    }

    #[test]
    fn test_idiv_rem_signs() {
        for d in -128..=127 {
            if d == 0 {
                continue;
            }
            for n1 in (-120..=120).step_by(15) {
                for n0 in (0u8..=255).step_by(15) {
                    let d = i128::from(d) << 120 | 1;
                    let n1 = i128::from(n1) << 120 | 1;
                    let n0 = u128::from(n0) << 120 | 1;
                    check_idiv_rem_signs(I256 { lo: n0, hi: n1 }, d);
                }
            }
        }
    }
}
