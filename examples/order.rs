use apca_datav2::{data::{AuthDataBuilder, OrderSide}, orders::{Client, PlaceOrderRequestBuilder}};
use dotenv_codegen::dotenv;
use anyhow::Result;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Args {
  symbol: String,
  qty   : f64,
  #[structopt(short, long)]
  sell  : bool,
  #[structopt(short, long)]
  extended_hours: bool
}

#[tokio::main]
async fn main() -> Result<()> {
    let args   = Args::from_args();
    let auth   = AuthDataBuilder::default()
      .key(dotenv!("APCA_KEY_ID").to_string())
      .secret(dotenv!("APCA_SECRET").to_string())
      .build()?;

    let client = Client::paper(auth);
    // Places a simple market order
    let order_req= PlaceOrderRequestBuilder::default()
      .symbol(args.symbol)
      .qty(args.qty)
      .side(if args.sell { OrderSide::Sell } else { OrderSide::Buy })
      .extended_hours(args.extended_hours)
      .build()?;
          
    // process message
    let placed = client.place_order(&order_req).await?;
    //println!("{:#?}", placed);
    println!("{} -- {:?}", placed.id, placed.status);

    Ok(())
}