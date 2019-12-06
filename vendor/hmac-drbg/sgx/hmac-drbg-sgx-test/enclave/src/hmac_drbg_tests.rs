//extern crate hmac_drbg;
//extern crate sha2;
//extern crate generic_array;
//extern crate hexutil;
//extern crate typenum;

use std::prelude::v1::*;

use hmac_drbg::*;
use sha2::Sha256;
use generic_array::GenericArray;
use hex::*;
use typenum::consts::*;

//extern crate serde;
//extern crate serde_json;
//#[macro_use]
//extern crate serde_derive;

//#[test]
pub fn test1_sha256() {
    let mut drbg = HmacDRBG::<Sha256>::new(
        "totally random0123456789".as_bytes(),
        "secret nonce".as_bytes(),
        "my drbg".as_bytes());
    assert_eq!(drbg.generate::<U32>(None).as_slice(), decode("018ec5f8e08c41e5ac974eb129ac297c5388ee1864324fa13d9b15cf98d9a157").unwrap().as_slice());
}

//#[test]
pub fn test2_sha256() {
    let mut drbg = HmacDRBG::<Sha256>::new(
        "totally random0123456789".as_bytes(),
        "secret nonce".as_bytes(),
        &[]);
    assert_eq!(drbg.generate::<U32>(None).as_slice(), decode("ed5d61ecf0ef38258e62f03bbb49f19f2cd07ba5145a840d83b134d5963b3633").unwrap().as_slice());
}

//#[test]
pub fn reseeding() {
    let mut original = HmacDRBG::<Sha256>::new(
        "totally random string with many chars that I typed in agony".as_bytes(),
        "nonce".as_bytes(),
        "pers".as_bytes());
    let mut reseeded = HmacDRBG::<Sha256>::new(
        "totally random string with many chars that I typed in agony".as_bytes(),
        "nonce".as_bytes(),
        "pers".as_bytes());

    assert_eq!(original.generate::<U32>(None), reseeded.generate::<U32>(None));
    reseeded.reseed("another absolutely random string".as_bytes(), None);
    assert_ne!(original.generate::<U32>(None), reseeded.generate::<U32>(None));
}

//#[test]
pub fn nist_victors() {
    #[derive(Deserialize, Debug)]
    struct Fixture {
        name: String,
        entropy: String,
        nonce: String,
        pers: Option<String>,
        add: [Option<String>; 2],
        expected: String,
    }

    let tests: Vec<Fixture> = serde_json::from_str(include_str!("./fixtures/hmac-drbg-nist.json")).unwrap();

    for test in tests {
        let mut drbg = HmacDRBG::<Sha256>::new(
            &decode(&test.entropy).unwrap(),
            &decode(&test.nonce).unwrap(),
            &decode(&test.pers.unwrap_or("".to_string())).unwrap());
        let mut expected = decode(&test.expected).unwrap();
        let mut result = Vec::new();
        result.resize(expected.len(), 0);

        let half_len = result.len() / 2;
        let full_len = result.len();

        let add0 = test.add[0].as_ref().map(|v| decode(&v).unwrap());
        let add1 = test.add[1].as_ref().map(|v| decode(&v).unwrap());

        drbg.generate_to_slice(&mut result[0..full_len],
                               match add0 {
                                   Some(ref add0) => Some(add0.as_ref()),
                                   None => None,
                               });
        drbg.generate_to_slice(&mut result[0..full_len],
                               match add1 {
                                   Some(ref add1) => Some(add1.as_ref()),
                                   None => None,
                               });
        assert_eq!(result, expected);
    }
}
