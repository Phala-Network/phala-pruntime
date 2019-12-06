use quickcheck::{QuickCheck, StdGen, Testable};
use rand::thread_rng;
//#[cfg(byteorder_i128)]
use rand::Rng;
//#[cfg(byteorder_i128)]
use quickcheck::{Arbitrary, Gen};

pub const U24_MAX: u32 = 16_777_215;
pub const I24_MAX: i32 = 8_388_607;
pub const U48_MAX: u64 = 281_474_976_710_655;
pub const I48_MAX: i64 = 140_737_488_355_327;

#[cfg(target_env = "sgx")]
extern crate core;

pub const U64_MAX: u64 = core::u64::MAX;
pub const I64_MAX: u64 = core::i64::MAX as u64;

macro_rules! calc_max {
    ($max:expr, $bytes:expr) => { calc_max!($max, $bytes, 8) };
    ($max:expr, $bytes:expr, $maxbytes:expr) => {
        ($max - 1) >> (8 * ($maxbytes - $bytes))
    };
}

#[derive(Clone, Debug)]
pub struct Wi128<T>(pub T);

//#[cfg(byteorder_i128)]
impl<T: Clone> Wi128<T> {
    pub fn clone(&self) -> T {
        self.0.clone()
    }
}

impl<T: PartialEq> PartialEq<T> for Wi128<T> {
    fn eq(&self, other: &T) -> bool {
        self.0.eq(other)
    }
}

//#[cfg(byteorder_i128)]
impl Arbitrary for Wi128<u128> {
    fn arbitrary<G: Gen>(gen: &mut G) -> Wi128<u128> {
        let max = calc_max!(core::u128::MAX, gen.size(), 16);
        let output =
            (gen.gen::<u64>() as u128) |
            ((gen.gen::<u64>() as u128) << 64);
        Wi128(output & (max - 1))
    }
}

//#[cfg(byteorder_i128)]
impl Arbitrary for Wi128<i128> {
    fn arbitrary<G: Gen>(gen: &mut G) -> Wi128<i128> {
        let max = calc_max!(core::i128::MAX, gen.size(), 16);
        let output =
            (gen.gen::<i64>() as i128) |
            ((gen.gen::<i64>() as i128) << 64);
        Wi128(output & (max - 1))
    }
}

pub fn qc_sized<A: Testable>(f: A, size: u64) {
    QuickCheck::new()
        .gen(StdGen::new(thread_rng(), size as usize))
        .tests(1_00)
        .max_tests(10_000)
        .quickcheck(f);
}

macro_rules! qc_byte_order {
    ($name:ident, $ty_int:ty, $max:expr,
     $bytes:expr, $read:ident, $write:ident) => (
        pub mod $name {
            use byteorder::{BigEndian, ByteOrder, NativeEndian, LittleEndian};
            #[allow(unused_imports)] use super::{ qc_sized, Wi128 };

            //#[test]
            pub fn big_endian() {
                fn prop(n: $ty_int) -> bool {
                    let mut buf = [0; 16];
                    BigEndian::$write(&mut buf, n.clone(), $bytes);
                    n == BigEndian::$read(&mut buf[..$bytes], $bytes)
                }
                qc_sized(prop as fn($ty_int) -> bool, $max);
            }

            //#[test]
            pub fn little_endian() {
                fn prop(n: $ty_int) -> bool {
                    let mut buf = [0; 16];
                    LittleEndian::$write(&mut buf, n.clone(), $bytes);
                    n == LittleEndian::$read(&mut buf[..$bytes], $bytes)
                }
                qc_sized(prop as fn($ty_int) -> bool, $max);
            }

            //#[test]
            pub fn native_endian() {
                fn prop(n: $ty_int) -> bool {
                    let mut buf = [0; 16];
                    NativeEndian::$write(&mut buf, n.clone(), $bytes);
                    n == NativeEndian::$read(&mut buf[..$bytes], $bytes)
                }
                qc_sized(prop as fn($ty_int) -> bool, $max);
            }
        }
    );
    ($name:ident, $ty_int:ty, $max:expr,
     $read:ident, $write:ident) => (
        pub mod $name {
            use std::mem::size_of;
            use byteorder::{BigEndian, ByteOrder, NativeEndian, LittleEndian};
            #[allow(unused_imports)] use super::{ qc_sized, Wi128 };

            //#[test]
            pub fn big_endian() {
                fn prop(n: $ty_int) -> bool {
                    let bytes = size_of::<$ty_int>();
                    let mut buf = [0; 16];
                    BigEndian::$write(&mut buf[16 - bytes..], n.clone());
                    n == BigEndian::$read(&mut buf[16 - bytes..])
                }
                qc_sized(prop as fn($ty_int) -> bool, $max - 1);
            }

            //#[test]
            pub fn little_endian() {
                fn prop(n: $ty_int) -> bool {
                    let bytes = size_of::<$ty_int>();
                    let mut buf = [0; 16];
                    LittleEndian::$write(&mut buf[..bytes], n.clone());
                    n == LittleEndian::$read(&mut buf[..bytes])
                }
                qc_sized(prop as fn($ty_int) -> bool, $max - 1);
            }

            //#[test]
            pub fn native_endian() {
                fn prop(n: $ty_int) -> bool {
                    let bytes = size_of::<$ty_int>();
                    let mut buf = [0; 16];
                    NativeEndian::$write(&mut buf[..bytes], n.clone());
                    n == NativeEndian::$read(&mut buf[..bytes])
                }
                qc_sized(prop as fn($ty_int) -> bool, $max - 1);
            }
        }
    );
}

