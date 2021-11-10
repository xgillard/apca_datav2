use anyhow::Result;
use apca_datav2::rest::Client;
use dotenv_codegen::dotenv;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Args {
  Show {symbol: Option<String>},
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::paper(
      dotenv!("APCA_KEY_ID").to_string(),
      dotenv!("APCA_SECRET").to_string()
    );
    match Args::from_args() {
        Args::Show  { symbol } => show(&client, symbol).await?,
    };
    Ok(())
}

async fn show(client: &Client, symbol: Option<String>) -> Result<()> {
  if let Some(symbol) = symbol {
    let ass = client.get_asset(&symbol).await?;
    println!("{:>8} ({:>7}) -- {:>9} -- fractionnable {:>5} -- shortable {:>5} -- easy to borrow {:>5}",
      ass.symbol, ass.exchange, ass.class,  ass.fractionable, ass.shortable, ass.easy_to_borrow)
  } else {
    let assets = client.list_assets(None, None).await?;
    for ass in assets {
      println!("{:>8} ({:>7}) -- {:>9} -- fractionnable {:>5} -- shortable {:>5} -- easy to borrow {:>5}",
        ass.symbol, ass.exchange, ass.class,  ass.fractionable, ass.shortable, ass.easy_to_borrow)
    }
  }
  Ok(())
}
