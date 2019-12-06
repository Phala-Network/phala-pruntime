use rand::rngs::mock::StepRng;
use rand::rngs::StdRng;
use rand::*;

use std::prelude::v1::*;
use std::panic;

pub fn rng(seed: u64) -> impl RngCore {
    StdRng::seed_from_u64(seed)
}

pub fn test_fill_bytes_default() {
    let mut r = StepRng::new(0x11_22_33_44_55_66_77_88, 0);

    // check every remainder mod 8, both in small and big vectors.
    let lengths = [0, 1, 2, 3, 4, 5, 6, 7,
                   80, 81, 82, 83, 84, 85, 86, 87];
    for &n in lengths.iter() {
        let mut buffer = [0u8; 87];
        let v = &mut buffer[0..n];
        r.fill_bytes(v);

        // use this to get nicer error messages.
        for (i, &byte) in v.iter().enumerate() {
            if byte == 0 {
                panic!("byte {} of {} is zero", i, n)
            }
        }
    }
}

//#[test]
pub fn test_fill() {
    let x = 9041086907909331047;    // a random u64
    let mut rng = StepRng::new(x, 0);

    // Convert to byte sequence and back to u64; byte-swap twice if BE.
    let mut array = [0u64; 2];
    rng.fill(&mut array[..]);
    assert_eq!(array, [x, x]);
    assert_eq!(rng.next_u64(), x);

    // Convert to bytes then u32 in LE order
    let mut array = [0u32; 2];
    rng.fill(&mut array[..]);
    assert_eq!(array, [x as u32, (x >> 32) as u32]);
    assert_eq!(rng.next_u32(), x as u32);
}

//#[test]
pub fn test_fill_empty() {
    let mut array = [0u32; 0];
    let mut rng = StepRng::new(0, 1);
    rng.fill(&mut array);
    rng.fill(&mut array[..]);
}

//#[test]
pub fn test_gen_range() {
    let mut r = rng(101);
    for _ in 0..1000 {
        let a = r.gen_range(-4711, 17);
        assert!(a >= -4711 && a < 17);
        let a = r.gen_range(-3i8, 42);
        assert!(a >= -3i8 && a < 42i8);
        let a = r.gen_range(&10u16, 99);
        assert!(a >= 10u16 && a < 99u16);
        let a = r.gen_range(-100i32, &2000);
        assert!(a >= -100i32 && a < 2000i32);
        let a = r.gen_range(&12u32, &24u32);
        assert!(a >= 12u32 && a < 24u32);

        assert_eq!(r.gen_range(0u32, 1), 0u32);
        assert_eq!(r.gen_range(-12i64, -11), -12i64);
        assert_eq!(r.gen_range(3_000_000, 3_000_001), 3_000_000);
    }
}

//#[test]
//#[should_panic]
pub fn test_gen_range_panic_int() {
    let mut r = rng(102);
    r.gen_range(5, -2);
}

//#[test]
//#[should_panic]
//pub fn test_gen_range_panic_usize() {
//    let mut r = rng(103);
//    should_panic!(r.gen_range(5, 2));
//}

//#[test]
pub fn test_gen_bool() {
    let mut r = rng(105);
    for _ in 0..5 {
        assert_eq!(r.gen_bool(0.0), false);
        assert_eq!(r.gen_bool(1.0), true);
    }
}

//#[test]
pub fn test_rng_trait_object() {
    use rand::distributions::{Distribution, Standard};
    let mut rng = rng(109);
    let mut r = &mut rng as &mut RngCore;
    r.next_u32();
    r.gen::<i32>();
    assert_eq!(r.gen_range(0, 1), 0);
    let _c: u8 = Standard.sample(&mut r);
}

//#[test]
//#[cfg(feature="alloc")]
pub fn test_rng_boxed_trait() {
    use rand::distributions::{Distribution, Standard};
    let rng = rng(110);
    let mut r = Box::new(rng) as Box<RngCore>;
    r.next_u32();
    r.gen::<i32>();
    assert_eq!(r.gen_range(0, 1), 0);
    let _c: u8 = Standard.sample(&mut r);
}

//#[test]
//#[cfg(feature="std")]
//pub fn test_random() {
//    // not sure how to test this aside from just getting some values
//    let _n : usize = random();
//    let _f : f32 = random();
//    let _o : Option<Option<i8>> = random();
//    let _many : ((),
//                 (usize,
//                  isize,
//                  Option<(u32, (bool,))>),
//                 (u8, i8, u16, i16, u32, i32, u64, i64),
//                 (f32, (f64, (f64,)))) = random();
//}

//#[test]
pub fn test_gen_ratio_average() {
    const NUM: u32 = 3;
    const DENOM: u32 = 10;
    const N: u32 = 100_000;

    let mut sum: u32 = 0;
    let mut rng = rng(111);
    for _ in 0..N {
        if rng.gen_ratio(NUM, DENOM) {
            sum += 1;
        }
    }
    // Have Binomial(N, NUM/DENOM) distribution
    let expected = (NUM * N) / DENOM;   // exact integer
    assert!(((sum - expected) as i32).abs() < 500);
}
