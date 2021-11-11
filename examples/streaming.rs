use apca_datav2::streaming::{Client, MessageStream, Response};
use dotenv_codegen::dotenv;
use anyhow::Result;
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = Client::paper().await?;
    //
    client.authenticate(
            dotenv!("APCA_KEY_ID").to_string(),
            dotenv!("APCA_SECRET").to_string(),
    ).await?;
    //
    client.listen(vec![MessageStream::TradeUpdates]).await?;
    
    // process message
    client.stream().for_each_concurrent(1000, |r| async move {
        match r {
            Response::Authorization { data } => 
              println!("AUTH {:?}", data),
            Response::Listening { data } => 
              println!("LISTENING {:?}", data),
            Response::TradeUpdates { data } => 
              println!("TRADE: {:?}", data),
        }
    }).await;

    Ok(())
}