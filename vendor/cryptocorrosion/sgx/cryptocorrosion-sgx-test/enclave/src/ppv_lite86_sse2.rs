use ppv_lite86::*;
use ppv_lite86::x86_64::{SSE2, SSE41, SSSE3};
use ppv_lite86::Machine;

//#[test]
pub fn test_bswap32_s2_vs_s3() {
    let xs = [0x0f0e_0d0c, 0x0b0a_0908, 0x0706_0504, 0x0302_0100];
    let ys = [0x0c0d_0e0f, 0x0809_0a0b, 0x0405_0607, 0x0001_0203];

    let s2 = unsafe { SSE2::instance() };
    let s3 = unsafe { SSSE3::instance() };

    let x_s2 = {
        let x_s2: <SSE2 as Machine>::u32x4 = s2.vec(xs);
        x_s2.bswap()
    };

    let x_s3 = {
        let x_s3: <SSSE3 as Machine>::u32x4 = s3.vec(xs);
        x_s3.bswap()
    };

    assert_eq!(x_s2, unsafe { core::mem::transmute(x_s3) });
    assert_eq!(x_s2, s2.vec(ys));
}

//#[test]
pub fn test_bswap64_s2_vs_s3() {
    let xs = [0x0f0e_0d0c_0b0a_0908, 0x0706_0504_0302_0100];
    let ys = [0x0809_0a0b_0c0d_0e0f, 0x0001_0203_0405_0607];

    let s2 = unsafe { SSE2::instance() };
    let s3 = unsafe { SSSE3::instance() };

    let x_s2 = {
        let x_s2: <SSE2 as Machine>::u64x2 = s2.vec(xs);
        x_s2.bswap()
    };

    let x_s3 = {
        let x_s3: <SSSE3 as Machine>::u64x2 = s3.vec(xs);
        x_s3.bswap()
    };

    assert_eq!(x_s2, s2.vec(ys));
    assert_eq!(x_s3, unsafe { core::mem::transmute(x_s3) });
}

//#[test]
pub fn test_shuffle32_s2_vs_s3() {
    let xs = [0x0, 0x1, 0x2, 0x3];
    let ys = [0x2, 0x3, 0x0, 0x1];
    let zs = [0x1, 0x2, 0x3, 0x0];

    let s2 = unsafe { SSE2::instance() };
    let s3 = unsafe { SSSE3::instance() };

    let x_s2 = {
        let x_s2: <SSE2 as Machine>::u32x4 = s2.vec(xs);
        x_s2.shuffle2301()
    };
    let x_s3 = {
        let x_s3: <SSSE3 as Machine>::u32x4 = s3.vec(xs);
        x_s3.shuffle2301()
    };
    assert_eq!(x_s2, s2.vec(ys));
    assert_eq!(x_s3, unsafe { core::mem::transmute(x_s3) });

    let x_s2 = {
        let x_s2: <SSE2 as Machine>::u32x4 = s2.vec(xs);
        x_s2.shuffle3012()
    };
    let x_s3 = {
        let x_s3: <SSSE3 as Machine>::u32x4 = s3.vec(xs);
        x_s3.shuffle3012()
    };
    assert_eq!(x_s2, s2.vec(zs));
    assert_eq!(x_s3, unsafe { core::mem::transmute(x_s3) });

    let x_s2 = x_s2.shuffle1230();
    let x_s3 = x_s3.shuffle1230();
    assert_eq!(x_s2, s2.vec(xs));
    assert_eq!(x_s3, unsafe { core::mem::transmute(x_s3) });
}

//#[test]
pub fn test_shuffle64_s2_vs_s3() {
    let xs = [0x0, 0x1, 0x2, 0x3];
    let ys = [0x2, 0x3, 0x0, 0x1];
    let zs = [0x1, 0x2, 0x3, 0x0];

    let s2 = unsafe { SSE2::instance() };
    let s3 = unsafe { SSSE3::instance() };

    let x_s2 = {
        let x_s2: <SSE2 as Machine>::u64x4 = s2.vec(xs);
        x_s2.shuffle2301()
    };
    let x_s3 = {
        let x_s3: <SSSE3 as Machine>::u64x4 = s3.vec(xs);
        x_s3.shuffle2301()
    };
    assert_eq!(x_s2, s2.vec(ys));
    assert_eq!(x_s3, unsafe { core::mem::transmute(x_s3) });

    let x_s2 = {
        let x_s2: <SSE2 as Machine>::u64x4 = s2.vec(xs);
        x_s2.shuffle3012()
    };
    let x_s3 = {
        let x_s3: <SSSE3 as Machine>::u64x4 = s3.vec(xs);
        x_s3.shuffle3012()
    };
    assert_eq!(x_s2, s2.vec(zs));
    assert_eq!(x_s3, unsafe { core::mem::transmute(x_s3) });

    let x_s2 = x_s2.shuffle1230();
    let x_s3 = x_s3.shuffle1230();
    assert_eq!(x_s2, s2.vec(xs));
    assert_eq!(x_s3, unsafe { core::mem::transmute(x_s3) });
}

