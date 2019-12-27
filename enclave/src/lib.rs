#![crate_name = "enclaveapp"]
#![crate_type = "staticlib"]

#![warn(unused_imports)]
#![warn(unused_extern_crates)]

#![cfg_attr(all(feature = "mesalock_sgx",
not(target_env = "sgx")), no_std)]
#![cfg_attr(all(target_env = "sgx", target_vendor = "mesalock"), feature(rustc_private))]

#[cfg(all(feature = "mesalock_sgx", not(target_env = "sgx")))]
#[macro_use]
extern crate sgx_tstd as std;

extern crate sgx_types;
// extern crate sgx_trts;
extern crate sgx_tcrypto;
extern crate sgx_tse;
extern crate sgx_rand;

extern crate rustls;
extern crate webpki;
extern crate webpki_roots;
extern crate itertools;
extern crate base64;
extern crate httparse;
extern crate yasna;
extern crate bit_vec;
extern crate num_bigint;
extern crate chrono;
extern crate secp256k1;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate lazy_static;
extern crate ring;
// extern crate rand;

use std::backtrace::{self, PrintFormat};
use sgx_types::*;
use sgx_tse::*;
//use sgx_trts::trts::{rsgx_raw_is_outside_enclave, rsgx_lfence};
use sgx_tcrypto::*;
use sgx_rand::*;

use std::prelude::v1::*;
use std::sync::Arc;
use std::mem;
use std::net::TcpStream;
use std::string::String;
use std::ptr;
use std::str;
use std::io::{Write, Read, BufReader};
use std::untrusted::fs;
use std::vec::Vec;
use std::collections::HashMap;
use std::sync::SgxMutex;
use itertools::Itertools;
use serde_json::{Map, Value};
use secp256k1::*;
use secp256k1::{SecretKey, PublicKey};
use ring::rand::SecureRandom;

mod cert;
mod hex;

extern "C" {
    pub fn ocall_sgx_init_quote(
        ret_val: *mut sgx_status_t,
        ret_ti: *mut sgx_target_info_t,
        ret_gid: *mut sgx_epid_group_id_t
    ) -> sgx_status_t;

    pub fn ocall_get_ias_socket(
        ret_val: *mut sgx_status_t,
        ret_fd: *mut i32
    ) -> sgx_status_t;

    pub fn ocall_get_quote(
        ret_val: *mut sgx_status_t,
        p_sigrl: *const u8,
        sigrl_len: u32,
        p_report: *const sgx_report_t,
        quote_type: sgx_quote_sign_type_t,
        p_spid: *const sgx_spid_t,
        p_nonce: *const sgx_quote_nonce_t,
        p_qe_report: *mut sgx_report_t,
        p_quote: *mut u8,
        maxlen: u32,
        p_quote_len: *mut u32
    ) -> sgx_status_t;

    pub fn ocall_dump_state(
        ret_val: *mut sgx_status_t,
        output_ptr : *mut u8,
        output_len_ptr: *mut usize,
        output_buf_len: usize
    ) -> sgx_status_t;
}

#[derive(Serialize, Deserialize, Debug)]
struct RuntimeState {
    contract: contract::Contract
}

struct GlobalState {
    initialized: bool,
    public_key: Box<PublicKey>,
    private_key: Box<SecretKey>,
    blocknum: u32,
}

lazy_static! {
    static ref STATE: SgxMutex<RuntimeState> = {
        SgxMutex::new(RuntimeState {
            contract: contract::Contract::new()
        })
    };

    static ref GLOBAL_STATE: SgxMutex<GlobalState> = {
        // TODO: Hard code init value
        let sk = SecretKey::parse_slice(hex::decode_hex("8431d995681fc0a8576a56bbc2e24a322f84b1408d4bf35694c325c9407dd2e8").as_slice()).unwrap();
        let pk = PublicKey::from_secret_key(&sk);

        SgxMutex::new(
            GlobalState {
                initialized: false,
                public_key: Box::new(pk),
                private_key: Box::new(sk),
                blocknum: 0
            }
        )
    };
}

pub const DEV_HOSTNAME:&'static str = "api.trustedservices.intel.com";
pub const SIGRL_SUFFIX:&'static str = "/sgx/dev/attestation/v3/sigrl/";
pub const REPORT_SUFFIX:&'static str = "/sgx/dev/attestation/v3/report";
pub const CERTEXPIRYDAYS: i64 = 90i64;

fn load_spid(filename: &str) -> sgx_spid_t {
    let mut spidfile = fs::File::open(filename).expect("cannot open spid file");
    let mut contents = String::new();
    spidfile.read_to_string(&mut contents).expect("cannot read the spid file");

    hex::decode_spid(&contents)
}

fn get_ias_api_key() -> String {
    let mut keyfile = fs::File::open("key.txt").expect("cannot open ias key file");
    let mut key = String::new();
    keyfile.read_to_string(&mut key).expect("cannot read the ias key file");
    key.trim_end().to_owned()
}

