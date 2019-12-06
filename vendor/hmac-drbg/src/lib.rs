#![no_std]

extern crate hmac;
extern crate generic_array;

use hmac::digest::{Input, BlockInput, FixedOutput, Reset};
use generic_array::{ArrayLength, GenericArray};
use hmac::{Mac, Hmac};
use hmac::crypto_mac::MacResult;

pub struct HmacDRBG<D>
    where D: Input + BlockInput + FixedOutput + Default + Reset + Clone,
          D::BlockSize: ArrayLength<u8>,
          D::OutputSize: ArrayLength<u8>
{
    //digest: D,
    k: MacResult<D::OutputSize>,
    v: MacResult<D::OutputSize>,
    count: usize,
}

/// The key that Hmac processes must be the same as the block size of the
/// underlying Digest. If the provided key is smaller than that, we just pad it
/// with zeros. If its larger, we hash it and then pad it with zeros.
fn expand_key<D>(key: &[u8]) -> GenericArray<u8, D::BlockSize>
    where D: Input + BlockInput + FixedOutput + Default,
          D::BlockSize: ArrayLength<u8>
{
    let mut exp_key = GenericArray::default();

    if key.len() <= exp_key.len() {
        exp_key[..key.len()].copy_from_slice(key);
    } else {
        let mut digest = D::default();
        digest.input(key);
        let output = digest.fixed_result();
        let n = core::cmp::min(output.len(), exp_key.len());
        exp_key[..n].copy_from_slice(&output[..n]);
    }
    exp_key
}

impl<D> HmacDRBG<D>
    where D: Input + BlockInput + FixedOutput + Default + Reset + Clone,
          D::BlockSize: ArrayLength<u8>,
          D::OutputSize: ArrayLength<u8>
{
    pub fn new(entropy: &[u8], nonce: &[u8], pers: &[u8]) -> Self {
        let mut k = GenericArray::<u8, D::OutputSize>::default();
        let mut v = GenericArray::<u8, D::OutputSize>::default();

        for i in 0..k.as_slice().len() {
            k[i] = 0x0;
        }

        for i in 0..v.as_slice().len() {
            v[i] = 0x01;
        }

        let mut this = Self {
            //digest: D::default(),
            k: MacResult::new(k),
            v: MacResult::new(v),
            count: 0,
        };

        this.update(Some(&[entropy, nonce, pers]));
        this.count = 1;

        this
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn reseed(&mut self, entropy: &[u8], add: Option<&[u8]>) {
        self.update(Some(&[entropy, add.unwrap_or(&[])]))
    }

    pub fn generate<T: ArrayLength<u8>>(&mut self, add: Option<&[u8]>) -> GenericArray<u8, T> {
        let mut result = GenericArray::default();
        self.generate_to_slice(result.as_mut_slice(), add);
        result
    }

    pub fn generate_to_slice(&mut self, result: &mut [u8], add: Option<&[u8]>) {
        if let Some(add) = add {
            self.update(Some(&[add]));
        }

        let mut i = 0;
        while i < result.len() {
            let mut vmac = self.hmac();
            vmac.input(self.v.code_ref());
            self.v = vmac.result();

            for j in 0..self.v.code_ref().len() {
                result[i + j] = self.v.code_ref()[j];
            }
            i += self.v.code_ref().len();
        }

        match add {
            Some(add) => {
                self.update(Some(&[add]));
            },
            None => {
                self.update(None);
            },
        }
        self.count += 1;
    }

    fn hmac(&self) -> Hmac<D> {
        let kk:  &GenericArray<u8, D::OutputSize> = self.k.code_ref();
        //let mut expanded = GenericArray::default();
        //expanded[..kk.len()].copy_from_slice(kk);
        let expanded = expand_key::<D>(kk);
        Hmac::new(&expanded)
    }

    fn update(&mut self, seeds: Option<&[&[u8]]>) {
        let mut kmac = self.hmac();
        kmac.input(self.v.code_ref());
        kmac.input(&[0x00]);
        if let Some(seeds) = seeds {
            for seed in seeds {
                kmac.input(seed);
            }
        }
        self.k = kmac.result();

        let mut vmac = self.hmac();
        vmac.input(self.v.code_ref());
        self.v = vmac.result();

        if seeds.is_none() {
            return;
        }

        let seeds = seeds.unwrap();

        let mut kmac = self.hmac();
        kmac.input(self.v.code_ref());
        kmac.input(&[0x01]);
        for seed in seeds {
            kmac.input(seed);
        }
        self.k = kmac.result();

        let mut vmac = self.hmac();
        vmac.input(self.v.code_ref());
        self.v = vmac.result();
    }
}
