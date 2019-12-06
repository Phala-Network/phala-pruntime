use serde_json::{Number, Value};

//#[test]
pub fn number() {
    assert_eq!(format!("{:?}", Number::from(1)), "Number(1)");
    assert_eq!(format!("{:?}", Number::from(-1)), "Number(-1)");
    assert_eq!(
        format!("{:?}", Number::from_f64(1.0).unwrap()),
        "Number(1.0)"
    );
}

//#[test]
pub fn value_null() {
    assert_eq!(format!("{:?}", json!(null)), "Null");
}

//#[test]
pub fn value_bool() {
    assert_eq!(format!("{:?}", json!(true)), "Bool(true)");
    assert_eq!(format!("{:?}", json!(false)), "Bool(false)");
}

//#[test]
pub fn value_number() {
    assert_eq!(format!("{:?}", json!(1)), "Number(1)");
    assert_eq!(format!("{:?}", json!(-1)), "Number(-1)");
    assert_eq!(format!("{:?}", json!(1.0)), "Number(1.0)");
}

//#[test]
pub fn value_string() {
    assert_eq!(format!("{:?}", json!("s")), "String(\"s\")");
}

//#[test]
pub fn value_array() {
    assert_eq!(format!("{:?}", json!([])), "Array([])");
}

//#[test]
pub fn value_object() {
    assert_eq!(format!("{:?}", json!({})), "Object({})");
}

//#[test]
pub fn error() {
    let err = serde_json::from_str::<Value>("{0}").unwrap_err();
    let expected = "Error(\"key must be a string\", line: 1, column: 2)";
    assert_eq!(format!("{:?}", err), expected);
}
