use webpki::{der, signed_data, Error};

use std::{self, io::BufRead, string::String, vec::Vec};

// TODO: The expected results need to be modified for SHA-1 deprecation.

macro_rules! test_verify_signed_data {
    ($fn_name:ident, $file_name:expr, $expected_result:expr) => {
        //#[test]
        pub fn $fn_name() { test_verify_signed_data($file_name, $expected_result); }
    };
}

fn test_verify_signed_data(file_name: &str, expected_result: Result<(), Error>) {
    let tsd = parse_test_signed_data(file_name);
    let spki_value = untrusted::Input::from(&tsd.spki);
    let spki_value = spki_value
        .read_all(Error::BadDER, |input| {
            der::expect_tag_and_get_value(input, der::Tag::Sequence)
        })
        .unwrap();

    // we can't use `parse_signed_data` because it requires `data`
    // to be an ASN.1 SEQUENCE, and that isn't the case with
    // Chromium's test data. TODO: The test data set should be
    // expanded with SEQUENCE-wrapped data so that we can actually
    // test `parse_signed_data`.

    let algorithm = untrusted::Input::from(&tsd.algorithm);
    let algorithm = algorithm
        .read_all(Error::BadDER, |input| {
            der::expect_tag_and_get_value(input, der::Tag::Sequence)
        })
        .unwrap();

    let signature = untrusted::Input::from(&tsd.signature);
    let signature = signature
        .read_all(Error::BadDER, |input| {
            der::bit_string_with_no_unused_bits(input)
        })
        .unwrap();

    let signed_data = signed_data::SignedData {
        data: untrusted::Input::from(&tsd.data),
        algorithm,
        signature,
    };

    assert_eq!(
        expected_result,
        signed_data::verify_signed_data(
            SUPPORTED_ALGORITHMS_IN_TESTS,
            spki_value,
            &signed_data
        )
    );
}

// XXX: This is testing code that isn't even in this module.
macro_rules! test_verify_signed_data_signature_outer {
    ($fn_name:ident, $file_name:expr, $expected_result:expr) => {
        //#[test]
        pub fn $fn_name() { test_verify_signed_data_signature_outer($file_name, $expected_result); }
    };
}

fn test_verify_signed_data_signature_outer(file_name: &str, expected_error: Error) {
    let tsd = parse_test_signed_data(file_name);
    let signature = untrusted::Input::from(&tsd.signature);
    assert_eq!(
        Err(expected_error),
        signature.read_all(Error::BadDER, |input| {
            der::bit_string_with_no_unused_bits(input)
        })
    );
}

// XXX: This is testing code that is not even in this module.
macro_rules! test_parse_spki_bad_outer {
    ($fn_name:ident, $file_name:expr, $error:expr) => {
        //#[test]
        pub fn $fn_name() { test_parse_spki_bad_outer($file_name, $error) }
    };
}

fn test_parse_spki_bad_outer(file_name: &str, expected_error: Error) {
    let tsd = parse_test_signed_data(file_name);
    let spki = untrusted::Input::from(&tsd.spki);
    assert_eq!(
        Err(expected_error),
        spki.read_all(Error::BadDER, |input| {
            der::expect_tag_and_get_value(input, der::Tag::Sequence)
        })
    );
}

// XXX: Some of the BadDER tests should have better error codes, maybe?

// XXX: We should have a variant of this test with a SHA-256 digest that gives
// `Error::UnsupportedSignatureAlgorithmForPublicKey`.
test_verify_signed_data!(
    test_ecdsa_prime256v1_sha512_spki_params_null,
    "ecdsa-prime256v1-sha512-spki-params-null.pem",
    Err(Error::UnsupportedSignatureAlgorithm)
);
test_verify_signed_data_signature_outer!(
    test_ecdsa_prime256v1_sha512_unused_bits_signature,
    "ecdsa-prime256v1-sha512-unused-bits-signature.pem",
    Error::BadDER
);
// XXX: We should have a variant of this test with a SHA-256 digest that gives
// `Error::UnsupportedSignatureAlgorithmForPublicKey`.
test_verify_signed_data!(
    test_ecdsa_prime256v1_sha512_using_ecdh_key,
    "ecdsa-prime256v1-sha512-using-ecdh-key.pem",
    Err(Error::UnsupportedSignatureAlgorithm)
);
// XXX: We should have a variant of this test with a SHA-256 digest that gives
// `Error::UnsupportedSignatureAlgorithmForPublicKey`.
test_verify_signed_data!(
    test_ecdsa_prime256v1_sha512_using_ecmqv_key,
    "ecdsa-prime256v1-sha512-using-ecmqv-key.pem",
    Err(Error::UnsupportedSignatureAlgorithm)
);
test_verify_signed_data!(
    test_ecdsa_prime256v1_sha512_using_rsa_algorithm,
    "ecdsa-prime256v1-sha512-using-rsa-algorithm.pem",
    Err(Error::UnsupportedSignatureAlgorithmForPublicKey)
);
// XXX: We should have a variant of this test with a SHA-256 digest that gives
// `Error::InvalidSignatureForPublicKey`.
test_verify_signed_data!(
    test_ecdsa_prime256v1_sha512_wrong_signature_format,
    "ecdsa-prime256v1-sha512-wrong-signature-format.pem",
    Err(Error::UnsupportedSignatureAlgorithm)
);
// Differs from Chromium because we don't support P-256 with SHA-512.
test_verify_signed_data!(
    test_ecdsa_prime256v1_sha512,
    "ecdsa-prime256v1-sha512.pem",
    Err(Error::UnsupportedSignatureAlgorithm)
);
test_verify_signed_data!(
    test_ecdsa_secp384r1_sha256_corrupted_data,
    "ecdsa-secp384r1-sha256-corrupted-data.pem",
    Err(Error::InvalidSignatureForPublicKey)
);
test_verify_signed_data!(
    test_ecdsa_secp384r1_sha256,
    "ecdsa-secp384r1-sha256.pem",
    Ok(())
);
test_verify_signed_data!(
    test_ecdsa_using_rsa_key,
    "ecdsa-using-rsa-key.pem",
    Err(Error::UnsupportedSignatureAlgorithmForPublicKey)
);

