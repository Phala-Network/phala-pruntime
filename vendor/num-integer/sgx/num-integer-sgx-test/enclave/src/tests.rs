use num_integer::*;

macro_rules! impl_integer_for_isize {
    ($T:ty, $test_mod:ident) => {
        //#[cfg(test)]
        pub mod $test_mod {
            use core::mem;
            use num_integer::{Integer};

            /// Checks that the division rule holds for:
            ///
            /// - `n`: numerator (dividend)
            /// - `d`: denominator (divisor)
            /// - `qr`: quotient and remainder
            //#[cfg(test)]
            pub fn test_division_rule((n, d): ($T, $T), (q, r): ($T, $T)) {
                assert_eq!(d * q + r, n);
            }

            //#[test]
            pub fn test_div_rem() {
                pub fn test_nd_dr(nd: ($T, $T), qr: ($T, $T)) {
                    let (n, d) = nd;
                    let separate_div_rem = (n / d, n % d);
                    let combined_div_rem = n.div_rem(&d);

                    assert_eq!(separate_div_rem, qr);
                    assert_eq!(combined_div_rem, qr);

                    test_division_rule(nd, separate_div_rem);
                    test_division_rule(nd, combined_div_rem);
                }

                test_nd_dr((8, 3), (2, 2));
                test_nd_dr((8, -3), (-2, 2));
                test_nd_dr((-8, 3), (-2, -2));
                test_nd_dr((-8, -3), (2, -2));

                test_nd_dr((1, 2), (0, 1));
                test_nd_dr((1, -2), (0, 1));
                test_nd_dr((-1, 2), (0, -1));
                test_nd_dr((-1, -2), (0, -1));
            }

            //#[test]
            pub fn test_div_mod_floor() {
                pub fn test_nd_dm(nd: ($T, $T), dm: ($T, $T)) {
                    let (n, d) = nd;
                    let separate_div_mod_floor = (n.div_floor(&d), n.mod_floor(&d));
                    let combined_div_mod_floor = n.div_mod_floor(&d);

                    assert_eq!(separate_div_mod_floor, dm);
                    assert_eq!(combined_div_mod_floor, dm);

                    test_division_rule(nd, separate_div_mod_floor);
                    test_division_rule(nd, combined_div_mod_floor);
                }

                test_nd_dm((8, 3), (2, 2));
                test_nd_dm((8, -3), (-3, -1));
                test_nd_dm((-8, 3), (-3, 1));
                test_nd_dm((-8, -3), (2, -2));

                test_nd_dm((1, 2), (0, 1));
                test_nd_dm((1, -2), (-1, -1));
                test_nd_dm((-1, 2), (-1, 1));
                test_nd_dm((-1, -2), (0, -1));
            }

            //#[test]
            pub fn test_gcd() {
                assert_eq!((10 as $T).gcd(&2), 2 as $T);
                assert_eq!((10 as $T).gcd(&3), 1 as $T);
                assert_eq!((0 as $T).gcd(&3), 3 as $T);
                assert_eq!((3 as $T).gcd(&3), 3 as $T);
                assert_eq!((56 as $T).gcd(&42), 14 as $T);
                assert_eq!((3 as $T).gcd(&-3), 3 as $T);
                assert_eq!((-6 as $T).gcd(&3), 3 as $T);
                assert_eq!((-4 as $T).gcd(&-2), 2 as $T);
            }

            //#[test]
            pub fn test_gcd_cmp_with_euclidean() {
                fn euclidean_gcd(mut m: $T, mut n: $T) -> $T {
                    while m != 0 {
                        mem::swap(&mut m, &mut n);
                        m %= n;
                    }

                    n.abs()
                }

                // gcd(-128, b) = 128 is not representable as positive value
                // for i8
                for i in -127..127 {
                    for j in -127..127 {
                        assert_eq!(euclidean_gcd(i, j), i.gcd(&j));
                    }
                }

                // last value
                // FIXME: Use inclusive ranges for above loop when implemented
                let i = 127;
                for j in -127..127 {
                    assert_eq!(euclidean_gcd(i, j), i.gcd(&j));
                }
                assert_eq!(127.gcd(&127), 127);
            }

            //#[test]
            pub fn test_gcd_min_val() {
                let min = <$T>::min_value();
                let max = <$T>::max_value();
                let max_pow2 = max / 2 + 1;
                assert_eq!(min.gcd(&max), 1 as $T);
                assert_eq!(max.gcd(&min), 1 as $T);
                assert_eq!(min.gcd(&max_pow2), max_pow2);
                assert_eq!(max_pow2.gcd(&min), max_pow2);
                assert_eq!(min.gcd(&42), 2 as $T);
                assert_eq!((42 as $T).gcd(&min), 2 as $T);
            }

            //#[test]
            //#[should_panic]
            pub fn test_gcd_min_val_min_val() {
                let min = <$T>::min_value();
                assert!(min.gcd(&min) >= 0);
            }

            //#[test]
            //#[should_panic]
            pub fn test_gcd_min_val_0() {
                let min = <$T>::min_value();
                assert!(min.gcd(&0) >= 0);
            }

            //#[test]
            //#[should_panic]
            pub fn test_gcd_0_min_val() {
                let min = <$T>::min_value();
                assert!((0 as $T).gcd(&min) >= 0);
            }

            //#[test]
            pub fn test_lcm() {
                assert_eq!((1 as $T).lcm(&0), 0 as $T);
                assert_eq!((0 as $T).lcm(&1), 0 as $T);
                assert_eq!((1 as $T).lcm(&1), 1 as $T);
                assert_eq!((-1 as $T).lcm(&1), 1 as $T);
                assert_eq!((1 as $T).lcm(&-1), 1 as $T);
                assert_eq!((-1 as $T).lcm(&-1), 1 as $T);
                assert_eq!((8 as $T).lcm(&9), 72 as $T);
                assert_eq!((11 as $T).lcm(&5), 55 as $T);
            }

            //#[test]
            pub fn test_gcd_lcm() {
                use core::iter::once;
                for i in once(0)
                    .chain((1..).take(127).flat_map(|a| once(a).chain(once(-a))))
                    .chain(once(-128))
                {
                    for j in once(0)
                        .chain((1..).take(127).flat_map(|a| once(a).chain(once(-a))))
                        .chain(once(-128))
                    {
                        assert_eq!(i.gcd_lcm(&j), (i.gcd(&j), i.lcm(&j)));
                    }
                }
            }

            //#[test]
            pub fn test_extended_gcd_lcm() {
                use core::fmt::Debug;
                use traits::NumAssign;
                use num_integer::ExtendedGcd;

                fn check<A: Copy + Debug + Integer + NumAssign>(a: A, b: A) {
                    let ExtendedGcd { gcd, x, y, .. } = a.extended_gcd(&b);
                    assert_eq!(gcd, x * a + y * b);
                }

                use core::iter::once;
                for i in once(0)
                    .chain((1..).take(127).flat_map(|a| once(a).chain(once(-a))))
                    .chain(once(-128))
                {
                    for j in once(0)
                        .chain((1..).take(127).flat_map(|a| once(a).chain(once(-a))))
                        .chain(once(-128))
                    {
                        check(i, j);
                        let (ExtendedGcd { gcd, .. }, lcm) = i.extended_gcd_lcm(&j);
                        assert_eq!((gcd, lcm), (i.gcd(&j), i.lcm(&j)));
                    }
                }
            }

            //#[test]
            pub fn test_even() {
                assert_eq!((-4 as $T).is_even(), true);
                assert_eq!((-3 as $T).is_even(), false);
                assert_eq!((-2 as $T).is_even(), true);
                assert_eq!((-1 as $T).is_even(), false);
                assert_eq!((0 as $T).is_even(), true);
                assert_eq!((1 as $T).is_even(), false);
                assert_eq!((2 as $T).is_even(), true);
                assert_eq!((3 as $T).is_even(), false);
                assert_eq!((4 as $T).is_even(), true);
            }

            //#[test]
            pub fn test_odd() {
                assert_eq!((-4 as $T).is_odd(), false);
                assert_eq!((-3 as $T).is_odd(), true);
                assert_eq!((-2 as $T).is_odd(), false);
                assert_eq!((-1 as $T).is_odd(), true);
                assert_eq!((0 as $T).is_odd(), false);
                assert_eq!((1 as $T).is_odd(), true);
                assert_eq!((2 as $T).is_odd(), false);
                assert_eq!((3 as $T).is_odd(), true);
                assert_eq!((4 as $T).is_odd(), false);
            }
        }
    };
}

