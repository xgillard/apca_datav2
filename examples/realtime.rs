use apca_datav2::realtime::Client;
use apca_datav2::realtime::{AuthDataBuilder, Response, Source, SubscriptionDataBuilder};
use dotenv_codegen::dotenv;
use anyhow::Result;
use futures::StreamExt;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Args {
    symbols: Vec<String>,
}


#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::from_args();
    let mut client = Client::new(Source::default()).await?;
    //
    client.authenticate(AuthDataBuilder::default()
            .key(dotenv!("APCA_KEY_ID").to_string())
            .secret(dotenv!("APCA_SECRET").to_string())
            .build()?
    ).await?;
    //
    client.subscribe(
        SubscriptionDataBuilder::default()
            .quotes(args.symbols)
            .build()?
    ).await?;
    
    // process message
    client.stream().for_each_concurrent(1000, |r| async move {
        match r {
            Response::Error(e) => println!("ERROR {:?}", e),
            Response::Trade(t) => println!("Trade {:?}", t),
            Response::Bar(b)   => println!("Bar   {:?}", b),
            Response::Quote(q) => 
                println!("{} -- bid: {:>5.3} ({:>5}) -- ask: {:>5.3} ({:>5})", 
                    q.symbol, q.data.bid_price, q.data.bid_size, 
                        q.data.ask_price, q.data.ask_size),
            _ => /* ignore */(),
            //Response::Success{message: s} =>  println!("SUCCESS {:?}", s),
            //Response::Subscription(s) => println!("SUBSCRIPTIONS {:?}", s)
        }
    }).await;

    Ok(())
}