fn parse_response_attn_report(resp : &[u8]) -> (String, String, String){
    println!("parse_response_attn_report");
    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut respp   = httparse::Response::new(&mut headers);
    let result = respp.parse(resp);
    println!("parse result {:?}", result);

    let msg : &'static str;

    match respp.code {
        Some(200) => msg = "OK Operation Successful",
        Some(401) => msg = "Unauthorized Failed to authenticate or authorize request.",
        Some(404) => msg = "Not Found GID does not refer to a valid EPID group ID.",
        Some(500) => msg = "Internal error occurred",
        Some(503) => msg = "Service is currently not able to process the request (due to
            a temporary overloading or maintenance). This is a
            temporary state – the same request can be repeated after
            some time. ",
        _ => {println!("DBG:{}", respp.code.unwrap()); msg = "Unknown error occured"},
    }

    println!("{}", msg);
    let mut len_num : u32 = 0;

    let mut sig = String::new();
    let mut cert = String::new();
    let mut attn_report = String::new();

    for i in 0..respp.headers.len() {
        let h = respp.headers[i];
        //println!("{} : {}", h.name, str::from_utf8(h.value).unwrap());
        match h.name{
            "Content-Length" => {
                let len_str = String::from_utf8(h.value.to_vec()).unwrap();
                len_num = len_str.parse::<u32>().unwrap();
                println!("content length = {}", len_num);
            }
            "X-IASReport-Signature" => sig = str::from_utf8(h.value).unwrap().to_string(),
            "X-IASReport-Signing-Certificate" => cert = str::from_utf8(h.value).unwrap().to_string(),
            _ => (),
        }
    }

    // Remove %0A from cert, and only obtain the signing cert
    cert = cert.replace("%0A", "");
    cert = cert::percent_decode(cert);
    let v: Vec<&str> = cert.split("-----").collect();
    let sig_cert = v[2].to_string();

    if len_num != 0 {
        let header_len = result.unwrap().unwrap();
        let resp_body = &resp[header_len..];
        attn_report = str::from_utf8(resp_body).unwrap().to_string();
        println!("Attestation report: {}", attn_report);
    }

    // len_num == 0
    (attn_report, sig, sig_cert)
}

fn parse_response_sigrl(resp : &[u8]) -> Vec<u8> {
    println!("parse_response_sigrl");
    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut respp   = httparse::Response::new(&mut headers);
    let result = respp.parse(resp);
    println!("parse result {:?}", result);
    println!("parse response{:?}", respp);

    let msg : &'static str;

    match respp.code {
        Some(200) => msg = "OK Operation Successful",
        Some(401) => msg = "Unauthorized Failed to authenticate or authorize request.",
        Some(404) => msg = "Not Found GID does not refer to a valid EPID group ID.",
        Some(500) => msg = "Internal error occurred",
        Some(503) => msg = "Service is currently not able to process the request (due to
            a temporary overloading or maintenance). This is a
            temporary state – the same request can be repeated after
            some time. ",
        _ => msg = "Unknown error occured",
    }

    println!("{}", msg);
    let mut len_num : u32 = 0;

    for i in 0..respp.headers.len() {
        let h = respp.headers[i];
        if h.name == "content-length" {
            let len_str = String::from_utf8(h.value.to_vec()).unwrap();
            len_num = len_str.parse::<u32>().unwrap();
            println!("content length = {}", len_num);
        }
    }

    if len_num != 0 {
        let header_len = result.unwrap().unwrap();
        let resp_body = &resp[header_len..];
        println!("Base64-encoded SigRL: {:?}", resp_body);

        return base64::decode(str::from_utf8(resp_body).unwrap()).unwrap();
    }

    // len_num == 0
    Vec::new()
}

pub fn make_ias_client_config() -> rustls::ClientConfig {
    let mut config = rustls::ClientConfig::new();

    config.root_store.add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);

    config
}

pub fn get_sigrl_from_intel(fd : c_int, gid : u32) -> Vec<u8> {
    println!("get_sigrl_from_intel fd = {:?}", fd);
    let config = make_ias_client_config();
    let ias_key = get_ias_api_key();

    let req = format!("GET {}{:08x} HTTP/1.1\r\nHOST: {}\r\nOcp-Apim-Subscription-Key: {}\r\nConnection: Close\r\n\r\n",
                      SIGRL_SUFFIX,
                      gid,
                      DEV_HOSTNAME,
                      ias_key);

    println!("{}", req);

    let dns_name = webpki::DNSNameRef::try_from_ascii_str(DEV_HOSTNAME).unwrap();
    let mut sess = rustls::ClientSession::new(&Arc::new(config), dns_name);
    let mut sock = TcpStream::new(fd).unwrap();
    let mut tls = rustls::Stream::new(&mut sess, &mut sock);

    let _result = tls.write(req.as_bytes());
    let mut plaintext = Vec::new();

    println!("write complete");

    match tls.read_to_end(&mut plaintext) {
        Ok(_) => (),
        Err(e) => {
            println!("get_sigrl_from_intel tls.read_to_end: {:?}", e);
            panic!("haha");
        }
    }
    println!("read_to_end complete");
    let resp_string = String::from_utf8(plaintext.clone()).unwrap();

    println!("{}", resp_string);

    parse_response_sigrl(&plaintext)
}