impl_integer_for_isize!(i8, test_integer_i8);
impl_integer_for_isize!(i16, test_integer_i16);
impl_integer_for_isize!(i32, test_integer_i32);
impl_integer_for_isize!(i64, test_integer_i64);
impl_integer_for_isize!(isize, test_integer_isize);
//#[cfg(has_i128)]
impl_integer_for_isize!(i128, test_integer_i128);

macro_rules! impl_integer_for_usize {
    ($T:ty, $test_mod:ident) => {
        //#[cfg(test)]
        pub mod $test_mod {
            use core::mem;
            use num_integer::{Integer};

            //#[test]
            pub fn test_div_mod_floor() {
                assert_eq!((10 as $T).div_floor(&(3 as $T)), 3 as $T);
                assert_eq!((10 as $T).mod_floor(&(3 as $T)), 1 as $T);
                assert_eq!((10 as $T).div_mod_floor(&(3 as $T)), (3 as $T, 1 as $T));
                assert_eq!((5 as $T).div_floor(&(5 as $T)), 1 as $T);
                assert_eq!((5 as $T).mod_floor(&(5 as $T)), 0 as $T);
                assert_eq!((5 as $T).div_mod_floor(&(5 as $T)), (1 as $T, 0 as $T));
                assert_eq!((3 as $T).div_floor(&(7 as $T)), 0 as $T);
                assert_eq!((3 as $T).mod_floor(&(7 as $T)), 3 as $T);
                assert_eq!((3 as $T).div_mod_floor(&(7 as $T)), (0 as $T, 3 as $T));
            }

            //#[test]
            pub fn test_gcd() {
                assert_eq!((10 as $T).gcd(&2), 2 as $T);
                assert_eq!((10 as $T).gcd(&3), 1 as $T);
                assert_eq!((0 as $T).gcd(&3), 3 as $T);
                assert_eq!((3 as $T).gcd(&3), 3 as $T);
                assert_eq!((56 as $T).gcd(&42), 14 as $T);
            }

            //#[test]
            pub fn test_gcd_cmp_with_euclidean() {
                fn euclidean_gcd(mut m: $T, mut n: $T) -> $T {
                    while m != 0 {
                        mem::swap(&mut m, &mut n);
                        m %= n;
                    }
                    n
                }

                for i in 0..255 {
                    for j in 0..255 {
                        assert_eq!(euclidean_gcd(i, j), i.gcd(&j));
                    }
                }

                // last value
                // FIXME: Use inclusive ranges for above loop when implemented
                let i = 255;
                for j in 0..255 {
                    assert_eq!(euclidean_gcd(i, j), i.gcd(&j));
                }
                assert_eq!(255.gcd(&255), 255);
            }

            //#[test]
            pub fn test_lcm() {
                assert_eq!((1 as $T).lcm(&0), 0 as $T);
                assert_eq!((0 as $T).lcm(&1), 0 as $T);
                assert_eq!((1 as $T).lcm(&1), 1 as $T);
                assert_eq!((8 as $T).lcm(&9), 72 as $T);
                assert_eq!((11 as $T).lcm(&5), 55 as $T);
                assert_eq!((15 as $T).lcm(&17), 255 as $T);
            }

            //#[test]
            pub fn test_gcd_lcm() {
                for i in (0..).take(256) {
                    for j in (0..).take(256) {
                        assert_eq!(i.gcd_lcm(&j), (i.gcd(&j), i.lcm(&j)));
                    }
                }
            }

            //#[test]
            pub fn test_is_multiple_of() {
                assert!((6 as $T).is_multiple_of(&(6 as $T)));
                assert!((6 as $T).is_multiple_of(&(3 as $T)));
                assert!((6 as $T).is_multiple_of(&(1 as $T)));
            }

            //#[test]
            pub fn test_even() {
                assert_eq!((0 as $T).is_even(), true);
                assert_eq!((1 as $T).is_even(), false);
                assert_eq!((2 as $T).is_even(), true);
                assert_eq!((3 as $T).is_even(), false);
                assert_eq!((4 as $T).is_even(), true);
            }

            //#[test]
            pub fn test_odd() {
                assert_eq!((0 as $T).is_odd(), false);
                assert_eq!((1 as $T).is_odd(), true);
                assert_eq!((2 as $T).is_odd(), false);
                assert_eq!((3 as $T).is_odd(), true);
                assert_eq!((4 as $T).is_odd(), false);
            }
        }
    };
}

