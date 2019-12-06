// test from FIPS 113

use daa::Daa;
use cmac::Mac;

pub fn daa_test() {
    let key = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
    let mut mac = Daa::new_varkey(&key).unwrap();
    mac.input(b"7654321 Now is the time for ");
    let correct = [0xF1, 0xD3, 0x0F, 0x68, 0x49, 0x31, 0x2C, 0xA4];
    mac.verify(&correct).unwrap();
}
