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

#![feature(wrapping_int_impl)]
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
use std::slice;
use sgx_tunittest::*;

extern crate num_traits;

mod test_lib;
mod test_identities;
mod test_sign;
mod test_bounds;
mod test_float;
mod test_mul_add;
mod test_wrapping;
mod cast;

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

    rsgx_unit_tests!(test_lib::clamp_test,
test_lib::from_str_radix_unwrap,
test_lib::from_str_radix_multi_byte_fail,
test_lib::wrapping_is_num,
test_lib::wrapping_from_str_radix,
test_lib::check_num_ops,
test_lib::check_numref_ops,
test_lib::check_refnum_ops,
test_lib::check_refref_ops,
test_lib::check_numassign_ops,
test_identities::wrapping_identities,
test_identities::wrapping_is_zero,
test_identities::wrapping_is_one,
test_sign::unsigned_wrapping_is_unsigned,
test_bounds::wrapping_bounded,
test_bounds::wrapping_bounded_i128,
test_bounds::wrapping_is_bounded,
test_float::convert_deg_rad,
test_float::convert_deg_rad_std,
test_mul_add::mul_add_integer,
test_mul_add::mul_add_float,
test_wrapping::test_wrapping_traits,
test_wrapping::wrapping_is_wrappingadd,
test_wrapping::wrapping_is_wrappingsub,
test_wrapping::wrapping_is_wrappingmul,
test_wrapping::wrapping_is_wrappingshl,
test_wrapping::wrapping_is_wrappingshr,
cast::to_primitive_float,
cast::wrapping_to_primitive,
cast::wrapping_is_toprimitive,
cast::wrapping_is_fromprimitive,
cast::wrapping_is_numcast,
cast::as_primitive,
cast::float_to_integer_checks_overflow,
cast::cast_to_int_checks_overflow,
cast::cast_to_unsigned_int_checks_overflow,
cast::cast_to_i128_checks_overflow,
cast::cast_float_to_int_edge_cases,
cast::cast_int_to_int_edge_cases,
cast::cast_float_to_i128_edge_cases,
cast::cast_int_to_128_edge_cases,
cast::newtype_from_primitive,
cast::newtype_to_primitive
);

    sgx_status_t::SGX_SUCCESS
}