impl_integer_for_usize!(u8, test_integer_u8);
impl_integer_for_usize!(u16, test_integer_u16);
impl_integer_for_usize!(u32, test_integer_u32);
impl_integer_for_usize!(u64, test_integer_u64);
impl_integer_for_usize!(usize, test_integer_usize);
//#[cfg(has_i128)]
impl_integer_for_usize!(u128, test_integer_u128);

//#[test]
pub fn test_lcm_overflow() {
    macro_rules! check {
        ($t:ty, $x:expr, $y:expr, $r:expr) => {{
            let x: $t = $x;
            let y: $t = $y;
            let o = x.checked_mul(y);
            assert!(
                o.is_none(),
                "sanity checking that {} input {} * {} overflows",
                stringify!($t),
                x,
                y
            );
            assert_eq!(x.lcm(&y), $r);
            assert_eq!(y.lcm(&x), $r);
        }};
    }

    // Original bug (Issue #166)
    check!(i64, 46656000000000000, 600, 46656000000000000);

    check!(i8, 0x40, 0x04, 0x40);
    check!(u8, 0x80, 0x02, 0x80);
    check!(i16, 0x40_00, 0x04, 0x40_00);
    check!(u16, 0x80_00, 0x02, 0x80_00);
    check!(i32, 0x4000_0000, 0x04, 0x4000_0000);
    check!(u32, 0x8000_0000, 0x02, 0x8000_0000);
    check!(i64, 0x4000_0000_0000_0000, 0x04, 0x4000_0000_0000_0000);
    check!(u64, 0x8000_0000_0000_0000, 0x02, 0x8000_0000_0000_0000);
}