// TODO: support pse
pub fn get_report_from_intel(fd : c_int, quote : Vec<u8>) -> (String, String, String) {
    println!("get_report_from_intel fd = {:?}", fd);
    let config = make_ias_client_config();
    let encoded_quote = base64::encode(&quote[..]);
    let encoded_json = format!("{{\"isvEnclaveQuote\":\"{}\"}}\r\n", encoded_quote);
    let ias_key = get_ias_api_key();

    let req = format!("POST {} HTTP/1.1\r\nHOST: {}\r\nOcp-Apim-Subscription-Key:{}\r\nContent-Length:{}\r\nContent-Type: application/json\r\nConnection: close\r\n\r\n{}",
                      REPORT_SUFFIX,
                      DEV_HOSTNAME,
                      ias_key,
                      encoded_json.len(),
                      encoded_json);

    println!("{}", req);
    let dns_name = webpki::DNSNameRef::try_from_ascii_str(DEV_HOSTNAME).unwrap();
    let mut sess = rustls::ClientSession::new(&Arc::new(config), dns_name);
    let mut sock = TcpStream::new(fd).unwrap();
    let mut tls = rustls::Stream::new(&mut sess, &mut sock);

    let _result = tls.write(req.as_bytes());
    let mut plaintext = Vec::new();

    println!("write complete");

    tls.read_to_end(&mut plaintext).unwrap();
    println!("read_to_end complete");
    let resp_string = String::from_utf8(plaintext.clone()).unwrap();

    println!("resp_string = {}", resp_string);

    let (attn_report, sig, cert) = parse_response_attn_report(&plaintext);

    (attn_report, sig, cert)
}

fn as_u32_le(array: &[u8; 4]) -> u32 {
    ((array[0] as u32) <<  0) +
        ((array[1] as u32) <<  8) +
        ((array[2] as u32) << 16) +
        ((array[3] as u32) << 24)
}

