use parity_scale_codec::{
    Input,
    Encode,
    Error,
    Decode,
    EncodeLike,
    EncodeAppend
};

use std::{
    vec::Vec,
    boxed::Box
};

pub fn vec_encode_append_works() {
    let max_value = 1_000_000;

    let encoded = (0..max_value).fold(Vec::new(), |encoded, v| {
        <Vec<u32> as EncodeAppend>::append_or_new(encoded, std::iter::once(&v)).unwrap()
    });

    let decoded = Vec::<u32>::decode(&mut &encoded[..]).unwrap();
    assert_eq!(decoded, (0..max_value).collect::<Vec<_>>());
}

pub fn vec_encode_append_multiple_items_works() {
    let max_value = 1_000_000u32;

    let encoded = (0..max_value).fold(Vec::new(), |encoded, v| {
        <Vec<u32> as EncodeAppend>::append_or_new(encoded, &[v, v, v, v]).unwrap()
    });

    let decoded = Vec::<u32>::decode(&mut &encoded[..]).unwrap();
    let expected = (0..max_value).fold(Vec::new(), |mut vec, i| {
        vec.append(&mut vec![i, i, i, i]);
        vec
    });
    assert_eq!(decoded, expected);
}

pub fn append_non_copyable() {
    #[derive(Eq, PartialEq, Debug)]
    struct NoCopy { data: u32 };

    impl EncodeLike for NoCopy {}

    impl Encode for NoCopy {
        fn encode(&self) -> Vec<u8> {
            self.data.encode()
        }
    }

    impl Decode for NoCopy {
        fn decode<I: Input>(input: &mut I) -> Result<Self, Error> {
            u32::decode(input).map(|data| Self { data })
        }
    }

    let append = NoCopy { data: 100 };
    let data = Vec::new();
    let encoded = <Vec<NoCopy> as EncodeAppend>::append_or_new(data, std::iter::once(&append)).unwrap();

    let decoded = <Vec<NoCopy>>::decode(&mut &encoded[..]).unwrap();
    assert_eq!(vec![append], decoded);
}

pub fn vec_encode_like_append_works() {
    let max_value = 1_000_000;

    let encoded = (0..max_value).fold(Vec::new(), |encoded, v| {
        <Vec<u32> as EncodeAppend>::append_or_new(encoded, std::iter::once(Box::new(v as u32))).unwrap()
    });

    let decoded = Vec::<u32>::decode(&mut &encoded[..]).unwrap();
    assert_eq!(decoded, (0..max_value).collect::<Vec<_>>());
}
