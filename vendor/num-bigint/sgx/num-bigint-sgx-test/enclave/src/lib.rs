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

extern crate sgx_types;
#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;
extern crate sgx_tunittest;

use sgx_types::*;
use std::string::String;
use std::vec::Vec;
use std::io::{self, Write};
use std::slice;
use std::panic;
use sgx_tunittest::*;

extern crate num_bigint;
extern crate num_traits;
extern crate num_integer;
extern crate serde_test;

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
tests::roots::biguint::test_sqrt,
tests::roots::biguint::test_cbrt,
tests::roots::biguint::test_nth_root,
|| should_panic!(tests::roots::biguint::test_nth_root_n_is_zero()),
tests::roots::biguint::test_nth_root_big,
tests::roots::biguint::test_nth_root_googol,
tests::roots::biguint::test_nth_root_twos,
tests::roots::biguint::test_roots_rand1,
tests::roots::bigint::test_nth_root,
|| should_panic!(tests::roots::bigint::test_nth_root_x_neg_n_even()),
|| should_panic!(tests::roots::bigint::test_sqrt_x_neg()),
tests::roots::bigint::test_cbrt,
tests::bigint_bitwise::test_not,
tests::bigint_bitwise::test_not_i64,
tests::bigint_bitwise::test_bitwise,
tests::bigint_bitwise::test_bitwise_i64,
tests::biguint_scalar::test_scalar_add,
tests::biguint_scalar::test_scalar_sub,
tests::biguint_scalar::test_scalar_mul,
tests::biguint_scalar::test_scalar_div_rem,
tests::bigint::test_from_bytes_be,
tests::bigint::test_to_bytes_be,
tests::bigint::test_from_bytes_le,
tests::bigint::test_to_bytes_le,
tests::bigint::test_to_signed_bytes_le,
tests::bigint::test_from_signed_bytes_le,
tests::bigint::test_to_signed_bytes_be,
tests::bigint::test_from_signed_bytes_be,
tests::bigint::test_signed_bytes_be_round_trip,
tests::bigint::test_signed_bytes_le_round_trip,
tests::bigint::test_cmp,
tests::bigint::test_hash,
tests::bigint::test_convert_i64,
tests::bigint::test_convert_u64,
tests::bigint::test_convert_f32,
tests::bigint::test_convert_f64,
tests::bigint::test_convert_to_biguint,
tests::bigint::test_convert_from_uint,
tests::bigint::test_convert_from_int,
tests::bigint::test_convert_from_biguint,
tests::bigint::test_add,
tests::bigint::test_sub,
tests::bigint::test_mul,
tests::bigint::test_div_mod_floor,
tests::bigint::test_div_rem,
tests::bigint::test_checked_add,
tests::bigint::test_checked_sub,
tests::bigint::test_checked_mul,
tests::bigint::test_checked_div,
tests::bigint::test_gcd,
tests::bigint::test_lcm,
tests::bigint::test_abs_sub,
tests::bigint::test_from_str_radix,
tests::bigint::test_lower_hex,
tests::bigint::test_upper_hex,
tests::bigint::test_binary,
tests::bigint::test_octal,
tests::bigint::test_display,
tests::bigint::test_neg,
tests::bigint::test_negative_shr,
tests::bigint::test_iter_sum,
tests::bigint::test_iter_product,
tests::bigint::test_iter_sum_generic,
tests::bigint::test_iter_product_generic,
tests::bigint::test_pow,
tests::bigint::test_convert_i128,
tests::bigint::test_convert_u128,
tests::bigint_scalar::test_scalar_add,
tests::bigint_scalar::test_scalar_sub,
tests::bigint_scalar::test_scalar_mul,
tests::bigint_scalar::test_scalar_div_rem,
tests::biguint::test_from_bytes_be,
tests::biguint::test_to_bytes_be,
tests::biguint::test_from_bytes_le,
tests::biguint::test_to_bytes_le,
tests::biguint::test_cmp,
tests::biguint::test_hash,
tests::biguint::test_bitand,
tests::biguint::test_bitor,
tests::biguint::test_bitxor,
tests::biguint::test_shl,
tests::biguint::test_shr,
tests::biguint::test_convert_i64,
tests::biguint::test_convert_u64,
tests::biguint::test_convert_f32,
tests::biguint::test_convert_f64,
tests::biguint::test_convert_to_bigint,
tests::biguint::test_convert_from_uint,
tests::biguint::test_add,
tests::biguint::test_sub,
tests::biguint::test_mul,
tests::biguint::test_div_rem,
tests::biguint::test_checked_add,
tests::biguint::test_checked_sub,
tests::biguint::test_checked_mul,
tests::biguint::test_mul_overflow,
tests::biguint::test_checked_div,
tests::biguint::test_gcd,
tests::biguint::test_lcm,
tests::biguint::test_is_even,
tests::biguint::test_to_str_radix,
tests::biguint::test_from_and_to_radix,
tests::biguint::test_from_str_radix,
tests::biguint::test_all_str_radix,
tests::biguint::test_lower_hex,
tests::biguint::test_upper_hex,
tests::biguint::test_binary,
tests::biguint::test_octal,
tests::biguint::test_display,
tests::biguint::test_factor,
tests::biguint::test_bits,
tests::biguint::test_iter_sum,
tests::biguint::test_iter_product,
tests::biguint::test_iter_sum_generic,
tests::biguint::test_iter_product_generic,
tests::biguint::test_pow,
tests::biguint::test_convert_i128,
tests::biguint::test_convert_u128,
tests::modpow::bigint::test_modpow,
tests::modpow::bigint::test_modpow_big,
tests::modpow::biguint::test_modpow,
tests::modpow::biguint::test_modpow_big,
tests::serde::biguint_zero,
tests::serde::bigint_zero,
tests::serde::biguint_one,
tests::serde::bigint_one,
tests::serde::bigint_negone,
tests::serde::biguint_factorial_100,
tests::serde::bigint_factorial_100,
);

    sgx_status_t::SGX_SUCCESS
}