fn create_attestation_report(report_data_payload: &[u8]) -> Result<(String, String, String), sgx_status_t> {
    // TODO: Check size of report data, it must < 64 bytes

    // Workflow:
    // (1) ocall to get the target_info structure (ti) and epid group id (eg)
    // (1.5) get sigrl
    // (2) call sgx_create_report with ti+data, produce an sgx_report_t
    // (3) ocall to sgx_get_quote to generate (*mut sgx-quote_t, uint32_t)

    // (1) get ti + eg
    let mut ti : sgx_target_info_t = sgx_target_info_t::default();
    let mut eg : sgx_epid_group_id_t = sgx_epid_group_id_t::default();
    let mut rt : sgx_status_t = sgx_status_t::SGX_ERROR_UNEXPECTED;

    let res = unsafe {
        ocall_sgx_init_quote(&mut rt as *mut sgx_status_t,
                             &mut ti as *mut sgx_target_info_t,
                             &mut eg as *mut sgx_epid_group_id_t)
    };

    println!("eg = {:?}", eg);

    if res != sgx_status_t::SGX_SUCCESS {
        return Err(res);
    }

    if rt != sgx_status_t::SGX_SUCCESS {
        return Err(rt);
    }

    let eg_num = as_u32_le(&eg);

    // (1.5) get sigrl
    let mut ias_sock : i32 = 0;

    let res = unsafe {
        ocall_get_ias_socket(&mut rt as *mut sgx_status_t,
                             &mut ias_sock as *mut i32)
    };

    if res != sgx_status_t::SGX_SUCCESS {
        return Err(res);
    }

    if rt != sgx_status_t::SGX_SUCCESS {
        return Err(rt);
    }

    //println!("Got ias_sock = {}", ias_sock);

    // Now sigrl_vec is the revocation list, a vec<u8>
    let sigrl_vec : Vec<u8> = get_sigrl_from_intel(ias_sock, eg_num);

    // (2) Generate the report
    // Fill ecc256 public key into report_data
    println!("Preparing report data");
    let mut report_data: sgx_report_data_t = sgx_report_data_t::default();
    report_data.d[..report_data_payload.len()].clone_from_slice(report_data_payload);

    let rep = match rsgx_create_report(&ti, &report_data) {
        Ok(r) =>{
            println!("Report creation => success {:?}", r.body.mr_signer.m);
            Some(r)
        },
        Err(e) =>{
            println!("Report creation => failed {:?}", e);
            None
        },
    };

    let mut quote_nonce = sgx_quote_nonce_t { rand : [0;16] };
    let mut os_rng = os::SgxRng::new().unwrap();
    os_rng.fill_bytes(&mut quote_nonce.rand);
    println!("rand finished");
    let mut qe_report = sgx_report_t::default();
    const RET_QUOTE_BUF_LEN : u32 = 2048;
    let mut return_quote_buf : [u8; RET_QUOTE_BUF_LEN as usize] = [0;RET_QUOTE_BUF_LEN as usize];
    let mut quote_len : u32 = 0;

    // (3) Generate the quote
    // Args:
    //       1. sigrl: ptr + len
    //       2. report: ptr 432bytes
    //       3. linkable: u32, unlinkable=0, linkable=1
    //       4. spid: sgx_spid_t ptr 16bytes
    //       5. sgx_quote_nonce_t ptr 16bytes
    //       6. p_sig_rl + sigrl size ( same to sigrl)
    //       7. [out]p_qe_report need further check
    //       8. [out]p_quote
    //       9. quote_size
    let (p_sigrl, sigrl_len) =
        if sigrl_vec.len() == 0 {
            (ptr::null(), 0)
        } else {
            (sigrl_vec.as_ptr(), sigrl_vec.len() as u32)
        };
    let p_report = (&rep.unwrap()) as * const sgx_report_t;
    let quote_type = sgx_quote_sign_type_t::SGX_LINKABLE_SIGNATURE;

    let spid : sgx_spid_t = load_spid("spid.txt");

    let p_spid = &spid as *const sgx_spid_t;
    let p_nonce = &quote_nonce as * const sgx_quote_nonce_t;
    let p_qe_report = &mut qe_report as *mut sgx_report_t;
    let p_quote = return_quote_buf.as_mut_ptr();
    let maxlen = RET_QUOTE_BUF_LEN;
    let p_quote_len = &mut quote_len as *mut u32;

    let result = unsafe {
        ocall_get_quote(&mut rt as *mut sgx_status_t,
                        p_sigrl,
                        sigrl_len,
                        p_report,
                        quote_type,
                        p_spid,
                        p_nonce,
                        p_qe_report,
                        p_quote,
                        maxlen,
                        p_quote_len)
    };

    if result != sgx_status_t::SGX_SUCCESS {
        return Err(result);
    }
    if rt != sgx_status_t::SGX_SUCCESS {
        println!("ocall_get_quote returned {}", rt);
        return Err(rt);
    }

    // Added 09-28-2018
    // Perform a check on qe_report to verify if the qe_report is valid
    match rsgx_verify_report(&qe_report) {
        Ok(()) => println!("rsgx_verify_report passed!"),
        Err(x) => {
            println!("rsgx_verify_report failed with {:?}", x);
            return Err(x);
        },
    }

    // Check if the qe_report is produced on the same platform
    if ti.mr_enclave.m != qe_report.body.mr_enclave.m ||
        ti.attributes.flags != qe_report.body.attributes.flags ||
        ti.attributes.xfrm  != qe_report.body.attributes.xfrm {
        println!("qe_report does not match current target_info!");
        return Err(sgx_status_t::SGX_ERROR_UNEXPECTED);
    }

    println!("qe_report check passed");

    // Debug
    // for i in 0..quote_len {
    //     print!("{:02X}", unsafe {*p_quote.offset(i as isize)});
    // }
    // println!("");

    // Check qe_report to defend against replay attack
    // The purpose of p_qe_report is for the ISV enclave to confirm the QUOTE
    // it received is not modified by the untrusted SW stack, and not a replay.
    // The implementation in QE is to generate a REPORT targeting the ISV
    // enclave (target info from p_report) , with the lower 32Bytes in
    // report.data = SHA256(p_nonce||p_quote). The ISV enclave can verify the
    // p_qe_report and report.data to confirm the QUOTE has not be modified and
    // is not a replay. It is optional.

    let mut rhs_vec : Vec<u8> = quote_nonce.rand.to_vec();
    rhs_vec.extend(&return_quote_buf[..quote_len as usize]);
    let rhs_hash = rsgx_sha256_slice(&rhs_vec[..]).unwrap();
    let lhs_hash = &qe_report.body.report_data.d[..32];

    println!("rhs hash = {:02X}", rhs_hash.iter().format(""));
    println!("report hs= {:02X}", lhs_hash.iter().format(""));

    if rhs_hash != lhs_hash {
        println!("Quote is tampered!");
        return Err(sgx_status_t::SGX_ERROR_UNEXPECTED);
    }

    let quote_vec : Vec<u8> = return_quote_buf[..quote_len as usize].to_vec();
    let res = unsafe {
        ocall_get_ias_socket(&mut rt as *mut sgx_status_t,
                             &mut ias_sock as *mut i32)
    };

    if res != sgx_status_t::SGX_SUCCESS {
        return Err(res);
    }

    if rt != sgx_status_t::SGX_SUCCESS {
        return Err(rt);
    }

    let (attn_report, sig, cert) = get_report_from_intel(ias_sock, quote_vec);
    Ok((attn_report, sig, cert))
}

const ACTION_TEST: u8 = 0;
const ACTION_INIT_RUNTIME: u8 = 1;
const ACTION_GET_INFO: u8 = 2;
const ACTION_DUMP_STATES: u8 = 3;
const ACTION_LOAD_STATES: u8 = 4;
const ACTION_SYNC_BLOCK: u8 = 5;
const ACTION_QUERY: u8 = 6;
const ACTION_SET: u8 = 21;
const ACTION_GET: u8 = 22;

#[no_mangle]
pub extern "C" fn ecall_set_state(
    input_ptr: *const u8, input_len: usize
) -> sgx_status_t {
    let input_slice = unsafe { std::slice::from_raw_parts(input_ptr, input_len) };
    let input_value: serde_json::value::Value = serde_json::from_slice(input_slice).unwrap();
    let input = input_value.as_object().unwrap();

    sgx_status_t::SGX_SUCCESS
}

