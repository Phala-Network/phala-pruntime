use groestl_aesni::compressor::*;

use core::arch::x86_64::*;

//#[test]
pub fn test_transpose_invertible() {
    unsafe {
        let x = X8(
            _mm_cvtsi64_si128(0),
            _mm_cvtsi64_si128(1),
            _mm_cvtsi64_si128(2),
            _mm_cvtsi64_si128(3),
            _mm_cvtsi64_si128(4),
            _mm_cvtsi64_si128(5),
            _mm_cvtsi64_si128(6),
            _mm_cvtsi64_si128(7),
        );
        assert_eq!(x, transpose_inv(transpose(x)));
        let y = X8(
            _mm_set_epi64x(0x0306_090c_0f02_0508, 0x0b0e_0104_070a_0d00),
            _mm_set_epi64x(0x0407_0a0d_0003_0609, 0x0c0f_0205_080b_0e01),
            _mm_set_epi64x(0x0508_0b0e_0104_070a, 0x0d00_0306_090c_0f02),
            _mm_set_epi64x(0x0609_0c0f_0205_080b, 0x0e01_0407_0a0d_0003),
            _mm_set_epi64x(0x070a_0d00_0306_090c, 0x0f02_0508_0b0e_0104),
            _mm_set_epi64x(0x080b_0e01_0407_0a0d, 0x0003_0609_0c0f_0205),
            _mm_set_epi64x(0x090c_0f02_0508_0b0e, 0x0104_070a_0d00_0306),
            _mm_set_epi64x(0x0e01_0407_0a0d_0003, 0x0609_0c0f_0205_080b),
        );
        assert_eq!(y, transpose_inv(transpose(y)));
    }
}