//#[test]
pub fn test_lanes_u32x4() {
    let xs = [0x1, 0x2, 0x3, 0x4];

    let s2 = unsafe { SSE2::instance() };
    let s3 = unsafe { SSSE3::instance() };
    let s4 = unsafe { SSE41::instance() };

    {
        let x_s2: <SSE2 as Machine>::u32x4 = s2.vec(xs);
        let y_s2 = <SSE2 as Machine>::u32x4::from_lanes(xs);
        assert_eq!(x_s2, y_s2);
        assert_eq!(xs, y_s2.to_lanes());
    }

    {
        let x_s3: <SSSE3 as Machine>::u32x4 = s3.vec(xs);
        let y_s3 = <SSSE3 as Machine>::u32x4::from_lanes(xs);
        assert_eq!(x_s3, y_s3);
        assert_eq!(xs, y_s3.to_lanes());
    }

    {
        let x_s4: <SSE41 as Machine>::u32x4 = s4.vec(xs);
        let y_s4 = <SSE41 as Machine>::u32x4::from_lanes(xs);
        assert_eq!(x_s4, y_s4);
        assert_eq!(xs, y_s4.to_lanes());
    }
}

//#[test]
pub fn test_lanes_u64x2() {
    let xs = [0x1, 0x2];

    let s2 = unsafe { SSE2::instance() };
    let s3 = unsafe { SSSE3::instance() };
    let s4 = unsafe { SSE41::instance() };

    {
        let x_s2: <SSE2 as Machine>::u64x2 = s2.vec(xs);
        let y_s2 = <SSE2 as Machine>::u64x2::from_lanes(xs);
        assert_eq!(x_s2, y_s2);
        assert_eq!(xs, y_s2.to_lanes());
    }

    {
        let x_s3: <SSSE3 as Machine>::u64x2 = s3.vec(xs);
        let y_s3 = <SSSE3 as Machine>::u64x2::from_lanes(xs);
        assert_eq!(x_s3, y_s3);
        assert_eq!(xs, y_s3.to_lanes());
    }

    {
        let x_s4: <SSE41 as Machine>::u64x2 = s4.vec(xs);
        let y_s4 = <SSE41 as Machine>::u64x2::from_lanes(xs);
        assert_eq!(x_s4, y_s4);
        assert_eq!(xs, y_s4.to_lanes());
    }
}

//#[test]
pub fn test_vec4_u32x4_s2() {
    let xs = [1, 2, 3, 4];
    let s2 = unsafe { SSE2::instance() };
    let x_s2: <SSE2 as Machine>::u32x4 = s2.vec(xs);
    assert_eq!(x_s2.extract(0), 1);
    assert_eq!(x_s2.extract(1), 2);
    assert_eq!(x_s2.extract(2), 3);
    assert_eq!(x_s2.extract(3), 4);
    assert_eq!(x_s2.insert(0xf, 0), s2.vec([0xf, 2, 3, 4]));
    assert_eq!(x_s2.insert(0xf, 1), s2.vec([1, 0xf, 3, 4]));
    assert_eq!(x_s2.insert(0xf, 2), s2.vec([1, 2, 0xf, 4]));
    assert_eq!(x_s2.insert(0xf, 3), s2.vec([1, 2, 3, 0xf]));
}

//#[test]
pub fn test_vec4_u32x4_s4() {
    let xs = [1, 2, 3, 4];
    let s4 = unsafe { SSE41::instance() };
    let x_s4: <SSE41 as Machine>::u32x4 = s4.vec(xs);
    assert_eq!(x_s4.extract(0), 1);
    assert_eq!(x_s4.extract(1), 2);
    assert_eq!(x_s4.extract(2), 3);
    assert_eq!(x_s4.extract(3), 4);
    assert_eq!(x_s4.insert(0xf, 0), s4.vec([0xf, 2, 3, 4]));
    assert_eq!(x_s4.insert(0xf, 1), s4.vec([1, 0xf, 3, 4]));
    assert_eq!(x_s4.insert(0xf, 2), s4.vec([1, 2, 0xf, 4]));
    assert_eq!(x_s4.insert(0xf, 3), s4.vec([1, 2, 3, 0xf]));
}

//#[test]
pub fn test_vec2_u64x2_s2() {
    let xs = [0x1, 0x2];
    let s2 = unsafe { SSE2::instance() };
    let x_s2: <SSE2 as Machine>::u64x2 = s2.vec(xs);
    assert_eq!(x_s2.extract(0), 1);
    assert_eq!(x_s2.extract(1), 2);
    assert_eq!(x_s2.insert(0xf, 0), s2.vec([0xf, 2]));
    assert_eq!(x_s2.insert(0xf, 1), s2.vec([1, 0xf]));
}

//#[test]
pub fn test_vec4_u64x2_s4() {
    let xs = [0x1, 0x2];
    let s4 = unsafe { SSE41::instance() };
    let x_s4: <SSE41 as Machine>::u64x2 = s4.vec(xs);
    assert_eq!(x_s4.extract(0), 1);
    assert_eq!(x_s4.extract(1), 2);
    assert_eq!(x_s4.insert(0xf, 0), s4.vec([0xf, 2]));
    assert_eq!(x_s4.insert(0xf, 1), s4.vec([1, 0xf]));
}