#[no_mangle]
pub extern "C" fn ecall_handle(
    action: u8,
    input_ptr: *const u8, input_len: usize,
    output_ptr : *mut u8, output_len_ptr: *mut usize, output_buf_len: usize
) -> sgx_status_t {
    let input_slice = unsafe { std::slice::from_raw_parts(input_ptr, input_len) };
    let input_value: serde_json::value::Value = serde_json::from_slice(input_slice).unwrap();
    let input = input_value.as_object().unwrap();
    let payload = input_value.get("input").unwrap().as_object().unwrap();

    let result = match action {
        ACTION_TEST => test(payload),
        ACTION_INIT_RUNTIME => init_runtime(payload),
        ACTION_GET_INFO => get_info(payload),
        ACTION_DUMP_STATES => dump_states(payload),
        ACTION_LOAD_STATES => load_states(payload),
        ACTION_SYNC_BLOCK => sync_block(payload),
        ACTION_QUERY => query(payload),
        ACTION_GET => get(payload),
        ACTION_SET => set(payload),
        _ => unknown()
    };

    let global_state = GLOBAL_STATE.lock().unwrap();

//    let nonce = input_value.get("nonce").unwrap().as_object().unwrap();
//    let nonce_vec = serde_json::to_vec(&nonce).unwrap();
//    let (attn_report, sig, cert) = create_attestation_report(&nonce_vec).unwrap();
//    let mut map = serde_json::Map::new();
//    map.insert("report".to_owned(), json!(attn_report));
//    map.insert("signature".to_owned(), json!(sig));
//    map.insert("signing_cert".to_owned(), json!(cert));

    let output_json = match result {
        Ok(payload) => {
            let s_payload = payload.to_string();

            let hash_payload = rsgx_sha256_slice(&s_payload.as_bytes()).unwrap();
            let message = secp256k1::Message::parse_slice(&hash_payload[..32]).unwrap();
            let (signature, _recovery_id) = secp256k1::sign(&message, &global_state.private_key);

            json!({
                "status": "ok",
                "payload": s_payload,
                "signature": hex::encode_hex_compact(signature.serialize().as_ref()),
//                "attestation": {
//                    "version": 1,
//                    "provider": "SGX",
//                    "payload": map
//                }
            })
        },
        Err(payload) => {
            let s_payload = payload.to_string();

            let hash_payload = rsgx_sha256_slice(&s_payload.as_bytes()).unwrap();
            let message = secp256k1::Message::parse_slice(&hash_payload[..32]).unwrap();
            let (signature, _recovery_id) = secp256k1::sign(&message, &global_state.private_key);

            json!({
                "status": "error",
                "payload": s_payload,
                "signature": hex::encode_hex_compact(signature.serialize().as_ref()),
//                "attestation": {
//                    "version": 1,
//                    "provider": "SGX",
//                    "payload": map
//                }
            })
        }
    };
    println!("{}", output_json.to_string());

    let output_json_vec = serde_json::to_vec(&output_json).unwrap();
    let output_json_vec_len = output_json_vec.len();
    let output_json_vec_len_ptr = &output_json_vec_len as *const usize;

    unsafe {
        ptr::copy_nonoverlapping(output_json_vec.as_ptr(),
                                 output_ptr,
                                 output_json_vec_len);
        ptr::copy_nonoverlapping(output_json_vec_len_ptr,
                                 output_len_ptr,
                                 std::mem::size_of_val(&output_json_vec_len));
    }

    sgx_status_t::SGX_SUCCESS
}


// --------------------------------

fn error_msg(msg: &str) -> Value {
    json!({ "message": msg })
}

fn unknown() -> Result<Value, Value> {
    Err(json!({
        "message": "Unknown action"
    }))
}

fn test(input: &Map<String, Value>) -> Result<Value, Value> {
    test_parse_block();
    Ok(json!({}))
}

const HARD_CODE_PASS: &[u8] = b"password";
const HARD_CODE_IV: &[u8] = b"iv";
const SECRET: &[u8; 32] = b"24e3e78e1f15150cdbad02f3205f6dd0";

const SECRET_ALICE: &[u8; 32] = b"00000000000000000000000000000001";
const SECRET_BOB: &[u8; 32] = b"00000000000000000000000000000002";

fn dump_states(input: &Map<String, Value>) -> Result<Value, Value> {
    let mut sessions = STATE.lock().unwrap();
    let serialized = serde_json::to_string(&*sessions).unwrap();

    // Your private data
    let content = serialized.as_bytes().to_vec();
    println!("Content to encrypt's size {}", content.len());
    println!("{}", serialized);

    // Ring uses the same input variable as output
    let mut in_out = content.clone();
    println!("in_out len {}", in_out.len());

    // Random data must be used only once per encryption
    let mut nonce_vec = [0 as u8; 12];

    // Fill nonce with random data
    let rand = ring::rand::SystemRandom::new();
    rand.fill(&mut nonce_vec).unwrap();
    let nonce = ring::aead::Nonce::assume_unique_for_key(nonce_vec);

    let unbound_key = ring::aead::UnboundKey::new(&ring::aead::AES_256_GCM, SECRET).unwrap();
    let key = ring::aead::LessSafeKey::new(unbound_key);

    key.seal_in_place_separate_tag(
        nonce,
        ring::aead::Aad::empty(),
        &mut in_out
    ).map(|tag| in_out.extend(tag.as_ref()));

    Ok(json!({
        "data": hex::encode_hex_compact(in_out.as_ref()),
        "nonce": hex::encode_hex_compact(&nonce_vec)
    }))
}

