use num_traits::*;
use core::num::Wrapping;

pub fn unsigned_wrapping_is_unsigned() {
    fn require_unsigned<T: Unsigned>(_: &T) {}
    require_unsigned(&Wrapping(42_u32));
}
/*
// Commenting this out since it doesn't compile on Rust 1.8,
// because on this version Wrapping doesn't implement Neg and therefore can't
// implement Signed.
#[test]
fn signed_wrapping_is_signed() {
    fn require_signed<T: Signed>(_: &T) {}
    require_signed(&Wrapping(-42));
}
*/
