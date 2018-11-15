extern crate sgx_types;
extern crate sgx_urts;
extern crate dirs;

use sgx_types::*;
use sgx_urts::SgxEnclave;

extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::io::{Read, Write};
use std::fs;
use std::path;

static ENCLAVE_FILE: &'static str = "enclave.signed.so";
static ENCLAVE_TOKEN: &'static str = "enclave.token";

const ENCLAVE_OUTPUT_BUF_MAX_LEN: usize = 4096 as usize;

extern {
    fn ecall_handle(
        eid: sgx_enclave_id_t, retval: *mut sgx_status_t,
        action: u8,
        input_ptr: *const u8, input_len: usize,
        output_ptr : *mut u8, output_len_ptr: *mut usize, output_buf_len: usize
    ) -> sgx_status_t;
}

fn init_enclave() -> SgxResult<SgxEnclave> {

    let mut launch_token: sgx_launch_token_t = [0; 1024];
    let mut launch_token_updated: i32 = 0;
    // Step 1: try to retrieve the launch token saved by last transaction
    //         if there is no token, then create a new one.
    //
    // try to get the token saved in $HOME */
    let mut home_dir = path::PathBuf::new();
    let use_token = match dirs::home_dir() {
        Some(path) => {
            println!("[+] Home dir is {}", path.display());
            home_dir = path;
            true
        },
        None => {
            println!("[-] Cannot get home dir");
            false
        }
    };

    let token_file: path::PathBuf = home_dir.join(ENCLAVE_TOKEN);;
    if use_token == true {
        match fs::File::open(&token_file) {
            Err(_) => {
                println!("[-] Open token file {} error! Will create one.", token_file.as_path().to_str().unwrap());
            },
            Ok(mut f) => {
                println!("[+] Open token file success! ");
                match f.read(&mut launch_token) {
                    Ok(1024) => {
                        println!("[+] Token file valid!");
                    },
                    _ => println!("[+] Token file invalid, will create new token file"),
                }
            }
        }
    }

    // Step 2: call sgx_create_enclave to initialize an enclave instance
    // Debug Support: set 2nd parameter to 1
    let debug = 1;
    let mut misc_attr = sgx_misc_attribute_t {secs_attr: sgx_attributes_t {flags:0, xfrm:0}, misc_select:0};
    let enclave = SgxEnclave::create(
        ENCLAVE_FILE,
        debug,
        &mut launch_token,
        &mut launch_token_updated,
        &mut misc_attr
    )?;

    // Step 3: save the launch token if it is updated
    if use_token == true && launch_token_updated != 0 {
        // reopen the file with write capablity
        match fs::File::create(&token_file) {
            Ok(mut f) => {
                match f.write_all(&launch_token) {
                    Ok(()) => println!("[+] Saved updated launch token!"),
                    Err(_) => println!("[-] Failed to save updated launch token!"),
                }
            },
            Err(_) => {
                println!("[-] Failed to save updated enclave token, but doesn't matter");
            },
        }
    }

    Ok(enclave)
}

fn main() {
    let enclave = match init_enclave() {
        Ok(r) => {
            println!("[+] Init Enclave Successful {}!", r.geteid());
            r
        },
        Err(x) => {
            println!("[-] Init Enclave Failed {}!", x.as_str());
            return;
        },
    };

    // Mock
    let mut input = serde_json::Map::new();
    input.insert("name".to_string(), json!("David".to_string()));
    input.insert("id".to_string(), json!("123456".to_string()));
    input.insert("email".to_string(), json!("david@foo.com".to_string()));
    let input_string = json!(input).to_string();

    let mut return_output_buf: [u8; ENCLAVE_OUTPUT_BUF_MAX_LEN] = [0; ENCLAVE_OUTPUT_BUF_MAX_LEN];
    let mut output_len : usize = 0;
    let output_slice = &mut return_output_buf;

    let mut retval = sgx_status_t::SGX_SUCCESS;
    let result = unsafe {
        let output_ptr = output_slice.as_mut_ptr();
        let output_len_ptr = &mut output_len as *mut usize;

        ecall_handle(
            enclave.geteid(), &mut retval,
            1,
            input_string.as_ptr(), input_string.len(),
            output_ptr, output_len_ptr, ENCLAVE_OUTPUT_BUF_MAX_LEN
        )
    };

    match result {
        sgx_status_t::SGX_SUCCESS => {},
        _ => {
            println!("[-] ECALL Enclave Failed {}!", result.as_str());
            return;
        }
    }

    enclave.destroy();
}