fn load_states(input: &Map<String, Value>) -> Result<Value, Value> {
    let unbound_key = ring::aead::UnboundKey::new(&ring::aead::AES_256_GCM, SECRET).unwrap();
    let key = ring::aead::LessSafeKey::new(unbound_key);

    let nonce_vec = hex::decode_hex(input.get("nonce").unwrap().as_str().unwrap());
    let mut nonce_arr= [0u8; 12];
    nonce_arr.copy_from_slice(&nonce_vec[..12]);
    let nonce = ring::aead::Nonce::assume_unique_for_key(nonce_arr);

    println!("{}", input.get("data").unwrap().as_str().unwrap());
    let mut in_out = hex::decode_hex(input.get("data").unwrap().as_str().unwrap());
    let decrypted_data = key.open_in_place(
        nonce,
        ring::aead::Aad::empty(),
        &mut in_out,
    ).unwrap();
    println!("{}", String::from_utf8(decrypted_data.to_vec()).unwrap());

    let deserialized: RuntimeState = serde_json::from_slice(decrypted_data).unwrap();

    println!("{}", serde_json::to_string_pretty(&deserialized).unwrap());

    let mut sessions = STATE.lock().unwrap();
    std::mem::replace(&mut *sessions, deserialized);

    Ok(json!({}))
}

fn init_runtime(input: &Map<String, Value>) -> Result<Value, Value> {
    // TODO: Guard only initialize once
    if GLOBAL_STATE.lock().unwrap().initialized {
        return Err(json!({"message": "Already initialized"}))
    }

    Ok(json!({}))
//    let mut prng = rand_os::OsRng::new().unwrap();
//
//    let sk = SecretKey::random(&mut prng);
//    let pk = PublicKey::from_secret_key(&sk);
//
//    let (attn_report, sig, cert) = match create_attestation_report(&pk.serialize_compressed()) {
//        Ok(r) => r,
//        Err(e) => {
//            println!("Error in create_attestation_report: {:?}", e);
//            return Err(json!({"message": "Error while connecting to IAS"}))
//        }
//    };
//
//    let mut global_state = GLOBAL_STATE.lock().unwrap();
//    (*global_state).initialized = true;
//    *global_state.public_key = pk.clone();
//    *global_state.private_key = sk.clone();
//
//    let mut map = serde_json::Map::new();
//    map.insert("report".to_owned(), json!(attn_report));
//    map.insert("signature".to_owned(), json!(sig));
//    map.insert("signing_cert".to_owned(), json!(cert));
//
//    let s_pk = hex::encode_hex_compact(pk.serialize_compressed().as_ref());
//    // let s_sk = hex::encode_hex_compact(sk.serialize().as_ref());
//
//    Ok(
//        json!({
//            "public_key": s_pk,
//            "attestation": {
//                "version": 1,
//                "provider": "SGX",
//                "payload": map
//            }
//        })
//    )
}

/*
bf192ec197592fda420383b1db2676e5b409a58cad489cc5c35dd94390c65a584c10987af003bd853a29992021a2f1a617b798ad3a151a3e2cbd6e4029370b7c68ec05dc5c1a11c17373b8d18d498fc1da0e94c35bdb3adc8116efda35b6025f3a080661757261206962a60f00000000056175726101013e3e4b72e1a133d129799aeaf43884493b6c30530f04be95f61089d73f21de2709108c407b8cb891ac8dbd1a498f1b6792f54e6f3b4f499d2e7010c81a902d8404280401000bf07ca2cb6e0101d90456000000000000007b968b3f7eca2df8cf39ee8c9538f02e1361f44d0b809450f8501676db28131a13000000087b968b3f7eca2df8cf39ee8c9538f02e1361f44d0b809450f8501676db28131a130000008dff9b2ca754cb5e80036647aab3fd25ccc4e4236c9fecd61968f121141149a7c402ebd3cdd43779a7e8d9afffee1989197d343ed9e936721168305a6da09d0188dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee7b968b3f7eca2df8cf39ee8c9538f02e1361f44d0b809450f8501676db28131a1300000090a870eb3be217aa99476919b2864539830a622e82a9355d0f91f28a777372ecb0c3a1164f68246cfa5577682ab7505aff139681953fe0cc7457300a53807303d17c2d7823ebf260fd138f2d7e27d114c0145d968b5ff5006125f2414fadae6900
SignedBlock {
    block: Block {
        header: Header {
            parent_hash: 0xbf192ec197592fda420383b1db2676e5b409a58cad489cc5c35dd94390c65a58,
            number: 19,
            state_root: 0x10987af003bd853a29992021a2f1a617b798ad3a151a3e2cbd6e4029370b7c68,
            extrinsics_root: 0xec05dc5c1a11c17373b8d18d498fc1da0e94c35bdb3adc8116efda35b6025f3a,
            digest: Digest {
                logs: [
                    DigestItem::PreRuntime([97, 117, 114, 97], [105, 98, 166, 15, 0, 0, 0, 0]),
                    DigestItem::Seal([97, 117, 114, 97], [62, 62, 75, 114, 225, 161, 51, 209, 41, 121, 154, 234, 244, 56, 132, 73, 59, 108, 48, 83, 15, 4, 190, 149, 246, 16, 137, 215, 63, 33, 222, 39, 9, 16, 140, 64, 123, 140, 184, 145, 172, 141, 189, 26, 73, 143, 27, 103, 146, 245, 78, 111, 59, 79, 73, 157, 46, 112, 16, 200, 26, 144, 45, 132])
                ]
            }
        },
        extrinsics: [
            UncheckedExtrinsic(None, Call::Timestamp(set(1575374454000,)))
        ]
    },
    justification: Some([86, 0, 0, 0, 0, 0, 0, 0, 123, 150, 139, 63, 126, 202, 45, 248, 207, 57, 238, 140, 149, 56, 240, 46, 19, 97, 244, 77, 11, 128, 148, 80, 248, 80, 22, 118, 219, 40, 19, 26, 19, 0, 0, 0, 8, 123, 150, 139, 63, 126, 202, 45, 248, 207, 57, 238, 140, 149, 56, 240, 46, 19, 97, 244, 77, 11, 128, 148, 80, 248, 80, 22, 118, 219, 40, 19, 26, 19, 0, 0, 0, 141, 255, 155, 44, 167, 84, 203, 94, 128, 3, 102, 71, 170, 179, 253, 37, 204, 196, 228, 35, 108, 159, 236, 214, 25, 104, 241, 33, 20, 17, 73, 167, 196, 2, 235, 211, 205, 212, 55, 121, 167, 232, 217, 175, 255, 238, 25, 137, 25, 125, 52, 62, 217, 233, 54, 114, 17, 104, 48, 90, 109, 160, 157, 1, 136, 220, 52, 23, 213, 5, 142, 196, 180, 80, 62, 12, 18, 234, 26, 10, 137, 190, 32, 15, 233, 137, 34, 66, 61, 67, 52, 1, 79, 166, 176, 238, 123, 150, 139, 63, 126, 202, 45, 248, 207, 57, 238, 140, 149, 56, 240, 46, 19, 97, 244, 77, 11, 128, 148, 80, 248, 80, 22, 118, 219, 40, 19, 26, 19, 0, 0, 0, 144, 168, 112, 235, 59, 226, 23, 170, 153, 71, 105, 25, 178, 134, 69, 57, 131, 10, 98, 46, 130, 169, 53, 93, 15, 145, 242, 138, 119, 115, 114, 236, 176, 195, 161, 22, 79, 104, 36, 108, 250, 85, 119, 104, 42, 183, 80, 90, 255, 19, 150, 129, 149, 63, 224, 204, 116, 87, 48, 10, 83, 128, 115, 3, 209, 124, 45, 120, 35, 235, 242, 96, 253, 19, 143, 45, 126, 39, 209, 20, 192, 20, 93, 150, 139, 95, 245, 0, 97, 37, 242, 65, 79, 173, 174, 105, 0])
}
*/

