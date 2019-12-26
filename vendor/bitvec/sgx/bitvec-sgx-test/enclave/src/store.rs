use std::prelude::v1::*;
use bitvec::testing::*;

//#[test]
pub fn jump_far_up() {
	//  isize::max_value() is 0x7f...ff, so the result bit will be one less
	//  than the start bit.
	for n in 1 .. 8 {
		let (elt, bit) = BitIdx::from(n).offset::<u8>(isize::max_value());
		assert_eq!(elt, (isize::max_value() >> u8::INDX) + 1);
		assert_eq!(*bit, n - 1);
	}
	let (elt, bit) = BitIdx::from(0).offset::<u8>(isize::max_value());
	assert_eq!(elt, isize::max_value() >> u8::INDX);
	assert_eq!(*bit, 7);
}

//#[test]
pub fn jump_far_down() {
	//  isize::min_value() is 0x80...00, so the result bit will be equal to
	//  the start bit
	for n in 0 .. 8 {
		let (elt, bit) = BitIdx::from(n).offset::<u8>(isize::min_value());
		assert_eq!(elt, isize::min_value() >> u8::INDX);
		assert_eq!(*bit, n);
	}
}

//#[test]
pub fn incr() {
	assert_eq!(BitIdx(6).incr::<u8>(), (BitIdx(7), false));
	assert_eq!(BitIdx(7).incr::<u8>(), (BitIdx(0), true));

	assert_eq!(BitIdx(14).incr::<u16>(), (BitIdx(15), false));
	assert_eq!(BitIdx(15).incr::<u16>(), (BitIdx(0), true));

	assert_eq!(BitIdx(30).incr::<u32>(), (BitIdx(31), false));
	assert_eq!(BitIdx(31).incr::<u32>(), (BitIdx(0), true));

	#[cfg(target_pointer_width = "64")]
	assert_eq!(BitIdx(62).incr::<u64>(), (BitIdx(63), false));
	#[cfg(target_pointer_width = "64")]
	assert_eq!(BitIdx(63).incr::<u64>(), (BitIdx(0), true));
}

//#[test]
pub fn incr_tail() {
	assert_eq!(BitIdx(7).incr_tail::<u8>(), (BitIdx(8), false));
	assert_eq!(BitIdx(8).incr_tail::<u8>(), (BitIdx(1), true));

	assert_eq!(BitIdx(15).incr_tail::<u16>(), (BitIdx(16), false));
	assert_eq!(BitIdx(16).incr_tail::<u16>(), (BitIdx(1), true));

	assert_eq!(BitIdx(31).incr_tail::<u32>(), (BitIdx(32), false));
	assert_eq!(BitIdx(32).incr_tail::<u32>(), (BitIdx(1), true));

	#[cfg(target_pointer_width = "64")]
	assert_eq!(BitIdx(63).incr_tail::<u64>(), (BitIdx(64), false));
	#[cfg(target_pointer_width = "64")]
	assert_eq!(BitIdx(64).incr_tail::<u64>(), (BitIdx(1), true));
}

//#[test]
pub fn decr() {
	assert_eq!(BitIdx(1).decr::<u8>(), (BitIdx(0), false));
	assert_eq!(BitIdx(0).decr::<u8>(), (BitIdx(7), true));

	assert_eq!(BitIdx(1).decr::<u16>(), (BitIdx(0), false));
	assert_eq!(BitIdx(0).decr::<u16>(), (BitIdx(15), true));

	assert_eq!(BitIdx(1).decr::<u32>(), (BitIdx(0), false));
	assert_eq!(BitIdx(0).decr::<u32>(), (BitIdx(31), true));

	#[cfg(target_pointer_width = "64")]
	assert_eq!(BitIdx(1).decr::<u64>(), (BitIdx(0), false));
	#[cfg(target_pointer_width = "64")]
	assert_eq!(BitIdx(0).decr::<u64>(), (BitIdx(63), true));
}

//#[test]
pub fn decr_tail() {
	assert_eq!(BitIdx(1).decr_tail::<u8>(), (BitIdx(8), true));
	assert_eq!(BitIdx(8).decr_tail::<u8>(), (BitIdx(7), false));

	assert_eq!(BitIdx(1).decr_tail::<u16>(), (BitIdx(16), true));
	assert_eq!(BitIdx(16).decr_tail::<u16>(), (BitIdx(15), false));

	assert_eq!(BitIdx(1).decr_tail::<u32>(), (BitIdx(32), true));
	assert_eq!(BitIdx(32).decr_tail::<u32>(), (BitIdx(31), false));

	#[cfg(target_pointer_width = "64")]
	assert_eq!(BitIdx(1).decr_tail::<u64>(), (BitIdx(64), true));
	#[cfg(target_pointer_width = "64")]
	assert_eq!(BitIdx(64).decr_tail::<u64>(), (BitIdx(63), false));
}

//#[test]
pub fn bits() {
	assert_eq!(u8::bits(false), 0);
	assert_eq!(u8::bits(true), u8::max_value());

	assert_eq!(u16::bits(false), 0);
	assert_eq!(u16::bits(true), u16::max_value());

	assert_eq!(u32::bits(false), 0);
	assert_eq!(u32::bits(true), u32::max_value());

	#[cfg(target_pointer_width = "64")]
	assert_eq!(u64::bits(false), 0);
	#[cfg(target_pointer_width = "64")]
	assert_eq!(u64::bits(true), u64::max_value());
}
