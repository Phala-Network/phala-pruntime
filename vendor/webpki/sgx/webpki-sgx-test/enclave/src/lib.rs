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

extern crate webpki;
extern crate base64;
extern crate untrusted;

mod signed_data;
mod calendar;
mod name;

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
signed_data::test_ecdsa_prime256v1_sha512_spki_params_null,
signed_data::test_ecdsa_prime256v1_sha512_unused_bits_signature,
signed_data::test_ecdsa_prime256v1_sha512_using_ecdh_key,
signed_data::test_ecdsa_prime256v1_sha512_using_ecmqv_key,
signed_data::test_ecdsa_prime256v1_sha512_using_rsa_algorithm,
signed_data::test_ecdsa_prime256v1_sha512_wrong_signature_format,
signed_data::test_ecdsa_prime256v1_sha512,
signed_data::test_ecdsa_secp384r1_sha256_corrupted_data,
signed_data::test_ecdsa_secp384r1_sha256,
signed_data::test_ecdsa_using_rsa_key,
signed_data::test_rsa_pkcs1_sha1_bad_key_der_length,
signed_data::test_rsa_pkcs1_sha1_bad_key_der_null,
signed_data::test_rsa_pkcs1_sha1_key_params_absent,
signed_data::test_rsa_pkcs1_sha1_using_pss_key_no_params,
signed_data::test_rsa_pkcs1_sha1_wrong_algorithm,
signed_data::test_rsa_pkcs1_sha1,
signed_data::test_rsa_pkcs1_sha256,
signed_data::test_rsa_pkcs1_sha256_key_encoded_ber,
signed_data::test_rsa_pkcs1_sha256_spki_non_null_params,
signed_data::test_rsa_pkcs1_sha256_using_ecdsa_algorithm,
signed_data::test_rsa_pkcs1_sha256_using_id_ea_rsa,
signed_data::test_rsa_pss_sha1_salt20_using_pss_key_no_params,
signed_data::test_rsa_pss_sha1_salt20_using_pss_key_with_null_params,
signed_data::test_rsa_pss_sha1_salt20,
signed_data::test_rsa_pss_sha1_wrong_salt,
signed_data::test_rsa_pss_sha256_mgf1_sha512_salt33,
signed_data::test_rsa_pss_sha256_salt10_using_pss_key_with_params,
signed_data::test_rsa_pss_sha256_salt10_using_pss_key_with_wrong_params,
signed_data::test_rsa_pss_sha256_salt10,
signed_data::test_rsa_pss_sha256_salt32,
signed_data::test_rsa_pss_sha384_salt48,
signed_data::test_rsa_pss_sha512_salt64,
signed_data::test_rsa_pss_sha256_salt32_corrupted_data,
signed_data::test_rsa_pss_sha384_salt48_corrupted_data,
signed_data::test_rsa_pss_sha512_salt64_corrupted_data,
signed_data::test_rsa_using_ec_key,
signed_data::test_rsa2048_pkcs1_sha512,
calendar::test_days_before_unix_epoch,
calendar::test_days_in_month,
calendar::test_time_from_ymdhms_utc,
name::presented_matches_reference_test,
);
    sgx_status_t::SGX_SUCCESS
}
