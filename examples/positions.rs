use anyhow::Result;
use apca_datav2::rest::Client;
use dotenv_codegen::dotenv;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Args {
  Show {symbol: Option<String>},
  Close{
    symbol: Option<String>, 
    #[structopt(short, long)]
    qty: Option<f64>, 
    #[structopt(short, long)]
    percentage: Option<f64>}
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::paper(
      dotenv!("APCA_KEY_ID").to_string(),
      dotenv!("APCA_SECRET").to_string()
    );
    match Args::from_args() {
        Args::Show  { symbol } => show(&client, symbol).await?,
        Args::Close { symbol, qty, percentage } => close(&client, symbol, qty, percentage).await?
    };
    Ok(())
}

async fn show(client: &Client, symbol: Option<String>) -> Result<()> {
  if let Some(symbol) = symbol {
    let pos = client.get_open_position(&symbol).await?;
    println!("{:<8} ({:>9.3}) -- entry {:>7.3} -- cost {:>9.3} -- pl ${:>8.3} ({:>7.3} %)", 
      pos.symbol, pos.qty, pos.avg_entry_price, pos.cost_basis, 
      pos.unrealized_pl, pos.unrealized_plpc * 100.0);
  } else {
    let positions = client.list_open_positions().await?;
    for pos in positions {
      println!("{:<8} ({:>9.3}) -- entry {:>7.3} -- cost {:>9.3} -- pl ${:>8.3} ({:>7.3} %)", 
        pos.symbol, pos.qty, pos.avg_entry_price, pos.cost_basis, 
        pos.unrealized_pl, pos.unrealized_plpc * 100.0);
    }
  }
  Ok(())
}

async fn close(client: &Client, symbol: Option<String>, qty: Option<f64>, percentage: Option<f64>) -> Result<()> {
  if let Some(symbol) = symbol {
    let order = client.close_position(&symbol, qty, percentage).await?;
    println!("{} -- {:?} -- {:<8} -- {:>3}/{:>3} ({:>11.3} $) -- {:?}", 
    order.id, order.created_at, order.symbol, 
    order.filled_qty, order.qty.unwrap_or(0.0), 
    order.filled_avg_price.map(|p| order.filled_qty * p).unwrap_or(0.0),
    order.status);
  } else {
    let closed = client.close_all_positions(true).await?;
    for data in closed {
      println!("CLOSED -- {:^38} -- {:?}", data.symbol, data.status);
    }
  }
  Ok(())
}