//#[test]
pub fn test_iter_binomial() {
    macro_rules! check_simple {
        ($t:ty) => {{
            let n: $t = 3;
            let expected = [1, 3, 3, 1];
            for (b, &e) in IterBinomial::new(n).zip(&expected) {
                assert_eq!(b, e);
            }
        }};
    }

    check_simple!(u8);
    check_simple!(i8);
    check_simple!(u16);
    check_simple!(i16);
    check_simple!(u32);
    check_simple!(i32);
    check_simple!(u64);
    check_simple!(i64);

    macro_rules! check_binomial {
        ($t:ty, $n:expr) => {{
            let n: $t = $n;
            let mut k: $t = 0;
            for b in IterBinomial::new(n) {
                assert_eq!(b, binomial(n, k));
                k += 1;
            }
        }};
    }

    // Check the largest n for which there is no overflow.
    check_binomial!(u8, 10);
    check_binomial!(i8, 9);
    check_binomial!(u16, 18);
    check_binomial!(i16, 17);
    check_binomial!(u32, 34);
    check_binomial!(i32, 33);
    check_binomial!(u64, 67);
    check_binomial!(i64, 66);
}

//#[test]
pub fn test_binomial() {
    macro_rules! check {
        ($t:ty, $x:expr, $y:expr, $r:expr) => {{
            let x: $t = $x;
            let y: $t = $y;
            let expected: $t = $r;
            assert_eq!(binomial(x, y), expected);
            if y <= x {
                assert_eq!(binomial(x, x - y), expected);
            }
        }};
    }
    check!(u8, 9, 4, 126);
    check!(u8, 0, 0, 1);
    check!(u8, 2, 3, 0);

    check!(i8, 9, 4, 126);
    check!(i8, 0, 0, 1);
    check!(i8, 2, 3, 0);

    check!(u16, 100, 2, 4950);
    check!(u16, 14, 4, 1001);
    check!(u16, 0, 0, 1);
    check!(u16, 2, 3, 0);

    check!(i16, 100, 2, 4950);
    check!(i16, 14, 4, 1001);
    check!(i16, 0, 0, 1);
    check!(i16, 2, 3, 0);

    check!(u32, 100, 2, 4950);
    check!(u32, 35, 11, 417225900);
    check!(u32, 14, 4, 1001);
    check!(u32, 0, 0, 1);
    check!(u32, 2, 3, 0);

    check!(i32, 100, 2, 4950);
    check!(i32, 35, 11, 417225900);
    check!(i32, 14, 4, 1001);
    check!(i32, 0, 0, 1);
    check!(i32, 2, 3, 0);

    check!(u64, 100, 2, 4950);
    check!(u64, 35, 11, 417225900);
    check!(u64, 14, 4, 1001);
    check!(u64, 0, 0, 1);
    check!(u64, 2, 3, 0);

    check!(i64, 100, 2, 4950);
    check!(i64, 35, 11, 417225900);
    check!(i64, 14, 4, 1001);
    check!(i64, 0, 0, 1);
    check!(i64, 2, 3, 0);
}

