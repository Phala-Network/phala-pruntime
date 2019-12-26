use alloc::vec::Vec;

use parity_scale_codec::{
    Error,
    Decode,
    Encode,
    Input,
    Compact,
    EncodeLike,
    DecodeAll
};

/// The error message returned when `decode_all` fails.
const DECODE_ALL_ERR_MSG: &str = "Input buffer has still data left after decoding!";

macro_rules! test_decode_all {
		(
			$( $type:ty => $value:expr; )*
		) => {
			$(
				{
					let mut encoded = <$type as Encode>::encode(&$value);
					<$type>::decode_all(&encoded).expect(
						&format!("`{} => {}` decodes all!", stringify!($type), stringify!($value)),
					);

					encoded.extend(&[1, 2, 3, 4, 5, 6]);
					assert_eq!(<$type>::decode_all(&encoded).unwrap_err().what(), DECODE_ALL_ERR_MSG);
				}
			)*
		};
	}

#[derive(Debug)]
struct TestStruct {
    data: Vec<u32>,
    other: u8,
    compact: Compact<u128>,
}

impl EncodeLike for TestStruct {}

impl Encode for TestStruct {
    fn encode(&self) -> Vec<u8> {
        let mut res = Vec::new();
        self.data.encode_to(&mut res);
        self.other.encode_to(&mut res);
        self.compact.encode_to(&mut res);
        res
    }
}

impl Decode for TestStruct {
    fn decode<I: Input>(input: &mut I) -> Result<Self, Error> {
        Ok(
            Self {
                data: Vec::<u32>::decode(input)?,
                other: u8::decode(input)?,
                compact: Compact::<u128>::decode(input)?,
            }
        )
    }
}

pub fn decode_all_works() {
    test_decode_all! {
			u8 => 120;
			u16 => 30;
			u32 => 1;
			u64 => 2343545;
			u128 => 34358394245459854;
			Vec<u8> => vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
			Vec<u32> => vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
			Compact<u32> => Compact(32445);
			Compact<u128> => Compact(34353454453545);
			TestStruct => TestStruct { data: vec![1, 2, 4, 5, 6], other: 45, compact: Compact(123234545) };
		}
}
