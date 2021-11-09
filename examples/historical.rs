use apca_datav2::rest::Client;
use dotenv_codegen::dotenv;
use anyhow::Result;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Args {
  symbol: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args   = Args::from_args();
    let client = Client::paper(
      dotenv!("APCA_KEY_ID").to_string(),
      dotenv!("APCA_SECRET").to_string()
    );
    let snap   = client.snapshot(&args.symbol).await?;
    println!("{:#?}", snap);

    Ok(())
}