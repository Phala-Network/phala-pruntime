// Copyright (C) 2017-2018 Baidu, Inc. All Rights Reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
//  * Redistributions of source code must retain the above copyright
//    notice, this list of conditions and the following disclaimer.
//  * Redistributions in binary form must reproduce the above copyright
//    notice, this list of conditions and the following disclaimer in
//    the documentation and/or other materials provided with the
//    distribution.
//  * Neither the name of Baidu, Inc., nor the names of its
//    contributors may be used to endorse or promote products derived
//    from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

#![crate_name = "helloworldsampleenclave"]
#![crate_type = "staticlib"]

#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

#[cfg(target_env = "sgx")]
extern crate core;

extern crate sgx_types;
#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;
extern crate sgx_tunittest;

use sgx_types::*;
use std::string::String;
use std::vec::Vec;
use std::io::{self, Write};
use std::{panic, slice};
use sgx_tunittest::*;

extern crate num_integer;
extern crate num_traits as traits;
mod tests;

#[no_mangle]
pub extern "C" fn say_something(some_string: *const u8, some_len: usize) -> sgx_status_t {

    let str_slice = unsafe { slice::from_raw_parts(some_string, some_len) };
    let _ = io::stdout().write(str_slice);

    // A sample &'static string
    let rust_raw_string = "This is a in-Enclave ";
    // An array
    let word:[u8;4] = [82, 117, 115, 116];
    // An vector
    let word_vec:Vec<u8> = vec![32, 115, 116, 114, 105, 110, 103, 33];

    // Construct a string from &'static string
    let mut hello_string = String::from(rust_raw_string);

    // Iterate on word array
    for c in word.iter() {
        hello_string.push(*c as char);
    }

    // Rust style convertion
    hello_string += String::from_utf8(word_vec).expect("Invalid UTF-8")
                                               .as_str();

    // Ocall to normal world for output
    println!("{}", &hello_string);

    rsgx_unit_tests!(
tests::test_integer_i8::test_div_rem,
tests::test_integer_i8::test_div_mod_floor,
tests::test_integer_i8::test_gcd,
tests::test_integer_i8::test_gcd_cmp_with_euclidean,
tests::test_integer_i8::test_gcd_min_val,
|| should_panic!(tests::test_integer_i8::test_gcd_min_val_min_val()),
|| should_panic!(tests::test_integer_i8::test_gcd_min_val_0()),
|| should_panic!(tests::test_integer_i8::test_gcd_0_min_val()),
tests::test_integer_i8::test_lcm,
tests::test_integer_i8::test_gcd_lcm,
tests::test_integer_i8::test_extended_gcd_lcm,
tests::test_integer_i8::test_even,
tests::test_integer_i8::test_odd,
tests::test_integer_i16::test_div_rem,
tests::test_integer_i16::test_div_mod_floor,
tests::test_integer_i16::test_gcd,
tests::test_integer_i16::test_gcd_cmp_with_euclidean,
tests::test_integer_i16::test_gcd_min_val,
|| should_panic!(tests::test_integer_i16::test_gcd_min_val_min_val()),
|| should_panic!(tests::test_integer_i16::test_gcd_min_val_0()),
|| should_panic!(tests::test_integer_i16::test_gcd_0_min_val()),
tests::test_integer_i16::test_lcm,
tests::test_integer_i16::test_gcd_lcm,
tests::test_integer_i16::test_extended_gcd_lcm,
tests::test_integer_i16::test_even,
tests::test_integer_i16::test_odd,
tests::test_integer_i32::test_div_rem,
tests::test_integer_i32::test_div_mod_floor,
tests::test_integer_i32::test_gcd,
tests::test_integer_i32::test_gcd_cmp_with_euclidean,
tests::test_integer_i32::test_gcd_min_val,
|| should_panic!(tests::test_integer_i32::test_gcd_min_val_min_val()),
|| should_panic!(tests::test_integer_i32::test_gcd_min_val_0()),
|| should_panic!(tests::test_integer_i32::test_gcd_0_min_val()),
tests::test_integer_i32::test_lcm,
tests::test_integer_i32::test_gcd_lcm,
tests::test_integer_i32::test_extended_gcd_lcm,
tests::test_integer_i32::test_even,
tests::test_integer_i32::test_odd,
tests::test_integer_i64::test_div_rem,
tests::test_integer_i64::test_div_mod_floor,
tests::test_integer_i64::test_gcd,
tests::test_integer_i64::test_gcd_cmp_with_euclidean,
tests::test_integer_i64::test_gcd_min_val,
|| should_panic!(tests::test_integer_i64::test_gcd_min_val_min_val()),
|| should_panic!(tests::test_integer_i64::test_gcd_min_val_0()),
|| should_panic!(tests::test_integer_i64::test_gcd_0_min_val()),
tests::test_integer_i64::test_lcm,
tests::test_integer_i64::test_gcd_lcm,
tests::test_integer_i64::test_extended_gcd_lcm,
tests::test_integer_i64::test_even,
tests::test_integer_i64::test_odd,
tests::test_integer_isize::test_div_rem,
tests::test_integer_isize::test_div_mod_floor,
tests::test_integer_isize::test_gcd,
tests::test_integer_isize::test_gcd_cmp_with_euclidean,
tests::test_integer_isize::test_gcd_min_val,
|| should_panic!(tests::test_integer_isize::test_gcd_min_val_min_val()),
|| should_panic!(tests::test_integer_isize::test_gcd_min_val_0()),
|| should_panic!(tests::test_integer_isize::test_gcd_0_min_val()),
tests::test_integer_isize::test_lcm,
tests::test_integer_isize::test_gcd_lcm,
tests::test_integer_isize::test_extended_gcd_lcm,
tests::test_integer_isize::test_even,
tests::test_integer_isize::test_odd,
tests::test_integer_i128::test_div_rem,
tests::test_integer_i128::test_div_mod_floor,
tests::test_integer_i128::test_gcd,
tests::test_integer_i128::test_gcd_cmp_with_euclidean,
tests::test_integer_i128::test_gcd_min_val,
|| should_panic!(tests::test_integer_i128::test_gcd_min_val_min_val()),
|| should_panic!(tests::test_integer_i128::test_gcd_min_val_0()),
|| should_panic!(tests::test_integer_i128::test_gcd_0_min_val()),
tests::test_integer_i128::test_lcm,
tests::test_integer_i128::test_gcd_lcm,
tests::test_integer_i128::test_extended_gcd_lcm,
tests::test_integer_i128::test_even,
tests::test_integer_i128::test_odd,
tests::test_integer_u8::test_div_mod_floor,
tests::test_integer_u8::test_gcd,
tests::test_integer_u8::test_gcd_cmp_with_euclidean,
tests::test_integer_u8::test_lcm,
tests::test_integer_u8::test_gcd_lcm,
tests::test_integer_u8::test_gcd_lcm,
tests::test_integer_u8::test_is_multiple_of,
tests::test_integer_u8::test_even,
tests::test_integer_u8::test_odd,
tests::test_integer_u16::test_div_mod_floor,
tests::test_integer_u16::test_gcd,
tests::test_integer_u16::test_gcd_cmp_with_euclidean,
tests::test_integer_u16::test_lcm,
tests::test_integer_u16::test_gcd_lcm,
tests::test_integer_u16::test_gcd_lcm,
tests::test_integer_u16::test_is_multiple_of,
tests::test_integer_u16::test_even,
tests::test_integer_u16::test_odd,
tests::test_integer_u32::test_div_mod_floor,
tests::test_integer_u32::test_gcd,
tests::test_integer_u32::test_gcd_cmp_with_euclidean,
tests::test_integer_u32::test_lcm,
tests::test_integer_u32::test_gcd_lcm,
tests::test_integer_u32::test_gcd_lcm,
tests::test_integer_u32::test_is_multiple_of,
tests::test_integer_u32::test_even,
tests::test_integer_u32::test_odd,
tests::test_integer_u64::test_div_mod_floor,
tests::test_integer_u64::test_gcd,
tests::test_integer_u64::test_gcd_cmp_with_euclidean,
tests::test_integer_u64::test_lcm,
tests::test_integer_u64::test_gcd_lcm,
tests::test_integer_u64::test_gcd_lcm,
tests::test_integer_u64::test_is_multiple_of,
tests::test_integer_u64::test_even,
tests::test_integer_u64::test_odd,
tests::test_integer_usize::test_div_mod_floor,
tests::test_integer_usize::test_gcd,
tests::test_integer_usize::test_gcd_cmp_with_euclidean,
tests::test_integer_usize::test_lcm,
tests::test_integer_usize::test_gcd_lcm,
tests::test_integer_usize::test_gcd_lcm,
tests::test_integer_usize::test_is_multiple_of,
tests::test_integer_usize::test_even,
tests::test_integer_usize::test_odd,
tests::test_integer_u128::test_div_mod_floor,
tests::test_integer_u128::test_gcd,
tests::test_integer_u128::test_gcd_cmp_with_euclidean,
tests::test_integer_u128::test_lcm,
tests::test_integer_u128::test_gcd_lcm,
tests::test_integer_u128::test_gcd_lcm,
tests::test_integer_u128::test_is_multiple_of,
tests::test_integer_u128::test_even,
tests::test_integer_u128::test_odd,
tests::test_lcm_overflow,
tests::test_iter_binomial,
tests::test_binomial,
tests::test_multinomial,
);

    sgx_status_t::SGX_SUCCESS
}