qc_byte_order!(prop_u16, u16, core::u16::MAX as u64, read_u16, write_u16);
qc_byte_order!(prop_i16, i16, core::i16::MAX as u64, read_i16, write_i16);
qc_byte_order!(prop_u24, u32, super::U24_MAX as u64, read_u24, write_u24);
qc_byte_order!(prop_i24, i32, super::I24_MAX as u64, read_i24, write_i24);
qc_byte_order!(prop_u32, u32, core::u32::MAX as u64, read_u32, write_u32);
qc_byte_order!(prop_i32, i32, core::i32::MAX as u64, read_i32, write_i32);
qc_byte_order!(prop_u48, u64, super::U48_MAX as u64, read_u48, write_u48);
qc_byte_order!(prop_i48, i64, super::I48_MAX as u64, read_i48, write_i48);
qc_byte_order!(prop_u64, u64, core::u64::MAX as u64, read_u64, write_u64);
qc_byte_order!(prop_i64, i64, core::i64::MAX as u64, read_i64, write_i64);
qc_byte_order!(prop_f32, f32, core::u64::MAX as u64, read_f32, write_f32);
qc_byte_order!(prop_f64, f64, core::i64::MAX as u64, read_f64, write_f64);

//#[cfg(byteorder_i128)]
qc_byte_order!(prop_u128, Wi128<u128>, 16 + 1, read_u128, write_u128);
//#[cfg(byteorder_i128)]
qc_byte_order!(prop_i128, Wi128<i128>, 16 + 1, read_i128, write_i128);

qc_byte_order!(prop_uint_1,
    u64, calc_max!(super::U64_MAX, 1), 1, read_uint, write_uint);
qc_byte_order!(prop_uint_2,
    u64, calc_max!(super::U64_MAX, 2), 2, read_uint, write_uint);
qc_byte_order!(prop_uint_3,
    u64, calc_max!(super::U64_MAX, 3), 3, read_uint, write_uint);
qc_byte_order!(prop_uint_4,
    u64, calc_max!(super::U64_MAX, 4), 4, read_uint, write_uint);
qc_byte_order!(prop_uint_5,
    u64, calc_max!(super::U64_MAX, 5), 5, read_uint, write_uint);
qc_byte_order!(prop_uint_6,
    u64, calc_max!(super::U64_MAX, 6), 6, read_uint, write_uint);
qc_byte_order!(prop_uint_7,
    u64, calc_max!(super::U64_MAX, 7), 7, read_uint, write_uint);
qc_byte_order!(prop_uint_8,
    u64, calc_max!(super::U64_MAX, 8), 8, read_uint, write_uint);

//#[cfg(byteorder_i128)]
qc_byte_order!(prop_uint128_1,
    Wi128<u128>, 1, 1, read_uint128, write_uint128);
