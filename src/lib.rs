//! # Market Data
//! ## Overview
//! Alpaca Data API v2 provides market data through an easy to use HTTP API for 
//! historical data and through websocket for real-time data.
//! 
//! ## Subscription Plans
//! Alpaca Data API v2 provides market data in 2 two different plans: Free 
//! and Unlimited. The Free plan is included in both paper-only and live 
//! trading accounts as the default plan for free. The Free plan consists of 
//! data from IEX (Investors Exchange LLC). For the Unlimited plan, we receive 
//! direct feeds from the CTA (administered by NYSE) and UTP (administered by 
//! Nasdaq) SIPs. These 2 feeds combined offer 100% market volume.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use data::{AuthData, BarData, MultiBars, MultiQuotes, MultiTrades, QuoteData, SingleQuote, SingleSnapshot, SingleTrade, SnapshotData, Source, TimeFrame, TradeData};
use errors::Error;
use futures::Stream;

pub mod errors;
pub mod data;
pub mod historical;
pub mod realtime;

/// This structure lets you interact easily with Alpaca's Market Data API v2.
/// This is the structure you will want to instanciate whenever you intend to
/// work with this crate.
///
/// The official documentation for that API is to be found here: 
/// <https://alpaca.markets/docs/api-documentation/api-v2/market-data/alpaca-data-api-v2/>
///
/// # Example Use
/// ```plain
/// # use dotenv_codegen::dotenv;
/// use apca_datav2::*;
/// use apca_datav2::data::Source;
///
///
/// let apca = Apca::new(
///     dotenv!("APCA_KEY_ID").to_string(), 
///     dotenv!("APCA_SECRET").to_string());
/// 
/// // Fetch a stream comprising the old quotes for some given ticker
/// let mut old_quotes = apca.quotes(
///          "AAPL", 
///          Utc.ymd(2021, 08, 02).and_hms(16, 0, 0),
///          Utc.ymd(2021, 08, 02).and_hms(16, 3, 0),
///          None
///      );
/// while let Some(x) = old_quotes.next().await {
///    println!("{:?}", x.ask_price);
/// }
/// 
/// // Fetch quotes in realtime from IEX
/// let realtime = apca.realtime(Source::IEX);
/// realtime.subscribe(SubscriptionDataBuilder::default()
///     .bars(vec!["AAPL".to_string(), "MSFT".to_string()])
///     .build().unwrap()).await.unwrap();
///
/// // React to real time events
/// realtime.stream().for_each_concurrent(1000, |r| async move {
///     match r {
///         Response::Error(e) => println!("ERROR {:?}", e),
///         Response::Trade(t) => println!("Trade {:?}", t),
///         Response::Quote(q) => println!("Quote {:?}", q),
///         Response::Bar(b)   => println!("Bar   {:?}", b),
///         _ => /* ignore */(),
///         //Response::Success{message: s} =>  println!("SUCCESS {:?}", s),
///         //Response::Subscription(s) => println!("SUBSCRIPTIONS {:?}", s)
///     }
/// }).await;
/// ```
pub struct Apca {
    auth: AuthData,
    hist: historical::Client,
}

impl Apca {
    /// Creates a new instance 
    pub fn new(key: String, secret: String) -> Self {
        let auth = AuthData{key, secret};
        let hist = historical::Client::new(auth.clone());
        Self {auth, hist}
    }
    /// This stream returns the desired trades history going through the several 
    /// "pages" of the history asynchoronously; upon request.
    pub fn trades<'a>(&'a self, symbol: &'a str, start: DateTime<Utc>, end: DateTime<Utc>, limit: Option<usize>) -> impl Stream<Item=TradeData> + 'a {
        self.hist.trades(symbol, start, end, limit)
    }
    /// This stream returns the desired quotes history going through the several 
    /// "pages" of the history asynchoronously; upon request.
    pub fn quotes<'a>(&'a self, symbol: &'a str, start: DateTime<Utc>, end: DateTime<Utc>, limit: Option<usize>) -> impl Stream<Item=QuoteData> + 'a {
        self.hist.quotes(symbol, start, end, limit)
    }
    /// This stream returns the desired trades history going through the several 
    /// "pages" of the history asynchoronously; upon request.
    pub fn bars<'a>(&'a self, symbol: &'a str, start: DateTime<Utc>, end: DateTime<Utc>, timeframe: TimeFrame,limit: Option<usize>) -> impl Stream<Item=BarData> + 'a {
        self.hist.bars(symbol, start, end, timeframe, limit)
    }
    /// This endpoint returns latest trade for the requested security.
    pub async fn latest_trade(&self, symbol: &str) -> Result<SingleTrade, Error> {
        self.hist.latest_trade(symbol).await
    }
    /// This endpoint returns latest quote for the requested security.
    pub async fn latest_quote(&self, symbol: &str) -> Result<SingleQuote, Error> {
        self.hist.latest_quote(symbol).await
    }
    /// The Snapshot API for one ticker provides the latest trade, latest quote, 
    /// minute bar daily bar and previous daily bar data for a given ticker symbol.
    pub async fn snapshot(&self, symbol: &str) -> Result<SingleSnapshot, Error> {
        self.hist.snapshot(symbol).await
    }
    /// The Snapshot API for multiple tickers provides the latest trade, 
    /// latest quote, minute bar daily bar and previous daily bar data for 
    /// the given ticker symbols.
    pub async fn snapshots_multi(&self, symbols: &str) -> Result<HashMap<String, SnapshotData>, Error> {
        self.hist.snapshots_multi(symbols).await
    }
    /// The Snapshot API for multiple tickers provides the latest trade, 
    /// latest quote, minute bar daily bar and previous daily bar data for 
    /// the given ticker symbols.
    pub async fn snapshots_multi_vec(&self, symbols: &[&str]) -> Result<HashMap<String, SnapshotData>, Error> {
        self.hist.snapshots_multi_vec(symbols).await
    }
    /// This endpoint returns trade historical data for the requested security
    pub async fn trades_paged(&self, symbol: &str, start: DateTime<Utc>, end: DateTime<Utc>, limit: Option<usize>, page_token: Option<String>) -> Result<MultiTrades, Error> {
        self.hist.trades_paged(symbol, start, end, limit, page_token).await
    }
    /// This endpoint returns quote (NBBO) historical data for the requested security.
    pub async fn quotes_paged(&self, symbol: &str, start: DateTime<Utc>, end: DateTime<Utc>, limit: Option<usize>, page_token: Option<String>) -> Result<MultiQuotes, Error> {
        self.hist.quotes_paged(symbol, start, end, limit, page_token).await
    }
    /// This endpoint returns aggregate historical data for the requested security.
    pub async fn bars_paged(&self, symbol: &str, start: DateTime<Utc>, end: DateTime<Utc>, timeframe: TimeFrame ,limit: Option<usize>, page_token: Option<String>) -> Result<MultiBars, Error> {
        self.hist.bars_paged(symbol, start, end, timeframe, limit, page_token).await
    }
    /// Provides an easy access to real time data API offered by Alpaca.
    pub async fn realtime(&self, source: Source) -> Result<realtime::Client, Error> {
        let mut client = realtime::Client::new(source).await?;
        client.authenticate(self.auth.clone()).await?;
        Ok(client)
    }
}