mod contract;
mod types;
use types::TxRef;

extern crate codec;
extern crate runtime as chain;
use crate::codec::Decode;

extern crate sp_runtime;
use crate::sp_runtime::generic::Header;

use std::fmt;

fn fmt_call(call: &chain::Call) -> String {
    match call {
        chain::Call::Timestamp(chain::TimestampCall::set(t)) =>
            format!("Timestamp::set({})", t),
        chain::Call::Balances(chain::BalancesCall::transfer(to, amount)) =>
            format!("Balance::transfer({:?}, {:?})", to, amount),
        _ => String::from("<Unparsed>")
    }
}

fn print_block(signed_block: &chain::SignedBlock) {
    let header: &chain::Header = &signed_block.block.header;
    let extrinsics: &Vec<chain::UncheckedExtrinsic> = &signed_block.block.extrinsics;

    println!("SignedBlock {{");
    println!("  block {{");
    println!("    header {{");
    println!("      number: {}", header.number);
    println!("      extrinsics_root: {}", header.extrinsics_root);
    println!("      state_root: {}", header.state_root);
    println!("      parent_hash: {}", header.parent_hash);
    println!("      digest: logs[{}]", header.digest.logs.len());
    println!("  extrinsics: [");
    for extrinsic in extrinsics {
        println!("    UncheckedExtrinsic {{");
        println!("      function: {}", fmt_call(&extrinsic.function));
        println!("      signature: {:?}", extrinsic.signature);
        println!("    }}");
    }
    println!("  ]");
    println!("  justification: <skipped...>");
    println!("}}");
}

fn parse_block(data: &Vec<u8>) -> Result<chain::SignedBlock, crate::codec::Error> {
    chain::SignedBlock::decode(&mut data.as_slice())
}

fn format_address(addr: &chain::Address) -> String {
    match addr {
        chain::Address::Id(id) => hex::encode_hex_compact(id.as_ref()),
        chain::Address::Index(index) => format!("index:{}", index)
    }
}

