use std::prelude::v1::*;
use bitvec::testing::*;

//#[test]
pub fn be_u8_range() {
	assert_eq!(BigEndian::at::<u8>(BitIdx(0)), BitPos(7));
	assert_eq!(BigEndian::at::<u8>(BitIdx(1)), BitPos(6));
	assert_eq!(BigEndian::at::<u8>(BitIdx(2)), BitPos(5));
	assert_eq!(BigEndian::at::<u8>(BitIdx(3)), BitPos(4));
	assert_eq!(BigEndian::at::<u8>(BitIdx(4)), BitPos(3));
	assert_eq!(BigEndian::at::<u8>(BitIdx(5)), BitPos(2));
	assert_eq!(BigEndian::at::<u8>(BitIdx(6)), BitPos(1));
	assert_eq!(BigEndian::at::<u8>(BitIdx(7)), BitPos(0));
}

//#[test]
//#[should_panic]
pub fn be_u8_ovf() {
	BigEndian::at::<u8>(BitIdx(8));
}

//#[test]
pub fn be_u16_range() {
	assert_eq!(BigEndian::at::<u16>(BitIdx(0)), BitPos(15));
	assert_eq!(BigEndian::at::<u16>(BitIdx(1)), BitPos(14));
	assert_eq!(BigEndian::at::<u16>(BitIdx(2)), BitPos(13));
	assert_eq!(BigEndian::at::<u16>(BitIdx(3)), BitPos(12));
	assert_eq!(BigEndian::at::<u16>(BitIdx(4)), BitPos(11));
	assert_eq!(BigEndian::at::<u16>(BitIdx(5)), BitPos(10));
	assert_eq!(BigEndian::at::<u16>(BitIdx(6)), BitPos(9));
	assert_eq!(BigEndian::at::<u16>(BitIdx(7)), BitPos(8));
	assert_eq!(BigEndian::at::<u16>(BitIdx(8)), BitPos(7));
	assert_eq!(BigEndian::at::<u16>(BitIdx(9)), BitPos(6));
	assert_eq!(BigEndian::at::<u16>(BitIdx(10)), BitPos(5));
	assert_eq!(BigEndian::at::<u16>(BitIdx(11)), BitPos(4));
	assert_eq!(BigEndian::at::<u16>(BitIdx(12)), BitPos(3));
	assert_eq!(BigEndian::at::<u16>(BitIdx(13)), BitPos(2));
	assert_eq!(BigEndian::at::<u16>(BitIdx(14)), BitPos(1));
	assert_eq!(BigEndian::at::<u16>(BitIdx(15)), BitPos(0));
}

//#[test]
//#[should_panic]
pub fn be_u16_ovf() {
	BigEndian::at::<u16>(BitIdx(16));
}