test_parse_spki_bad_outer!(
    test_rsa_pkcs1_sha1_bad_key_der_length,
    "rsa-pkcs1-sha1-bad-key-der-length.pem",
    Error::BadDER
);
test_parse_spki_bad_outer!(
    test_rsa_pkcs1_sha1_bad_key_der_null,
    "rsa-pkcs1-sha1-bad-key-der-null.pem",
    Error::BadDER
);
test_verify_signed_data!(
    test_rsa_pkcs1_sha1_key_params_absent,
    "rsa-pkcs1-sha1-key-params-absent.pem",
    Err(Error::UnsupportedSignatureAlgorithm)
);
test_verify_signed_data!(
    test_rsa_pkcs1_sha1_using_pss_key_no_params,
    "rsa-pkcs1-sha1-using-pss-key-no-params.pem",
    Err(Error::UnsupportedSignatureAlgorithm)
);
test_verify_signed_data!(
    test_rsa_pkcs1_sha1_wrong_algorithm,
    "rsa-pkcs1-sha1-wrong-algorithm.pem",
    Err(Error::InvalidSignatureForPublicKey)
);
test_verify_signed_data!(
    test_rsa_pkcs1_sha1,
    "rsa-pkcs1-sha1.pem",
    Err(Error::UnsupportedSignatureAlgorithm)
);
// XXX: RSA PKCS#1 with SHA-1 is a supported algorithm, but we only accept
// 2048-8192 bit keys, and this test file is using a 1024 bit key. Thus,
// our results differ from Chromium's. TODO: this means we need a 2048+ bit
// version of this test.
test_verify_signed_data!(
    test_rsa_pkcs1_sha256,
    "rsa-pkcs1-sha256.pem",
    Err(Error::InvalidSignatureForPublicKey)
);
test_parse_spki_bad_outer!(
    test_rsa_pkcs1_sha256_key_encoded_ber,
    "rsa-pkcs1-sha256-key-encoded-ber.pem",
    Error::BadDER
);
test_verify_signed_data!(
    test_rsa_pkcs1_sha256_spki_non_null_params,
    "rsa-pkcs1-sha256-spki-non-null-params.pem",
    Err(Error::UnsupportedSignatureAlgorithmForPublicKey)
);
test_verify_signed_data!(
    test_rsa_pkcs1_sha256_using_ecdsa_algorithm,
    "rsa-pkcs1-sha256-using-ecdsa-algorithm.pem",
    Err(Error::UnsupportedSignatureAlgorithmForPublicKey)
);
test_verify_signed_data!(
    test_rsa_pkcs1_sha256_using_id_ea_rsa,
    "rsa-pkcs1-sha256-using-id-ea-rsa.pem",
    Err(Error::UnsupportedSignatureAlgorithmForPublicKey)
);

// Chromium's PSS test are for parameter combinations we don't support.
test_verify_signed_data!(
    test_rsa_pss_sha1_salt20_using_pss_key_no_params,
    "rsa-pss-sha1-salt20-using-pss-key-no-params.pem",
    Err(Error::UnsupportedSignatureAlgorithm)
);
test_verify_signed_data!(
    test_rsa_pss_sha1_salt20_using_pss_key_with_null_params,
    "rsa-pss-sha1-salt20-using-pss-key-with-null-params.pem",
    Err(Error::UnsupportedSignatureAlgorithm)
);
test_verify_signed_data!(
    test_rsa_pss_sha1_salt20,
    "rsa-pss-sha1-salt20.pem",
    Err(Error::UnsupportedSignatureAlgorithm)
);
test_verify_signed_data!(
    test_rsa_pss_sha1_wrong_salt,
    "rsa-pss-sha1-wrong-salt.pem",
    Err(Error::UnsupportedSignatureAlgorithm)
);
test_verify_signed_data!(
    test_rsa_pss_sha256_mgf1_sha512_salt33,
    "rsa-pss-sha256-mgf1-sha512-salt33.pem",
    Err(Error::UnsupportedSignatureAlgorithm)
);
test_verify_signed_data!(
    test_rsa_pss_sha256_salt10_using_pss_key_with_params,
    "rsa-pss-sha256-salt10-using-pss-key-with-params.pem",
    Err(Error::UnsupportedSignatureAlgorithm)
);
test_verify_signed_data!(
    test_rsa_pss_sha256_salt10_using_pss_key_with_wrong_params,
    "rsa-pss-sha256-salt10-using-pss-key-with-wrong-params.pem",
    Err(Error::UnsupportedSignatureAlgorithm)
);
test_verify_signed_data!(
    test_rsa_pss_sha256_salt10,
    "rsa-pss-sha256-salt10.pem",
    Err(Error::UnsupportedSignatureAlgorithm)
);

