use yasna::models::*;
//#[test]
pub fn test_display_oid() {
    let pkcs1 = ObjectIdentifier::from_slice(&[1, 2, 840, 113549, 1, 1]);
    assert_eq!(format!("{}", pkcs1), "1.2.840.113549.1.1");
}

//#[test]
pub fn parse_oid() {
    assert_eq!("1.2.840.113549.1.1".parse::<ObjectIdentifier>().unwrap().components(), &[1, 2, 840, 113549, 1, 1]);
    "1.2.840.113549.1.1.".parse::<ObjectIdentifier>().unwrap_err();
    "1.2.840.113549.1.1x".parse::<ObjectIdentifier>().unwrap_err();
    "".parse::<ObjectIdentifier>().unwrap_err();
}