//#[test]
pub fn be_u32_range() {
	assert_eq!(BigEndian::at::<u32>(BitIdx(0)), BitPos(31));
	assert_eq!(BigEndian::at::<u32>(BitIdx(1)), BitPos(30));
	assert_eq!(BigEndian::at::<u32>(BitIdx(2)), BitPos(29));
	assert_eq!(BigEndian::at::<u32>(BitIdx(3)), BitPos(28));
	assert_eq!(BigEndian::at::<u32>(BitIdx(4)), BitPos(27));
	assert_eq!(BigEndian::at::<u32>(BitIdx(5)), BitPos(26));
	assert_eq!(BigEndian::at::<u32>(BitIdx(6)), BitPos(25));
	assert_eq!(BigEndian::at::<u32>(BitIdx(7)), BitPos(24));
	assert_eq!(BigEndian::at::<u32>(BitIdx(8)), BitPos(23));
	assert_eq!(BigEndian::at::<u32>(BitIdx(9)), BitPos(22));
	assert_eq!(BigEndian::at::<u32>(BitIdx(10)), BitPos(21));
	assert_eq!(BigEndian::at::<u32>(BitIdx(11)), BitPos(20));
	assert_eq!(BigEndian::at::<u32>(BitIdx(12)), BitPos(19));
	assert_eq!(BigEndian::at::<u32>(BitIdx(13)), BitPos(18));
	assert_eq!(BigEndian::at::<u32>(BitIdx(14)), BitPos(17));
	assert_eq!(BigEndian::at::<u32>(BitIdx(15)), BitPos(16));
	assert_eq!(BigEndian::at::<u32>(BitIdx(16)), BitPos(15));
	assert_eq!(BigEndian::at::<u32>(BitIdx(17)), BitPos(14));
	assert_eq!(BigEndian::at::<u32>(BitIdx(18)), BitPos(13));
	assert_eq!(BigEndian::at::<u32>(BitIdx(19)), BitPos(12));
	assert_eq!(BigEndian::at::<u32>(BitIdx(20)), BitPos(11));
	assert_eq!(BigEndian::at::<u32>(BitIdx(21)), BitPos(10));
	assert_eq!(BigEndian::at::<u32>(BitIdx(22)), BitPos(9));
	assert_eq!(BigEndian::at::<u32>(BitIdx(23)), BitPos(8));
	assert_eq!(BigEndian::at::<u32>(BitIdx(24)), BitPos(7));
	assert_eq!(BigEndian::at::<u32>(BitIdx(25)), BitPos(6));
	assert_eq!(BigEndian::at::<u32>(BitIdx(26)), BitPos(5));
	assert_eq!(BigEndian::at::<u32>(BitIdx(27)), BitPos(4));
	assert_eq!(BigEndian::at::<u32>(BitIdx(28)), BitPos(3));
	assert_eq!(BigEndian::at::<u32>(BitIdx(29)), BitPos(2));
	assert_eq!(BigEndian::at::<u32>(BitIdx(30)), BitPos(1));
	assert_eq!(BigEndian::at::<u32>(BitIdx(31)), BitPos(0));
}

//#[test]
//#[should_panic]
pub fn be_u32_ovf() {
	BigEndian::at::<u32>(BitIdx(32));
}

