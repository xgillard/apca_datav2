# apca_datav2
This crate provides bindings for Alpaca's Data API v2. It provides both the
realtime and historical data information. The point of this crate is by no means
to provide a full coverage of all the available api. But only the market data
api (v2). I tried to stay faithful to the code and documentation provided by 
Alpaca. The original doc is to be found here:
https://alpaca.markets/docs/api-documentation/api-v2/market-data/alpaca-data-api-v2/

## Example Usage
```rust
let apca = Apca::new(
    dotenv!("APCA_KEY_ID").to_string(), 
    dotenv!("APCA_SECRET").to_string());

// Fetch a stream comprising the old quotes for some given ticker
let mut old_quotes = apca.quotes(
         "AAPL", 
         Utc.ymd(2021, 08, 02).and_hms(16, 0, 0),
         Utc.ymd(2021, 08, 02).and_hms(16, 3, 0),
         None
     );
while let Some(x) = old_quotes.next().await {
   println!("{:?}", x.ask_price);
}

// Fetch quotes in realtime from IEX
let realtime = apca.realtime(Source::IEX);
realtime.subscribe(SubscriptionDataBuilder::default()
    .bars(vec!["AAPL".to_string(), "MSFT".to_string()])
    .build().unwrap()).await.unwrap();
// React to real time events
realtime.stream().for_each_concurrent(1000, |r| async move {
    match r {
        Response::Error(e) => println!("ERROR {:?}", e),
        Response::Trade(t) => println!("Trade {:?}", t),
        Response::Quote(q) => println!("Quote {:?}", q),
        Response::Bar(b)   => println!("Bar   {:?}", b),
        _ => /* ignore */(),
        //Response::Success{message: s} =>  println!("SUCCESS {:?}", s),
        //Response::Subscription(s) => println!("SUBSCRIPTIONS {:?}", s)
    }
}).await;
```

## Why is it v2 ?
The v2 by no means intends to say that these are the 2nd implementations of 
bindings for Alpaca's market data. Rather do they mean the bindings are meant
to exploit the 2nd version of Alpaca's API. (v1 is decommissioned on august 26, 
2021)

## Note
As far as I am concerned, this project is mostly thought of as a big fat pull
request to be applied on top of https://github.com/d-e-s-o/apca. But the truth
is, I feel a bit lazy about making a smooth integration for it in `apca` right
now (might do it later though).