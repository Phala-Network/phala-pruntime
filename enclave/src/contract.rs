use std::prelude::v1::*;
use std::vec::Vec;
use serde::{de, Serialize, Deserialize, Serializer, Deserializer};
use core::str::FromStr;

extern crate runtime as chain;

use crate::types::TxRef;

pub type ItemId = u32;
pub type OrderId = u32;

// item

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
  id: ItemId,
  txref: TxRef,
  seller: String,
  details: ItemDetails,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemDetails {
  pub name: String,
  pub category: String,
  pub description: String,
  pub price: PricePolicy,
  pub dataset_link: String,
  pub dataset_preview: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PricePolicy {
  PerRow {
    #[serde(serialize_with = "se_to_str", deserialize_with = "de_from_str")]
    price: chain::Balance
  },
}

// order

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Order {
  id: OrderId,
  txref: TxRef,
  buyer: String,
  item_id: ItemId,
  state: OrderState,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderState {
  data_ready: bool,
  query_ready: bool,
  result_ready: bool
}

// deesr

fn se_to_str<S>(value: &chain::Balance, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer {
  let s = value.to_string();
  String::serialize(&s, serializer)
}

fn de_from_str<'de, D>(deserializer: D) -> Result<chain::Balance, D::Error>
where D: Deserializer<'de> {
    let s = String::deserialize(deserializer)?;
    chain::Balance::from_str(&s).map_err(de::Error::custom)
}

// contract

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
  List(ItemDetails),
  OpenOrder(ItemId),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
  GetItems,
  GetOrders,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
  GetItems {items: Vec<Item>},
  GetOrders {orders: Vec<Order>}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Contract {
  items: Vec<Item>,
  orders: Vec<Order>,
}

impl Contract {
  pub fn new() -> Contract {
    Contract {
      items: Vec::<Item>::new(),
      orders: Vec::<Order>::new(),
    }
  }

  pub fn handle_command(&mut self, origin: &String, txref: &TxRef, cmd: Command) {
    match cmd {
      Command::List(details) => {
        self.items.push(Item {
          id: self.items.len() as ItemId,
          txref: txref.clone(),
          seller: origin.clone(),
          details: details,
        })
      },
      Command::OpenOrder(item_id) => {
        self.orders.push(Order {
          id: self.orders.len() as OrderId,
          txref: txref.clone(),
          buyer: origin.clone(),
          item_id: item_id,
          state: OrderState {  // TODO
            data_ready: false,
            query_ready: false,
            result_ready: false,
          }
        });
      },
    }
  }

  pub fn query(&self, req: Request) -> Response {
    match req {
      Request::GetItems => Response::GetItems { items: self.items.clone() },
      Request::GetOrders => Response::GetOrders { orders: self.orders.clone() },
    }
  }

}