//#[cfg(target_pointer_width = "64")]
//#[test]
pub fn be_u64_range() {
	assert_eq!(BigEndian::at::<u64>(BitIdx(0)), BitPos(63));
	assert_eq!(BigEndian::at::<u64>(BitIdx(1)), BitPos(62));
	assert_eq!(BigEndian::at::<u64>(BitIdx(2)), BitPos(61));
	assert_eq!(BigEndian::at::<u64>(BitIdx(3)), BitPos(60));
	assert_eq!(BigEndian::at::<u64>(BitIdx(4)), BitPos(59));
	assert_eq!(BigEndian::at::<u64>(BitIdx(5)), BitPos(58));
	assert_eq!(BigEndian::at::<u64>(BitIdx(6)), BitPos(57));
	assert_eq!(BigEndian::at::<u64>(BitIdx(7)), BitPos(56));
	assert_eq!(BigEndian::at::<u64>(BitIdx(8)), BitPos(55));
	assert_eq!(BigEndian::at::<u64>(BitIdx(9)), BitPos(54));
	assert_eq!(BigEndian::at::<u64>(BitIdx(10)), BitPos(53));
	assert_eq!(BigEndian::at::<u64>(BitIdx(11)), BitPos(52));
	assert_eq!(BigEndian::at::<u64>(BitIdx(12)), BitPos(51));
	assert_eq!(BigEndian::at::<u64>(BitIdx(13)), BitPos(50));
	assert_eq!(BigEndian::at::<u64>(BitIdx(14)), BitPos(49));
	assert_eq!(BigEndian::at::<u64>(BitIdx(15)), BitPos(48));
	assert_eq!(BigEndian::at::<u64>(BitIdx(16)), BitPos(47));
	assert_eq!(BigEndian::at::<u64>(BitIdx(17)), BitPos(46));
	assert_eq!(BigEndian::at::<u64>(BitIdx(18)), BitPos(45));
	assert_eq!(BigEndian::at::<u64>(BitIdx(19)), BitPos(44));
	assert_eq!(BigEndian::at::<u64>(BitIdx(20)), BitPos(43));
	assert_eq!(BigEndian::at::<u64>(BitIdx(21)), BitPos(42));
	assert_eq!(BigEndian::at::<u64>(BitIdx(22)), BitPos(41));
	assert_eq!(BigEndian::at::<u64>(BitIdx(23)), BitPos(40));
	assert_eq!(BigEndian::at::<u64>(BitIdx(24)), BitPos(39));
	assert_eq!(BigEndian::at::<u64>(BitIdx(25)), BitPos(38));
	assert_eq!(BigEndian::at::<u64>(BitIdx(26)), BitPos(37));
	assert_eq!(BigEndian::at::<u64>(BitIdx(27)), BitPos(36));
	assert_eq!(BigEndian::at::<u64>(BitIdx(28)), BitPos(35));
	assert_eq!(BigEndian::at::<u64>(BitIdx(29)), BitPos(34));
	assert_eq!(BigEndian::at::<u64>(BitIdx(30)), BitPos(33));
	assert_eq!(BigEndian::at::<u64>(BitIdx(31)), BitPos(32));
	assert_eq!(BigEndian::at::<u64>(BitIdx(32)), BitPos(31));
	assert_eq!(BigEndian::at::<u64>(BitIdx(33)), BitPos(30));
	assert_eq!(BigEndian::at::<u64>(BitIdx(34)), BitPos(29));
	assert_eq!(BigEndian::at::<u64>(BitIdx(35)), BitPos(28));
	assert_eq!(BigEndian::at::<u64>(BitIdx(36)), BitPos(27));
	assert_eq!(BigEndian::at::<u64>(BitIdx(37)), BitPos(26));
	assert_eq!(BigEndian::at::<u64>(BitIdx(38)), BitPos(25));
	assert_eq!(BigEndian::at::<u64>(BitIdx(39)), BitPos(24));
	assert_eq!(BigEndian::at::<u64>(BitIdx(40)), BitPos(23));
	assert_eq!(BigEndian::at::<u64>(BitIdx(41)), BitPos(22));
	assert_eq!(BigEndian::at::<u64>(BitIdx(42)), BitPos(21));
	assert_eq!(BigEndian::at::<u64>(BitIdx(43)), BitPos(20));
	assert_eq!(BigEndian::at::<u64>(BitIdx(44)), BitPos(19));
	assert_eq!(BigEndian::at::<u64>(BitIdx(45)), BitPos(18));
	assert_eq!(BigEndian::at::<u64>(BitIdx(46)), BitPos(17));
	assert_eq!(BigEndian::at::<u64>(BitIdx(47)), BitPos(16));
	assert_eq!(BigEndian::at::<u64>(BitIdx(48)), BitPos(15));
	assert_eq!(BigEndian::at::<u64>(BitIdx(49)), BitPos(14));
	assert_eq!(BigEndian::at::<u64>(BitIdx(50)), BitPos(13));
	assert_eq!(BigEndian::at::<u64>(BitIdx(51)), BitPos(12));
	assert_eq!(BigEndian::at::<u64>(BitIdx(52)), BitPos(11));
	assert_eq!(BigEndian::at::<u64>(BitIdx(53)), BitPos(10));
	assert_eq!(BigEndian::at::<u64>(BitIdx(54)), BitPos(9));
	assert_eq!(BigEndian::at::<u64>(BitIdx(55)), BitPos(8));
	assert_eq!(BigEndian::at::<u64>(BitIdx(56)), BitPos(7));
	assert_eq!(BigEndian::at::<u64>(BitIdx(57)), BitPos(6));
	assert_eq!(BigEndian::at::<u64>(BitIdx(58)), BitPos(5));
	assert_eq!(BigEndian::at::<u64>(BitIdx(59)), BitPos(4));
	assert_eq!(BigEndian::at::<u64>(BitIdx(60)), BitPos(3));
	assert_eq!(BigEndian::at::<u64>(BitIdx(61)), BitPos(2));
	assert_eq!(BigEndian::at::<u64>(BitIdx(62)), BitPos(1));
	assert_eq!(BigEndian::at::<u64>(BitIdx(63)), BitPos(0));
}

//#[cfg(target_pointer_width = "64")]
//#[test]
//#[should_panic]
pub fn be_u64_ovf() {
	BigEndian::at::<u64>(BitIdx(64));
}

