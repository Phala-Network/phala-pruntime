//extern crate quickcheck;
//extern crate rand;

use quickcheck::{QuickCheck, StdGen, Testable};
use rand::thread_rng;

#[cfg(target_env = "sgx")]
extern crate core;

fn qc_unsized<A: Testable>(f: A) {

    QuickCheck::new()
        .gen(StdGen::new(thread_rng(), 16))
        .tests(1_00)
        .max_tests(10_000)
        .quickcheck(f);
}

macro_rules! calc_max {
    ($max:expr, $bytes:expr) => { ($max - 1) >> (8 * (8 - $bytes)) };
}

macro_rules! qc_bytes_ext {
    ($name:ident, $ty_int:ty, $max:expr,
     $bytes:expr, $read:ident, $write:ident) => (
        pub mod $name {
            use std::prelude::v1::*;
            use std::io::Cursor;
            use byteorder::{
                ReadBytesExt, WriteBytesExt,
                BigEndian, NativeEndian, LittleEndian,
            };
            #[allow(unused_imports)] use crate::tests::{qc_sized, Wi128};

            //#[test]
            pub fn big_endian() {
                fn prop(n: $ty_int) -> bool {
                    let mut wtr = vec![];
                    wtr.$write::<BigEndian>(n.clone()).unwrap();
                    let offset = wtr.len() - $bytes;
                    let mut rdr = Cursor::new(&mut wtr[offset..]);
                    n == rdr.$read::<BigEndian>($bytes).unwrap()
                }
                qc_sized(prop as fn($ty_int) -> bool, $max);
            }

            //#[test]
            pub fn little_endian() {
                fn prop(n: $ty_int) -> bool {
                    let mut wtr = vec![];
                    wtr.$write::<LittleEndian>(n.clone()).unwrap();
                    let mut rdr = Cursor::new(wtr);
                    n == rdr.$read::<LittleEndian>($bytes).unwrap()
                }
                qc_sized(prop as fn($ty_int) -> bool, $max);
            }

            //#[test]
            pub fn native_endian() {
                fn prop(n: $ty_int) -> bool {
                    let mut wtr = vec![];
                    wtr.$write::<NativeEndian>(n.clone()).unwrap();
                    let offset = if cfg!(target_endian = "big") {
                        wtr.len() - $bytes
                    } else {
                        0
                    };
                    let mut rdr = Cursor::new(&mut wtr[offset..]);
                    n == rdr.$read::<NativeEndian>($bytes).unwrap()
                }
                qc_sized(prop as fn($ty_int) -> bool, $max);
            }
        }
    );
    ($name:ident, $ty_int:ty, $max:expr, $read:ident, $write:ident) => (
        pub mod $name {
            use std::io::Cursor;
            use byteorder::{
                ReadBytesExt, WriteBytesExt,
                BigEndian, NativeEndian, LittleEndian,
            };
            #[allow(unused_imports)] use tests::{qc_sized, Wi128};

            //#[test]
            pub fn big_endian() {
                fn prop(n: $ty_int) -> bool {
                    let mut wtr = vec![];
                    wtr.$write::<BigEndian>(n.clone()).unwrap();
                    let mut rdr = Cursor::new(wtr);
                    n == rdr.$read::<BigEndian>().unwrap()
                }
                qc_sized(prop as fn($ty_int) -> bool, $max - 1);
            }

            //#[test]
            pub fn little_endian() {
                fn prop(n: $ty_int) -> bool {
                    let mut wtr = vec![];
                    wtr.$write::<LittleEndian>(n.clone()).unwrap();
                    let mut rdr = Cursor::new(wtr);
                    n == rdr.$read::<LittleEndian>().unwrap()
                }
                qc_sized(prop as fn($ty_int) -> bool, $max - 1);
            }

            //#[test]
            pub fn native_endian() {
                fn prop(n: $ty_int) -> bool {
                    let mut wtr = vec![];
                    wtr.$write::<NativeEndian>(n.clone()).unwrap();
                    let mut rdr = Cursor::new(wtr);
                    n == rdr.$read::<NativeEndian>().unwrap()
                }
                qc_sized(prop as fn($ty_int) -> bool, $max - 1);
            }
        }
    );
}

qc_bytes_ext!(prop_ext_u16,
    u16, ::std::u16::MAX as u64, read_u16, write_u16);
