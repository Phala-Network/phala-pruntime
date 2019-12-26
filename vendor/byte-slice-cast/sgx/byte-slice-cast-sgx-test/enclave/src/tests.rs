use std::prelude::v1::*;
use std::mem;
use byte_slice_cast::*;

//#[test]
pub fn u8() {
    let input: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

    let output: &[u8] = input.as_slice_of::<u8>().unwrap();
    assert_eq!(&input, output);

    let output2: &[u8] = input.as_byte_slice();
    assert_eq!(&input, output2);
}

//#[test]
pub fn u16() {
    let slice: [u16; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
    let bytes = slice.as_byte_slice();

    if cfg!(target_endian = "big") {
        assert_eq!(bytes, &[0, 0, 0, 1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7]);
    } else {
        assert_eq!(bytes, &[0, 0, 1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7, 0]);
    }

    assert_eq!(
        (&bytes[1..]).as_slice_of::<u16>(),
        Err(Error::AlignmentMismatch {
            dst_type: "u16",
            dst_minimum_alignment: mem::align_of::<u16>()
        })
    );
    assert_eq!(
        (&bytes[0..15]).as_slice_of::<u16>(),
        Err(Error::LengthMismatch {
            dst_type: "u16",
            src_slice_size: 15,
            dst_type_size: 2
        })
    );
    assert_eq!(bytes.as_slice_of::<u16>(), Ok(slice.as_ref()));
}

//#[cfg(feature = "std")]
//#[test]
pub fn u16_error_string() {
    let slice: [u16; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
    let bytes = slice.as_byte_slice();

    let error = (&bytes[1..]).as_slice_of::<u16>().unwrap_err().to_string();
    assert_eq!(
        error,
        "cannot cast a &[u8] into a &[u16]: the slice's address is not divisible by the minimum alignment (2) of u16",
    );
    let error = (&bytes[0..15])
        .as_slice_of::<u16>()
        .unwrap_err()
        .to_string();
    assert_eq!(
        error,
        "cannot cast a &[u8] into a &[u16]: the size (15) of the slice is not divisible by the size (2) of u16"
    );
}

//#[test]
pub fn u32() {
    let slice: [u32; 4] = [0, 1, 2, 3];
    let bytes = slice.as_byte_slice();

    if cfg!(target_endian = "big") {
        assert_eq!(bytes, &[0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3]);
    } else {
        assert_eq!(bytes, &[0, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0]);
    }

    assert_eq!(
        (&bytes[1..]).as_slice_of::<u32>(),
        Err(Error::AlignmentMismatch {
            dst_type: "u32",
            dst_minimum_alignment: mem::align_of::<u32>()
        })
    );
    assert_eq!(
        (&bytes[0..15]).as_slice_of::<u32>(),
        Err(Error::LengthMismatch {
            dst_type: "u32",
            src_slice_size: 15,
            dst_type_size: 4
        })
    );
    assert_eq!(bytes.as_slice_of::<u32>(), Ok(slice.as_ref()));
}

//#[test]
pub fn u64() {
    let slice: [u64; 2] = [0, 1];
    let bytes = slice.as_byte_slice();

    if cfg!(target_endian = "big") {
        assert_eq!(bytes, &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
    } else {
        assert_eq!(bytes, &[0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0]);
    }

    assert_eq!(
        (&bytes[1..]).as_slice_of::<u64>(),
        Err(Error::AlignmentMismatch {
            dst_type: "u64",
            dst_minimum_alignment: mem::align_of::<u64>()
        })
    );
    assert_eq!(
        (&bytes[0..15]).as_slice_of::<u64>(),
        Err(Error::LengthMismatch {
            dst_type: "u64",
            src_slice_size: 15,
            dst_type_size: 8
        })
    );
    assert_eq!(bytes.as_slice_of::<u64>(), Ok(slice.as_ref()));
}

//#[test]
pub fn f32() {
    let slice: [f32; 4] = [2.0, 1.0, 0.5, 0.25];
    let bytes = slice.as_byte_slice();

    if cfg!(target_endian = "big") {
        assert_eq!(
            bytes,
            [
                0x40, 0x00, 0x00, 0x00, 0x3f, 0x80, 0x00, 0x00, 0x3f, 0x00, 0x00, 0x00, 0x3e,
                0x80, 0x00, 0x00
            ]
        );
    } else {
        assert_eq!(
            bytes,
            [
                0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x80, 0x3f, 0x00, 0x00, 0x00, 0x3f, 0x00,
                0x00, 0x80, 0x3e
            ]
        );
    };

    assert_eq!(
        (&bytes[1..]).as_slice_of::<f32>(),
        Err(Error::AlignmentMismatch {
            dst_type: "f32",
            dst_minimum_alignment: mem::align_of::<f32>()
        })
    );
    assert_eq!(
        (&bytes[0..15]).as_slice_of::<f32>(),
        Err(Error::LengthMismatch {
            dst_type: "f32",
            src_slice_size: 15,
            dst_type_size: 4
        })
    );
    assert_eq!(bytes.as_slice_of::<f32>(), Ok(slice.as_ref()));
}

//#[test]
pub fn f64() {
    let slice: [f64; 2] = [2.0, 0.5];
    let bytes = slice.as_byte_slice();

    if cfg!(target_endian = "big") {
        assert_eq!(
            bytes,
            [
                0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3f, 0xe0, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00
            ]
        );
    } else {
        assert_eq!(
            bytes,
            [
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0xe0, 0x3f
            ]
        );
    };

    assert_eq!(
        (&bytes[1..]).as_slice_of::<f64>(),
        Err(Error::AlignmentMismatch {
            dst_type: "f64",
            dst_minimum_alignment: mem::align_of::<f64>()
        })
    );
    assert_eq!(
        (&bytes[0..15]).as_slice_of::<f64>(),
        Err(Error::LengthMismatch {
            dst_type: "f64",
            src_slice_size: 15,
            dst_type_size: 8
        })
    );
    assert_eq!(bytes.as_slice_of::<f64>(), Ok(slice.as_ref()));
}

//#[test]
pub fn u16_mut() {
    let mut slice: [u16; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
    let mut slice_2: [u16; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
    let bytes = slice_2.as_mut_byte_slice();

    if cfg!(target_endian = "big") {
        assert_eq!(bytes, &[0, 0, 0, 1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7]);
    } else {
        assert_eq!(bytes, &[0, 0, 1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7, 0]);
    }

    assert_eq!(
        (&mut bytes[1..]).as_mut_slice_of::<u16>(),
        Err(Error::AlignmentMismatch {
            dst_type: "u16",
            dst_minimum_alignment: mem::align_of::<u16>()
        })
    );
    assert_eq!(
        (&mut bytes[0..15]).as_mut_slice_of::<u16>(),
        Err(Error::LengthMismatch {
            dst_type: "u16",
            src_slice_size: 15,
            dst_type_size: 2
        })
    );
    assert_eq!(bytes.as_mut_slice_of::<u16>(), Ok(slice.as_mut()));
}

//#[cfg(feature = "std")]
//#[test]
pub fn u16_vec() {
    let vec: Vec<u16> = vec![0, 1, 2, 3, 4, 5, 6, 7];
    let bytes = vec.as_byte_slice();

    if cfg!(target_endian = "big") {
        assert_eq!(bytes, &[0, 0, 0, 1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7]);
    } else {
        assert_eq!(bytes, &[0, 0, 1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7, 0]);
    }

    assert_eq!(
        (&bytes[1..]).as_slice_of::<u16>(),
        Err(Error::AlignmentMismatch {
            dst_type: "u16",
            dst_minimum_alignment: mem::align_of::<u16>()
        })
    );
    assert_eq!(
        (&bytes[0..15]).as_slice_of::<u16>(),
        Err(Error::LengthMismatch {
            dst_type: "u16",
            src_slice_size: 15,
            dst_type_size: 2
        })
    );
    assert_eq!(bytes.as_slice_of::<u16>(), Ok(vec.as_ref()));
}

//#[cfg(feature = "std")]
//#[test]
pub fn u16_mut_vec() {
    let mut vec: Vec<u16> = vec![0, 1, 2, 3, 4, 5, 6, 7];
    let mut vec_clone = vec.clone();
    let bytes = vec_clone.as_mut_byte_slice();

    if cfg!(target_endian = "big") {
        assert_eq!(bytes, &[0, 0, 0, 1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7]);
    } else {
        assert_eq!(bytes, &[0, 0, 1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7, 0]);
    }

    assert_eq!(
        (&mut bytes[1..]).as_mut_slice_of::<u16>(),
        Err(Error::AlignmentMismatch {
            dst_type: "u16",
            dst_minimum_alignment: mem::align_of::<u16>()
        })
    );
    assert_eq!(
        (&mut bytes[0..15]).as_mut_slice_of::<u16>(),
        Err(Error::LengthMismatch {
            dst_type: "u16",
            src_slice_size: 15,
            dst_type_size: 2
        })
    );
    assert_eq!(bytes.as_mut_slice_of::<u16>(), Ok(vec.as_mut()));
}

//#[cfg(feature = "std")]
//#[test]
pub fn u16_box_slice() {
    let vec: Box<[u16]> = vec![0, 1, 2, 3, 4, 5, 6, 7].into_boxed_slice();
    let bytes = vec.as_byte_slice();

    if cfg!(target_endian = "big") {
        assert_eq!(bytes, &[0, 0, 0, 1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7]);
    } else {
        assert_eq!(bytes, &[0, 0, 1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7, 0]);
    }

    assert_eq!(
        (&bytes[1..]).as_slice_of::<u16>(),
        Err(Error::AlignmentMismatch {
            dst_type: "u16",
            dst_minimum_alignment: mem::align_of::<u16>()
        })
    );
    assert_eq!(
        (&bytes[0..15]).as_slice_of::<u16>(),
        Err(Error::LengthMismatch {
            dst_type: "u16",
            src_slice_size: 15,
            dst_type_size: 2
        })
    );
    assert_eq!(bytes.as_slice_of::<u16>(), Ok(vec.as_ref()));
}

//#[cfg(feature = "std")]
//#[test]
pub fn u16_mut_box_slice() {
    let mut vec: Box<[u16]> = vec![0, 1, 2, 3, 4, 5, 6, 7].into_boxed_slice();
    let mut vec_clone: Box<[u16]> = vec![0, 1, 2, 3, 4, 5, 6, 7].into_boxed_slice();
    let bytes = vec_clone.as_mut_byte_slice();

    if cfg!(target_endian = "big") {
        assert_eq!(bytes, &[0, 0, 0, 1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7]);
    } else {
        assert_eq!(bytes, &[0, 0, 1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7, 0]);
    }

    assert_eq!(
        (&mut bytes[1..]).as_mut_slice_of::<u16>(),
        Err(Error::AlignmentMismatch {
            dst_type: "u16",
            dst_minimum_alignment: mem::align_of::<u16>()
        })
    );
    assert_eq!(
        (&mut bytes[0..15]).as_mut_slice_of::<u16>(),
        Err(Error::LengthMismatch {
            dst_type: "u16",
            src_slice_size: 15,
            dst_type_size: 2
        })
    );
    assert_eq!(bytes.as_mut_slice_of::<u16>(), Ok(vec.as_mut()));
}

//#[test]
pub fn u16_empty_to_byte_slice() {
    let slice: [u16; 0] = [];
    let bytes = slice.as_byte_slice();

    assert_eq!(bytes, &[]);
}

//#[test]
pub fn u16_empty_from_byte_slice() {
    let bytes: [u8; 0] = [];
    let slice = bytes.as_slice_of::<u16>().unwrap();

    assert_eq!(slice, &[]);
}