//#[test]
pub fn le_u8_range() {
	assert_eq!(LittleEndian::at::<u8>(BitIdx(0)), BitPos(0));
	assert_eq!(LittleEndian::at::<u8>(BitIdx(1)), BitPos(1));
	assert_eq!(LittleEndian::at::<u8>(BitIdx(2)), BitPos(2));
	assert_eq!(LittleEndian::at::<u8>(BitIdx(3)), BitPos(3));
	assert_eq!(LittleEndian::at::<u8>(BitIdx(4)), BitPos(4));
	assert_eq!(LittleEndian::at::<u8>(BitIdx(5)), BitPos(5));
	assert_eq!(LittleEndian::at::<u8>(BitIdx(6)), BitPos(6));
	assert_eq!(LittleEndian::at::<u8>(BitIdx(7)), BitPos(7));
}

//#[test]
//#[should_panic]
pub fn le_u8_ovf() {
	LittleEndian::at::<u8>(BitIdx(8));
}

//#[test]
pub fn le_u16_range() {
	assert_eq!(LittleEndian::at::<u16>(BitIdx(0)), BitPos(0));
	assert_eq!(LittleEndian::at::<u16>(BitIdx(1)), BitPos(1));
	assert_eq!(LittleEndian::at::<u16>(BitIdx(2)), BitPos(2));
	assert_eq!(LittleEndian::at::<u16>(BitIdx(3)), BitPos(3));
	assert_eq!(LittleEndian::at::<u16>(BitIdx(4)), BitPos(4));
	assert_eq!(LittleEndian::at::<u16>(BitIdx(5)), BitPos(5));
	assert_eq!(LittleEndian::at::<u16>(BitIdx(6)), BitPos(6));
	assert_eq!(LittleEndian::at::<u16>(BitIdx(7)), BitPos(7));
	assert_eq!(LittleEndian::at::<u16>(BitIdx(8)), BitPos(8));
	assert_eq!(LittleEndian::at::<u16>(BitIdx(9)), BitPos(9));
	assert_eq!(LittleEndian::at::<u16>(BitIdx(10)), BitPos(10));
	assert_eq!(LittleEndian::at::<u16>(BitIdx(11)), BitPos(11));
	assert_eq!(LittleEndian::at::<u16>(BitIdx(12)), BitPos(12));
	assert_eq!(LittleEndian::at::<u16>(BitIdx(13)), BitPos(13));
	assert_eq!(LittleEndian::at::<u16>(BitIdx(14)), BitPos(14));
	assert_eq!(LittleEndian::at::<u16>(BitIdx(15)), BitPos(15));
}

//#[test]
//#[should_panic]
pub fn le_u16_ovf() {
	LittleEndian::at::<u16>(BitIdx(16));
}

