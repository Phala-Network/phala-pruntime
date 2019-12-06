use core::num::Wrapping;
use num_traits::*;

//#[test]
pub fn clamp_test() {
    // Int test
    assert_eq!(1, clamp(1, -1, 2));
    assert_eq!(-1, clamp(-2, -1, 2));
    assert_eq!(2, clamp(3, -1, 2));
    assert_eq!(1, clamp_min(1, -1));
    assert_eq!(-1, clamp_min(-2, -1));
    assert_eq!(-1, clamp_max(1, -1));
    assert_eq!(-2, clamp_max(-2, -1));

    // Float test
    assert_eq!(1.0, clamp(1.0, -1.0, 2.0));
    assert_eq!(-1.0, clamp(-2.0, -1.0, 2.0));
    assert_eq!(2.0, clamp(3.0, -1.0, 2.0));
    assert_eq!(1.0, clamp_min(1.0, -1.0));
    assert_eq!(-1.0, clamp_min(-2.0, -1.0));
    assert_eq!(-1.0, clamp_max(1.0, -1.0));
    assert_eq!(-2.0, clamp_max(-2.0, -1.0));
    assert!(clamp(::core::f32::NAN, -1.0, 1.0).is_nan());
    assert!(clamp_min(::core::f32::NAN, 1.0).is_nan());
    assert!(clamp_max(::core::f32::NAN, 1.0).is_nan());
}

//#[test]
pub fn from_str_radix_unwrap() {
    // The Result error must impl Debug to allow unwrap()

    let i: i32 = Num::from_str_radix("0", 10).unwrap();
    assert_eq!(i, 0);

    let f: f32 = Num::from_str_radix("0.0", 10).unwrap();
    assert_eq!(f, 0.0);
}

//#[test]
pub fn from_str_radix_multi_byte_fail() {
    // Ensure parsing doesn't panic, even on invalid sign characters
    assert!(f32::from_str_radix("™0.2", 10).is_err());

    // Even when parsing the exponent sign
    assert!(f32::from_str_radix("0.2E™1", 10).is_err());
}

//#[test]
pub fn wrapping_is_num() {
    fn require_num<T: Num>(_: &T) {}
    require_num(&Wrapping(42_u32));
    require_num(&Wrapping(-42));
}

//#[test]
pub fn wrapping_from_str_radix() {
    macro_rules! test_wrapping_from_str_radix {
        ($($t:ty)+) => {
            $(
                for &(s, r) in &[("42", 10), ("42", 2), ("-13.0", 10), ("foo", 10)] {
                    let w = Wrapping::<$t>::from_str_radix(s, r).map(|w| w.0);
                    assert_eq!(w, <$t as Num>::from_str_radix(s, r));
                }
            )+
        };
    }

    test_wrapping_from_str_radix!(usize u8 u16 u32 u64 isize i8 i16 i32 i64);
}

//#[test]
pub fn check_num_ops() {
    fn compute<T: Num + Copy>(x: T, y: T) -> T {
        x * y / y % y + y - y
    }
    assert_eq!(compute(1, 2), 1)
}

//#[test]
pub fn check_numref_ops() {
    fn compute<T: NumRef>(x: T, y: &T) -> T {
        x * y / y % y + y - y
    }
    assert_eq!(compute(1, &2), 1)
}

//#[test]
pub fn check_refnum_ops() {
    fn compute<T: Copy>(x: &T, y: T) -> T
    where
        for<'a> &'a T: RefNum<T>,
    {
        &(&(&(&(x * y) / y) % y) + y) - y
    }
    assert_eq!(compute(&1, 2), 1)
}

//#[test]
pub fn check_refref_ops() {
    fn compute<T>(x: &T, y: &T) -> T
    where
        for<'a> &'a T: RefNum<T>,
    {
        &(&(&(&(x * y) / y) % y) + y) - y
    }
    assert_eq!(compute(&1, &2), 1)
}

//#[test]
pub fn check_numassign_ops() {
    fn compute<T: NumAssign + Copy>(mut x: T, y: T) -> T {
        x *= y;
        x /= y;
        x %= y;
        x += y;
        x -= y;
        x
    }
    assert_eq!(compute(1, 2), 1)
}

// TODO test `NumAssignRef`, but even the standard numeric types don't
// implement this yet. (see rust pr41336)
