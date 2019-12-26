use bitvec::testing::*;

const PTR_BITS: usize = std::mem::size_of::<*const u8>() * 8;

//#[test]
pub fn associated_consts_u8() {
	assert_eq!(BitPtr::<u8>::PTR_DATA_BITS, PTR_BITS);
	assert_eq!(BitPtr::<u8>::PTR_HEAD_BITS, 0);

	assert_eq!(BitPtr::<u8>::PTR_DATA_MASK, !0);
	assert_eq!(BitPtr::<u8>::PTR_HEAD_MASK, 0);
}

//#[test]
pub fn associated_consts_u16() {
	assert_eq!(BitPtr::<u16>::PTR_DATA_BITS, PTR_BITS - 1);
	assert_eq!(BitPtr::<u16>::PTR_HEAD_BITS, 1);

	assert_eq!(BitPtr::<u16>::PTR_DATA_MASK, !0 << 1);
	assert_eq!(BitPtr::<u16>::PTR_HEAD_MASK, 1);
}

//#[test]
pub fn associated_consts_u32() {
	assert_eq!(BitPtr::<u32>::PTR_DATA_BITS, PTR_BITS - 2);
	assert_eq!(BitPtr::<u32>::PTR_HEAD_BITS, 2);

	assert_eq!(BitPtr::<u32>::PTR_DATA_MASK, !0 << 2);
	assert_eq!(BitPtr::<u32>::PTR_HEAD_MASK, 3);
}

//#[test]
pub fn associated_consts_u64() {
	assert_eq!(BitPtr::<u64>::PTR_DATA_BITS, PTR_BITS - 3);
	assert_eq!(BitPtr::<u64>::PTR_HEAD_BITS, 3);

	assert_eq!(BitPtr::<u64>::PTR_DATA_MASK, !0 << 3);
	assert_eq!(BitPtr::<u64>::PTR_HEAD_MASK, 7);
}

//#[test]
pub fn ctors() {
	let data: [u32; 4] = [0x756c6153, 0x2c6e6f74, 0x6e6f6d20, 0x00216f64];
	let bp = BitPtr::<u32>::new(&data as *const u32, 0, 32 * 4);
	assert_eq!(bp.pointer().r(), &data as *const u32);
	assert_eq!(bp.elements(), 4);
	assert_eq!(*bp.head(), 0);
	assert_eq!(*bp.tail(), 32);
}

//#[test]
pub fn empty() {
	let data = [0u8; 4];
	//  anything with 0 bits is unconditionally empty
	let bp = BitPtr::<u8>::new(&data as *const u8, 2, 0);

	assert!(bp.is_empty());
	assert_eq!(*bp.head(), 2);
	assert_eq!(*bp.tail(), 2);
}

//#[test]
//#[should_panic]
pub fn overfull() {
	BitPtr::<u32>::new(8 as *const u32, 1, BitPtr::<u32>::MAX_BITS + 1);
}
