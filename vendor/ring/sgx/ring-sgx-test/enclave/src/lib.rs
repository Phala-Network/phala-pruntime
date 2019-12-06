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
#[cfg(target_env = "sgx")]
extern crate core;
#[cfg(not(target_env = "sgx"))]
extern crate alloc;

extern crate sgx_tunittest;

use sgx_types::*;
use std::string::String;
use std::vec::Vec;
use std::io::{self, Write};
use std::slice;
use sgx_tunittest::*;

#[macro_use]
extern crate ring;
extern crate untrusted;

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

    println!("aes: {:?}", &ring::aead::AES_256_GCM);

    rsgx_unit_tests!(
tests::aead_tests::aead_aes_gcm_128,
tests::aead_tests::aead_aes_gcm_256,
tests::aead_tests::aead_chacha20_poly1305,
tests::aead_tests::test_aead_nonce_sizes,
tests::aead_tests::aead_chacha20_poly1305_openssh,
tests::aead_tests::test_aead_key_debug,
tests::aead_tests::test_tag_traits,
tests::agreement_tests::agreement_traits,
tests::agreement_tests::agreement_agree_ephemeral,
tests::agreement_tests::test_agreement_ecdh_x25519_rfc_iterated,
tests::digest_tests::digest_misc,
tests::digest_tests::test_fmt_algorithm,
tests::digest_tests::digest_test_fmt,
tests::digest_tests::digest_shavs::SHA1_FOR_LEGACY_USE_ONLY::short_msg_known_answer_test,
tests::digest_tests::digest_shavs::SHA1_FOR_LEGACY_USE_ONLY::long_msg_known_answer_test,
tests::digest_tests::digest_shavs::SHA1_FOR_LEGACY_USE_ONLY::monte_carlo_test,
tests::digest_tests::digest_shavs::SHA256::short_msg_known_answer_test,
tests::digest_tests::digest_shavs::SHA256::long_msg_known_answer_test,
tests::digest_tests::digest_shavs::SHA256::monte_carlo_test,
tests::digest_tests::digest_shavs::SHA384::short_msg_known_answer_test,
tests::digest_tests::digest_shavs::SHA384::long_msg_known_answer_test,
tests::digest_tests::digest_shavs::SHA384::monte_carlo_test,
tests::digest_tests::digest_shavs::SHA512::short_msg_known_answer_test,
tests::digest_tests::digest_shavs::SHA512::long_msg_known_answer_test,
tests::digest_tests::digest_shavs::SHA512::monte_carlo_test,
tests::digest_tests::digest_test_i_u_f_sha1,
tests::digest_tests::digest_test_i_u_f_sha256,
tests::digest_tests::digest_test_i_u_f_sha384,
tests::digest_tests::digest_test_i_u_f_sha512,
tests::digest_tests::digest_test_large_digest_sha1,
tests::digest_tests::digest_test_large_digest_sha256,
tests::digest_tests::digest_test_large_digest_sha384,
tests::digest_tests::digest_test_large_digest_sha512,
tests::ecdsa_tests::ecdsa_from_pkcs8_test,
tests::ecdsa_tests::ecdsa_generate_pkcs8_test,
tests::ecdsa_tests::signature_ecdsa_verify_asn1_test,
tests::ecdsa_tests::signature_ecdsa_verify_fixed_test,
tests::ecdsa_tests::ecdsa_test_public_key_coverage,
tests::ed25519_tests::test_signature_ed25519,
tests::ed25519_tests::test_ed25519_from_seed_and_public_key_misuse,
tests::ed25519_tests::test_ed25519_from_pkcs8_unchecked,
tests::ed25519_tests::test_ed25519_from_pkcs8,
tests::ed25519_tests::ed25519_test_public_key_coverage,
tests::hkdf_tests::hkdf_tests,
tests::hmac_tests::hmac_tests,
tests::hmac_tests::hmac_debug,
tests::pbkdf2_tests::pbkdf2_tests,
tests::quic_tests::quic_aes_128,
tests::quic_tests::quic_aes_256,
tests::quic_tests::quic_chacha20,
tests::rsa_tests::rsa_from_pkcs8_test,
tests::rsa_tests::test_signature_rsa_pkcs1_sign,
tests::rsa_tests::test_signature_rsa_pss_sign,
tests::rsa_tests::test_signature_rsa_pkcs1_verify,
tests::rsa_tests::test_signature_rsa_pss_verify,
tests::rsa_tests::test_signature_rsa_primitive_verification,
tests::rsa_tests::rsa_test_public_key_coverage,
tests::signature_tests::signature_impl_test,
);

    sgx_status_t::SGX_SUCCESS
}