//#[cfg(byteorder_i128)]
qc_byte_order!(prop_uint128_2,
    Wi128<u128>, 2, 2, read_uint128, write_uint128);
//#[cfg(byteorder_i128)]
qc_byte_order!(prop_uint128_3,
    Wi128<u128>, 3, 3, read_uint128, write_uint128);
//#[cfg(byteorder_i128)]
qc_byte_order!(prop_uint128_4,
    Wi128<u128>, 4, 4, read_uint128, write_uint128);
//#[cfg(byteorder_i128)]
qc_byte_order!(prop_uint128_5,
    Wi128<u128>, 5, 5, read_uint128, write_uint128);
//#[cfg(byteorder_i128)]
qc_byte_order!(prop_uint128_6,
    Wi128<u128>, 6, 6, read_uint128, write_uint128);
//#[cfg(byteorder_i128)]
qc_byte_order!(prop_uint128_7,
    Wi128<u128>, 7, 7, read_uint128, write_uint128);
//#[cfg(byteorder_i128)]
qc_byte_order!(prop_uint128_8,
    Wi128<u128>, 8, 8, read_uint128, write_uint128);
//#[cfg(byteorder_i128)]
qc_byte_order!(prop_uint128_9,
    Wi128<u128>, 9, 9, read_uint128, write_uint128);
//#[cfg(byteorder_i128)]
qc_byte_order!(prop_uint128_10,
    Wi128<u128>, 10, 10, read_uint128, write_uint128);
//#[cfg(byteorder_i128)]
qc_byte_order!(prop_uint128_11,
    Wi128<u128>, 11, 11, read_uint128, write_uint128);
//#[cfg(byteorder_i128)]
qc_byte_order!(prop_uint128_12,
    Wi128<u128>, 12, 12, read_uint128, write_uint128);
//#[cfg(byteorder_i128)]
qc_byte_order!(prop_uint128_13,
    Wi128<u128>, 13, 13, read_uint128, write_uint128);
//#[cfg(byteorder_i128)]
qc_byte_order!(prop_uint128_14,
    Wi128<u128>, 14, 14, read_uint128, write_uint128);
//#[cfg(byteorder_i128)]
qc_byte_order!(prop_uint128_15,
    Wi128<u128>, 15, 15, read_uint128, write_uint128);
//#[cfg(byteorder_i128)]
qc_byte_order!(prop_uint128_16,
    Wi128<u128>, 16, 16, read_uint128, write_uint128);

qc_byte_order!(prop_int_1,
    i64, calc_max!(super::I64_MAX, 1), 1, read_int, write_int);
qc_byte_order!(prop_int_2,
    i64, calc_max!(super::I64_MAX, 2), 2, read_int, write_int);
qc_byte_order!(prop_int_3,
    i64, calc_max!(super::I64_MAX, 3), 3, read_int, write_int);
qc_byte_order!(prop_int_4,
    i64, calc_max!(super::I64_MAX, 4), 4, read_int, write_int);
qc_byte_order!(prop_int_5,
    i64, calc_max!(super::I64_MAX, 5), 5, read_int, write_int);
qc_byte_order!(prop_int_6,
    i64, calc_max!(super::I64_MAX, 6), 6, read_int, write_int);
qc_byte_order!(prop_int_7,
    i64, calc_max!(super::I64_MAX, 7), 7, read_int, write_int);
qc_byte_order!(prop_int_8,
    i64, calc_max!(super::I64_MAX, 8), 8, read_int, write_int);

//#[cfg(byteorder_i128)]
qc_byte_order!(prop_int128_1,
    Wi128<i128>, 1, 1, read_int128, write_int128);
//#[cfg(byteorder_i128)]
qc_byte_order!(prop_int128_2,
    Wi128<i128>, 2, 2, read_int128, write_int128);
//#[cfg(byteorder_i128)]
qc_byte_order!(prop_int128_3,
    Wi128<i128>, 3, 3, read_int128, write_int128);
//#[cfg(byteorder_i128)]
qc_byte_order!(prop_int128_4,
    Wi128<i128>, 4, 4, read_int128, write_int128);