qc_bytes_ext!(prop_ext_i16,
    i16, ::std::i16::MAX as u64, read_i16, write_i16);
qc_bytes_ext!(prop_ext_u32,
    u32, ::std::u32::MAX as u64, read_u32, write_u32);
qc_bytes_ext!(prop_ext_i32,
    i32, ::std::i32::MAX as u64, read_i32, write_i32);
qc_bytes_ext!(prop_ext_u64,
    u64, ::std::u64::MAX as u64, read_u64, write_u64);
qc_bytes_ext!(prop_ext_i64,
    i64, ::std::i64::MAX as u64, read_i64, write_i64);
qc_bytes_ext!(prop_ext_f32,
    f32, ::std::u64::MAX as u64, read_f32, write_f32);
qc_bytes_ext!(prop_ext_f64,
    f64, ::std::i64::MAX as u64, read_f64, write_f64);

//#[cfg(byteorder_i128)]
qc_bytes_ext!(prop_ext_u128, Wi128<u128>, 16 + 1, read_u128, write_u128);
//#[cfg(byteorder_i128)]
qc_bytes_ext!(prop_ext_i128, Wi128<i128>, 16 + 1, read_i128, write_i128);

qc_bytes_ext!(prop_ext_uint_1,
    u64, calc_max!(crate::tests::U64_MAX, 1), 1, read_uint, write_u64);
qc_bytes_ext!(prop_ext_uint_2,
    u64, calc_max!(crate::tests::U64_MAX, 2), 2, read_uint, write_u64);
qc_bytes_ext!(prop_ext_uint_3,
    u64, calc_max!(crate::tests::U64_MAX, 3), 3, read_uint, write_u64);
qc_bytes_ext!(prop_ext_uint_4,
    u64, calc_max!(crate::tests::U64_MAX, 4), 4, read_uint, write_u64);
qc_bytes_ext!(prop_ext_uint_5,
    u64, calc_max!(crate::tests::U64_MAX, 5), 5, read_uint, write_u64);
qc_bytes_ext!(prop_ext_uint_6,
    u64, calc_max!(crate::tests::U64_MAX, 6), 6, read_uint, write_u64);
qc_bytes_ext!(prop_ext_uint_7,
    u64, calc_max!(crate::tests::U64_MAX, 7), 7, read_uint, write_u64);
qc_bytes_ext!(prop_ext_uint_8,
    u64, calc_max!(crate::tests::U64_MAX, 8), 8, read_uint, write_u64);

//#[cfg(byteorder_i128)]
qc_bytes_ext!(prop_ext_uint128_1,
    Wi128<u128>, 1, 1, read_uint128, write_u128);
//#[cfg(byteorder_i128)]
qc_bytes_ext!(prop_ext_uint128_2,
    Wi128<u128>, 2, 2, read_uint128, write_u128);
//#[cfg(byteorder_i128)]
qc_bytes_ext!(prop_ext_uint128_3,
    Wi128<u128>, 3, 3, read_uint128, write_u128);
//#[cfg(byteorder_i128)]
qc_bytes_ext!(prop_ext_uint128_4,
    Wi128<u128>, 4, 4, read_uint128, write_u128);
//#[cfg(byteorder_i128)]
qc_bytes_ext!(prop_ext_uint128_5,
    Wi128<u128>, 5, 5, read_uint128, write_u128);
//#[cfg(byteorder_i128)]
qc_bytes_ext!(prop_ext_uint128_6,
    Wi128<u128>, 6, 6, read_uint128, write_u128);
//#[cfg(byteorder_i128)]
qc_bytes_ext!(prop_ext_uint128_7,
    Wi128<u128>, 7, 7, read_uint128, write_u128);
//#[cfg(byteorder_i128)]
qc_bytes_ext!(prop_ext_uint128_8,
    Wi128<u128>, 8, 8, read_uint128, write_u128);
//#[cfg(byteorder_i128)]
qc_bytes_ext!(prop_ext_uint128_9,
    Wi128<u128>, 9, 9, read_uint128, write_u128);
//#[cfg(byteorder_i128)]
qc_bytes_ext!(prop_ext_uint128_10,
    Wi128<u128>, 10, 10, read_uint128, write_u128);
