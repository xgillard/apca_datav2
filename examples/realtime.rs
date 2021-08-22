use anyhow::Result;
use apca_datav2::data::{AuthDataBuilder, Response, Source, SubscriptionDataBuilder};
use apca_datav2::realtime::Client;
use dotenv_codegen::dotenv;
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
  let mut client = Client::new(Source::default()).await?;
  //
  client
    .authenticate(
      AuthDataBuilder::default()
        .key(dotenv!("APCA_KEY_ID").to_string())
        .secret(dotenv!("APCA_SECRET").to_string())
        .build()?,
    )
    .await?;
  //
  client
    .subscribe(
      SubscriptionDataBuilder::default()
        .quotes(vec!["AAPL".to_string()])
        .build()?,
    )
    .await?;

  // process message
  client
    .stream()
    .for_each_concurrent(1000, |r| async move {
      match r {
        Response::Error(e) => println!("ERROR {:?}", e),
        Response::Trade(t) => println!("Trade {:?}", t),
        Response::Quote(q) => println!("Quote {:?}", q),
        Response::Bar(b) => println!("Bar   {:?}", b),
        _ =>
        /* ignore */
        {
          ()
        }, //Response::Success{message: s} =>  println!("SUCCESS {:?}", s),
           //Response::Subscription(s) => println!("SUBSCRIPTIONS {:?}", s)
      }
    })
    .await;

  Ok(())
}
