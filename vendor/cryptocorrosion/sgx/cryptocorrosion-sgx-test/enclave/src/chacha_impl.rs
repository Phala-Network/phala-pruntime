use c2_chacha::*;
use c2_chacha::stream_cipher::NewStreamCipher;
use c2_chacha::stream_cipher::SyncStreamCipherSeek;
use c2_chacha::stream_cipher::SyncStreamCipher;
use c2_chacha::guts::generic_array::GenericArray;

//#[test]
pub fn chacha20_case_1() {
    let key = hex!("fa44478c59ca70538e3549096ce8b523232c50d9e8e8d10c203ef6c8d07098a5");
    let nonce = hex!("8d3a0d6d7827c007");
    let expected = hex!("
            1546a547ff77c5c964e44fd039e913c6395c8f19d43efaa880750f6687b4e6e2d8f42f63546da2d133b5aa2f1ef3f218b6c72943089e4012
            210c2cbed0e8e93498a6825fc8ff7a504f26db33b6cbe36299436244c9b2eff88302c55933911b7d5dea75f2b6d4761ba44bb6f814c9879d
            2ba2ac8b178fa1104a368694872339738ffb960e33db39efb8eaef885b910eea078e7a1feb3f8185dafd1455b704d76da3a0ce4760741841
            217bba1e4ece760eaf68617133431feb806c061173af6b8b2a23be90c5d145cc258e3c119aab2800f0c7bc1959dae75481712cab731b7dfd
            783fa3a228f9968aaea68f36a92f43c9b523337a55b97bcaf5f5774447bf41e8");
    let mut state = ChaCha20::new(
        GenericArray::from_slice(&key),
        GenericArray::from_slice(&nonce),
    );
    let offset = 0x3fffffff70u64;
    assert!((offset >> 38) != ((offset + 240) >> 38)); // This will overflow the small word of the counter
    state.seek(offset);
    let mut result = [0; 256];
    state.apply_keystream(&mut result);
    assert_eq!(&expected[..], &result[..]);
}

//#[test]
pub fn chacha12_case_1() {
    let key = hex!("27fc120b013b829f1faeefd1ab417e8662f43e0d73f98de866e346353180fdb7");
    let nonce = hex!("db4b4a41d8df18aa");
    let expected = hex!("
            5f3c8c190a78ab7fe808cae9cbcb0a9837c893492d963a1c2eda6c1558b02c83fc02a44cbbb7e6204d51d1c2430e9c0b58f2937bf593840c
            850bda9051a1f051ddf09d2a03ebf09f01bdba9da0b6da791b2e645641047d11ebf85087d4de5c015fddd044");
    let mut state = ChaCha12::new(
        GenericArray::from_slice(&key),
        GenericArray::from_slice(&nonce),
    );
    let mut result = [0u8; 100];
    state.apply_keystream(&mut result);
    assert_eq!(&expected[..], &result[..]);
}

//#[test]
pub fn chacha8_case_1() {
    let key = hex!("641aeaeb08036b617a42cf14e8c5d2d115f8d7cb6ea5e28b9bfaf83e038426a7");
    let nonce = hex!("a14a1168271d459b");
    let mut state = ChaCha8::new(
        GenericArray::from_slice(&key),
        GenericArray::from_slice(&nonce),
    );
    let expected = hex!(
    "1721c044a8a6453522dddb3143d0be3512633ca3c79bf8ccc3594cb2c2f310f7bd544f55ce0db38123412d6c45207d5cf9af0c6c680cce1f
    7e43388d1b0346b7133c59fd6af4a5a568aa334ccdc38af5ace201df84d0a3ca225494ca6209345fcf30132e");
    let mut result = [0u8; 100];
    state.apply_keystream(&mut result);
    assert_eq!(&expected[..], &result[..]);
}

//#[test]
pub fn test_ietf() {
    let key = hex!("000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f");
    let nonce = hex!("000000090000004a00000000");
    let expected = hex!(
        "
        10f1e7e4d13b5915500fdd1fa32071c4c7d1f4c733c068030422aa9ac3d46c4e
        d2826446079faa0914c2d705d98b02a2b5129cd1de164eb9cbd083e8a2503c4e"
    );
    let mut state = Ietf::new(
        GenericArray::from_slice(&key),
        GenericArray::from_slice(&nonce),
    );
    let mut result = [0; 64];
    state.seek(64);
    state.apply_keystream(&mut result);
    assert_eq!(&expected[..], &result[..]);
}

//#[test]
pub fn rfc_7539_case_1() {
    let key = hex!("000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f");
    let nonce = hex!("000000090000004a00000000");
    let mut state = Ietf::new(
        GenericArray::from_slice(&key),
        GenericArray::from_slice(&nonce),
    );
    let mut result = [0; 128];
    state.apply_keystream(&mut result);
    let expected = hex!(
        "10f1e7e4d13b5915500fdd1fa32071c4c7d1f4c733c068030422aa9ac3d46c4e
        d2826446079faa0914c2d705d98b02a2b5129cd1de164eb9cbd083e8a2503c4e"
    );
    assert_eq!(&expected[..], &result[64..]);
}

//#[test]
pub fn rfc_7539_case_2() {
    let key = hex!("000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f");
    let nonce = hex!("000000000000004a00000000");
    let mut state = Ietf::new(
        GenericArray::from_slice(&key),
        GenericArray::from_slice(&nonce),
    );
    let plaintext = b"Ladies and Gentlemen of the class of '99: If I could offer you only one tip for the future, sunscreen would be it.";
    let mut buf = [0u8; 178];
    buf[64..].copy_from_slice(plaintext);
    state.apply_keystream(&mut buf);
    let expected = hex!("
        6e2e359a2568f98041ba0728dd0d6981e97e7aec1d4360c20a27afccfd9fae0bf91b65c5524733ab8f593dabcd62b3571639d624e65152ab
        8f530c359f0861d807ca0dbf500d6a6156a38e088a22b65e52bc514d16ccf806818ce91ab77937365af90bbf74a35be6b40b8eedf2785e42
        874d");
    assert_eq!(&expected[..], &buf[64..]);
}

//#[test]
pub fn rfc_7539_case_2_chunked() {
    let key = hex!("000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f");
    let nonce = hex!("000000000000004a00000000");
    let mut state = Ietf::new(
        GenericArray::from_slice(&key),
        GenericArray::from_slice(&nonce),
    );
    let plaintext = b"Ladies and Gentlemen of the class of '99: If I could offer you only one tip for the future, sunscreen would be it.";
    let mut buf = [0u8; 178];
    buf[64..].copy_from_slice(plaintext);
    state.apply_keystream(&mut buf[..40]);
    state.apply_keystream(&mut buf[40..78]);
    state.apply_keystream(&mut buf[78..79]);
    state.apply_keystream(&mut buf[79..128]);
    state.apply_keystream(&mut buf[128..]);
    let expected = hex!("
        6e2e359a2568f98041ba0728dd0d6981e97e7aec1d4360c20a27afccfd9fae0bf91b65c5524733ab8f593dabcd62b3571639d624e65152ab
        8f530c359f0861d807ca0dbf500d6a6156a38e088a22b65e52bc514d16ccf806818ce91ab77937365af90bbf74a35be6b40b8eedf2785e42
        874d");
    assert_eq!(&expected[..], &buf[64..]);
}

//#[test]
pub fn xchacha20_case_1() {
    let key = hex!("82f411a074f656c66e7dbddb0a2c1b22760b9b2105f4ffdbb1d4b1e824e21def");
    let nonce = hex!("3b07ca6e729eb44a510b7a1be51847838a804f8b106b38bd");
    let mut state = XChaCha20::new(
        GenericArray::from_slice(&key),
        GenericArray::from_slice(&nonce),
    );
    let mut xs = [0u8; 100];
    state.apply_keystream(&mut xs);
    let expected = hex!("
        201863970b8e081f4122addfdf32f6c03e48d9bc4e34a59654f49248b9be59d3eaa106ac3376e7e7d9d1251f2cbf61ef27000f3d19afb76b
        9c247151e7bc26467583f520518eccd2055ccd6cc8a195953d82a10c2065916778db35da2be44415d2f5efb0");
    assert_eq!(&expected[..], &xs[..]);
}

//#[test]
pub fn seek_off_end() {
    let mut st = Ietf::new(
        GenericArray::from_slice(&[0xff; 32]),
        GenericArray::from_slice(&[0; 12]),
    );
    st.seek(0x40_0000_0000);

    assert!(st.try_apply_keystream(&mut [0u8; 1]).is_err());
}

//#[test]
pub fn read_last_bytes() {
    let mut st = Ietf::new(
        GenericArray::from_slice(&[0xff; 32]),
        GenericArray::from_slice(&[0; 12]),
    );

    st.seek(0x40_0000_0000 - 10);
    st.apply_keystream(&mut [0u8; 10]);
    assert!(st.try_apply_keystream(&mut [0u8; 1]).is_err());

    st.seek(0x40_0000_0000 - 10);
    assert!(st.try_apply_keystream(&mut [0u8; 11]).is_err());
}

//#[test]
pub fn seek_consistency() {
    let mut st = Ietf::new(
        GenericArray::from_slice(&[50; 32]),
        GenericArray::from_slice(&[44; 12]),
    );

    let mut continuous = [0u8; 1000];
    st.apply_keystream(&mut continuous);

    let mut chunks = [0u8; 1000];

    st.seek(128);
    st.apply_keystream(&mut chunks[128..300]);

    st.seek(0);
    st.apply_keystream(&mut chunks[0..10]);

    st.seek(300);
    st.apply_keystream(&mut chunks[300..533]);

    st.seek(533);
    st.apply_keystream(&mut chunks[533..]);

    st.seek(10);
    st.apply_keystream(&mut chunks[10..128]);

    assert_eq!(&continuous[..], &chunks[..]);
}

//#[test]
pub fn wide_matches_narrow() {
    let key = hex!("fa44478c59ca70538e3549096ce8b523232c50d9e8e8d10c203ef6c8d07098a5");
    let nonce = hex!("8d3a0d6d7827c007");
    let mut buf = [0; 2048];
    let mut state = ChaCha20::new(
        GenericArray::from_slice(&key),
        GenericArray::from_slice(&nonce),
    );

    let lens = [
        2048, 2047, 1537, 1536, 1535, 1025, 1024, 1023, 768, 513, 512, 511, 200, 100, 50,
    ];

    for &len in &lens {
        let buf = &mut buf[0..len];

        // encrypt with hybrid wide/narrow
        state.seek(0);
        state.apply_keystream(buf);
        state.seek(0);
        // decrypt with narrow only
        state.try_apply_keystream_narrow(buf).unwrap();
        for &byte in buf.iter() {
            assert_eq!(byte, 0);
        }

        // encrypt with hybrid wide/narrow
        let offset = 0x3fffffff70u64;
        state.seek(offset);
        state.apply_keystream(buf);
        // decrypt with narrow only
        state.seek(offset);
        state.try_apply_keystream_narrow(buf).unwrap();
        for &byte in buf.iter() {
            assert_eq!(byte, 0);
        }
    }
}