//#[cfg(byteorder_i128)]
qc_byte_order!(prop_int128_5,
    Wi128<i128>, 5, 5, read_int128, write_int128);
//#[cfg(byteorder_i128)]
qc_byte_order!(prop_int128_6,
    Wi128<i128>, 6, 6, read_int128, write_int128);
//#[cfg(byteorder_i128)]
qc_byte_order!(prop_int128_7,
    Wi128<i128>, 7, 7, read_int128, write_int128);
//#[cfg(byteorder_i128)]
qc_byte_order!(prop_int128_8,
    Wi128<i128>, 8, 8, read_int128, write_int128);
//#[cfg(byteorder_i128)]
qc_byte_order!(prop_int128_9,
    Wi128<i128>, 9, 9, read_int128, write_int128);
//#[cfg(byteorder_i128)]
qc_byte_order!(prop_int128_10,
    Wi128<i128>, 10, 10, read_int128, write_int128);
//#[cfg(byteorder_i128)]
qc_byte_order!(prop_int128_11,
    Wi128<i128>, 11, 11, read_int128, write_int128);
//#[cfg(byteorder_i128)]
qc_byte_order!(prop_int128_12,
    Wi128<i128>, 12, 12, read_int128, write_int128);
//#[cfg(byteorder_i128)]
qc_byte_order!(prop_int128_13,
    Wi128<i128>, 13, 13, read_int128, write_int128);
//#[cfg(byteorder_i128)]
qc_byte_order!(prop_int128_14,
    Wi128<i128>, 14, 14, read_int128, write_int128);
//#[cfg(byteorder_i128)]
qc_byte_order!(prop_int128_15,
    Wi128<i128>, 15, 15, read_int128, write_int128);
//#[cfg(byteorder_i128)]
qc_byte_order!(prop_int128_16,
    Wi128<i128>, 16, 16, read_int128, write_int128);


// Test that all of the byte conversion functions panic when given a
// buffer that is too small.
//
// These tests are critical to ensure safety, otherwise we might end up
// with a buffer overflow.
macro_rules! too_small {
    ($name:ident, $maximally_small:expr, $zero:expr,
     $read:ident, $write:ident) => (
        pub mod $name {
            use byteorder::{BigEndian, ByteOrder, NativeEndian, LittleEndian};

            //#[test]
            //#[should_panic]
            pub fn read_big_endian() {
                let buf = [0; $maximally_small];
                BigEndian::$read(&buf);
            }

            //#[test]
            //#[should_panic]
            pub fn read_little_endian() {
                let buf = [0; $maximally_small];
                LittleEndian::$read(&buf);
            }

            //#[test]
            //#[should_panic]
            pub fn read_native_endian() {
                let buf = [0; $maximally_small];
                NativeEndian::$read(&buf);
            }

            //#[test]
            //#[should_panic]
            pub fn write_big_endian() {
                let mut buf = [0; $maximally_small];
                BigEndian::$write(&mut buf, $zero);
            }

            //#[test]
            //#[should_panic]
            pub fn write_little_endian() {
                let mut buf = [0; $maximally_small];
                LittleEndian::$write(&mut buf, $zero);
            }

            //#[test]
            //#[should_panic]
            pub fn write_native_endian() {
                let mut buf = [0; $maximally_small];
                NativeEndian::$write(&mut buf, $zero);
            }
        }
    );
    ($name:ident, $maximally_small:expr, $read:ident) => (
        pub mod $name {
            use byteorder::{BigEndian, ByteOrder, NativeEndian, LittleEndian};

            //#[test]
            //#[should_panic]
            pub fn read_big_endian() {
                let buf = [0; $maximally_small];
                BigEndian::$read(&buf, $maximally_small + 1);
            }

            //#[test]
            //#[should_panic]
            pub fn read_little_endian() {
                let buf = [0; $maximally_small];
                LittleEndian::$read(&buf, $maximally_small + 1);
            }

            //#[test]
            //#[should_panic]
            pub fn read_native_endian() {
                let buf = [0; $maximally_small];
                NativeEndian::$read(&buf, $maximally_small + 1);
            }
        }
    );
}

