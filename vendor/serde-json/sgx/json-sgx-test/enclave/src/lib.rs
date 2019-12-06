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

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_bytes;
#[macro_use]
extern crate serde_json;

#[macro_use]
mod macros;
mod tests;
mod debug;
mod preserve_order;
mod regression;

#[cfg(not(feature = "preserve_order"))]
mod stream;

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

    let r = rsgx_unit_tests!(tests::test_write_null,
tests::test_write_u64,
tests::test_write_i64,
tests::test_write_f64,
tests::test_encode_nonfinite_float_yields_null,
tests::test_write_str,
tests::test_write_bool,
tests::test_write_char,
tests::test_write_list,
tests::test_write_object,
tests::test_write_tuple,
tests::test_write_enum,
tests::test_write_option,
tests::test_write_newtype_struct,
tests::test_deserialize_number_to_untagged_enum,
tests::test_parse_null,
tests::test_parse_bool,
tests::test_parse_char,
tests::test_parse_number_errors,
tests::test_parse_i64,
tests::test_parse_u64,
tests::test_parse_negative_zero,
tests::test_parse_f64,
tests::test_serialize_char,
//tests::test_malicious_number,
tests::test_parse_number,
tests::test_parse_string,
tests::test_parse_list,
tests::test_parse_object,
tests::test_parse_struct,
tests::test_parse_option,
tests::test_parse_enum_errors,
tests::test_parse_enum,
tests::test_parse_trailing_whitespace,
tests::test_multiline_errors,
tests::test_missing_option_field,
tests::test_missing_nonoption_field,
tests::test_missing_renamed_field,
tests::test_serialize_seq_with_no_len,
tests::test_serialize_map_with_no_len,
//tests::test_deserialize_from_stream,
tests::test_serialize_rejects_bool_keys,
tests::test_serialize_rejects_adt_keys,
tests::test_bytes_ser,
tests::test_byte_buf_ser,
tests::test_byte_buf_de,
tests::test_byte_buf_de_multiple,
tests::test_json_pointer,
tests::test_json_pointer_mut,
tests::test_stack_overflow,
tests::test_integer_key,
tests::test_integer128_key,
tests::test_deny_float_key,
tests::test_borrowed_key,
tests::test_effectively_string_keys,
tests::test_json_macro,
tests::issue_220,
tests::test_partialeq_number,
tests::test_partialeq_string,
tests::test_partialeq_bool,
tests::test_category,
tests::test_borrow,
tests::null_invalid_type,
tests::test_integer128,
tests::test_borrow_in_map_key,
preserve_order::test_map_order,
debug::number,
debug::value_null,
debug::value_bool,
debug::value_number,
debug::value_string,
debug::value_array,
debug::value_object,
debug::error);
    assert_eq!(r, 0);

    #[cfg(feature = "arbitrary_precision")]
    assert_eq!(rsgx_unit_tests!(tests::test_malicious_number),0);

    #[cfg(feature = "raw_value")]
    assert_eq!(rsgx_unit_tests!(tests::test_boxed_raw_value,
                                tests::test_borrowed_raw_value),0);

    #[cfg(feature = "unbounded_depth")]
    assert_eq!(rsgx_unit_tests!(tests::test_disable_recursion_limit),0);

    assert_eq!(rsgx_unit_tests!(regression::test),0);

    #[cfg(not(feature = "preserve_order"))]
    assert_eq!(rsgx_unit_tests!(stream::test_json_stream_newlines,
                                stream::test_json_stream_trailing_whitespaces,
                                stream::test_json_stream_truncated,
                                stream::test_json_stream_truncated_decimal,
                                stream::test_json_stream_truncated_negative,
                                stream::test_json_stream_truncated_exponent,
                                stream::test_json_stream_empty,
                                stream::test_json_stream_primitive,
                                stream::test_json_stream_invalid_literal,
                                stream::test_json_stream_invalid_number),0);

    sgx_status_t::SGX_SUCCESS
}
