use core::num::Wrapping;
use num_traits::*;

//#[test]
pub fn wrapping_identities() {
    macro_rules! test_wrapping_identities {
        ($($t:ty)+) => {
            $(
                assert_eq!(zero::<$t>(), zero::<Wrapping<$t>>().0);
                assert_eq!(one::<$t>(), one::<Wrapping<$t>>().0);
                assert_eq!((0 as $t).is_zero(), Wrapping(0 as $t).is_zero());
                assert_eq!((1 as $t).is_zero(), Wrapping(1 as $t).is_zero());
            )+
        };
    }

    test_wrapping_identities!(isize i8 i16 i32 i64 usize u8 u16 u32 u64);
}

//#[test]
pub fn wrapping_is_zero() {
    fn require_zero<T: Zero>(_: &T) {}
    require_zero(&Wrapping(42));
}
//#[test]
pub fn wrapping_is_one() {
    fn require_one<T: One>(_: &T) {}
    require_one(&Wrapping(42));
}
