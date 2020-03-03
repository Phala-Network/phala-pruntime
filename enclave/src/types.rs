use crate::std::string::String;
use serde::{Serialize, Deserialize};

use crate::cryptography::AeadCipher;

extern crate runtime as chain;

// supportive

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TxRef {
  pub blocknum: chain::BlockNumber,
  pub index: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Payload {
  Plain(String),
  Cipher(AeadCipher),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignedQuery {
  query: String,
  origin: String,
  sig_b64: String,
}
