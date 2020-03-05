use std::collections::{BTreeMap};
use serde::{de::{self,Visitor}, Serialize, Deserialize, Serializer, Deserializer};
use crate::std::string::String;
use core::{fmt,str};
use core::cmp::Ord;

use crate::contracts;
use crate::types::TxRef;

extern crate runtime as chain;

const ALICE: &'static [u8] = b"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";

#[derive(Serialize, Deserialize, Debug)]
pub struct Balance{
    accounts: BTreeMap<AccountIdWrapper, chain::Balance>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    Transfer {
        dest: AccountIdWrapper,
        #[serde(with = "super::serde_balance")]
        value: chain::Balance,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    FreeBalance {
        account: AccountIdWrapper
    },
}
#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    FreeBalance {
        balance: chain::Balance
    },
}

impl Balance{
    pub fn new() -> Self{
        let mut accounts = BTreeMap::<AccountIdWrapper, chain::Balance>::new();
        accounts.insert(AccountIdWrapper::from(ALICE), 1024 * 10^14);
        Balance{ accounts }
    }
}

impl contracts::Contract<Command, Request, Response> for Balance{
    fn id(&self) -> contracts::ContractId { contracts::BALANCE }

    fn handle_command(&mut self, origin: &String, txref: &TxRef, cmd: Command){
        match cmd {
            Command::Transfer {dest, mut value} => {
                let origin = AccountIdWrapper::from(origin.as_bytes());
                if let Some(orgin_amount) = self.accounts.get_mut(&origin){
                    if orgin_amount >= &mut value {
                        *orgin_amount -= value;
                        if let Some(dest_amount) = self.accounts.get_mut(&dest) {
                            *dest_amount += value;
                        }else {
                            self.accounts.insert(dest, value);
                        }
                    }
                }
            }
        }
    }

    fn handle_query(&mut self, req: Request) -> Response{
        // todo: should validate user id first.

        match req {
            Request::FreeBalance {account} => {
                let mut balance: chain::Balance = 0;
                if let Some(ba) = self.accounts.get(&account) {
                    balance = *ba;
                }
                Response::FreeBalance { balance }
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
        let data_hex = crate::hex::encode_hex_compact(self.0.as_ref());
        serializer.serialize_str(&data_hex)
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
        let value_str = str::from_utf8(&v).map_err(|_| E::custom("Invalid utf8 string"))?;
        if v.len() == 64 {
            let bytes = crate::hex::decode_hex(value_str);  // TODO: error handling
            Ok(AccountIdWrapper::from(&bytes))
        } else {
            Err(E::custom(format!("AccountId bytes length wrong: {}", value_str)))
        }
    }
}
