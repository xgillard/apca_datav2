use apca_datav2::{data::AuthDataBuilder, rest::Client};
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
    let auth   = AuthDataBuilder::default()
      .key(dotenv!("APCA_KEY_ID").to_string())
      .secret(dotenv!("APCA_SECRET").to_string())
      .build()?;

    let client = Client::paper(auth);
    let snap   = client.snapshot(&args.symbol).await?;
    println!("{:#?}", snap);

    Ok(())
}