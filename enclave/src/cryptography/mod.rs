use crate::std::string::String;
use crate::std::vec::Vec;
use serde::{Serialize, Deserialize};

pub mod aead;
pub mod ecdh;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AeadCipher {
  pub iv_b64: String,
  pub cipher_b64: String,
  pub pubkey_b64: String
}

// Decrypt by AEAD-AES-GCM with secret key agreeded by ECDH.
pub fn decrypt(cipher: &AeadCipher, privkey: &ring::agreement::EphemeralPrivateKey)
  -> Result<Vec<u8>, Error> 
{
  let pubkey = base64::decode(&cipher.pubkey_b64)
      .map_err(|_| Error::BadInput("pubkey_b64"))?;
  let mut data = base64::decode(&cipher.cipher_b64)
      .map_err(|_| Error::BadInput("cipher_b64"))?;
  let iv = base64::decode(&cipher.iv_b64)
      .map_err(|_| Error::BadInput("iv_b64"))?;
  // ECDH derived secret
  let secret = ecdh::agree(privkey, &pubkey);
  println!("Agreed SK: {:?}", secret);
  let msg = aead::decrypt(iv.as_slice(), secret.as_slice(), &mut data);
  Ok(msg.to_vec())
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
  BadInput(&'static str),
}