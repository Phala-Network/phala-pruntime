use num_traits::MulAdd;
//#[test]
pub fn mul_add_integer() {
    macro_rules! test_mul_add {
        ($($t:ident)+) => {
            $(
                {
                    let m: $t = 2;
                    let x: $t = 3;
                    let b: $t = 4;

                    assert_eq!(MulAdd::mul_add(m, x, b), (m*x + b));
                }
            )+
        };
    }

    test_mul_add!(usize u8 u16 u32 u64 isize i8 i16 i32 i64);
}

//#[test]
//#[cfg(feature = "std")]
pub fn mul_add_float() {
    macro_rules! test_mul_add {
        ($($t:ident)+) => {
            $(
                {
                    use core::$t;

                    let m: $t = 12.0;
                    let x: $t = 3.4;
                    let b: $t = 5.6;

                    let abs_difference = (MulAdd::mul_add(m, x, b) - (m*x + b)).abs();

                    assert!(abs_difference <= $t::EPSILON);
                }
            )+
        };
    }

    test_mul_add!(f32 f64);
}