too_small!(small_u16, 1, 0, read_u16, write_u16);
too_small!(small_i16, 1, 0, read_i16, write_i16);
too_small!(small_u32, 3, 0, read_u32, write_u32);
too_small!(small_i32, 3, 0, read_i32, write_i32);
too_small!(small_u64, 7, 0, read_u64, write_u64);
too_small!(small_i64, 7, 0, read_i64, write_i64);
too_small!(small_f32, 3, 0.0, read_f32, write_f32);
too_small!(small_f64, 7, 0.0, read_f64, write_f64);
//#[cfg(byteorder_i128)]
too_small!(small_u128, 15, 0, read_u128, write_u128);
//#[cfg(byteorder_i128)]
too_small!(small_i128, 15, 0, read_i128, write_i128);

too_small!(small_uint_1, 1, read_uint);
too_small!(small_uint_2, 2, read_uint);
too_small!(small_uint_3, 3, read_uint);
too_small!(small_uint_4, 4, read_uint);
too_small!(small_uint_5, 5, read_uint);
too_small!(small_uint_6, 6, read_uint);
too_small!(small_uint_7, 7, read_uint);

//#[cfg(byteorder_i128)]
too_small!(small_uint128_1, 1, read_uint128);
//#[cfg(byteorder_i128)]
too_small!(small_uint128_2, 2, read_uint128);
//#[cfg(byteorder_i128)]
too_small!(small_uint128_3, 3, read_uint128);
//#[cfg(byteorder_i128)]
too_small!(small_uint128_4, 4, read_uint128);
//#[cfg(byteorder_i128)]
too_small!(small_uint128_5, 5, read_uint128);
//#[cfg(byteorder_i128)]
too_small!(small_uint128_6, 6, read_uint128);
//#[cfg(byteorder_i128)]
too_small!(small_uint128_7, 7, read_uint128);
//#[cfg(byteorder_i128)]
too_small!(small_uint128_8, 8, read_uint128);
//#[cfg(byteorder_i128)]
too_small!(small_uint128_9, 9, read_uint128);
//#[cfg(byteorder_i128)]
too_small!(small_uint128_10, 10, read_uint128);
//#[cfg(byteorder_i128)]
too_small!(small_uint128_11, 11, read_uint128);
//#[cfg(byteorder_i128)]
too_small!(small_uint128_12, 12, read_uint128);
//#[cfg(byteorder_i128)]
too_small!(small_uint128_13, 13, read_uint128);
//#[cfg(byteorder_i128)]
too_small!(small_uint128_14, 14, read_uint128);
//#[cfg(byteorder_i128)]
too_small!(small_uint128_15, 15, read_uint128);

too_small!(small_int_1, 1, read_int);
too_small!(small_int_2, 2, read_int);
too_small!(small_int_3, 3, read_int);
too_small!(small_int_4, 4, read_int);
too_small!(small_int_5, 5, read_int);
too_small!(small_int_6, 6, read_int);
too_small!(small_int_7, 7, read_int);

//#[cfg(byteorder_i128)]
too_small!(small_int128_1, 1, read_int128);
//#[cfg(byteorder_i128)]
too_small!(small_int128_2, 2, read_int128);
//#[cfg(byteorder_i128)]
too_small!(small_int128_3, 3, read_int128);
//#[cfg(byteorder_i128)]
too_small!(small_int128_4, 4, read_int128);
//#[cfg(byteorder_i128)]
too_small!(small_int128_5, 5, read_int128);
//#[cfg(byteorder_i128)]
too_small!(small_int128_6, 6, read_int128);
//#[cfg(byteorder_i128)]
too_small!(small_int128_7, 7, read_int128);
//#[cfg(byteorder_i128)]
too_small!(small_int128_8, 8, read_int128);
//#[cfg(byteorder_i128)]
too_small!(small_int128_9, 9, read_int128);
//#[cfg(byteorder_i128)]
too_small!(small_int128_10, 10, read_int128);
//#[cfg(byteorder_i128)]
too_small!(small_int128_11, 11, read_int128);
//#[cfg(byteorder_i128)]
too_small!(small_int128_12, 12, read_int128);
//#[cfg(byteorder_i128)]
too_small!(small_int128_13, 13, read_int128);
//#[cfg(byteorder_i128)]
too_small!(small_int128_14, 14, read_int128);
//#[cfg(byteorder_i128)]
too_small!(small_int128_15, 15, read_int128);

