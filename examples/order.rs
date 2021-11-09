use apca_datav2::{data::{AuthDataBuilder, OrderSide}, orders::{Client, ListOrderRequestBuilder, PlaceOrderRequestBuilder}};
use chrono::{DateTime, Duration, Utc};
use dotenv_codegen::dotenv;
use anyhow::Result;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Args {
  symbol: String,
  #[structopt(default_value="0.0")]
  qty   : f64,
  #[structopt(short, long)]
  sell  : bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args   = Args::from_args();
    let auth   = AuthDataBuilder::default()
      .key(dotenv!("APCA_KEY_ID").to_string())
      .secret(dotenv!("APCA_SECRET").to_string())
      .build()?;

    let client = Client::paper(auth);
    /* 
    // Places a simple market order
    let order_req= PlaceOrderRequestBuilder::default()
      .symbol(args.symbol.clone())
      .qty(args.qty)
      .side(if args.sell { OrderSide::Sell } else { OrderSide::Buy })
      .build()?;
          
    // process message
    let placed = client.place_order(&order_req).await?;
    println!("### Just placed ################################################");
    println!("{} -- {:?}", placed.id, placed.status);
    */
    println!("### History ####################################################");
    let list_req = ListOrderRequestBuilder::default()
      .symbols(args.symbol)
      .status(apca_datav2::orders::SearchOrderStatus::Closed)
      .build()?;
    
    println!("{}", serde_json::to_string(&list_req)?);

    let list = client.list_orders(&list_req).await?;
    for order in list {
      println!("{} -- {:?} -- {:<8} -- {:>3} ({:>3}) -- {:?}", order.id, order.created_at, order.symbol, order.qty.unwrap_or(0.0), order.filled_qty, order.status);
    }

    Ok(())
}