use core::mem;

use bitvec::{vec::BitVec, slice::BitSlice, boxed::BitBox};

use parity_scale_codec::{Encode, Decode};

use bitvec::{bitvec, cursor::BigEndian};

// Calculates bytes required to store given amount of `bits` as if they were stored in the array of `T`.
fn required_bytes<T>(bits: usize) -> usize {
    let element_bits = mem::size_of::<T>() * 8;
    (bits + element_bits - 1) / element_bits * mem::size_of::<T>()
}

macro_rules! test_data {
		($inner_type: ty) => (
			[
				BitVec::<BigEndian, $inner_type>::new(),
				bitvec![BigEndian, $inner_type; 0],
				bitvec![BigEndian, $inner_type; 1],
				bitvec![BigEndian, $inner_type; 0, 0],
				bitvec![BigEndian, $inner_type; 1, 0],
				bitvec![BigEndian, $inner_type; 0, 1],
				bitvec![BigEndian, $inner_type; 1, 1],
				bitvec![BigEndian, $inner_type; 1, 0, 1],
				bitvec![BigEndian, $inner_type; 0, 1, 0, 1, 0, 1, 1],
				bitvec![BigEndian, $inner_type; 0, 1, 0, 1, 0, 1, 1, 0],
				bitvec![BigEndian, $inner_type; 1, 1, 0, 1, 0, 1, 1, 0, 1],
				bitvec![BigEndian, $inner_type; 1, 0, 1, 0, 1, 1, 0, 0, 1, 0, 1, 0, 1, 1, 0],
				bitvec![BigEndian, $inner_type; 0, 1, 0, 1, 0, 1, 1, 0, 0, 1, 0, 1, 0, 1, 1, 0],
				bitvec![BigEndian, $inner_type; 0, 1, 0, 1, 0, 1, 1, 0, 0, 1, 0, 1, 0, 1, 1, 0, 0],
				bitvec![BigEndian, $inner_type; 0; 15],
				bitvec![BigEndian, $inner_type; 1; 16],
				bitvec![BigEndian, $inner_type; 0; 17],
				bitvec![BigEndian, $inner_type; 1; 31],
				bitvec![BigEndian, $inner_type; 0; 32],
				bitvec![BigEndian, $inner_type; 1; 33],
				bitvec![BigEndian, $inner_type; 0; 63],
				bitvec![BigEndian, $inner_type; 1; 64],
				bitvec![BigEndian, $inner_type; 0; 65],
			]
		)
	}

pub fn required_bytes_test() {
    assert_eq!(0, required_bytes::<u8>(0));
    assert_eq!(1, required_bytes::<u8>(1));
    assert_eq!(1, required_bytes::<u8>(7));
    assert_eq!(1, required_bytes::<u8>(8));
    assert_eq!(2, required_bytes::<u8>(9));

    assert_eq!(0, required_bytes::<u16>(0));
    assert_eq!(2, required_bytes::<u16>(1));
    assert_eq!(2, required_bytes::<u16>(15));
    assert_eq!(2, required_bytes::<u16>(16));
    assert_eq!(4, required_bytes::<u16>(17));

    assert_eq!(0, required_bytes::<u32>(0));
    assert_eq!(4, required_bytes::<u32>(1));
    assert_eq!(4, required_bytes::<u32>(31));
    assert_eq!(4, required_bytes::<u32>(32));
    assert_eq!(8, required_bytes::<u32>(33));

    assert_eq!(0, required_bytes::<u64>(0));
    assert_eq!(8, required_bytes::<u64>(1));
    assert_eq!(8, required_bytes::<u64>(63));
    assert_eq!(8, required_bytes::<u64>(64));
    assert_eq!(16, required_bytes::<u64>(65));
}

pub fn bitvec_u8() {
    for v in &test_data!(u8) {
        let encoded = v.encode();
        assert_eq!(*v, BitVec::<BigEndian, u8>::decode(&mut &encoded[..]).unwrap());
    }
}

pub fn bitvec_u16() {
    for v in &test_data!(u16) {
        let encoded = v.encode();
        assert_eq!(*v, BitVec::<BigEndian, u16>::decode(&mut &encoded[..]).unwrap());
    }
}

pub fn bitvec_u32() {
    for v in &test_data!(u32) {
        let encoded = v.encode();
        assert_eq!(*v, BitVec::<BigEndian, u32>::decode(&mut &encoded[..]).unwrap());
    }
}

pub fn bitvec_u64() {
    for v in &test_data!(u64) {
        let encoded = v.encode();
        assert_eq!(*v, BitVec::<BigEndian, u64>::decode(&mut &encoded[..]).unwrap());
    }
}

pub fn bitslice() {
    let data: &[u8] = &[0x69];
    let slice: &BitSlice = data.into();
    let encoded = slice.encode();
    let decoded = BitVec::<BigEndian, u8>::decode(&mut &encoded[..]).unwrap();
    assert_eq!(slice, decoded.as_bitslice());
}

pub fn bitbox() {
    let data: &[u8] = &[5, 10];
    let bb: BitBox = data.into();
    let encoded = bb.encode();
    let decoded = BitBox::<BigEndian, u8>::decode(&mut &encoded[..]).unwrap();
    assert_eq!(bb, decoded);
}
