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
use sgx_tunittest::*;

extern crate yasna;
extern crate num_bigint;
extern crate num_traits;
extern crate chrono;
extern crate bit_vec;

mod reader_tests;
mod writer_tests;
mod oid;
mod time;

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
reader_tests::test_der_read_bool_ok,
reader_tests::test_der_read_bool_err,
reader_tests::test_ber_read_bool_ok,
reader_tests::test_ber_read_bool_err,
reader_tests::test_der_read_i64_ok,
reader_tests::test_der_read_i64_err,
reader_tests::test_ber_read_i64_ok,
reader_tests::test_ber_read_i64_err,
reader_tests::test_der_read_u64_ok,
reader_tests::test_der_read_u64_err,
reader_tests::test_ber_read_u64_ok,
reader_tests::test_ber_read_u64_err,
reader_tests::test_der_read_i32_ok,
reader_tests::test_der_read_i32_err,
reader_tests::test_ber_read_i32_ok,
reader_tests::test_ber_read_i32_err,
reader_tests::test_der_read_u32_err,
reader_tests::test_ber_read_u32_ok,
reader_tests::test_ber_read_u32_err,
reader_tests::test_der_read_i16_ok,
reader_tests::test_der_read_i16_err,
reader_tests::test_ber_read_i16_ok,
reader_tests::test_ber_read_i16_err,
reader_tests::test_der_read_u16_ok,
reader_tests::test_der_read_u16_err,
reader_tests::test_ber_read_u16_ok,
reader_tests::test_ber_read_u16_err,
reader_tests::test_der_read_i8_ok,
reader_tests::test_der_read_i8_err,
reader_tests::test_ber_read_i8_ok,
reader_tests::test_ber_read_i8_err,
reader_tests::test_der_read_u8_ok,
reader_tests::test_der_read_u8_err,
reader_tests::test_ber_read_u8_ok,
reader_tests::test_ber_read_u8_err,
reader_tests::test_der_read_bigint_ok,
reader_tests::test_der_read_bigint_err,
reader_tests::test_ber_read_bigint_ok,
reader_tests::test_ber_read_bigint_err,
reader_tests::test_der_read_biguint_ok,
reader_tests::test_der_read_biguint_err,
reader_tests::test_ber_read_biguint_ok,
reader_tests::test_ber_read_biguint_err,
reader_tests::test_der_read_bytes_ok,
reader_tests::test_der_read_bytes_err,
reader_tests::test_ber_read_bytes_ok,
reader_tests::test_ber_read_bytes_err,
reader_tests::test_der_read_null_ok,
reader_tests::test_der_read_null_err,
reader_tests::test_ber_read_null_ok,
reader_tests::test_ber_read_null_err,
reader_tests::test_der_read_sequence_ok,
reader_tests::test_der_read_sequence_err,
reader_tests::test_der_read_tagged_der_err,
reader_tests::test_ber_read_sequence_ok,
reader_tests::test_ber_read_sequence_err,
reader_tests::test_der_read_set_ok,
reader_tests::test_der_read_set_err,
reader_tests::test_ber_read_set_ok,
reader_tests::test_ber_read_set_err,
reader_tests::test_der_read_set_of_ok,
reader_tests::test_der_read_set_of_err,
reader_tests::test_ber_read_set_of_ok,
reader_tests::test_ber_read_set_of_err,
reader_tests::test_der_read_tagged_ok,
reader_tests::test_der_read_tagged_err,
reader_tests::test_ber_read_tagged_ok,
reader_tests::test_ber_read_tagged_err,
reader_tests::test_der_read_tagged_implicit_ok,
reader_tests::test_der_read_tagged_implicit_err,
reader_tests::test_ber_read_tagged_implicit_ok,
reader_tests::test_ber_read_tagged_implicit_err,
writer_tests::test_der_write_bool,
writer_tests::test_der_write_i64,
writer_tests::test_der_write_u64,
writer_tests::test_der_write_i32,
writer_tests::test_der_write_u32,
writer_tests::test_der_write_i16,
writer_tests::test_der_write_u16,
writer_tests::test_der_write_i8,
writer_tests::test_der_write_u8,
writer_tests::test_der_write_bigint,
writer_tests::test_der_write_biguint,
writer_tests::test_der_write_bytes,
writer_tests::test_der_write_null,
writer_tests::test_der_write_sequence_small,
writer_tests::test_der_write_sequence_medium,
writer_tests::test_der_write_set,
writer_tests::test_der_write_set_of,
writer_tests::test_der_write_tagged,
writer_tests::test_der_write_tagged_implicit,
oid::test_display_oid,
oid::parse_oid,
time::test_utctime_parse,
time::test_generalized_time_parse,
);

    sgx_status_t::SGX_SUCCESS
}
