use std::prelude::v1::*;
use std::vec::Vec;
use std::collections::HashMap;
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
  details: OrderDetails,
  state: OrderState,  // maybe shouldn't serialize this
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderDetails {
  item_id: ItemId,
  query_link: String,
  // query parameters...
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderState {
  data_ready: bool,
  query_ready: bool,
  result_ready: bool,
  matched_rows: u32,
  result_path: String,
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
  OpenOrder(OrderDetails),
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
  #[serde(skip)]
  dataset: HashMap<String, Vec<u8>>
}

impl Contract {
  pub fn new() -> Contract {
    Contract {
      items: Vec::<Item>::new(),
      orders: Vec::<Order>::new(),
      dataset: HashMap::<String, Vec<u8>>::new(),
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
      Command::OpenOrder(details) => {
        self.orders.push(Order {
          id: self.orders.len() as OrderId,
          txref: txref.clone(),
          buyer: origin.clone(),
          details: details,
          state: OrderState {  // TODO
            data_ready: false,
            query_ready: false,
            result_ready: false,
            matched_rows: 0,
            result_path: String::new(),
          }
        });
      },
    }
  }

  pub fn query(&mut self, req: Request) -> Response {
    match req {
      Request::GetItems => Response::GetItems { items: self.items.clone() },
      Request::GetOrders => {
        self.update_order_state();
        Response::GetOrders { orders: self.orders.clone() }
      },
    }
  }

  pub fn set(&mut self, key: String, value: Vec<u8>) {
    self.dataset.insert(key, value);
  }

  pub fn get(&self, key: &String) -> Option<&Vec<u8>> {
    self.dataset.get(key)
  }

  fn update_order_state(&mut self) {
    for order in &mut self.orders {
      let item_id = order.details.item_id;
      let item = &self.items[item_id as usize];
      // check data available
      let data_link = &item.details.dataset_link;
      if self.dataset.contains_key(data_link) {
        order.state.data_ready = true;
      }
      // check query available
      let query_link = &order.details.query_link;
      if self.dataset.contains_key(query_link) {
        order.state.query_ready = true;
      }
      // compute if possible
      if order.state.data_ready && order.state.query_ready {
        let data = Self::compute(order);
        let path = order.state.result_path.clone();
        self.dataset.insert(path, data);
      }
    }
  }

  fn compute(order: &mut Order) -> Vec<u8> {
    // config
    let selected_query = vec!["name", "phone"];
    let selected_data = vec!["name", "phone"];

    // TODO: join the csv

    order.state.result_ready = true;
    order.state.matched_rows = 10;
    order.state.result_path = format!("/order/{}", order.id);

    vec![0, 1, 2, 3]
  }

}