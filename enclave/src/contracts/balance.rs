use std::collections::{BTreeMap};
use serde::{de::{self,Visitor}, Serialize, Deserialize, Serializer, Deserializer};
use core::str::FromStr;
use core::cmp::Ord;
use std::prelude::v1::*;
use std::{fmt,vec::Vec};
use std::str;

use crate::contracts;
use crate::types::TxRef;

extern crate runtime as chain;

#[derive(Serialize, Deserialize, Debug)]
pub struct Balance{
    accounts: BTreeMap<AccountIdWrapper, chain::Balance>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Command{
    Transfer(TransferDetails),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransferDetails{
    accounts: Vec<(AccountIdWrapper,chain::Balance)>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Request{
    //#[serde(serialize_with = "se_to_str", deserialize_with = "de_from_str")]
    account: AccountIdWrapper,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response{
    balance: chain::Balance,
}

impl Balance{
    pub fn new() -> Self{
        Balance{
            accounts: BTreeMap::<AccountIdWrapper, chain::Balance>::new(),
        }
    }
}

impl contracts::Contract<Command, Request, Response> for Balance{
    fn id(&self) -> contracts::ContractId { contracts::BALANCE }

    fn handle_command(&mut self, origin: &String, txref: &TxRef, cmd: Command){
        match cmd {
            Command::Transfer(details) => {
                for account in details.accounts.iter() {
                    self.accounts.insert(account.0, account.1);
                }
            }
        }
    }

    fn handle_query(&mut self, req: Request) -> Response{
        // todo: should validate user id first.


        let mut balance: chain::Balance = 0;
        if let Some(ba) = self.accounts.get(&req.account) {
            balance = *ba;
        }

        Response{
            balance,
        }
    }
}

#[derive(Default, Debug, Ord, PartialOrd, Eq, PartialEq, Copy)]
pub struct AccountIdWrapper( chain::AccountId );

impl<'a> AccountIdWrapper{
    fn from(b: &'a [u8]) -> Self {
        let mut a = AccountIdWrapper::default();
        use core::convert::TryFrom;
        a.0 = sp_core::crypto::AccountId32::try_from(b).unwrap();
        a
    }
    fn into(self) -> chain::AccountId {self.0}
}

impl Serialize for AccountIdWrapper{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer,
    {
        serializer.serialize_bytes(self.0.as_ref())
    }
}

impl<'de> Deserialize<'de> for AccountIdWrapper{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        deserializer.deserialize_bytes(AcidVisitor)
    }
}

struct AcidVisitor;

impl<'de> Visitor<'de> for AcidVisitor{
    type Value = AccountIdWrapper;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("AccountID is [u8;32]")
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
        where E: de::Error,
    {
        if v.len() == 32 {
            Ok(AccountIdWrapper::from(v))
        }else {
            Err(E::custom(format!("AccountId bytes length wrong: {}", str::from_utf8(&v).unwrap())))
        }
    }
}
/*
pub struct BalanceWrapper( chain::Balance );

impl BalanceWrapper{
    pub fn into(&self) -> chain::Balance {self.0}
}

impl Serialize for BalanceWrapper{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer,
    {
        let s = self.0.to_string();
        String::serialize(&s, serializer)
    }
}

impl Deserialize for BalanceWrapper{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>
    {

    }
}
*/

/*
fn se_to_str<S>(value: chain::AccountId, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
    use std::fmt::Write;
    let mut s = String::new();
    for a in sp_core::crypto::AccountId32::from(value).iter() {
        write!(s,"{:02x}", a);
    }
    String::serialize(&s, serializer)
}

fn de_from_str<'de, D>(deserializer: D) -> Result<chain::Balance, D::Error>
    where D: Deserializer<'de> {
    let s = String::deserialize(deserializer)?;
    chain::AccountId::from_str(&s).map_err(de::Error::custom)
}


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
