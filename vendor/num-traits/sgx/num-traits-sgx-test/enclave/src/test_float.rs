use core::f64::consts;

const DEG_RAD_PAIRS: [(f64, f64); 7] = [
    (0.0, 0.),
    (22.5, consts::FRAC_PI_8),
    (30.0, consts::FRAC_PI_6),
    (45.0, consts::FRAC_PI_4),
    (60.0, consts::FRAC_PI_3),
    (90.0, consts::FRAC_PI_2),
    (180.0, consts::PI),
];

//#[test]
pub fn convert_deg_rad() {
    use num_traits::float::FloatCore;

    for &(deg, rad) in &DEG_RAD_PAIRS {
        assert!((FloatCore::to_degrees(rad) - deg).abs() < 1e-6);
        assert!((FloatCore::to_radians(deg) - rad).abs() < 1e-6);

        let (deg, rad) = (deg as f32, rad as f32);
        assert!((FloatCore::to_degrees(rad) - deg).abs() < 1e-5);
        assert!((FloatCore::to_radians(deg) - rad).abs() < 1e-5);
    }
}

//#[cfg(feature = "std")]
//#[test]
pub fn convert_deg_rad_std() {
    for &(deg, rad) in &DEG_RAD_PAIRS {
        use num_traits::Float;

        assert!((Float::to_degrees(rad) - deg).abs() < 1e-6);
        assert!((Float::to_radians(deg) - rad).abs() < 1e-6);

        let (deg, rad) = (deg as f32, rad as f32);
        assert!((Float::to_degrees(rad) - deg).abs() < 1e-5);
        assert!((Float::to_radians(deg) - rad).abs() < 1e-5);
    }
}

//#[test]
//// This fails with the forwarded `std` implementation in Rust 1.8.
//// To avoid the failure, the test is limited to `no_std` builds.
//#[cfg(not(feature = "std"))]
//fn to_degrees_rounding() {
//    use float::FloatCore;
//
//    assert_eq!(
//        FloatCore::to_degrees(1_f32),
//        57.2957795130823208767981548141051703
//    );
//}
