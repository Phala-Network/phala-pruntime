use yasna::models::*;
use chrono::*;

//#[test]
pub fn test_utctime_parse() {
    let datetime = *UTCTime::parse(b"8201021200Z").unwrap().datetime();
    assert_eq!(datetime.year(), 1982);
    assert_eq!(datetime.month(), 1);
    assert_eq!(datetime.day(), 2);
    assert_eq!(datetime.hour(), 12);
    assert_eq!(datetime.minute(), 0);
    assert_eq!(datetime.second(), 0);
    assert_eq!(datetime.nanosecond(), 0);

    let datetime = *UTCTime::parse(b"0101021200Z").unwrap().datetime();
    assert_eq!(datetime.year(), 2001);
    assert_eq!(datetime.month(), 1);
    assert_eq!(datetime.day(), 2);
    assert_eq!(datetime.hour(), 12);
    assert_eq!(datetime.minute(), 0);
    assert_eq!(datetime.second(), 0);
    assert_eq!(datetime.nanosecond(), 0);

    let datetime = UTCTime::parse(b"8201021200Z").unwrap();
    assert_eq!(&datetime.to_string(), "820102120000Z");

    let datetime = UTCTime::parse(b"8201020700-0500").unwrap();
    assert_eq!(&datetime.to_string(), "820102120000Z");

    let datetime = UTCTime::parse(b"0101021200Z").unwrap();
    assert_eq!(&datetime.to_string(), "010102120000Z");

    let datetime = UTCTime::parse(b"010102120034Z").unwrap();
    assert_eq!(&datetime.to_string(), "010102120034Z");

    let datetime = UTCTime::parse(b"000229123456Z").unwrap();
    assert_eq!(&datetime.to_string(), "000229123456Z");
}

//#[test]
pub fn test_generalized_time_parse() {
    let datetime =
        *GeneralizedTime::parse(b"19851106210627.3Z").unwrap().datetime();
    assert_eq!(datetime.year(), 1985);
    assert_eq!(datetime.month(), 11);
    assert_eq!(datetime.day(), 6);
    assert_eq!(datetime.hour(), 21);
    assert_eq!(datetime.minute(), 6);
    assert_eq!(datetime.second(), 27);
    assert_eq!(datetime.nanosecond(), 300_000_000);

    let datetime = GeneralizedTime::parse(b"19851106210627.3-0500").unwrap();
    assert_eq!(&datetime.to_string(), "19851107020627.3Z");

    let datetime = GeneralizedTime::parse(b"198511062106Z").unwrap();
    assert_eq!(&datetime.to_string(), "19851106210600Z");

    let datetime = GeneralizedTime::parse(b"198511062106.456Z").unwrap();
    assert_eq!(&datetime.to_string(), "19851106210627.36Z");

    let datetime = GeneralizedTime::parse(b"1985110621Z").unwrap();
    assert_eq!(&datetime.to_string(), "19851106210000Z");

    let datetime = GeneralizedTime::parse(b"1985110621.14159Z").unwrap();
    assert_eq!(&datetime.to_string(), "19851106210829.724Z");

    let datetime =
        GeneralizedTime::parse(b"19990101085960.1234+0900").unwrap();
    assert_eq!(&datetime.to_string(), "19981231235960.1234Z");

    let datetime =
        GeneralizedTime::parse(
            b"20080229033411.3625431984612391672391625532918636000680000-0500"
        ).unwrap();
    assert_eq!(&datetime.to_string(),
        "20080229083411.362543198461239167239162553291863600068Z");
}