//#[test]
pub fn le_u32_range() {
	assert_eq!(LittleEndian::at::<u32>(BitIdx(0)), BitPos(0));
	assert_eq!(LittleEndian::at::<u32>(BitIdx(1)), BitPos(1));
	assert_eq!(LittleEndian::at::<u32>(BitIdx(2)), BitPos(2));
	assert_eq!(LittleEndian::at::<u32>(BitIdx(3)), BitPos(3));
	assert_eq!(LittleEndian::at::<u32>(BitIdx(4)), BitPos(4));
	assert_eq!(LittleEndian::at::<u32>(BitIdx(5)), BitPos(5));
	assert_eq!(LittleEndian::at::<u32>(BitIdx(6)), BitPos(6));
	assert_eq!(LittleEndian::at::<u32>(BitIdx(7)), BitPos(7));
	assert_eq!(LittleEndian::at::<u32>(BitIdx(8)), BitPos(8));
	assert_eq!(LittleEndian::at::<u32>(BitIdx(9)), BitPos(9));
	assert_eq!(LittleEndian::at::<u32>(BitIdx(10)), BitPos(10));
	assert_eq!(LittleEndian::at::<u32>(BitIdx(11)), BitPos(11));
	assert_eq!(LittleEndian::at::<u32>(BitIdx(12)), BitPos(12));
	assert_eq!(LittleEndian::at::<u32>(BitIdx(13)), BitPos(13));
	assert_eq!(LittleEndian::at::<u32>(BitIdx(14)), BitPos(14));
	assert_eq!(LittleEndian::at::<u32>(BitIdx(15)), BitPos(15));
	assert_eq!(LittleEndian::at::<u32>(BitIdx(16)), BitPos(16));
	assert_eq!(LittleEndian::at::<u32>(BitIdx(17)), BitPos(17));
	assert_eq!(LittleEndian::at::<u32>(BitIdx(18)), BitPos(18));
	assert_eq!(LittleEndian::at::<u32>(BitIdx(19)), BitPos(19));
	assert_eq!(LittleEndian::at::<u32>(BitIdx(20)), BitPos(20));
	assert_eq!(LittleEndian::at::<u32>(BitIdx(21)), BitPos(21));
	assert_eq!(LittleEndian::at::<u32>(BitIdx(22)), BitPos(22));
	assert_eq!(LittleEndian::at::<u32>(BitIdx(23)), BitPos(23));
	assert_eq!(LittleEndian::at::<u32>(BitIdx(24)), BitPos(24));
	assert_eq!(LittleEndian::at::<u32>(BitIdx(25)), BitPos(25));
	assert_eq!(LittleEndian::at::<u32>(BitIdx(26)), BitPos(26));
	assert_eq!(LittleEndian::at::<u32>(BitIdx(27)), BitPos(27));
	assert_eq!(LittleEndian::at::<u32>(BitIdx(28)), BitPos(28));
	assert_eq!(LittleEndian::at::<u32>(BitIdx(29)), BitPos(29));
	assert_eq!(LittleEndian::at::<u32>(BitIdx(30)), BitPos(30));
	assert_eq!(LittleEndian::at::<u32>(BitIdx(31)), BitPos(31));
}

//#[test]
//#[should_panic]
pub fn le_u32_ovf() {
	LittleEndian::at::<u32>(BitIdx(32));
}

//#[cfg(target_pointer_width = "64")]
//#[test]
pub fn le_u64_range() {
	assert_eq!(LittleEndian::at::<u64>(BitIdx(0)), BitPos(0));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(1)), BitPos(1));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(2)), BitPos(2));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(3)), BitPos(3));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(4)), BitPos(4));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(5)), BitPos(5));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(6)), BitPos(6));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(7)), BitPos(7));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(8)), BitPos(8));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(9)), BitPos(9));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(10)), BitPos(10));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(11)), BitPos(11));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(12)), BitPos(12));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(13)), BitPos(13));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(14)), BitPos(14));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(15)), BitPos(15));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(16)), BitPos(16));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(17)), BitPos(17));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(18)), BitPos(18));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(19)), BitPos(19));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(20)), BitPos(20));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(21)), BitPos(21));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(22)), BitPos(22));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(23)), BitPos(23));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(24)), BitPos(24));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(25)), BitPos(25));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(26)), BitPos(26));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(27)), BitPos(27));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(28)), BitPos(28));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(29)), BitPos(29));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(30)), BitPos(30));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(31)), BitPos(31));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(32)), BitPos(32));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(33)), BitPos(33));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(34)), BitPos(34));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(35)), BitPos(35));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(36)), BitPos(36));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(37)), BitPos(37));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(38)), BitPos(38));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(39)), BitPos(39));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(40)), BitPos(40));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(41)), BitPos(41));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(42)), BitPos(42));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(43)), BitPos(43));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(44)), BitPos(44));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(45)), BitPos(45));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(46)), BitPos(46));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(47)), BitPos(47));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(48)), BitPos(48));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(49)), BitPos(49));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(50)), BitPos(50));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(51)), BitPos(51));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(52)), BitPos(52));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(53)), BitPos(53));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(54)), BitPos(54));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(55)), BitPos(55));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(56)), BitPos(56));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(57)), BitPos(57));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(58)), BitPos(58));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(59)), BitPos(59));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(60)), BitPos(60));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(61)), BitPos(61));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(62)), BitPos(62));
	assert_eq!(LittleEndian::at::<u64>(BitIdx(63)), BitPos(63));
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn le_u64_ovf() {
	LittleEndian::at::<u64>(BitIdx(64));
}
