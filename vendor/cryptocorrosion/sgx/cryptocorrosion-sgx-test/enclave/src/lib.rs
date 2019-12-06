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

#[cfg(target_env = "sgx")]
extern crate core;

use sgx_types::*;
use std::string::String;
use std::vec::Vec;
use std::io::{self, Write};
use std::slice;
use sgx_tunittest::*;

extern crate block_cipher_trait;
extern crate blake_hash;
extern crate groestl_aesni;
extern crate jh_x86_64;
extern crate skein_hash;
//extern crate crypto_simd;
extern crate ppv_lite86;
extern crate c2_chacha;
extern crate threefish_cipher;
#[macro_use]
extern crate hex_literal;
#[macro_use]
extern crate digest;

mod groestl_test;
mod ppv_lite86_sse2;
mod chacha_impl;
mod threefish_test;
mod blake_test;
mod groestl_digesttest;
mod jh_tests;
mod skein_tests;

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
groestl_test::test_transpose_invertible,
ppv_lite86_sse2::test_bswap32_s2_vs_s3,
ppv_lite86_sse2::test_bswap64_s2_vs_s3,
ppv_lite86_sse2::test_shuffle32_s2_vs_s3,
ppv_lite86_sse2::test_shuffle64_s2_vs_s3,
ppv_lite86_sse2::test_lanes_u32x4,
ppv_lite86_sse2::test_lanes_u64x2,
ppv_lite86_sse2::test_vec4_u32x4_s2,
ppv_lite86_sse2::test_vec4_u32x4_s4,
ppv_lite86_sse2::test_vec2_u64x2_s2,
ppv_lite86_sse2::test_vec4_u64x2_s4,
chacha_impl::chacha20_case_1,
chacha_impl::chacha12_case_1,
chacha_impl::chacha8_case_1,
chacha_impl::test_ietf,
chacha_impl::rfc_7539_case_1,
chacha_impl::rfc_7539_case_2,
chacha_impl::rfc_7539_case_2_chunked,
chacha_impl::xchacha20_case_1,
chacha_impl::seek_off_end,
chacha_impl::read_last_bytes,
chacha_impl::seek_consistency,
chacha_impl::wide_matches_narrow,
threefish_test::test_256,
threefish_test::test_512,
threefish_test::test_1024,
blake_test::blake224,
blake_test::blake256,
blake_test::blake384,
blake_test::blake512,
groestl_digesttest::groestl_224,
groestl_digesttest::groestl_256,
groestl_digesttest::groestl_384,
groestl_digesttest::groestl_512,
jh_tests::short_224,
jh_tests::short_256,
jh_tests::short_384,
jh_tests::short_512,
jh_tests::long_224,
jh_tests::long_256,
jh_tests::long_384,
jh_tests::long_512,
skein_tests::skein256_64,
skein_tests::skein512_64,
skein_tests::skein1024_64,
skein_tests::skein256_32,
skein_tests::skein512_32,
skein_tests::skein1024_32,
);
    sgx_status_t::SGX_SUCCESS
}