//#[test]
pub fn test_multinomial() {
    macro_rules! check_binomial {
        ($t:ty, $k:expr) => {{
            let n: $t = $k.iter().fold(0, |acc, &x| acc + x);
            let k: &[$t] = $k;
            assert_eq!(k.len(), 2);
            assert_eq!(multinomial(k), binomial(n, k[0]));
        }};
    }

    check_binomial!(u8, &[4, 5]);

    check_binomial!(i8, &[4, 5]);

    check_binomial!(u16, &[2, 98]);
    check_binomial!(u16, &[4, 10]);

    check_binomial!(i16, &[2, 98]);
    check_binomial!(i16, &[4, 10]);

    check_binomial!(u32, &[2, 98]);
    check_binomial!(u32, &[11, 24]);
    check_binomial!(u32, &[4, 10]);

    check_binomial!(i32, &[2, 98]);
    check_binomial!(i32, &[11, 24]);
    check_binomial!(i32, &[4, 10]);

    check_binomial!(u64, &[2, 98]);
    check_binomial!(u64, &[11, 24]);
    check_binomial!(u64, &[4, 10]);

    check_binomial!(i64, &[2, 98]);
    check_binomial!(i64, &[11, 24]);
    check_binomial!(i64, &[4, 10]);

    macro_rules! check_multinomial {
        ($t:ty, $k:expr, $r:expr) => {{
            let k: &[$t] = $k;
            let expected: $t = $r;
            assert_eq!(multinomial(k), expected);
        }};
    }

    check_multinomial!(u8, &[2, 1, 2], 30);
    check_multinomial!(u8, &[2, 3, 0], 10);

    check_multinomial!(i8, &[2, 1, 2], 30);
    check_multinomial!(i8, &[2, 3, 0], 10);

    check_multinomial!(u16, &[2, 1, 2], 30);
    check_multinomial!(u16, &[2, 3, 0], 10);

    check_multinomial!(i16, &[2, 1, 2], 30);
    check_multinomial!(i16, &[2, 3, 0], 10);

    check_multinomial!(u32, &[2, 1, 2], 30);
    check_multinomial!(u32, &[2, 3, 0], 10);

    check_multinomial!(i32, &[2, 1, 2], 30);
    check_multinomial!(i32, &[2, 3, 0], 10);

    check_multinomial!(u64, &[2, 1, 2], 30);
    check_multinomial!(u64, &[2, 3, 0], 10);

    check_multinomial!(i64, &[2, 1, 2], 30);
    check_multinomial!(i64, &[2, 3, 0], 10);

    check_multinomial!(u64, &[], 1);
    check_multinomial!(u64, &[0], 1);
    check_multinomial!(u64, &[12345], 1);
}
