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
    amounts: BTreeMap<chain::AccountId, chain::Balance>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Command{
    Transfer(TransferDetails),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransferDetails{
    amounts: BTreeMap<chain::AccountId,chain::Balance>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Request{
    accounts: Vec<chain::AccountId>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response{
    amounts: BTreeMap<chain::AccountId, Option<chain::Balance>>,
}

impl Balance{
    pub fn new() -> Self{
        Balance{
            amounts: BTreeMap::<chain::AccountId, chain::Balance>::new(),
        }
    }
}

impl contracts::Contract<Command, Request, Response> for Balance{
    fn id(&self) -> contracts::ContractId { contracts::BALANCE }

    fn handle_command(&mut self, origin: &String, txref: &TxRef, cmd: Command){
        match cmd {
            Command::Transfer(details) => {
                for (account, balance) in details.amounts.iter() {
                    self.amounts.insert(*account, *balance);
                }
            }
        }
    }

    fn handle_query(&mut self, req: Request) -> Response{
        // todo: should validate user id first.

        let mut resp_map = BTreeMap::<chain::AccountId, Option<chain::Balance>>::new();
        for i in req.accounts {
            resp_map.insert(*i, self.amounts.get(&i).copied());
        };
        Response{
            amounts: resp_map
        }
    }
}

