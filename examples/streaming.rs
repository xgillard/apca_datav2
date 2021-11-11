use apca_datav2::entities::OrderData;
use apca_datav2::streaming::OrderUpdate;
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
            Response::TradeUpdates { data } => 
              match data {
                OrderUpdate::New { order }                  => summarize(&order),
                OrderUpdate::Fill { order, .. }             => summarize(&order),
                OrderUpdate::PartialFill { order, .. }      => summarize(&order),
                OrderUpdate::Canceled { order, .. }         => summarize(&order),
                OrderUpdate::Expired { order, .. }          => summarize(&order),
                OrderUpdate::DoneForDay { order }           => summarize(&order),
                OrderUpdate::Replaced { order, .. }         => summarize(&order),
                OrderUpdate::Rejected { order, .. }         => summarize(&order),
                OrderUpdate::PendingNew { order }           => summarize(&order),
                OrderUpdate::Stopped { order }              => summarize(&order),
                OrderUpdate::PendingCancel { order }        => summarize(&order),
                OrderUpdate::PendingReplace { order }       => summarize(&order),
                OrderUpdate::Calculated { order }           => summarize(&order),
                OrderUpdate::Suspended { order }            => summarize(&order),
                OrderUpdate::OrderReplaceRejected { order } => summarize(&order),
                OrderUpdate::OrderCancelRejected { order }  => summarize(&order),
            },
            _ => /* ignore */ (),
        }
    }).await;

    Ok(())
}

fn summarize(order: &OrderData) {
  println!("{} -- {:?} -- {:<8} -- {:>3}/{:>3} ({:>11.3} $) -- {:?}", 
    order.id, order.created_at, order.symbol, 
    order.filled_qty, order.qty.unwrap_or(0.0), 
    order.filled_avg_price.map(|p| order.filled_qty * p).unwrap_or(0.0),
    order.status);
}