fn test_parse_block() {
    let raw_block: Vec<u8> = base64::decode("iAKMDRPbdbAZ0eev9OZ1QgaAkoEnazAp6JzH2GeRFYdsR+pFUBbOaAW0+k5K+jPtsEr/P/JKJQDSobnB98Qhf8ug8HkDygkapC5T++CNvzYORIFimatwYSu/U53t66xzpQgGYXVyYSCGvagPAAAAAAVhdXJhAQEuXZ5zy2+qk+60y+/m1r0oZv/+LEiDCxMotfkvjP9aebuUVxBTmd2LCpu645AAjpRUNhqOmVuiKreUoV1aMpWLCCgEAQALoPTZAm8BQQKE/9Q1k8cV/dMcYRQavQSpn9aCLIVYhUzN45pWhOelbaJ9AU5gayhZiGwAEAthrYW6Ucm+acGAR3whdfUk17jp4NMearo4+NxR2w0VsVkEF0gQ/U6AHggnM+BZmvrhhMdSygqlAQAABAD/jq8EFRaHc2Mmyf6hfiX8UodhNpPJEpCcsiaqR5TyakgHABCl1OgA")
                                    .unwrap();
    println!("SignedBlock data[{}]", raw_block.len());
    let block = match parse_block(&raw_block) {
        Ok(b) => b,
        Err(err) => {
            println!("test_parse_block: Failed to parse block ({:?})", err);
            return;
        }
    };
    print_block(&block);

    // test parse address
    let ref_sig = block.block.extrinsics[1].signature.as_ref().unwrap();
    let ref_addr = &ref_sig.0;
    println!("test_parse_block: addr = {}", format_address(ref_addr));

    let cmd_json = serde_json::to_string_pretty(
        &contract::Command::List(contract::ItemDetails {
            name: "nname".to_owned(),
            category: "ccategory".to_owned(),
            description: "ddesc".to_owned(),
            price: contract::PricePolicy::PerRow { price: 100_u128 },
            dataset_link: "llink".to_owned(),
            dataset_preview: "pprev".to_owned()
        })).expect("jah");
    println!("sample command: {}", cmd_json);
}

const CONTRACT_ONE: u32 = 1;

fn handle_execution(state : &mut RuntimeState, pos: &TxRef,
                    origin: Option<(chain::Address, chain::Signature, chain::SignedExtra)>,
                    contract_id: u32, payload: &Vec<u8>) {
    if contract_id != CONTRACT_ONE {
        println!("handle_executiioin: Skipped unknown contract: {}", contract_id);
        return
    }
    let cmd: contract::Command = serde_json::from_slice(payload.as_slice()).unwrap();
    // TODO: handle error ^^

    let addr = &origin.unwrap().0;
    let origin = format_address(&addr);

    println!("handle_execution: about to call handle_command");
    state.contract.handle_command(&origin, pos, cmd)
}

fn dispatch(block: &chain::SignedBlock) {
    let ref mut state = STATE.lock().unwrap();
    for (i, xt) in block.block.extrinsics.iter().enumerate() {
        if let chain::Call::Execution(chain::ExecutionCall::push_command(contract_id, payload)) = &xt.function {
            println!("push_command(contract_id: {}, payload: data[{}])", contract_id, payload.len());
            let pos = TxRef {
                blocknum: block.block.header.number,
                index: i as u32
            };
            handle_execution(state, &pos, xt.signature.clone(), *contract_id, payload);
        }
        // skip other unknown extrinsics
    }
}

fn sync_block(input: &Map<String, Value>) -> Result<Value, Value> {
    /*
        input: {
            data: "<b64 encoded plain data>"
        }
    */
    let block_b64 = input.get("data").unwrap().as_str().unwrap();
    let block_data = base64::decode(block_b64).unwrap();
    // TODO: handle error ^^

    let block = parse_block(&block_data).map_err(|_| error_msg("Invalid block (err)"))?;

    // it's the block needed
    let mut global_state = GLOBAL_STATE.lock().unwrap();
    if block.block.header.number != global_state.blocknum {
        return Err(error_msg("Unexpected block"))
    }

    // TODO: validate the block (light client validation logic here)

    // trigger updates
    dispatch(&block);

    // move forward
    (*global_state).blocknum = block.block.header.number + 1;

    Ok(json!({
        "synced_to": block.block.header.number
    }))
}

fn get_info(input: &Map<String, Value>) -> Result<Value, Value> {
    let global_state = GLOBAL_STATE.lock().unwrap();

    let initialized = global_state.initialized;
    let pk = &global_state.public_key;
    let s_pk = hex::encode_hex_compact(pk.serialize_compressed().as_ref());
    let blocknum = global_state.blocknum;

    Ok(json!({
        "initialized": initialized,
        "public_key": s_pk,
        "blocknum": blocknum
    }))
}

fn query(input: &Map<String, Value>) -> Result<Value, Value> {
    let mut state = STATE.lock().unwrap();

    let err = error_msg("Malformed request");

    let req_value = input.get("request").ok_or(err.clone())?.clone();
    let req: contract::Request = serde_json::from_value(req_value).map_err(|_| err)?;
    let res = state.contract.query(req);
    let res_value = serde_json::to_value(res).unwrap();

    Ok(res_value)
}

fn get(input: &Map<String, Value>) -> Result<Value, Value> {
    let mut state = STATE.lock().unwrap();
    let path = input.get("path").unwrap().as_str().unwrap();

    let data = match state.contract.get(&path.to_string()) {
        Some(d) => d,
        None => {
            return Err(error_msg("Data doesn't exist"))
        }
    };

    let data_b64 = base64::encode(data);

    Ok(json!({
        "path": path.to_string(),
        "value": data_b64
    }))
}

fn set(input: &Map<String, Value>) -> Result<Value, Value> {
    let mut state = STATE.lock().unwrap();
    let path = input.get("path").unwrap().as_str().unwrap();
    let data_b64 = input.get("data").unwrap().as_str().unwrap();

    let data = base64::decode(data_b64).map_err(|_| error_msg("Failed to decode base64 data"))?;
    state.contract.set(path.to_string(), data);

    Ok(json!({
        "path": path.to_string(),
        "data": data_b64.to_string()
    }))
}
