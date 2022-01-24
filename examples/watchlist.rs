use anyhow::Result;
use apca_datav2::rest::Client;
use dotenv_codegen::dotenv;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::paper(
      dotenv!("APCA_KEY_ID").to_string(),
      dotenv!("APCA_SECRET").to_string()
    );

    //let list = client.create_watchlist("xaviergillard:bux", &vec!["AAPL"]).await?;
    //println!("created {} ({})", list.id, list.name);

    let watchlists = client.list_watchlists().await?;

    for watchlist in watchlists {
        println!("### {} -- {}", watchlist.name, watchlist.id);

        client.add_asset_to_watchlist(&watchlist.id, "MSFT").await?;
        client.remove_asset_from_watchlist(&watchlist.id, "AAPL").await?;

        let wl = client.get_watchlist(&watchlist.id).await?;
        for asset in wl.assets {
            println!("{}", asset.symbol);
        }

        //client.delete_watchlist(&watchlist.id).await?;
    }
    Ok(())
}
