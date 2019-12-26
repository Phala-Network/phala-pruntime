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

#[macro_use]
pub extern crate alloc;

extern crate byte_slice_cast;
extern crate bitvec;
extern crate parity_scale_codec;
extern crate arrayvec;
extern crate generic_array;
extern crate serde;

use sgx_types::*;
use std::string::String;
use alloc::vec::Vec;
use std::io::{self, Write};
use std::slice;
use sgx_tunittest::*;
use std::panic;

mod bit_vec;
mod codec;
mod compact;
mod decode_all;
mod encode_append;
mod encode_like;
mod generic_array_feature;

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

    let r = rsgx_unit_tests!(
        bit_vec::required_bytes_test,
        bit_vec::bitvec_u8,
        bit_vec::bitvec_u16,
        bit_vec::bitvec_u32,
        bit_vec::bitvec_u64,
        bit_vec::bitslice,
        bit_vec::bitbox,
        codec::vec_is_slicable,
        codec::encode_borrowed_tuple,
        codec::cow_works,
        codec::cow_string_works,
        codec::string_encoded_as_expected,
        codec::vec_of_u8_encoded_as_expected,
        codec::vec_of_i16_encoded_as_expected,
        codec::vec_of_option_int_encoded_as_expected,
        codec::vec_of_option_bool_encoded_as_expected,
        codec::len_works_for_decode_collection_types,
        codec::vec_of_string_encoded_as_expected,
        codec::should_work_for_wrapper_types,
        codec::codec_vec_deque_u8_and_u16,
        codec::codec_iterator,
        codec::io_reader,
        codec::shared_references_implement_encode,
        codec::not_limit_input_test,
        codec::boolean,
        codec::some_encode_like,
        compact::compact_128_encoding_works,
        compact::compact_64_encoding_works,
        compact::compact_32_encoding_works,
        compact::compact_16_encoding_works,
        compact::compact_8_encoding_works,
        compact::compact_integers_encoded_as_expected,
        compact::compact_as_8_encoding_works,
        compact::compact_as_has_compact,
        compact::compact_using_encoded_arrayvec_size,
        || should_panic!(compact::array_vec_output_oob()),
        compact::array_vec_output,
        compact::compact_u64_test,
        compact::compact_u128_test,
        compact::should_avoid_overlapping_definition,
        decode_all::decode_all_works,
        encode_append::vec_encode_append_works,
        encode_append::vec_encode_append_multiple_items_works,
        encode_append::append_non_copyable,
        encode_append::vec_encode_like_append_works,
        encode_like::vec_and_slice_are_working,
        encode_like::btreemap_and_slice_are_working,
        encode_like::interface_testing,
        generic_array_feature::generic_array
    );
    assert_eq!(r, 0);

    sgx_status_t::SGX_SUCCESS
}