// Our PSS tests that should work.
test_verify_signed_data!(
    test_rsa_pss_sha256_salt32,
    "ours/rsa-pss-sha256-salt32.pem",
    Ok(())
);
test_verify_signed_data!(
    test_rsa_pss_sha384_salt48,
    "ours/rsa-pss-sha384-salt48.pem",
    Ok(())
);
test_verify_signed_data!(
    test_rsa_pss_sha512_salt64,
    "ours/rsa-pss-sha512-salt64.pem",
    Ok(())
);
test_verify_signed_data!(
    test_rsa_pss_sha256_salt32_corrupted_data,
    "ours/rsa-pss-sha256-salt32-corrupted-data.pem",
    Err(Error::InvalidSignatureForPublicKey)
);
test_verify_signed_data!(
    test_rsa_pss_sha384_salt48_corrupted_data,
    "ours/rsa-pss-sha384-salt48-corrupted-data.pem",
    Err(Error::InvalidSignatureForPublicKey)
);
test_verify_signed_data!(
    test_rsa_pss_sha512_salt64_corrupted_data,
    "ours/rsa-pss-sha512-salt64-corrupted-data.pem",
    Err(Error::InvalidSignatureForPublicKey)
);

test_verify_signed_data!(
    test_rsa_using_ec_key,
    "rsa-using-ec-key.pem",
    Err(Error::UnsupportedSignatureAlgorithmForPublicKey)
);
test_verify_signed_data!(
    test_rsa2048_pkcs1_sha512,
    "rsa2048-pkcs1-sha512.pem",
    Ok(())
);

struct TestSignedData {
    spki: Vec<u8>,
    data: Vec<u8>,
    algorithm: Vec<u8>,
    signature: Vec<u8>,
}

fn parse_test_signed_data(file_name: &str) -> TestSignedData {
    let path = std::path::PathBuf::from("third-party/chromium/data/verify_signed_data")
        .join(file_name);
    let file = std::untrusted::fs::File::open(path).unwrap();
    let mut lines = std::io::BufReader::new(&file).lines();

    let spki = read_pem_section(&mut lines, "PUBLIC KEY");
    let algorithm = read_pem_section(&mut lines, "ALGORITHM");
    let data = read_pem_section(&mut lines, "DATA");
    let signature = read_pem_section(&mut lines, "SIGNATURE");

    TestSignedData {
        spki,
        data,
        algorithm,
        signature,
    }
}

type FileLines<'a> = std::io::Lines<std::io::BufReader<&'a std::untrusted::fs::File>>;

fn read_pem_section(lines: &mut FileLines, section_name: &str) -> Vec<u8> {
    // Skip comments and header
    let begin_section = format!("-----BEGIN {}-----", section_name);
    loop {
        let line = lines.next().unwrap().unwrap();
        if line == begin_section {
            break;
        }
    }

    let mut base64 = String::new();

    let end_section = format!("-----END {}-----", section_name);
    loop {
        let line = lines.next().unwrap().unwrap();
        if line == end_section {
            break;
        }
        base64.push_str(&line);
    }

    base64::decode(&base64).unwrap()
}

static SUPPORTED_ALGORITHMS_IN_TESTS: &[&signed_data::SignatureAlgorithm] = &[
    // Reasonable algorithms.
    &signed_data::RSA_PKCS1_2048_8192_SHA256,
    &signed_data::ECDSA_P256_SHA256,
    &signed_data::ECDSA_P384_SHA384,
    &signed_data::RSA_PKCS1_2048_8192_SHA384,
    &signed_data::RSA_PKCS1_2048_8192_SHA512,
    &signed_data::RSA_PKCS1_3072_8192_SHA384,
    &signed_data::RSA_PSS_2048_8192_SHA256_LEGACY_KEY,
    &signed_data::RSA_PSS_2048_8192_SHA384_LEGACY_KEY,
    &signed_data::RSA_PSS_2048_8192_SHA512_LEGACY_KEY,
    &signed_data::ED25519,
    // Algorithms deprecated because they are annoying (P-521) or because
    // they are nonsensical combinations.
    &signed_data::ECDSA_P256_SHA384, // Truncates digest.
    &signed_data::ECDSA_P384_SHA256, // Digest is unnecessarily short.
];
