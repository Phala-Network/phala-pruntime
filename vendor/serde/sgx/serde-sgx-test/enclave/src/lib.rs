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
#![feature(never_type)]

extern crate sgx_types;
#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;

extern crate sgx_tunittest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_test;
extern crate fnv;

use sgx_types::*;
use std::string::String;
use std::vec::Vec;
use std::io::{self, Write};
use std::{slice};
use sgx_tunittest::*;

#[macro_use]
mod macros;
mod bytes;
pub mod unstable;

mod test_de;
mod test_annotations;
mod test_borrow;
mod test_gen;
mod test_identifier;
mod test_macros;
mod test_remote;
mod test_roundtrip;
mod test_ser;
mod test_unstable;
mod test_value;
mod test_ignored_any;

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

    rsgx_unit_tests!(test_de::test_osstring,
                     test_de::test_cstr,
                     test_de::test_cstr_internal_null,
                     test_de::test_cstr_internal_null_end,
                     test_de::test_never_type,
                     test_de::test_bool,
                     test_de::test_isize,
                     test_de::test_ints,
                     test_de::test_uints,
                     test_de::test_floats,
                     test_de::test_small_int_to_128,
                     test_de::test_char,
                     test_de::test_string,
                     test_de::test_option,
                     test_de::test_result,
                     test_de::test_unit,
                     test_de::test_unit_struct,
                     test_de::test_newtype_struct,
                     test_de::test_tuple_struct,
                     test_de::test_btreeset,
                     test_de::test_hashset,
                     test_de::test_vec,
                     test_de::test_array,
                     test_de::test_tuple,
                     test_de::test_btreemap,
                     test_de::test_hashmap,
                     test_de::test_struct,
                     test_de::test_struct_with_skip,
                     test_de::test_struct_skip_all,
                     test_de::test_struct_skip_default,
                     test_de::test_struct_skip_all_deny_unknown,
                     test_de::test_struct_default,
                     test_de::test_enum_unit,
                     test_de::test_enum_simple,
                     test_de::test_enum_seq,
                     test_de::test_enum_map,
                     test_de::test_enum_unit_usize,
                     test_de::test_enum_unit_bytes,
                     test_de::test_enum_other_unit,
                     test_de::test_enum_other,
                     test_de::test_box,
                     test_de::test_boxed_slice,
                     test_de::test_duration,
                     test_de::test_system_time,
                     test_de::test_range,
                     test_de::test_range_inclusive,
                     test_de::test_bound,
                     test_de::test_path,
                     test_de::test_path_buf,
                     test_de::test_cstring,
                     test_de::test_rc,
                     test_de::test_rc_weak_some,
                     test_de::test_rc_weak_none,
                     test_de::test_arc,
                     test_de::test_arc_weak_some,
                     test_de::test_arc_weak_none,
                     test_de::test_wrapping,
                     test_de::test_rc_dst,
                     test_de::test_arc_dst,
                     test_de::test_net_ipv4addr_readable,
                     test_de::test_net_ipv6addr_readable,
                     test_de::test_net_ipaddr_readable,
                     test_de::test_net_socketaddr_readable,
                     test_de::test_net_ipv4addr_compact,
                     test_de::test_net_ipv6addr_compact,
                     test_de::test_net_ipaddr_compact,
                     test_de::test_net_socketaddr_compact,
                     test_de::test_never_result,
                     test_de::test_unknown_field,
                     test_de::test_skipped_field_is_unknown,
                     test_de::test_skip_all_deny_unknown,
                     test_de::test_unknown_variant,
                     test_de::test_enum_skipped_variant,
                     test_de::test_enum_skip_all,
                     test_de::test_duplicate_field_struct,
                     test_de::test_duplicate_field_enum,
                     test_de::test_enum_out_of_range,
                     test_de::test_short_tuple,
                     test_de::test_short_array,
                     test_de::test_cstring_internal_null,
                     test_de::test_cstring_internal_null_end,
                     test_de::test_unit_from_empty_seq,
                     test_de::test_unit_from_empty_seq_without_len,
                     test_de::test_unit_from_tuple_struct,
                     test_de::test_string_from_unit,
                     test_de::test_btreeset_from_unit,
                     test_de::test_btreeset_from_unit_struct,
                     test_de::test_hashset_from_unit,
                     test_de::test_hashset_from_unit_struct,
                     test_de::test_vec_from_unit,
                     test_de::test_vec_from_unit_struct,
                     test_de::test_zero_array_from_unit,
                     test_de::test_zero_array_from_unit_struct,
                     test_de::test_btreemap_from_unit,
                     test_de::test_btreemap_from_unit_struct,
                     test_de::test_hashmap_from_unit,
                     test_de::test_hashmap_from_unit_struct,
                     test_de::test_bool_from_string,
                     test_de::test_number_from_string,
                     test_de::test_integer_from_float,
                     test_de::test_unit_struct_from_seq,
                     test_de::test_wrapping_overflow,
                     test_de::test_ignored_any,
                     test_annotations::test_default_struct,
                     test_annotations::test_default_tuple,
                     test_annotations::test_default_struct_variant,
                     test_annotations::test_default_tuple_variant,
                     test_annotations::test_no_std_default,
                     test_annotations::test_elt_not_deserialize,
                     test_annotations::test_ignore_unknown,
                     test_annotations::test_rename_struct,
                     test_annotations::test_unknown_field_rename_struct,
                     test_annotations::test_rename_enum,
                     test_annotations::test_unknown_field_rename_enum,
                     test_annotations::test_skip_serializing_struct,
                     test_annotations::test_skip_serializing_tuple_struct,
                     test_annotations::test_skip_struct,
                     test_annotations::test_skip_serializing_enum,
                     test_annotations::test_elt_not_serialize,
                     test_annotations::test_serialize_with_struct,
                     test_annotations::test_serialize_with_enum,
                     test_annotations::test_serialize_with_variant,
                     test_annotations::test_deserialize_with_variant,
                     test_annotations::test_deserialize_with_struct,
                     test_annotations::test_deserialize_with_enum,
                     test_annotations::test_missing_renamed_field_struct,
                     test_annotations::test_missing_renamed_field_enum,
                     test_annotations::test_invalid_length_enum,
                     test_annotations::test_from_into_traits,
                     test_annotations::test_collect_other,
                     test_annotations::test_flatten_struct_enum,
                     test_annotations::test_flatten_struct_tag_content_enum,
                     test_annotations::test_flatten_struct_tag_content_enum_newtype,
                     test_annotations::test_unknown_field_in_flatten,
                     test_annotations::test_complex_flatten,
                     test_annotations::test_flatten_map_twice,
                     test_annotations::test_flatten_unsupported_type,
                     test_annotations::test_non_string_keys,
                     test_annotations::test_lifetime_propagation_for_flatten,
                     test_annotations::test_flatten_enum_newtype,
                     test_annotations::test_flatten_internally_tagged,
                     test_annotations::test_externally_tagged_enum_containing_flatten,
                     test_annotations::test_internally_tagged_enum_containing_flatten,
                     test_annotations::test_adjacently_tagged_enum_containing_flatten,
                     test_annotations::test_untagged_enum_containing_flatten,
                     test_annotations::test_flatten_untagged_enum,
                     test_annotations::test_flatten_option,
                     test_annotations::test_transparent_struct,
                     test_annotations::test_transparent_tuple_struct,
                     test_annotations::test_internally_tagged_unit_enum_with_unknown_fields,
                     test_annotations::test_flattened_internally_tagged_unit_enum_with_unknown_fields,
                     test_annotations::test_flatten_any_after_flatten_struct,
                     test_borrow::test_borrowed_str,
                     test_borrow::test_borrowed_str_from_string,
                     test_borrow::test_borrowed_str_from_str,
                     test_borrow::test_string_from_borrowed_str,
                     test_borrow::test_borrowed_bytes,
                     test_borrow::test_borrowed_bytes_from_bytebuf,
                     test_borrow::test_borrowed_bytes_from_bytes,
                     test_borrow::test_tuple,
                     test_borrow::test_struct,
                     test_borrow::test_cow,
                     test_borrow::test_lifetimes,
                     test_gen::test_gen,
                     test_identifier::test_variant_identifier,
                     test_identifier::test_field_identifier,
                     test_identifier::test_unit_fallthrough,
                     test_identifier::test_newtype_fallthrough,
                     test_identifier::test_newtype_fallthrough_generic,
                     test_macros::test_named_unit,
                     test_macros::test_ser_named_tuple,
                     test_macros::test_de_named_tuple,
                     test_macros::test_ser_named_map,
                     test_macros::test_de_named_map,
                     test_macros::test_ser_enum_unit,
                     test_macros::test_ser_enum_seq,
                     test_macros::test_ser_enum_map,
                     test_macros::test_de_enum_unit,
                     test_macros::test_de_enum_seq,
                     test_macros::test_de_enum_map,
                     test_macros::test_lifetimes,
                     test_macros::test_generic_struct,
                     test_macros::test_generic_newtype_struct,
                     test_macros::test_untagged_newtype_struct,
                     test_macros::test_adjacently_tagged_newtype_struct,
                     test_macros::test_generic_tuple_struct,
                     test_macros::test_generic_enum_unit,
                     test_macros::test_generic_enum_newtype,
                     test_macros::test_generic_enum_seq,
                     test_macros::test_generic_enum_map,
                     test_macros::test_default_ty_param,
                     test_macros::test_enum_state_field,
                     test_macros::test_untagged_enum,
                     test_macros::test_internally_tagged_enum,
                     test_macros::test_internally_tagged_bytes,
                     test_macros::test_internally_tagged_struct_variant_containing_unit_variant,
                     test_macros::test_internally_tagged_borrow,
                     test_macros::test_adjacently_tagged_enum,
                     test_macros::test_adjacently_tagged_enum_deny_unknown_fields,
                     test_macros::test_enum_in_internally_tagged_enum,
                     test_macros::test_internally_tagged_struct,
                     test_macros::test_internally_tagged_braced_struct_with_zero_fields,
                     test_macros::test_internally_tagged_struct_with_flattened_field,
                     test_macros::test_enum_in_untagged_enum,
                     test_macros::test_untagged_bytes,
                     test_macros::test_rename_all,
                     test_macros::test_untagged_newtype_variant_containing_unit_struct_not_map,
                     test_macros::test_internally_tagged_newtype_variant_containing_unit_struct,
                     test_macros::test_untagged_enum_with_flattened_integer_key,
                     test_roundtrip::ip_addr_roundtrip,
                     test_roundtrip::socket_addr_roundtrip,
                     test_ser::test_unit,
                     test_ser::test_bool,
                     test_ser::test_isizes,
                     test_ser::test_usizes,
                     test_ser::test_floats,
                     test_ser::test_char,
                     test_ser::test_str,
                     test_ser::test_option,
                     test_ser::test_result,
                     test_ser::test_slice,
                     test_ser::test_array,
                     test_ser::test_vec,
                     test_ser::test_btreeset,
                     test_ser::test_hashset,
                     test_ser::test_tuple,
                     test_ser::test_btreemap,
                     test_ser::test_hashmap,
                     test_ser::test_unit_struct,
                     test_ser::test_tuple_struct,
                     test_ser::test_struct,
                     test_ser::test_enum,
                     test_ser::test_box,
                     test_ser::test_boxed_slice,
                     test_ser::test_duration,
                     test_ser::test_system_time,
                     test_ser::test_range,
                     test_ser::test_range_inclusive,
                     test_ser::test_bound,
                     test_ser::test_path,
                     test_ser::test_path_buf,
                     test_ser::test_cstring,
                     test_ser::test_cstr,
                     test_ser::test_rc,
                     test_ser::test_rc_weak_some,
                     test_ser::test_rc_weak_none,
                     test_ser::test_arc,
                     test_ser::test_arc_weak_some,
                     test_ser::test_arc_weak_none,
                     test_ser::test_wrapping,
                     test_ser::test_rc_dst,
                     test_ser::test_arc_dst,
                     test_ser::test_fmt_arguments,
                     test_ser::test_net_ipv4addr_readable,
                     test_ser::test_net_ipv6addr_readable,
                     test_ser::test_net_ipaddr_readable,
                     test_ser::test_net_socketaddr_readable,
                     test_ser::test_net_ipv4addr_compact,
                     test_ser::test_net_ipv6addr_compact,
                     test_ser::test_net_ipaddr_compact,
                     test_ser::test_net_socketaddr_compact,
                     test_ser::test_never_result,
                     test_ser::test_cannot_serialize_paths,
                     test_ser::test_cannot_serialize_mutably_borrowed_ref_cell,
                     test_ser::test_enum_skipped,
                     test_ser::test_integer128,
                     test_unstable::unstable::test_raw_identifiers,
                     test_value::test_u32_to_enum,
                     test_value::test_integer128,
                     test_ignored_any::test_deserialize_enum,
);
    println!("All tests finished!");
    sgx_status_t::SGX_SUCCESS
}
