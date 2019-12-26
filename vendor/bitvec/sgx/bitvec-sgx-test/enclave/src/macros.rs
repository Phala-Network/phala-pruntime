use bitvec::testing::*;

//#[test]
pub fn compile_bitvec_macros() {
	bitvec![0, 1];
	bitvec![BigEndian; 0, 1];
	bitvec![LittleEndian; 0, 1];
	bitvec![BigEndian, u8; 0, 1];
	bitvec![LittleEndian, u8; 0, 1];
	bitvec![BigEndian, u16; 0, 1];
	bitvec![LittleEndian, u16; 0, 1];
	bitvec![BigEndian, u32; 0, 1];
	bitvec![LittleEndian, u32; 0, 1];
	bitvec![BigEndian, u64; 0, 1];
	bitvec![LittleEndian, u64; 0, 1];

	bitvec![1; 70];
	bitvec![BigEndian; 0; 70];
	bitvec![LittleEndian; 1; 70];
	bitvec![BigEndian, u8; 0; 70];
	bitvec![LittleEndian, u8; 1; 70];
	bitvec![BigEndian, u16; 0; 70];
	bitvec![LittleEndian, u16; 1; 70];
	bitvec![BigEndian, u32; 0; 70];
	bitvec![LittleEndian, u32; 1; 70];
	bitvec![BigEndian, u64; 0; 70];
	bitvec![LittleEndian, u64; 1; 70];
}

//#[test]
pub fn compile_bitbox_macros() {
	bitbox![0, 1];
	bitbox![BigEndian; 0, 1];
	bitbox![LittleEndian; 0, 1];
	bitbox![BigEndian, u8; 0, 1];
	bitbox![LittleEndian, u8; 0, 1];
	bitbox![BigEndian, u16; 0, 1];
	bitbox![LittleEndian, u16; 0, 1];
	bitbox![BigEndian, u32; 0, 1];
	bitbox![LittleEndian, u32; 0, 1];
	bitbox![BigEndian, u64; 0, 1];
	bitbox![LittleEndian, u64; 0, 1];

	bitbox![1; 70];
	bitbox![BigEndian; 0; 70];
	bitbox![LittleEndian; 1; 70];
	bitbox![BigEndian, u8; 0; 70];
	bitbox![LittleEndian, u8; 1; 70];
	bitbox![BigEndian, u16; 0; 70];
	bitbox![LittleEndian, u16; 1; 70];
	bitbox![BigEndian, u32; 0; 70];
	bitbox![LittleEndian, u32; 1; 70];
	bitbox![BigEndian, u64; 0; 70];
	bitbox![LittleEndian, u64; 1; 70];
}
