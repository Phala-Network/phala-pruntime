use core::num::Wrapping;
use num_traits::*;
use core::{i128, u128};
use core::{i16, i32, i64, i8, isize};
use core::{u16, u32, u64, u8, usize};

//#[test]
pub fn wrapping_bounded() {
    macro_rules! test_wrapping_bounded {
        ($($t:ty)+) => {
            $(
                assert_eq!(Wrapping::<$t>::min_value().0, <$t>::min_value());
                assert_eq!(Wrapping::<$t>::max_value().0, <$t>::max_value());
            )+
        };
    }

    test_wrapping_bounded!(usize u8 u16 u32 u64 isize i8 i16 i32 i64);
}

//#[cfg(has_i128)]
//#[test]
pub fn wrapping_bounded_i128() {
    macro_rules! test_wrapping_bounded {
        ($($t:ty)+) => {
            $(
                assert_eq!(Wrapping::<$t>::min_value().0, <$t>::min_value());
                assert_eq!(Wrapping::<$t>::max_value().0, <$t>::max_value());
            )+
        };
    }

    test_wrapping_bounded!(u128 i128);
}

//#[test]
pub fn wrapping_is_bounded() {
    fn require_bounded<T: Bounded>(_: &T) {}
    require_bounded(&Wrapping(42_u32));
    require_bounded(&Wrapping(-42));
}