//#[cfg(byteorder_i128)]
qc_bytes_ext!(prop_ext_uint128_11,
    Wi128<u128>, 11, 11, read_uint128, write_u128);
//#[cfg(byteorder_i128)]
qc_bytes_ext!(prop_ext_uint128_12,
    Wi128<u128>, 12, 12, read_uint128, write_u128);
//#[cfg(byteorder_i128)]
qc_bytes_ext!(prop_ext_uint128_13,
    Wi128<u128>, 13, 13, read_uint128, write_u128);
//#[cfg(byteorder_i128)]
qc_bytes_ext!(prop_ext_uint128_14,
    Wi128<u128>, 14, 14, read_uint128, write_u128);
//#[cfg(byteorder_i128)]
qc_bytes_ext!(prop_ext_uint128_15,
    Wi128<u128>, 15, 15, read_uint128, write_u128);
//#[cfg(byteorder_i128)]
qc_bytes_ext!(prop_ext_uint128_16,
    Wi128<u128>, 16, 16, read_uint128, write_u128);

qc_bytes_ext!(prop_ext_int_1,
    i64, calc_max!(crate::tests::I64_MAX, 1), 1, read_int, write_i64);
qc_bytes_ext!(prop_ext_int_2,
    i64, calc_max!(crate::tests::I64_MAX, 2), 2, read_int, write_i64);
qc_bytes_ext!(prop_ext_int_3,
    i64, calc_max!(crate::tests::I64_MAX, 3), 3, read_int, write_i64);
qc_bytes_ext!(prop_ext_int_4,
    i64, calc_max!(crate::tests::I64_MAX, 4), 4, read_int, write_i64);
qc_bytes_ext!(prop_ext_int_5,
    i64, calc_max!(crate::tests::I64_MAX, 5), 5, read_int, write_i64);
qc_bytes_ext!(prop_ext_int_6,
    i64, calc_max!(crate::tests::I64_MAX, 6), 6, read_int, write_i64);
qc_bytes_ext!(prop_ext_int_7,
    i64, calc_max!(crate::tests::I64_MAX, 1), 7, read_int, write_i64);
qc_bytes_ext!(prop_ext_int_8,
    i64, calc_max!(crate::tests::I64_MAX, 8), 8, read_int, write_i64);

//#[cfg(byteorder_i128)]
qc_bytes_ext!(prop_ext_int128_1,
    Wi128<i128>, 1, 1, read_int128, write_i128);
//#[cfg(byteorder_i128)]
qc_bytes_ext!(prop_ext_int128_2,
    Wi128<i128>, 2, 2, read_int128, write_i128);
//#[cfg(byteorder_i128)]
qc_bytes_ext!(prop_ext_int128_3,
    Wi128<i128>, 3, 3, read_int128, write_i128);
//#[cfg(byteorder_i128)]
qc_bytes_ext!(prop_ext_int128_4,
    Wi128<i128>, 4, 4, read_int128, write_i128);
//#[cfg(byteorder_i128)]
qc_bytes_ext!(prop_ext_int128_5,
    Wi128<i128>, 5, 5, read_int128, write_i128);
//#[cfg(byteorder_i128)]
qc_bytes_ext!(prop_ext_int128_6,
    Wi128<i128>, 6, 6, read_int128, write_i128);
//#[cfg(byteorder_i128)]
qc_bytes_ext!(prop_ext_int128_7,
    Wi128<i128>, 7, 7, read_int128, write_i128);
//#[cfg(byteorder_i128)]
qc_bytes_ext!(prop_ext_int128_8,
    Wi128<i128>, 8, 8, read_int128, write_i128);
//#[cfg(byteorder_i128)]
qc_bytes_ext!(prop_ext_int128_9,
    Wi128<i128>, 9, 9, read_int128, write_i128);
//#[cfg(byteorder_i128)]
qc_bytes_ext!(prop_ext_int128_10,
    Wi128<i128>, 10, 10, read_int128, write_i128);
//#[cfg(byteorder_i128)]
qc_bytes_ext!(prop_ext_int128_11,
    Wi128<i128>, 11, 11, read_int128, write_i128);
//#[cfg(byteorder_i128)]
qc_bytes_ext!(prop_ext_int128_12,
    Wi128<i128>, 12, 12, read_int128, write_i128);
