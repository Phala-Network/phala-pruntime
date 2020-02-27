use crate::std::string::String;
use crate::std::fmt::Debug;

use serde::{de::DeserializeOwned, Serialize, Deserialize};

use crate::types::TxRef;

pub mod data_plaza;

pub type ContractId = u32;

pub trait Contract<Cmd, QReq, QResp>: Serialize + DeserializeOwned + Debug
where
  Cmd: Serialize + DeserializeOwned + Debug,
  QReq: Serialize + DeserializeOwned + Debug,
  QResp: Serialize + DeserializeOwned + Debug
{
  fn id(&self) -> ContractId;
  fn handle_command(&mut self, origin: &String, txref: &TxRef, cmd: Cmd);
  fn handle_query(&mut self, req: QReq) -> QResp;
}
