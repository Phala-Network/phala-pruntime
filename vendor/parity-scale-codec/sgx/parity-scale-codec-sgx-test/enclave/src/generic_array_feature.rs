use parity_scale_codec::{
    Encode,
    Decode,
};

use generic_array::{GenericArray, arr, arr_impl};

pub fn generic_array() {
    let test = arr![u8; 3, 4, 5];
    let encoded = test.encode();
    assert_eq!(test, GenericArray::<u8, typenum::U3>::decode(&mut &encoded[..]).unwrap());

    let test = arr![u16; 3, 4, 5, 6, 7, 8, 0];
    let encoded = test.encode();
    assert_eq!(test, GenericArray::<u16, typenum::U7>::decode(&mut &encoded[..]).unwrap());

    let test = arr![u32; 3, 4, 5, 0, 1];
    let encoded = test.encode();
    assert_eq!(test, GenericArray::<u32, typenum::U5>::decode(&mut &encoded[..]).unwrap());

    let test = arr![u64; 3];
    let encoded = test.encode();
    assert_eq!(test, GenericArray::<u64, typenum::U1>::decode(&mut &encoded[..]).unwrap());
}
