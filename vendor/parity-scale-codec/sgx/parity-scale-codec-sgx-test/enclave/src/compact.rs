use std::{
    string::String
};

use alloc::vec::Vec;

use serde::{Serialize, Deserialize};

use arrayvec::ArrayVec;

use parity_scale_codec::{
    Encode,
    Decode,
    Compact,
    CompactLen,
    CompactAs,
    HasCompact,
    CompactRef,
    Output,
    ArrayVecWrapper
};

pub fn compact_128_encoding_works() {
    let tests = [
        (0u128, 1usize), (63, 1), (64, 2), (16383, 2),
        (16384, 4), (1073741823, 4),
        (1073741824, 5), ((1 << 32) - 1, 5),
        (1 << 32, 6), (1 << 40, 7), (1 << 48, 8), ((1 << 56) - 1, 8), (1 << 56, 9), ((1 << 64) - 1, 9),
        (1 << 64, 10), (1 << 72, 11), (1 << 80, 12), (1 << 88, 13), (1 << 96, 14), (1 << 104, 15),
        (1 << 112, 16), ((1 << 120) - 1, 16), (1 << 120, 17), (u128::max_value(), 17)
    ];
    for &(n, l) in &tests {
        let encoded = Compact(n as u128).encode();
        assert_eq!(encoded.len(), l);
        assert_eq!(Compact::compact_len(&n), l);
        assert_eq!(<Compact<u128>>::decode(&mut &encoded[..]).unwrap().0, n);
    }
}

pub fn compact_64_encoding_works() {
    let tests = [
        (0u64, 1usize), (63, 1), (64, 2), (16383, 2),
        (16384, 4), (1073741823, 4),
        (1073741824, 5), ((1 << 32) - 1, 5),
        (1 << 32, 6), (1 << 40, 7), (1 << 48, 8), ((1 << 56) - 1, 8), (1 << 56, 9), (u64::max_value(), 9)
    ];
    for &(n, l) in &tests {
        let encoded = Compact(n as u64).encode();
        assert_eq!(encoded.len(), l);
        assert_eq!(Compact::compact_len(&n), l);
        assert_eq!(<Compact<u64>>::decode(&mut &encoded[..]).unwrap().0, n);
    }
}

pub fn compact_32_encoding_works() {
    let tests = [(0u32, 1usize), (63, 1), (64, 2), (16383, 2), (16384, 4), (1073741823, 4), (1073741824, 5), (u32::max_value(), 5)];
    for &(n, l) in &tests {
        let encoded = Compact(n as u32).encode();
        assert_eq!(encoded.len(), l);
        assert_eq!(Compact::compact_len(&n), l);
        assert_eq!(<Compact<u32>>::decode(&mut &encoded[..]).unwrap().0, n);
    }
}

pub fn compact_16_encoding_works() {
    let tests = [(0u16, 1usize), (63, 1), (64, 2), (16383, 2), (16384, 4), (65535, 4)];
    for &(n, l) in &tests {
        let encoded = Compact(n as u16).encode();
        assert_eq!(encoded.len(), l);
        assert_eq!(Compact::compact_len(&n), l);
        assert_eq!(<Compact<u16>>::decode(&mut &encoded[..]).unwrap().0, n);
    }
    assert!(<Compact<u16>>::decode(&mut &Compact(65536u32).encode()[..]).is_err());
}

pub fn compact_8_encoding_works() {
    let tests = [(0u8, 1usize), (63, 1), (64, 2), (255, 2)];
    for &(n, l) in &tests {
        let encoded = Compact(n as u8).encode();
        assert_eq!(encoded.len(), l);
        assert_eq!(Compact::compact_len(&n), l);
        assert_eq!(<Compact<u8>>::decode(&mut &encoded[..]).unwrap().0, n);
    }
    assert!(<Compact<u8>>::decode(&mut &Compact(256u32).encode()[..]).is_err());
}

fn hexify(bytes: &[u8]) -> String {
    bytes.iter().map(|ref b| format!("{:02x}", b)).collect::<Vec<String>>().join(" ")
}

