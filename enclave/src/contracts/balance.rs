use std::collections::{BTreeMap};
use serde::{de::{self,Visitor}, Serialize, Deserialize, Serializer, Deserializer};
use crate::std::string::String;
use core::{fmt,str};
use core::cmp::Ord;

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransferDetails{
    target: AccountIdWrapper,
    amount: chain::Balance,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Request{
    FreeBalance(FreeBalanceReq),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FreeBalanceReq{
    account: AccountIdWrapper,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Response{
    FreeBalance(FreeBalanceResp),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FreeBalanceResp{
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
            Command::Transfer(detail) => {
                self.accounts.insert(detail.clone().target, detail.amount);
            }
        }
    }

    fn handle_query(&mut self, req: Request) -> Response{
        // todo: should validate user id first.

        match req {
            Request::FreeBalance(fbreq) => {
                let mut balance: chain::Balance = 0;
                if let Some(ba) = self.accounts.get(&fbreq.account) {
                    balance = *ba;
                }
                Response::FreeBalance(FreeBalanceResp{ balance,})
            },
        }
    }
}

#[derive(Default, Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
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
        } else {
            Err(E::custom(format!("AccountId bytes length wrong: {}", str::from_utf8(&v).unwrap())))
        }
    }
}
