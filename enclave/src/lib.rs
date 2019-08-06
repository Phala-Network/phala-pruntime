#![crate_name = "enclaveapp"]
#![crate_type = "staticlib"]

#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

#![warn(unused_imports)]
#![warn(unused_extern_crates)]

#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;
extern crate sgx_types;
extern crate sgx_trts;
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
extern crate serde_json;
#[macro_use]
extern crate lazy_static;
extern crate ring;

use sgx_types::*;
use sgx_tse::*;
use sgx_tcrypto::*;
use sgx_rand::*;

use std::prelude::v1::*;
use std::sync::Arc;
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
use secp256k1::curve::*;
use sgx_rand::thread_rng;
use secp256k1::{SecretKey, PublicKey};
use ring::aead::*;
use ring::pbkdf2::*;
use ring::rand::SystemRandom;
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

lazy_static! {
    static ref SESSIONS: SgxMutex<Map<String, Value>> = {
        let mut m = Map::new();
        SgxMutex::new(m)
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
const ACTION_REGISTER: u8 = 1;
const ACTION_STATUS: u8 = 2;
const ACTION_TRANSFER: u8 = 3;
const ACTION_DUMP_SESSIONS: u8 = 4;
const ACTION_LOAD_SESSIONS: u8 = 5;

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
    println!("----1----");
    let input_slice = unsafe { std::slice::from_raw_parts(input_ptr, input_len) };
    println!("----2----");
    let input_value: serde_json::value::Value = serde_json::from_slice(input_slice).unwrap();
    println!("----3----");
    let input = input_value.as_object().unwrap();
    println!("----4----");
    let payload = input_value.get("input").unwrap().as_object().unwrap();
    println!("----5----");

    let result = match action {
        ACTION_REGISTER => register(payload),
        ACTION_STATUS => status(payload),
        ACTION_TRANSFER => transfer(payload),
        ACTION_DUMP_SESSIONS => dump_sessions(payload),
        ACTION_LOAD_SESSIONS => load_sessions(payload),
        _ => unknown()
    };

    let nonce = input_value.get("nonce").unwrap().as_object().unwrap();
    let nonce_vec = serde_json::to_vec(&nonce).unwrap();
    let (attn_report, sig, cert) = create_attestation_report(&nonce_vec).unwrap();
    let mut map = serde_json::Map::new();
    map.insert("report".to_owned(), json!(attn_report));
    map.insert("signature".to_owned(), json!(sig));
    map.insert("signing_cert".to_owned(), json!(cert));

    let output_json = match result {
        Ok(payload) => json!({
            "status": "ok",
            "payload": payload.to_string(),
            "attestation": {
                "version": 1,
                "provider": "SGX",
                "payload": map
            }
        }),
        Err(payload) => json!({
            "status": "error",
            "payload": payload.to_string(),
            "attestation": {
                "version": 1,
                "provider": "SGX",
                "payload": map
            }
        })
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

fn unknown() -> Result<Value, Value> {
    Err(json!({
        "message": "Unknown action"
    }))
}

const DEFAULT_CURRENCY: u64 = 1000;

fn register(input: &Map<String, Value>) -> Result<Value, Value> {
    let mut sessions = SESSIONS.lock().unwrap();

    let mut prng = os::SgxRng::new().unwrap();
    let sk = SecretKey::random(&mut prng);
    let pk = PublicKey::from_secret_key(&sk);

    let s_pk = hex::encode_hex_compact(pk.serialize().as_ref());
    let s_sk = hex::encode_hex_compact(sk.serialize().as_ref());

    let account_currency = match sessions.get(&s_pk) {
        Some(r) => {
            r.as_u64().unwrap()
        },
        None => {
            DEFAULT_CURRENCY
        }
    };

    sessions.insert(s_pk.clone(), json!(account_currency));

    Ok(json!({
        "quantity": json!(account_currency),
        "account": s_pk,
        "sk": s_sk
        }))
}

fn status(input: &Map<String, Value>) -> Result<Value, Value> {
    let mut sessions = SESSIONS.lock().unwrap();

    let account_name = match input.get("account") {
        Some(n) => {
            n.as_str().unwrap()
        },
        None => {
            return Err(json!({
                "message": "Unknown account"
            }))
        }
    };

    match sessions.get(&account_name.to_string()) {
        Some(r) => {
            return Ok(json!({
                "account": account_name,
                "quantity": json!(r.as_u64().unwrap())
            }))
        },
        None => {
            return Err(json!({
                "message": "Unknown account"
            }))
        }
    };
}

fn transfer(input: &Map<String, Value>) -> Result<Value, Value> {
    let mut sessions = SESSIONS.lock().unwrap();

    let s_sk = input.get("sk").unwrap().as_str().unwrap();
    let sk = match SecretKey::parse_slice(hex::decode_hex(s_sk).as_slice()) {
        Ok(r) => {
            r
        },
        _ => {
            return Err(json!({
                "message": "Unknown account"
            }))
        }
    };

    let pk = PublicKey::from_secret_key(&sk);
    let s_pk = hex::encode_hex_compact(pk.serialize().as_ref());

    let account_currency = match sessions.get(&s_pk) {
        Some(r) => {
            r.as_u64().unwrap()
        },
        None => {
            0
        }
    };

    let quantity = input.get("quantity").unwrap().as_u64().unwrap();

    if account_currency < quantity {
        return Err(json!({
                "message": "Insufficient currency"
            }))
    }

    let to_account_name = input.get("to_account").unwrap().as_str().unwrap();
    let to_account_currency = match sessions.get(&to_account_name.to_string()) {
        Some(r) => {
            r.as_u64().unwrap()
        },
        None => {
            0
        }
    };

    sessions.insert(s_pk.clone(), json!(account_currency - quantity));
    sessions.insert(to_account_name.to_string(), json!(to_account_currency + quantity));

    Ok(json!({
        "account": s_pk,
        "quantity": json!(account_currency - quantity)
        }))
}

const SECRET: &[u8; 32] = b"24e3e78e1f15150cdbad02f3205f6dd0";

fn dump_sessions(input: &Map<String, Value>) -> Result<Value, Value> {
    let mut sessions = SESSIONS.lock().unwrap();
    let serialized = serde_json::to_string(&*sessions).unwrap();

    // Your private data
    let content = serialized.as_bytes().to_vec();
    println!("Content to encrypt's size {}", content.len());

    // Additional data that you would like to send and it would not be encrypted but it would
    // be signed
    let additional_data: [u8; 0] = [];

    // Ring uses the same input variable as output
    let mut in_out = content.clone();

    // The input/output variable need some space for a suffix
    println!("Tag len {}", CHACHA20_POLY1305.tag_len());
    for _ in 0..CHACHA20_POLY1305.tag_len() {
        in_out.push(0);
    }

    // Opening key used to decrypt data
    let opening_key = OpeningKey::new(&CHACHA20_POLY1305, SECRET).unwrap();

    // Sealing key used to encrypt data
    let sealing_key = SealingKey::new(&CHACHA20_POLY1305, SECRET).unwrap();

    // Random data must be used only once per encryption
    let mut nonce = vec![0; 12];

    // Fill nonce with random data
    let rand = SystemRandom::new();
    rand.fill(&mut nonce).unwrap();

    // Encrypt data into in_out variable
    let output_size = seal_in_place(
        &sealing_key,
        Nonce::try_assume_unique_for_key(&nonce).unwrap(),
        Aad::from(&additional_data),
        &mut in_out,
        CHACHA20_POLY1305.tag_len()
    ).unwrap();

    Ok(json!({
       "data": hex::encode_hex_compact(in_out.as_ref()),
       "nonce": hex::encode_hex_compact(nonce.as_ref())
    }))
}

fn load_sessions(input: &Map<String, Value>) -> Result<Value, Value> {
    // Additional data that you would like to send and it would not be encrypted but it would
    // be signed
    let additional_data: [u8; 0] = [];

    // Opening key used to decrypt data
    let opening_key = OpeningKey::new(&CHACHA20_POLY1305, SECRET).unwrap();

    println!("----1----");
    let nonce = hex::decode_hex(input.get("nonce").unwrap().as_str().unwrap());
    println!("nonce len {}", nonce.len());
    println!("----2----");
    let mut in_out = hex::decode_hex(input.get("data").unwrap().as_str().unwrap());
    println!("----3----");
    let decrypted_data = open_in_place(
        &opening_key,
        Nonce::try_assume_unique_for_key(&nonce).unwrap(),
        Aad::from(&additional_data),
        0,
        &mut in_out
    ).unwrap();
    println!("----4----");

    let deserialized: Map<String, Value> = serde_json::from_slice(decrypted_data).unwrap();

    println!("{}", serde_json::to_string_pretty(&deserialized).unwrap());

    let mut sessions = SESSIONS.lock().unwrap();
    std::mem::replace(&mut *sessions, deserialized);

    Ok(json!({}))
}
