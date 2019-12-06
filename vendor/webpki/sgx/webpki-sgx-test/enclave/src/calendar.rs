//#[test]
pub fn test_days_before_unix_epoch() {
    use webpki::calendar::{days_before_year_ad, DAYS_BEFORE_UNIX_EPOCH_AD};
    assert_eq!(DAYS_BEFORE_UNIX_EPOCH_AD, days_before_year_ad(1970));
}

//#[test]
pub fn test_days_in_month() {
    use webpki::calendar::days_in_month;
    assert_eq!(days_in_month(2017, 1), 31);
    assert_eq!(days_in_month(2017, 2), 28);
    assert_eq!(days_in_month(2017, 3), 31);
    assert_eq!(days_in_month(2017, 4), 30);
    assert_eq!(days_in_month(2017, 5), 31);
    assert_eq!(days_in_month(2017, 6), 30);
    assert_eq!(days_in_month(2017, 7), 31);
    assert_eq!(days_in_month(2017, 8), 31);
    assert_eq!(days_in_month(2017, 9), 30);
    assert_eq!(days_in_month(2017, 10), 31);
    assert_eq!(days_in_month(2017, 11), 30);
    assert_eq!(days_in_month(2017, 12), 31);

    // leap cases
    assert_eq!(days_in_month(2000, 2), 29);
    assert_eq!(days_in_month(2004, 2), 29);
    assert_eq!(days_in_month(2016, 2), 29);
    assert_eq!(days_in_month(2100, 2), 28);
}

//#[test]
pub fn test_time_from_ymdhms_utc() {
    use webpki::calendar::time_from_ymdhms_utc;
    use webpki::Time;

    // year boundary
    assert_eq!(
        Time::from_seconds_since_unix_epoch(1483228799),
        time_from_ymdhms_utc(2016, 12, 31, 23, 59, 59).unwrap()
    );
    assert_eq!(
        Time::from_seconds_since_unix_epoch(1483228800),
        time_from_ymdhms_utc(2017, 1, 1, 0, 0, 0).unwrap()
    );

    // not a leap year
    assert_eq!(
        Time::from_seconds_since_unix_epoch(1492449162),
        time_from_ymdhms_utc(2017, 4, 17, 17, 12, 42).unwrap()
    );

    // leap year, post-feb
    assert_eq!(
        Time::from_seconds_since_unix_epoch(1460913162),
        time_from_ymdhms_utc(2016, 4, 17, 17, 12, 42).unwrap()
    );
}