// Test that reading/writing slices enforces the correct lengths.
macro_rules! slice_lengths {
    ($name:ident, $read:ident, $write:ident,
     $num_bytes:expr, $numbers:expr) => {
        pub mod $name {
            use byteorder::{ByteOrder, BigEndian, NativeEndian, LittleEndian};

            //#[test]
            //#[should_panic]
            pub fn read_big_endian() {
                let bytes = [0; $num_bytes];
                let mut numbers = $numbers;
                BigEndian::$read(&bytes, &mut numbers);
            }

            //#[test]
            //#[should_panic]
            pub fn read_little_endian() {
                let bytes = [0; $num_bytes];
                let mut numbers = $numbers;
                LittleEndian::$read(&bytes, &mut numbers);
            }

            //#[test]
            //#[should_panic]
            pub fn read_native_endian() {
                let bytes = [0; $num_bytes];
                let mut numbers = $numbers;
                NativeEndian::$read(&bytes, &mut numbers);
            }

            //#[test]
            //#[should_panic]
            pub fn write_big_endian() {
                let mut bytes = [0; $num_bytes];
                let numbers = $numbers;
                BigEndian::$write(&numbers, &mut bytes);
            }

            //#[test]
            //#[should_panic]
            pub fn write_little_endian() {
                let mut bytes = [0; $num_bytes];
                let numbers = $numbers;
                LittleEndian::$write(&numbers, &mut bytes);
            }

            //#[test]
            //#[should_panic]
            pub fn write_native_endian() {
                let mut bytes = [0; $num_bytes];
                let numbers = $numbers;
                NativeEndian::$write(&numbers, &mut bytes);
            }
        }
    }
}

slice_lengths!(
    slice_len_too_small_u16, read_u16_into, write_u16_into, 3, [0, 0]);
slice_lengths!(
    slice_len_too_big_u16, read_u16_into, write_u16_into, 5, [0, 0]);
slice_lengths!(
    slice_len_too_small_i16, read_i16_into, write_i16_into, 3, [0, 0]);
slice_lengths!(
    slice_len_too_big_i16, read_i16_into, write_i16_into, 5, [0, 0]);

slice_lengths!(
    slice_len_too_small_u32, read_u32_into, write_u32_into, 7, [0, 0]);
slice_lengths!(
    slice_len_too_big_u32, read_u32_into, write_u32_into, 9, [0, 0]);
slice_lengths!(
    slice_len_too_small_i32, read_i32_into, write_i32_into, 7, [0, 0]);
slice_lengths!(
    slice_len_too_big_i32, read_i32_into, write_i32_into, 9, [0, 0]);

slice_lengths!(
    slice_len_too_small_u64, read_u64_into, write_u64_into, 15, [0, 0]);
slice_lengths!(
    slice_len_too_big_u64, read_u64_into, write_u64_into, 17, [0, 0]);
slice_lengths!(
    slice_len_too_small_i64, read_i64_into, write_i64_into, 15, [0, 0]);
slice_lengths!(
    slice_len_too_big_i64, read_i64_into, write_i64_into, 17, [0, 0]);

//#[cfg(byteorder_i128)]
slice_lengths!(
    slice_len_too_small_u128, read_u128_into, write_u128_into, 31, [0, 0]);
//#[cfg(byteorder_i128)]
slice_lengths!(
    slice_len_too_big_u128, read_u128_into, write_u128_into, 33, [0, 0]);
//#[cfg(byteorder_i128)]
slice_lengths!(
    slice_len_too_small_i128, read_i128_into, write_i128_into, 31, [0, 0]);
//#[cfg(byteorder_i128)]
slice_lengths!(
    slice_len_too_big_i128, read_i128_into, write_i128_into, 33, [0, 0]);

//#[test]
pub fn uint_bigger_buffer() {
    use byteorder::{ByteOrder, LittleEndian};
    let n = LittleEndian::read_uint(&[1, 2, 3, 4, 5, 6, 7, 8], 5);
    assert_eq!(n, 0x0504030201);
}
