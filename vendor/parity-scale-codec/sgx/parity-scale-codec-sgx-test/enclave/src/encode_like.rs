use parity_scale_codec::{
    Encode,
    EncodeLike
};

use std::{
    vec::Vec,
    collections::BTreeMap,
    string::String
};

struct ComplexStuff<T>(T);

impl<T: Encode> ComplexStuff<T> {
    fn complex_method<R: Encode>(value: &R) -> Vec<u8> where T: EncodeLike<R> {
        value.encode()
    }
}

pub fn vec_and_slice_are_working() {
    let slice: &[u8] = &[1, 2, 3, 4];
    let data: Vec<u8> = slice.iter().copied().collect();

    let data_encoded = data.encode();
    let slice_encoded = ComplexStuff::<Vec<u8>>::complex_method(&slice);

    assert_eq!(slice_encoded, data_encoded);
}

pub fn btreemap_and_slice_are_working() {
    let slice: &[(u32, u32)] = &[(1, 2), (23, 24), (28, 30), (45, 80)];
    let data: BTreeMap<u32, u32> = slice.iter().copied().collect();

    let data_encoded = data.encode();
    let slice_encoded = ComplexStuff::<BTreeMap<u32, u32>>::complex_method(&slice);

    assert_eq!(slice_encoded, data_encoded);
}

pub fn interface_testing() {
    let value = 10u32;
    let data = (value, value, value);
    let encoded = ComplexStuff::<(u32, u32, u32)>::complex_method(&data);
    assert_eq!(data.encode(), encoded);
    let data = (&value, &value, &value);
    let encoded = ComplexStuff::<(u32, u32, u32)>::complex_method(&data);
    assert_eq!(data.encode(), encoded);
    let data = (&value, value, &value);
    let encoded = ComplexStuff::<(u32, u32, u32)>::complex_method(&data);
    assert_eq!(data.encode(), encoded);

    let vec_data: Vec<u8> = vec![1, 2, 3];
    ComplexStuff::<Vec<u8>>::complex_method(&vec_data);
    ComplexStuff::<&'static str>::complex_method(&String::from("test"));
    ComplexStuff::<&'static str>::complex_method(&"test");

    let slice: &[u8] = &vec_data;
    assert_eq!(
        ComplexStuff::<(u32, Vec<u8>)>::complex_method(&(1u32, slice.to_vec())),
        ComplexStuff::<(u32, Vec<u8>)>::complex_method(&(1u32, slice))
    );
}
