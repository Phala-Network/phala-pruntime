use std::prelude::v1::*;
use bitvec::testing::*;

//#[test]
pub fn minor() {
	let data: u8 = 0u8;
	let bp = BitPtr::new(&data, 1, 6);

	assert!(bp.domain_kind().is_minor());
}

//#[test]
pub fn major() {
	let data: &[u16] = &[0u16, !0u16];
	let bp = BitPtr::new(&data[0], 1, 28);

	assert!(bp.domain_kind().is_major());
}

//#[test]
pub fn partial_head() {
	let data: u32 = 0u32;
	let bp = BitPtr::new(&data, 4, 28);

	assert!(bp.domain_kind().is_partial_head());

	let data: &[u32] = &[0u32, !0u32];
	let bp = BitPtr::new(&data[0], 4, 60);

	assert!(bp.domain_kind().is_partial_head());
}

//#[test]
pub fn partial_tail() {
	let data: u64 = 0u64;
	let bp = BitPtr::new(&data, 0, 60);

	assert!(bp.domain_kind().is_partial_tail());

	let data: &[u64] = &[0u64, !0u64];
	let bp = BitPtr::new(&data[0], 0, 124);

	assert!(bp.domain_kind().is_partial_tail());
}

//#[test]
pub fn spanning() {
	let data: u8 = 0u8;
	let bp = BitPtr::new(&data, 0, 8);

	assert!(bp.domain_kind().is_spanning());

	let data: &[u16] = &[0u16, !0u16];
	let bp = BitPtr::new(&data[0], 0, 32);

	assert!(bp.domain_kind().is_spanning());
}
