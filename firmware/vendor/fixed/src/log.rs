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

macro_rules! impl_int_part {
    ($u:ident) => {
        pub const fn $u(val: $u, base: u32) -> i32 {
            const MAX_TABLE_SIZE: usize = (u32::BITS - $u::BITS.leading_zeros() - 2) as usize;

            debug_assert!(val > 0);
            debug_assert!(base >= 2);

            let baseu = base as $u;
            if baseu as u32 != base || val < baseu {
                return 0;
            }

            // base^1, base^2, base^4, etc.
            let mut base_powers: [$u; MAX_TABLE_SIZE] = [0; MAX_TABLE_SIZE];

            let mut i = 0;
            let mut partial_log = 1;
            let mut partial_val = baseu;

            loop {
                let square = match partial_val.checked_mul(partial_val) {
                    Some(s) if val >= s => s,
                    _ => break,
                };
                base_powers[i] = partial_val;
                i += 1;
                partial_log *= 2;
                partial_val = square;
            }
            let mut dlog = partial_log;
            while i > 0 {
                i -= 1;
                dlog /= 2;
                if let Some(mid) = partial_val.checked_mul(base_powers[i]) {
                    if val >= mid {
                        partial_val = mid;
                        partial_log += dlog;
                    }
                }
            }
            return partial_log;
        }
    };
}

pub mod int_part {
    impl_int_part! { u8 }
    impl_int_part! { u16 }
    impl_int_part! { u32 }
    impl_int_part! { u64 }
    impl_int_part! { u128 }
}

macro_rules! impl_frac_part {
    ($u:ident) => {
        pub const fn $u(val: $u, base: u32) -> i32 {
            const MAX_TABLE_SIZE: usize = (u32::BITS - $u::BITS.leading_zeros() - 2) as usize;

            debug_assert!(val > 0);
            debug_assert!(base >= 2);

            let baseu = base as $u;
            if baseu as u32 != base || val.checked_mul(baseu).is_none() {
                return -1;
            }

            // base^1, base^2, base^4, etc.
            let mut base_powers: [$u; MAX_TABLE_SIZE] = [0; MAX_TABLE_SIZE];

            let mut i = 0;
            let mut partial_log = 1;
            let mut partial_val = baseu;

            loop {
                let square = match partial_val.checked_mul(partial_val) {
                    Some(s) if val.checked_mul(s).is_some() => s,
                    _ => break,
                };
                base_powers[i] = partial_val;
                i += 1;
                partial_log *= 2;
                partial_val = square;
            }
            let mut dlog = partial_log;
            while i > 0 {
                i -= 1;
                dlog /= 2;
                if let Some(mid) = partial_val.checked_mul(base_powers[i]) {
                    if val.checked_mul(mid).is_some() {
                        partial_val = mid;
                        partial_log += dlog;
                    }
                }
            }
            return -1 - partial_log;
        }
    };
}

pub mod frac_part {
    impl_frac_part! { u8 }
    impl_frac_part! { u16 }
    impl_frac_part! { u32 }
    impl_frac_part! { u64 }
    impl_frac_part! { u128 }
}

#[cfg(test)]
mod tests {
    use crate::log;

    // these tests require the maximum table sizes
    #[test]
    fn check_table_size_is_sufficient() {
        assert_eq!(log::int_part::u8(u8::MAX, 2), 7);
        assert_eq!(log::int_part::u16(u16::MAX, 2), 15);
        assert_eq!(log::int_part::u32(u32::MAX, 2), 31);
        assert_eq!(log::int_part::u64(u64::MAX, 2), 63);
        assert_eq!(log::int_part::u128(u128::MAX, 2), 127);

        assert_eq!(log::frac_part::u8(1, 2), -8);
        assert_eq!(log::frac_part::u16(1, 2), -16);
        assert_eq!(log::frac_part::u32(1, 2), -32);
        assert_eq!(log::frac_part::u64(1, 2), -64);
        assert_eq!(log::frac_part::u128(1, 2), -128);
    }
}
