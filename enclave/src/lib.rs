#![crate_name = "enclaveapp"]
#![crate_type = "staticlib"]

#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

extern crate sgx_types;
#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;

use sgx_types::*;
use std::string::String;
use std::io::{self, Write};
use std::slice;

#[no_mangle]
pub extern "C" fn ecall_handle(
    action: u8,
    input_ptr: *const u8, input_len: usize,
    output_ptr : *mut u8, output_len_ptr: *mut usize, output_buf_len: usize
) -> sgx_status_t {
    let input_slice = unsafe { std::slice::from_raw_parts(input_ptr, input_len) };
    let input_value: serde_json::value::Value = serde_json::from_slice(input_slice).unwrap();
    let input = input_value.as_object().unwrap();

    let name = input.get("name").unwrap();
    println!("Hello from enclave! you're {}", name);

    
    
    sgx_status_t::SGX_SUCCESS
}
