use std::str::FromStr;

use apca_datav2::{data::{AuthDataBuilder, OrderSide}, orders::{ListOrderRequestBuilder, PlaceOrderRequestBuilder}, rest::Client};
use dotenv_codegen::dotenv;
use anyhow::Result;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Args {
  Buy {symbol: String, qty: f64, limit: Option<f64>},
  Sell{symbol: String, qty: f64, limit: Option<f64>},
  List{#[structopt(default_value="*")] status: OrderStatus, symbols: Option<String>},
  Cancel{id: Option<String>},
}

#[derive(Debug, StructOpt)]
pub enum OrderStatus {
  All, Open, Closed
}
impl Default for OrderStatus {
  fn default() -> Self {
    Self::All
  }
}
impl FromStr for OrderStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
          "*"      => Ok(Self::All),
          "all"    => Ok(Self::All),
          "open"   => Ok(Self::Open),
          "closed" => Ok(Self::Closed),
          _        => Err(s.to_string())
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let auth   = AuthDataBuilder::default()
      .key(dotenv!("APCA_KEY_ID").to_string())
      .secret(dotenv!("APCA_SECRET").to_string())
      .build()?;

    let client = Client::paper(auth);
    match Args::from_args() {
        Args::Buy  { symbol, qty, limit } => buy(&client, symbol, qty, limit).await?,
        Args::Sell { symbol, qty, limit } => sell(&client, symbol, qty, limit).await?,
        Args::List { symbols, status }    => list(&client, symbols, status).await?,
        Args::Cancel{ id }                => cancel(&client, id).await?,
    }

    Ok(())
}

async fn buy(client: &Client, symbol: String, qty: f64, limit: Option<f64>) -> Result<()> {
  // Places a simple market order
  let mut req_builder = PlaceOrderRequestBuilder::default();
  req_builder
    .symbol(symbol.clone())
    .qty(qty)
    .side(OrderSide::Buy);
  
    if let Some(limit) = limit {
      req_builder
        .order_type(apca_datav2::data::OrderType::Limit)
        .limit_price(limit);
    }
  let order_req = req_builder.build()?;
        
  // process message
  let placed = client.place_order(&order_req).await?;
  println!("### Just placed ################################################");
  println!("{} -- {:?}", placed.id, placed.status);

  Ok(())
}
async fn sell(client: &Client, symbol: String, qty: f64, limit: Option<f64>) -> Result<()> {
  // Places a simple market order
  let mut req_builder = PlaceOrderRequestBuilder::default();
  req_builder
    .symbol(symbol.clone())
    .qty(qty)
    .side(OrderSide::Sell);
  
    if let Some(limit) = limit {
      req_builder
        .order_type(apca_datav2::data::OrderType::Limit)
        .limit_price(limit);
    }
  let order_req = req_builder.build()?;

  // process message
  let placed = client.place_order(&order_req).await?;
  println!("### Just placed ################################################");
  println!("{} -- {:?}", placed.id, placed.status);

  Ok(())
}
async fn list(client: &Client, symbols: Option<String>, status: OrderStatus) -> Result<()> {
  println!("### Orders ####################################################");
  let mut builder = ListOrderRequestBuilder::default();

  if let Some(symbols) = symbols {
    builder.symbols(symbols);
  }

  match status {
    OrderStatus::All => builder.status(apca_datav2::orders::SearchOrderStatus::All),
    OrderStatus::Open => builder.status(apca_datav2::orders::SearchOrderStatus::Open),
    OrderStatus::Closed => builder.status(apca_datav2::orders::SearchOrderStatus::Closed),
  };

  let list_req = builder.build()?;

  let list = client.list_orders(&list_req).await?;
  for order in list {
    println!("{} -- {:?} -- {:<8} -- {:>3}/{:>3} ({:>11.3} $) -- {:?}", 
    order.id, order.created_at, order.symbol, 
    order.filled_qty, order.qty.unwrap_or(0.0), 
    order.filled_avg_price.map(|p| order.filled_qty * p).unwrap_or(0.0),
    order.status);
  }
  
  Ok(())
}
async fn cancel(client: &Client, id: Option<String>) -> Result<()> {
  if let Some(id) = id {
    let canceled = client.cancel_by_id(&id).await;
    if canceled.is_ok() {
      println!("CANCELED -- {} ", id);
    } else {
      println!("COULD NOT CANCEL");
    }
  } else {
    let canceled = client.cancel_all_orders().await?;
    for data in canceled {
      println!("CANCELED -- {} -- {:?}", data.id, data.status);
    }
  }
  Ok(())
}