pub fn compact_integers_encoded_as_expected() {
    let tests = [
        (0u64, "00"),
        (63, "fc"),
        (64, "01 01"),
        (16383, "fd ff"),
        (16384, "02 00 01 00"),
        (1073741823, "fe ff ff ff"),
        (1073741824, "03 00 00 00 40"),
        ((1 << 32) - 1, "03 ff ff ff ff"),
        (1 << 32, "07 00 00 00 00 01"),
        (1 << 40, "0b 00 00 00 00 00 01"),
        (1 << 48, "0f 00 00 00 00 00 00 01"),
        ((1 << 56) - 1, "0f ff ff ff ff ff ff ff"),
        (1 << 56, "13 00 00 00 00 00 00 00 01"),
        (u64::max_value(), "13 ff ff ff ff ff ff ff ff")
    ];
    for &(n, s) in &tests {
        // Verify u64 encoding
        let encoded = Compact(n as u64).encode();
        assert_eq!(hexify(&encoded), s);
        assert_eq!(<Compact<u64>>::decode(&mut &encoded[..]).unwrap().0, n);

        // Verify encodings for lower-size uints are compatible with u64 encoding
        if n <= u32::max_value() as u64 {
            assert_eq!(<Compact<u32>>::decode(&mut &encoded[..]).unwrap().0, n as u32);
            let encoded = Compact(n as u32).encode();
            assert_eq!(hexify(&encoded), s);
            assert_eq!(<Compact<u64>>::decode(&mut &encoded[..]).unwrap().0, n as u64);
        }
        if n <= u16::max_value() as u64 {
            assert_eq!(<Compact<u16>>::decode(&mut &encoded[..]).unwrap().0, n as u16);
            let encoded = Compact(n as u16).encode();
            assert_eq!(hexify(&encoded), s);
            assert_eq!(<Compact<u64>>::decode(&mut &encoded[..]).unwrap().0, n as u64);
        }
        if n <= u8::max_value() as u64 {
            assert_eq!(<Compact<u8>>::decode(&mut &encoded[..]).unwrap().0, n as u8);
            let encoded = Compact(n as u8).encode();
            assert_eq!(hexify(&encoded), s);
            assert_eq!(<Compact<u64>>::decode(&mut &encoded[..]).unwrap().0, n as u64);
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[derive(PartialEq, Eq, Clone)]
struct Wrapper(u8);

impl CompactAs for Wrapper {
    type As = u8;
    fn encode_as(&self) -> &u8 {
        &self.0
    }
    fn decode_from(x: u8) -> Wrapper {
        Wrapper(x)
    }
}

impl From<Compact<Wrapper>> for Wrapper {
    fn from(x: Compact<Wrapper>) -> Wrapper {
        x.0
    }
}

pub fn compact_as_8_encoding_works() {
    let tests = [(0u8, 1usize), (63, 1), (64, 2), (255, 2)];
    for &(n, l) in &tests {
        let compact: Compact<Wrapper> = Wrapper(n).into();
        let encoded = compact.encode();
        assert_eq!(encoded.len(), l);
        assert_eq!(Compact::compact_len(&n), l);
        let decoded = <Compact<Wrapper>>::decode(&mut & encoded[..]).unwrap();
        let wrapper: Wrapper = decoded.into();
        assert_eq!(wrapper, Wrapper(n));
    }
}

struct WithCompact<T: HasCompact> {
    _data: T,
}

pub fn compact_as_has_compact() {
    let _data = WithCompact { _data: Wrapper(1) };
}

pub fn compact_using_encoded_arrayvec_size() {
    Compact(std::u8::MAX).using_encoded(|_| {});
    Compact(std::u16::MAX).using_encoded(|_| {});
    Compact(std::u32::MAX).using_encoded(|_| {});
    Compact(std::u64::MAX).using_encoded(|_| {});
    Compact(std::u128::MAX).using_encoded(|_| {});

    CompactRef(&std::u8::MAX).using_encoded(|_| {});
    CompactRef(&std::u16::MAX).using_encoded(|_| {});
    CompactRef(&std::u32::MAX).using_encoded(|_| {});
    CompactRef(&std::u64::MAX).using_encoded(|_| {});
    CompactRef(&std::u128::MAX).using_encoded(|_| {});
}

pub fn array_vec_output_oob() {
    let mut v = ArrayVecWrapper(ArrayVec::<[u8; 4]>::new());
    v.write(&[1, 2, 3, 4, 5]);
}

pub fn array_vec_output() {
    let mut v = ArrayVecWrapper(ArrayVec::<[u8; 4]>::new());
    v.write(&[1, 2, 3, 4]);
}

macro_rules! check_bound {
		( $m:expr, $ty:ty, $typ1:ty, [ $(($ty2:ty, $ty2_err:expr)),* ]) => {
			$(
				check_bound!($m, $ty, $typ1, $ty2, $ty2_err);
			)*
		};
		( $m:expr, $ty:ty, $typ1:ty, $ty2:ty, $ty2_err:expr) => {
			let enc = ((<$ty>::max_value() >> 2) as $typ1 << 2) | $m;
			assert_eq!(Compact::<$ty2>::decode(&mut &enc.to_le_bytes()[..]),
				Err($ty2_err.into()));
		};
	}
macro_rules! check_bound_u32 {
		( [ $(($ty2:ty, $ty2_err:expr)),* ]) => {
			$(
				check_bound_u32!($ty2, $ty2_err);
			)*
		};
		( $ty2:ty, $ty2_err:expr ) => {
			assert_eq!(Compact::<$ty2>::decode(&mut &[0b11, 0xff, 0xff, 0xff, 0xff >> 2][..]),
				Err($ty2_err.into()));
		};
	}
macro_rules! check_bound_high {
		( $m:expr, [ $(($ty2:ty, $ty2_err:expr)),* ]) => {
			$(
				check_bound_high!($m, $ty2, $ty2_err);
			)*
		};
		( $s:expr, $ty2:ty, $ty2_err:expr) => {
			let mut dest = Vec::new();
			dest.push(0b11 + (($s - 4) << 2) as u8);
			for _ in 0..($s - 1) {
				dest.push(u8::max_value());
			}
			dest.push(0);
			assert_eq!(Compact::<$ty2>::decode(&mut &dest[..]),
				Err($ty2_err.into()));
		};
	}

pub fn compact_u64_test() {
    for a in [
        u64::max_value(),
        u64::max_value() - 1,
        u64::max_value() << 8,
        (u64::max_value() << 8) - 1,
        u64::max_value() << 16,
        (u64::max_value() << 16) - 1,
    ].into_iter() {
        let e = Compact::<u64>::encode(&Compact(*a));
        let d = Compact::<u64>::decode(&mut &e[..]).unwrap().0;
        assert_eq!(*a, d);
    }
}

pub fn compact_u128_test() {
    for a in [
        u64::max_value() as u128,
        (u64::max_value() - 10) as u128,
        u128::max_value(),
        u128::max_value() - 10,
    ].into_iter() {
        let e = Compact::<u128>::encode(&Compact(*a));
        let d = Compact::<u128>::decode(&mut &e[..]).unwrap().0;
        assert_eq!(*a, d);
    }
}

const U8_OUT_OF_RANGE: &str = "out of range decoding Compact<u8>";
const U16_OUT_OF_RANGE: &str = "out of range decoding Compact<u16>";
const U32_OUT_OF_RANGE: &str = "out of range decoding Compact<u32>";
const U64_OUT_OF_RANGE: &str = "out of range decoding Compact<u64>";
const U128_OUT_OF_RANGE: &str = "out of range decoding Compact<u128>";

pub fn should_avoid_overlapping_definition() {
    check_bound!(
			0b01, u8, u16, [ (u8, U8_OUT_OF_RANGE), (u16, U16_OUT_OF_RANGE),
			(u32, U32_OUT_OF_RANGE), (u64, U64_OUT_OF_RANGE), (u128, U128_OUT_OF_RANGE)]
		);
    check_bound!(
			0b10, u16, u32, [ (u16, U16_OUT_OF_RANGE),
			(u32, U32_OUT_OF_RANGE), (u64, U64_OUT_OF_RANGE), (u128, U128_OUT_OF_RANGE)]
		);
    check_bound_u32!(
			[(u32, U32_OUT_OF_RANGE), (u64, U64_OUT_OF_RANGE), (u128, U128_OUT_OF_RANGE)]
		);
    for i in 5..=8 {
        check_bound_high!(i, [(u64, U64_OUT_OF_RANGE), (u128, U128_OUT_OF_RANGE)]);
    }
    for i in 8..=16 {
        check_bound_high!(i, [(u128, U128_OUT_OF_RANGE)]);
    }
}
