use apca_datav2::{data::{AuthDataBuilder, OrderSide}, orders::{ListOrderRequestBuilder, PlaceOrderRequestBuilder}, rest::Client};
use dotenv_codegen::dotenv;
use anyhow::Result;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Args {
  Buy {symbol: String, qty: f64, limit: Option<f64>},
  Sell{symbol: String, qty: f64, limit: Option<f64>},
  List{symbols: String},
  Cancel{id: Option<String>},
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
        Args::List { symbols }            => list(&client, symbols).await?,
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
async fn list(client: &Client, symbols: String) -> Result<()> {
  println!("### Orders ####################################################");
  let list_req = ListOrderRequestBuilder::default()
    .symbols(symbols)
    .status(apca_datav2::orders::SearchOrderStatus::All)
    .build()?;

  let list = client.list_orders(&list_req).await?;
  for order in list {
    println!("{} -- {:?} -- {:<8} -- {:>3} ({:>3}) -- {:?}", order.id, order.created_at, order.symbol, order.qty.unwrap_or(0.0), order.filled_qty, order.status);
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