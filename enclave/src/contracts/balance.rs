use crate::std::collections::{BTreeMap};
use serde::{de, Serialize, Deserialize, Serializer, Deserializer};
use core::str::FromStr;
use crate::std::prelude::v1::*;
use crate::std::vec::Vec;

use crate::contracts;
use crate::types::TxRef;

extern crate runtime as chain;

#[derive(Serialize, Deserialize, Debug)]
pub struct Balance{
    accounts: BTreeMap<chain::AccountId, chain::Balance>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Command{
    Transfer(TransferDetails),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransferDetails{
    accounts: Vec<Account>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Request{
    #[serde(serialize_with = "se_to_str", deserialize_with = "de_from_str")]
    account: chain::AccountId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response{
    balance: chain::Balance,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Account{
    #[serde(serialize_with = "se_to_str", deserialize_with = "de_from_str")]
    account_id: chain::AccountId,

    balance: chain::Balance,
}

impl Balance{
    pub fn new() -> Self{
        Balance{
            accounts: BTreeMap::<chain::AccountId, chain::Balance>::new(),
        }
    }
}

impl contracts::Contract<Command, Request, Response> for Balance{
    fn id(&self) -> contracts::ContractId { contracts::BALANCE }

    fn handle_command(&mut self, origin: &String, txref: &TxRef, cmd: Command){
        match cmd {
            Command::Transfer(details) => {
                for account in details.accounts.iter() {
                    self.accounts.insert(account.account_id, account.balance);
                }
            }
        }
    }

    fn handle_query(&mut self, req: Request) -> Response{
        // todo: should validate user id first.


        let mut balance: chain::Balance = ();
        if let Some(ba) = self.accounts.get(&req.account) {
            balance = *ba;
        }

        Response{
            balance,
        }
    }
}

fn se_to_str<S>(value: &chain::AccountId, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
    use std::fmt::Write;
    let mut s = String::new();
    for a in value.0.iter() {
        write!(s,"{:02x}", a)
    }
    String::serialize(&s, serializer)
}

fn de_from_str<'de, D>(deserializer: D) -> Result<chain::Balance, D::Error>
    where D: Deserializer<'de> {
    let s = String::deserialize(deserializer)?;
    chain::AccountId::from_str(&s).map_err(de::Error::custom)
}

/*
fn map_to_str<S>(value: BTreeMap<chain::AccountId,chain::Balance>, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
    let mut map = serializer.serialize_map(Some(value.len()))?;
    for (k,v) in value{
        let ks = acid_to_str(k);
        let vs = v.to_string();
        map.serialize_entry(ks, vs)?;
    }
    map.end()
}

fn map_from_str<'de, D>(deserializer: D) -> Result<BTreeMap<chain::AccountId,chain::Balance>, D::Error>
    where D: Deserializer<'de> {
    let s = String::deserialize(deserializer)?;
    BTreeMap::<chain::AccountId,chain::Balance>::from_str(&s).map_err(de::Error::custom)
}

fn vec_to_str<S>(value: Vec<chain::AccountId>, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
    let mut seq = serializer.serialize_seq(Some(value.len()))?;
    for e in value {
        let es = acid_to_str(e);
        seq.serialize_element(es)?;
    }
    seq.end()
}

fn vec_from_str<'de, D>(deserializer: D) -> Result<Vec<chain::AccountId>, D::Error>
    where D: Deserializer<'de>{

}

fn acid_to_str(acid: chain::AccountId) -> String{
    use std::fmt::Write;
    let mut s = String::new();
    for a in acid.0.iter() {
        write!(s,"{:02x}", a)
    }
    s
}

fn str_to_acid(s: String) -> Result<chain::AccountId,()>{
    let b = s.as_bytes();
    sp_core::crypto::AccountId32::try_from(b)
}
*/
