use serde::{Serialize, Deserialize};

extern crate runtime as chain;

// supportive

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TxRef {
  pub blocknum: chain::BlockNumber,
  pub index: u32,
}