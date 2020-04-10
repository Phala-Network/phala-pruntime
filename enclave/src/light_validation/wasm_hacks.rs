use crate::std::slice;
use sp_core::hashing::blake2_256;
use sp_core::H256;
use parity_scale_codec::Encode;

use sc_executor::native_executor_instance;
// Our native executor instance.
native_executor_instance!(
	pub Executor,
	(sp_io::hashing::HostFunctions)
);


pub fn header_hash<H: Encode>(header: &H) -> H256 {
	let data = header.encode();
	blake2_256(&data.as_slice()).into()
}

// Ugly hack to emulate a wasm-interface
#[no_mangle]
pub extern "C" fn ext_blake2_256(data: *const u8, len: u32, out: *mut u8) {
    let result: [u8; 32] = if len == 0 {
      blake2_256(&[0u8; 0])
    } else {
      let slice = unsafe { slice::from_raw_parts(data, len as usize) };
      blake2_256(&slice)
    };
    let out_slice = unsafe { slice::from_raw_parts_mut(out, 32) };
    out_slice.copy_from_slice(&result);
}