//#[cfg(byteorder_i128)]
qc_bytes_ext!(prop_ext_int128_13,
    Wi128<i128>, 13, 13, read_int128, write_i128);
//#[cfg(byteorder_i128)]
qc_bytes_ext!(prop_ext_int128_14,
    Wi128<i128>, 14, 14, read_int128, write_i128);
//#[cfg(byteorder_i128)]
qc_bytes_ext!(prop_ext_int128_15,
    Wi128<i128>, 15, 15, read_int128, write_i128);
//#[cfg(byteorder_i128)]
qc_bytes_ext!(prop_ext_int128_16,
    Wi128<i128>, 16, 16, read_int128, write_i128);

// Test slice serialization/deserialization.
macro_rules! qc_slice {
    ($name:ident, $ty_int:ty, $read:ident, $write:ident, $zero:expr) => {
        pub mod $name {
            use std::prelude::v1::*;
            #[cfg(not(target_env = "sgx"))]
            use core::mem::size_of;
            #[cfg(target_env = "sgx")]
            use super::core::mem::size_of;
            use byteorder::{ByteOrder, BigEndian, NativeEndian, LittleEndian};
            use super::qc_unsized;
            #[allow(unused_imports)]
            use tests::Wi128;

            //#[test]
            pub fn big_endian() {
                #[allow(unused_unsafe)]
                fn prop(numbers: Vec<$ty_int>) -> bool {
                    let numbers: Vec<_> = numbers
                        .into_iter()
                        .map(|x| x.clone())
                        .collect();
                    let num_bytes = size_of::<$ty_int>() * numbers.len();
                    let mut bytes = vec![0; num_bytes];

                    BigEndian::$write(&numbers, &mut bytes);

                    let mut got = vec![$zero; numbers.len()];
                    unsafe { BigEndian::$read(&bytes, &mut got); }

                    numbers == got
                }
                qc_unsized(prop as fn(_) -> bool);
            }

            //#[test]
            pub fn little_endian() {
                #[allow(unused_unsafe)]
                fn prop(numbers: Vec<$ty_int>) -> bool {
                    let numbers: Vec<_> = numbers
                        .into_iter()
                        .map(|x| x.clone())
                        .collect();
                    let num_bytes = size_of::<$ty_int>() * numbers.len();
                    let mut bytes = vec![0; num_bytes];

                    LittleEndian::$write(&numbers, &mut bytes);

                    let mut got = vec![$zero; numbers.len()];
                    unsafe { LittleEndian::$read(&bytes, &mut got); }

                    numbers == got
                }
                qc_unsized(prop as fn(_) -> bool);
            }

            //#[test]
            pub fn native_endian() {
                #[allow(unused_unsafe)]
                fn prop(numbers: Vec<$ty_int>) -> bool {
                    let numbers: Vec<_> = numbers
                        .into_iter()
                        .map(|x| x.clone())
                        .collect();
                    let num_bytes = size_of::<$ty_int>() * numbers.len();
                    let mut bytes = vec![0; num_bytes];

                    NativeEndian::$write(&numbers, &mut bytes);

                    let mut got = vec![$zero; numbers.len()];
                    unsafe { NativeEndian::$read(&bytes, &mut got); }

                    numbers == got
                }
                qc_unsized(prop as fn(_) -> bool);
            }
        }
    }
}

qc_slice!(prop_slice_u16, u16, read_u16_into, write_u16_into, 0);
qc_slice!(prop_slice_i16, i16, read_i16_into, write_i16_into, 0);
qc_slice!(prop_slice_u32, u32, read_u32_into, write_u32_into, 0);
qc_slice!(prop_slice_i32, i32, read_i32_into, write_i32_into, 0);
qc_slice!(prop_slice_u64, u64, read_u64_into, write_u64_into, 0);
qc_slice!(prop_slice_i64, i64, read_i64_into, write_i64_into, 0);
//#[cfg(byteorder_i128)]
qc_slice!(prop_slice_u128, Wi128<u128>, read_u128_into, write_u128_into, 0);
//#[cfg(byteorder_i128)]
qc_slice!(prop_slice_i128, Wi128<i128>, read_i128_into, write_i128_into, 0);

qc_slice!(prop_slice_f32, f32, read_f32_into, write_f32_into, 0.0);
qc_slice!(prop_slice_f64, f64, read_f64_into, write_f64_into, 0.0);
