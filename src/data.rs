//! This module provides the definition of the protocol objects used in 
//! Alpaca's data API v2.

extern crate serde;
use std::fmt::Display;

use derive_builder::Builder;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use serde_json::Value;

use crate::errors::RealtimeErrorCode;

/******************************************************************************
 * CLIENT TO SERVER ***********************************************************
 ******************************************************************************/
 
/// The data source for the real time data
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Source {
    /// Investor's Exchange (IEX) is the default datasource, and the one 
    /// included in the free subscription plan
    IEX,
    /// If you intend to use SIP as data source (unlimited plan only)
    SIP
}
impl Default for Source {
    fn default() -> Self { Self::IEX }
}
impl std::fmt::Display for Source {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::IEX => write!(fmt, "iex"),
            Self::SIP => write!(fmt, "sip"),
        }
    }
}

/// In order to interact with the server over the websocket, you'll need to 
/// tell it what you want to do. Basically, the very first thing you'll want to
/// do after connecting is to authenticate (failure to to so within a few 
/// seconds will result in the receipt of an error control message).
///
/// Once authenticated you will have the opportunity to subscribe and 
/// unsubscribe from messages you want to receive from Alpaca.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "action")]
pub enum Action {
    #[serde(rename = "auth")] 
    Authenticate(AuthData), 
    #[serde(rename = "subscribe")] 
    Subscribe(SubscriptionData),
    #[serde(rename = "unsubscribe")] 
    Unsubscribe(SubscriptionData),
}

/// After connecting you will have to authenticate as follows:
/// ```{"action":"auth","key":"PK************","secret":"************"}```
#[derive(Debug, Clone, Serialize, Builder)]
pub struct AuthData {
    pub key:    String,
    pub secret: String,
}

/// You can subscribe to trades, quotes and bars of a particular symbol 
/// (or * for every symbol in the case of bars). A subscribe message should 
/// contain what subscription you want to add to your current subscriptions in 
/// your session so you don’t have to send what you’re already subscribed to.
///
/// You can also omit either one of them (trades,quotes or bars) if you don’t 
/// want to subscribe to any symbols in that category but be sure to include at 
/// least one of the three.
///
/// Subscription data is also used when you mean to send an `unsubscribe` 
/// message that subtracts the list of subscriptions specified from your current
/// set of subscriptions.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct SubscriptionData {
    #[builder(setter(strip_option), default)]
    pub trades: Option<Vec<String>>,
    #[builder(setter(strip_option), default)]
    pub quotes: Option<Vec<String>>,
    #[builder(setter(strip_option), default)]
    pub bars  : Option<Vec<String>>,
}


/******************************************************************************
 * SERVER TO CLIENT ***********************************************************
 ******************************************************************************/
/// Every message you receive from the server will be in the format:
///
/// ```json
/// [{"T": "{message_type}", {contents}},...]
/// ```
/// Control messages (i.e. where "T" is error, success or subscription) always 
/// arrive in arrays of size one to make their processing easier.
/// 
/// Data points however may arrive in arrays that have a length that is greater 
/// than one. This is to facilitate clients whose connection is not fast enough 
/// to handle data points sent one by one. Our server buffers the outgoing 
/// messages but slow clients may get disconnected if their buffer becomes full.
///
/// # Communication flow
/// The communication can be thought of as two separate phases: 
/// establishment and receiving data.
/// 
/// ## Establishment
/// To establish the connection first you will need to connect to our server 
/// using the URL above. Upon successfully connecting, you will receive the 
/// welcome message: 
/// ```json
/// [{"T":"success","msg":"connected"}]
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "T")]
pub enum Response {
    /// Obviously, this variant is used to denote control message informing 
    /// you that some error has happened. You may receive an error during your 
    /// session. You can differentiate between them using the list below:
    /// 
    /// * The message you sent to the server did not follow the specification
    ///   ```[{"T":"error","code":400,"msg":"invalid syntax"}]```
    /// 
    /// * You have attempted to subscribe or unsubscribe before authentication
    ///   ```[{"T":"error","code":401,"msg":"not authenticated"}]```
    ///
    /// * You have provided invalid authentication credentials.
    ///   ```[{"T":"error","code":402,"msg":"auth failed"}]```
    ///
    /// * You have already successfully authenticated during your current session.
    ///   ```[{"T":"error","code":404,"msg":"auth timeout"}]```
    ///
    /// * You failed to successfully authenticate after connecting. 
    ///   You have a few seconds to authenticate after connecting.
    ///   ```[{"T":"error","code":404,"msg":"auth timeout"}]```
    /// 
    /// * The symbol subscription request you sent would put you over the limit 
    ///   set by your subscription package. If this happens your symbol 
    ///   subscriptions are the same as they were before you sent the request 
    ///   that failed.
    ///   ```[{"T":"error","code":405,"msg":"symbol limit exceeded"}]```
    /// 
    /// * You already have an ongoing authenticated session.
    ///   ```[{"T":"error","code":406,"msg":"connection limit exceeded"}]```
    ///
    /// * You may receive this if you are too slow to process the messages sent 
    ///   by the server. Please note that this is not guaranteed to arrive 
    ///   before you are disconnected to avoid keeping slow connections active 
    ///   forever
    ///   ```[{"T":"error","code":407,"msg":"slow client"}]```
    ///
    /// * Your account does not have access to Data v2.
    ///   ```[{"T":"error","code":408,"msg":"v2 not enabled"}]```
    ///
    /// * You have attempted to access a data source not available in your 
    ///   subscription package.
    ///   ```[{"T":"error","code":409,"msg":"insufficient subscription"}]```
    ///
    /// * An unexpected error occurred on our end and we are investigating the issue.
    ///   ```[{"T":"error","code":500,"msg":"internal error"}```
    #[serde(rename="error")]
    Error(RealtimeErrorCode),
    /// This variant denotes a **control message** meant to inform you of the
    /// successful completion of the action you requested. For instance, 
    /// upon successfully connecting, you will receive the  welcome message: 
    /// ```json
    /// [{"T":"success","msg":"connected"}]
    /// ```
    ///
    /// Similarly, after connecting with proper credentials you will receive 
    /// another success message: 
    /// ```json
    /// [{"T":"success","msg":"authenticated"}]
    /// ```
    #[serde(rename="success")]
    Success{#[serde(rename="msg")] message: String},
    /// After subscribing or unsubscribing you will receive a message that 
    /// describes your current list of subscriptions.
    /// ```json
    /// [{"T":"subscription","trades":["AAPL"],"quotes":["AMD","CLDR"],"bars":["IBM","AAPL","VOO"]}]
    /// ```
    ///
    /// **Note**: 
    /// You will always receive your entire list of subscriptions, as  
    /// illustrated by the sample communication excerpt below: 
    /// ```json
    /// > {"action": "subscribe", "trades": ["AAPL"], "quotes": ["AMD", "CLDR"], "bars": ["*"]}
    /// < [{"T":"subscription","trades":["AAPL"],"quotes":["AMD","CLDR"],"bars":["*"]}]
    /// > {"action": "unsubscribe", "bars": ["*"]}
    /// > [{"T":"subscription","trades":["AAPL"],"quotes":["AMD","CLDR"],"bars":[]}]
    /// ```
    #[serde(rename="subscription")]
    Subscription(SubscriptionData),

    // --- DATA POINTS --------------------------------------------------------
    #[serde(rename="t")]
    Trade(DataPoint<TradeData>),
    #[serde(rename="q")]
    Quote(DataPoint<QuoteData>),
    #[serde(rename="b")]
    Bar(DataPoint<BarData>),
}

/******************************************************************************
 * DATA POINTS ****************************************************************
 ******************************************************************************/

/// A generic datapoint that holds information related to a given symbol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPoint<T> {
    /// The symbol
    #[serde(rename="S")]
    pub symbol: String,
    /// The actual payload
    #[serde(flatten)]
    pub data  : T,
}

 /// Datapoint encapsulating informations about a given trade
 #[derive(Debug, Clone, Serialize, Deserialize)]
 pub struct TradeData {
     /// Trade identifier
     #[serde(rename="i")]
     pub trade_id: i64,
     /// exchange code where the trade occurred
     #[serde(rename="x")]
     pub exchange_code: Exchange,
     /// trade price
     #[serde(rename="p")]
     pub trade_price: f64,
     /// trade size
     #[serde(rename="s")]
     pub trade_size: u64,
     /// RFC-3339 formatted timestamp with nanosecond precision.
     #[serde(rename="t")]
     pub timestamp: DateTime<Utc>,
     /// Condition.
     ///
     /// # Note 
     /// Each feed/exchange uses its own set of codes to identify trade and quote 
     /// conditions, so the same condition may have a different code depending on 
     /// the originator of the data. For more details, please refer to alpaca's 
     /// documentation page:
     /// <https://alpaca.markets/docs/api-documentation/api-v2/market-data/alpaca-data-api-v2/#conditions>
     /// and 
     /// <https://alpaca.markets/docs/api-documentation/api-v2/market-data/alpaca-data-api-v2/#quote-conditions>
     #[serde(rename="c")]
     pub conditions: Vec<String>,
     /// Tape
     #[serde(rename="z")]
     pub tape: String,
 }

 /// Datapoint encapsulating a quote
 #[derive(Debug, Clone, Serialize, Deserialize)]
 pub struct QuoteData {
     /// ask exchange code
     #[serde(rename="ax")]
     pub ask_exchange: Exchange,
     /// ask price
     #[serde(rename="ap")]
     pub ask_price: f64,
     /// ask size
     #[serde(rename="as")]
     pub ask_size: usize,
     /// bid exchange code
     #[serde(rename="bx")]
     pub bid_exchange: Exchange,
     /// bid price
     #[serde(rename="bp")]
     pub bid_price: f64,
     /// ask size
     #[serde(rename="bs")]
     pub bid_size: usize,
     /// RFC-3339 formatted timestamp with nanosecond precision.
     #[serde(rename="t")]
     pub timestamp: DateTime<Utc>,
     /// Condition.
     ///
     /// # Note 
     /// Each feed/exchange uses its own set of codes to identify trade and quote 
     /// conditions, so the same condition may have a different code depending on 
     /// the originator of the data. For more details, please refer to alpaca's 
     /// documentation page:
     /// <https://alpaca.markets/docs/api-documentation/api-v2/market-data/alpaca-data-api-v2/#conditions>
     /// and 
     /// <https://alpaca.markets/docs/api-documentation/api-v2/market-data/alpaca-data-api-v2/#quote-conditions>
     #[serde(rename="c")]
     pub conditions: Vec<String>,
     /// Tape
     #[serde(rename="z")]
     pub tape: String,
 }

/// Datapoint encapsulating a 'bar' (a.k.a. OHLC)
 #[derive(Debug, Clone, Serialize, Deserialize)]
 pub struct BarData {
    // open price
    #[serde(rename="o")]
    pub open_price: f64,
    // high price
    #[serde(rename="h")]
    pub high_price: f64,
    // low price
    #[serde(rename="l")]
    pub low_price: f64,
    // close price
    #[serde(rename="c")]
    pub close_price: f64,
    // volume
    #[serde(rename="v")]
    pub volume: u64,
    /// RFC-3339 formatted timestamp with nanosecond precision.
    #[serde(rename="t")]
    pub timestamp: DateTime<Utc>,
}

/// List of stock exchanges which are supported by Alpaca.
/// The tape id of each exchange is returned in all market data requests. 
/// You can use this table to map the code to an exchange.
 #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
 pub enum Exchange {
    /// A     NYSE American (AMEX)     
    #[serde(rename="A")]
    Amex, 
    /// B     NASDAQ OMX BX     
    #[serde(rename="B")]
    NasdaqOmxBx,
    /// C     National Stock Exchange     
    #[serde(rename="C")]
    NationalStockExchange,
    /// D     FINRA ADF     
    #[serde(rename="D")]
    FinraAdf,
    /// E     Market Independent     
    #[serde(rename="E")]
    MarketIndependent,
    /// H     MIAX     
    #[serde(rename="H")]
    Miax,
    /// I     International Securities Exchange     
    #[serde(rename="I")]
    InternationalSecuritiesExchange,
    /// J     Cboe EDGA     
    #[serde(rename="J")]
    CboeEdga,
    /// K     Cboe EDGX     
    #[serde(rename="K")]
    CboeEdgx,
    /// L     Long Term Stock Exchange     
    #[serde(rename="L")]
    LongTermStockExchange,
    /// M     Chicago Stock Exchange     
    #[serde(rename="M")]
    ChicagoStockExchange,
    /// N     New York Stock Exchange     
    #[serde(rename="N")]
    NewYorkStockExchange,
    /// P     NYSE Arca     
    #[serde(rename="P")]
    NyseArca,
    /// Q     NASDAQ OMX     
    #[serde(rename="Q")]
    NasdaqOmx,
    /// S     NASDAQ Small Cap     
    #[serde(rename="S")]
    NasdaqSmallCap,
    /// T     NASDAQ Int     
    #[serde(rename="T")]
    NasdaqInt,
    /// U     Members Exchange     
    #[serde(rename="U")]
    MembersExchange,
    /// V     IEX     
    #[serde(rename="V")]
    Iex,
    /// W     CBOE     
    #[serde(rename="W")]
    Cboe,
    /// X     NASDAQ OMX PSX     
    #[serde(rename="X")]
    NasdaqOmxPsx,
    /// Y     Cboe BYX     
    #[serde(rename="Y")]
    CboeByx,
    /// Z     Cboe BZX
    #[serde(rename="Z")]
    CboeBzx,
 }

/******************************************************************************
 * HISTORY DATA POINTS ********************************************************
 ******************************************************************************/

 /// Timeframe for the aggregation. Available values are: 1Min, 1Hour, 1Day.
 #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
 pub enum TimeFrame {
    #[serde(rename="1Min")]
    Minute, 
    #[serde(rename="1Hour")]
    Hour,
    #[serde(rename="1Day")]
    Day
 }
 impl Display for TimeFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Minute => write!(f, "1Min"),
            Self::Hour   => write!(f, "1Hour"),
            Self::Day    => write!(f, "1Day"),
        }
    }
}

/// A datapoint that holds one single quote
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SingleQuote {
    /// The symbol
    pub symbol: String,
    /// The actual payload
    pub quote  : QuoteData,
}
/// A datapoint that holds one single quote
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiQuotes {
    /// The actual payload
    #[serde(deserialize_with="null_as_emptyvec")]
    pub quotes : Vec<QuoteData>,
    /// The symbol
    pub symbol: String,
    #[serde(rename="next_page_token")]
    pub token : Option<String>,
}
/// A datapoint that holds one single trade
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SingleTrade {
    /// The symbol
    pub symbol: String,
    /// The actual payload
    pub trade  : TradeData,
}
/// A datapoint that holds one single trade
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiTrades {
    /// The actual payload
    #[serde(deserialize_with="null_as_emptyvec")]
    pub trades : Vec<TradeData>,
    /// The symbol
    pub symbol: String,
    #[serde(rename="next_page_token")]
    pub token : Option<String>,
}
/// A datapoint that holds one single bar
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SingleBar {
    /// The actual payload
    pub bar  : BarData,
    /// The symbol
    pub symbol: String,
}
/// A datapoint that holds one single trade
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiBars {
    /// The actual payload
    #[serde(deserialize_with="null_as_emptyvec")]
    pub bars  : Vec<BarData>,
    /// The symbol
    pub symbol: String,
    #[serde(rename="next_page_token")]
    pub token : Option<String>,
}

fn null_as_emptyvec<'de, T, D>(d: D) -> Result<Vec<T>, D::Error>
where D: serde::Deserializer<'de>,
      T: serde::Deserialize<'de>
{
    Deserialize::deserialize(d)
        .map(|x: Option<_>| {
            x.unwrap_or_default()
        })
}

fn number_as_f64<'de, D>(d: D) -> Result<f64, D::Error>
where D: serde::Deserializer<'de>,
{
    match Value::deserialize(d)? {
        Value::String(txt) => 
            if let Ok(val) = txt.parse::<f64>() {
                Ok(val)
            } else {
                Err(serde::de::Error::custom("expected a number"))
            },
        Value::Number(num) => 
            Ok(num.as_f64().ok_or_else(|| serde::de::Error::custom("Invalid number"))?),
        _ => 
            Err(serde::de::Error::custom("expected a number"))
    }
}

fn option_as_f64<'de, D>(d: D) -> Result<Option<f64>, D::Error>
where D: serde::Deserializer<'de>,
{
    match Value::deserialize(d)? {
        Value::String(txt) => 
            if let Ok(val) = txt.parse::<f64>() {
                Ok(Some(val))
            } else {
                Err(serde::de::Error::custom("expected a number"))
            },
        Value::Number(num) => 
            Ok(Some(num.as_f64().ok_or_else(|| serde::de::Error::custom("Invalid number"))?)),
        Value::Null => 
            Ok(None),
        _ => 
            Err(serde::de::Error::custom("expected a number"))
    }
}



/******************************************************************************
 * SNAPSHOTS ******************************************************************
 ******************************************************************************/

/// The Snapshot API for one ticker provides the latest trade, latest quote, 
/// minute bar daily bar and previous daily bar data for a given ticker symbol.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotData {
    /// Latest trade object.
    #[serde(rename = "latestTrade")]
    pub latest_trade: TradeData,
    /// Latest quote object
    #[serde(rename = "latestQuote")]
    pub latest_quote: QuoteData,
    /// Minute bar object.
    #[serde(rename = "minuteBar")]
    pub minute_bar: BarData,
    /// Daily bar object.
    #[serde(rename = "dailyBar")]
    pub daily_bar: BarData,
    /// Previous daily close bar object
    #[serde(rename = "prevDailyBar")]
    pub prev_daily_bar: BarData,
}

/// The Snapshot API for one ticker provides the latest trade, latest quote, 
/// minute bar daily bar and previous daily bar data for a given ticker symbol.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SingleSnapshot {
    /// The symbol
    pub symbol: String, 
    /// The actual payload
    #[serde(flatten)]
    pub data: SnapshotData,
}

/******************************************************************************
 * ORDERS *********************************************************************
 ******************************************************************************/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderClass {
    #[serde(rename="simple")]
    Simple,
    #[serde(rename="bracket")]
    Bracket,
    #[serde(rename="oto")]
    OneTriggersOther,
    #[serde(rename="oco")]
    OneCancelsOther,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum OrderType {
    #[serde(rename="market")]
    Market,
    #[serde(rename="limit")]
    Limit,
    #[serde(rename="stop")]
    Stop, 
    #[serde(rename="stop_limit")]
    StopLimit,
    #[serde(rename="trailing_stop")]
    TrailingStop,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum OrderSide {
    #[serde(rename="buy")]
    Buy,
    #[serde(rename="sell")]
    Sell
}


#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum Direction {
    #[serde(rename="asc")]
    Ascending,
    #[serde(rename="desc")]
    Descending
}

/// # Time in Force
/// 
/// Note: 
/// For Crypto Trading, Alpaca supports the following Time-In-Force designations: 
/// day, gtc, ioc and fok. OPG and CLS are not supported.
/// 
/// Alpaca supports the following Time-In-Force designations:
/// 
/// * day: A day order is eligible for execution only on the day it is live. 
///     By default, the order is only valid during Regular Trading Hours 
///     (9:30am - 4:00pm ET). If unfilled after the closing auction, it is 
///     automatically canceled. If submitted after the close, it is queued and 
///     submitted the following trading day. However, if marked as eligible for 
///     extended hours, the order can also execute during supported extended 
///     hours.
/// 
/// * gtc: The order is good until canceled. Non-marketable GTC limit orders are 
///     subject to price adjustments to offset corporate actions affecting the 
///     issue. We do not currently support Do Not Reduce(DNR) orders to opt out 
///     of such price adjustments.
/// 
/// * opg: Use this TIF with a market/limit order type to submit “market on open” 
///     (MOO) and “limit on open” (LOO) orders. This order is eligible to execute 
///     only in the market opening auction. Any unfilled orders after the open 
///     will be cancelled. OPG orders submitted after 9:28am but before 7:00pm ET 
///     will be rejected. OPG orders submitted after 7:00pm will be queued and 
///     routed to the following day’s opening auction. On open/on close orders 
///     are routed to the primary exchange. Such orders do not necessarily 
///     execute exactly at 9:30am / 4:00pm ET but execute per the exchange’s 
///     auction rules.
/// 
/// * cls: Use this TIF with a market/limit order type to submit 
///     “market on close” (MOC) and “limit on close” (LOC) orders. This order is 
///     eligible to execute only in the market closing auction. Any unfilled 
///     orders after the close will be cancelled. CLS orders submitted after 
///     3:50pm but before 7:00pm ET will be rejected. CLS orders submitted after 
///     7:00pm will be queued and routed to the following day’s closing auction. 
///     Only available with API v2.
/// 
/// * ioc: An Immediate Or Cancel (IOC) order requires all or part of the order 
///     to be executed immediately. Any unfilled portion of the order is 
///     canceled. Only available with API v2. Most market makers who receive IOC 
///     orders will attempt to fill the order on a principal basis only, and 
///     cancel any unfilled balance. On occasion, this can result in the entire 
///     order being cancelled if the market maker does not have any existing 
///     inventory of the security in question.
/// 
/// * fok: A Fill or Kill (FOK) order is only executed if the entire order 
///     quantity can be filled, otherwise the order is canceled. 
///     Only available with API v2.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum TimeInForce {
    /// A day order is eligible for execution only on the day it is live. 
    /// By default, the order is only valid during Regular Trading Hours 
    /// (9:30am - 4:00pm ET). If unfilled after the closing auction, it is 
    /// automatically canceled. If submitted after the close, it is queued and 
    /// submitted the following trading day. However, if marked as eligible for 
    /// extended hours, the order can also execute during supported extended 
    /// hours.
    #[serde(rename="day")]
    Day,
    /// The order is good until canceled. Non-marketable GTC limit orders are 
    /// subject to price adjustments to offset corporate actions affecting the 
    /// issue. We do not currently support Do Not Reduce(DNR) orders to opt out 
    /// of such price adjustments.
    #[serde(rename="day")]
    GoodUntilCanceled,
    /// Use this TIF with a market/limit order type to submit “market on open” 
    /// (MOO) and “limit on open” (LOO) orders. This order is eligible to execute 
    /// only in the market opening auction. Any unfilled orders after the open 
    /// will be cancelled. OPG orders submitted after 9:28am but before 7:00pm ET 
    /// will be rejected. OPG orders submitted after 7:00pm will be queued and 
    /// routed to the following day’s opening auction. On open/on close orders 
    /// are routed to the primary exchange. Such orders do not necessarily 
    /// execute exactly at 9:30am / 4:00pm ET but execute per the exchange’s 
    /// auction rules.
    #[serde(rename="day")]
    OpeningAuction,
    /// Use this TIF with a market/limit order type to submit 
    /// “market on close” (MOC) and “limit on close” (LOC) orders. This order is 
    /// eligible to execute only in the market closing auction. Any unfilled 
    /// orders after the close will be cancelled. CLS orders submitted after 
    /// 3:50pm but before 7:00pm ET will be rejected. CLS orders submitted after 
    /// 7:00pm will be queued and routed to the following day’s closing auction. 
    /// Only available with API v2.
    #[serde(rename="day")]
    ClosingAuction,
    /// An Immediate Or Cancel (IOC) order requires all or part of the order 
    /// to be executed immediately. Any unfilled portion of the order is 
    /// canceled. Only available with API v2. Most market makers who receive IOC 
    /// orders will attempt to fill the order on a principal basis only, and 
    /// cancel any unfilled balance. On occasion, this can result in the entire 
    /// order being cancelled if the market maker does not have any existing 
    /// inventory of the security in question.
    #[serde(rename="day")]
    ImmediateOrCancel,
    /// A Fill or Kill (FOK) order is only executed if the entire order 
    /// quantity can be filled, otherwise the order is canceled. 
    /// Only available with API v2.
    #[serde(rename="day")]
    FillOrKill,
}

/// # Order Lifecycle
/// 
/// An order executed through Alpaca can experience several status changes 
/// during its lifecycle. The most common statuses are described in detail below:
/// 
/// * new: The order has been received by Alpaca, and routed to exchanges for 
///     execution. This is the usual initial state of an order.
/// 
/// * partially_filled: The order has been partially filled.
/// 
/// * filled: The order has been filled, and no further updates will occur 
///     for the order.
/// 
/// * done_for_day: The order is done executing for the day, and will not 
///     receive further updates until the next trading day.
/// 
/// * canceled: The order has been canceled, and no further updates will occur 
///     for the order. This can be either due to a cancel request by the user, 
///     or the order has been canceled by the exchanges due to its time-in-force.
/// 
/// * expired: The order has expired, and no further updates will occur for 
///     the order.
/// 
/// * replaced: The order was replaced by another order, or was updated due to 
///     a market event such as corporate action.
/// 
/// * pending_cancel: The order is waiting to be canceled.
/// 
/// * pending_replace: The order is waiting to be replaced by another order. 
///     The order will reject cancel request while in this state.
/// 
/// * accepted: The order has been received by Alpaca, but hasn’t yet been 
///     routed to the execution venue. This could be seen often out side of 
///     trading session hours.
/// 
/// * pending_new: The order has been received by Alpaca, and routed to the 
///     exchanges, but has not yet been accepted for execution. This state only 
///     occurs on rare occasions.
/// 
/// * accepted_for_bidding: The order has been received by exchanges, and is 
///     evaluated for pricing. This state only occurs on rare occasions.
/// 
/// * stopped: The order has been stopped, and a trade is guaranteed for the 
///     order, usually at a stated price or better, but has not yet occurred. 
///     This state only occurs on rare occasions.
/// 
/// * rejected: The order has been rejected, and no further updates will occur 
///     for the order. This state occurs on rare occasions and may occur based 
///     on various conditions decided by the exchanges.
/// 
/// * suspended: The order has been suspended, and is not eligible for trading. 
///     This state only occurs on rare occasions.
/// 
/// * calculated: The order has been completed for the day 
///     (either filled or done for day), but remaining settlement calculations 
///     are still pending. This state only occurs on rare occasions.
/// 
/// An order may be canceled through the API up until the point it reaches a state of either filled, canceled, or expired.
/// 
#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum OrderStatus {
    /// The order has been received by Alpaca, and routed to exchanges for 
    /// execution. This is the usual initial state of an order.
    #[serde(rename="new")]
    New,
    /// The order has been partially filled.
    #[serde(rename="partially_filled")]
    PartiallyFilled,
    /// The order has been filled, and no further updates will occur for the order.
    #[serde(rename="filled")]
    Filled,
    /// The order is done executing for the day, and will not 
    /// receive further updates until the next trading day.
    #[serde(rename="done_for_day")]
    DoneForDay,
    /// The order has been canceled, and no further updates will occur 
    /// for the order. This can be either due to a cancel request by the user, 
    /// or the order has been canceled by the exchanges due to its time-in-force.
    #[serde(rename="canceled")]
    Canceled,
    /// The order has expired, and no further updates will occur for the order.
    #[serde(rename="expired")]
    Expired,
    /// The order was replaced by another order, or was updated due to 
    /// a market event such as corporate action.
    #[serde(rename="replaced")]
    Replaced,
    /// The order is waiting to be canceled.
    #[serde(rename="pending_cancel")]
    PendingCancel,
    /// The order is waiting to be replaced by another order. 
    /// The order will reject cancel request while in this state.
    #[serde(rename="pending_replace")]
    PendingReplace,
    /// The order has been received by Alpaca, but hasn’t yet been 
    /// routed to the execution venue. This could be seen often out side of 
    /// trading session hours.
    #[serde(rename="accepted")]
    Accepted,
    /// The order has been received by Alpaca, and routed to the 
    /// exchanges, but has not yet been accepted for execution. This state only 
    /// occurs on rare occasions.
    #[serde(rename="pending_new")]
    PendingNew,
    /// The order has been received by exchanges, and is evaluated for pricing. 
    /// This state only occurs on rare occasions.
    #[serde(rename="accepted_for_bidding")]
    AcceptedForBidding,
    /// The order has been stopped, and a trade is guaranteed for the 
    /// order, usually at a stated price or better, but has not yet occurred. 
    /// This state only occurs on rare occasions.
    #[serde(rename="stopped")]
    Stopped,
    /// The order has been rejected, and no further updates will occur 
    /// for the order. This state occurs on rare occasions and may occur based 
    /// on various conditions decided by the exchanges.
    #[serde(rename="rejected")]
    Rejected,
    /// The order has been suspended, and is not eligible for trading. 
    /// This state only occurs on rare occasions.
    #[serde(rename="suspended")]
    Suspended,
    /// The order has been completed for the day 
    /// (either filled or done for day), but remaining settlement calculations 
    /// are still pending. This state only occurs on rare occasions.
    #[serde(rename="calculated")]
    Calculated
}

/// The Snapshot API for one ticker provides the latest trade, latest quote, 
/// minute bar daily bar and previous daily bar data for a given ticker symbol.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    /// Order ID
    pub id: String,
    /// Client unique order ID
    pub client_order_id: String,
    /// Timestamp ot the order creation
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub submitted_at: Option<DateTime<Utc>>,
    pub filled_at: Option<DateTime<Utc>>,
    pub expired_at: Option<DateTime<Utc>>,
    pub canceled_at: Option<DateTime<Utc>>,
    pub failed_at: Option<DateTime<Utc>>,
    pub replaced_at: Option<DateTime<Utc>>,
    /// The order ID that this order was replaced by
    pub replaced_by: Option<String>,
    /// The order ID that this order replaces
    pub replaces: Option<String>,
    /// Asset uuid
    pub asset_id: String,
    /// Asset symbol
    pub symbol: String,
    /// Asset class
    pub asset_class: String,
    /// Ordered notional amount. If entered, qty will be null. 
    /// Can take up to 9 decimal points.
    pub notional: Option<f64>,
    /// Ordered quantity. If entered, notional will be null. 
    /// Can take up to 9 decimal points.
    #[serde(deserialize_with="option_as_f64")]
    pub qty: Option<f64>,
    /// Filled quantity
    #[serde(deserialize_with="number_as_f64")]
    pub filled_qty: f64,
    /// Filled average price
    #[serde(deserialize_with="option_as_f64")]
    pub filled_avg_price: Option<f64>,
    /// simple, bracket, oco or oto. For details of non-simple order 
    /// classes, please see ![bracket](https://alpaca.markets/docs/trading-on-alpaca/orders#bracket-orders "Bracket Order Overview")
    pub order_class: OrderClass,
    /// Valid values: market, limit, stop, stop_limit, trailing_stop
    #[serde(rename="type")]
    pub order_type: OrderType,
    /// Valid values: buy, sell
    pub side: OrderSide,
    /// See ![TimeInForce](https://alpaca.markets/docs/trading-on-alpaca/orders/#time-in-force "Time in Force")
    pub time_in_force: TimeInForce,
    /// Limit price
    #[serde(deserialize_with="option_as_f64")]
    pub limit_price: Option<f64>,
    /// Stop price
    #[serde(deserialize_with="option_as_f64")]
    pub stop_price: Option<f64>,
    /// The current status of the order in its lifecycle
    pub status: OrderStatus,
    /// If true, eligible for execution outside regular trading hours.
    pub extended_hours: bool,
    /// When querying non-simple order_class orders in a nested style, an array 
    /// of Order entities associated with this order. Otherwise, null.
    pub legs: Option<Vec<Order>>,
    /// The percent value away from the high water mark for trailing stop orders.
    #[serde(deserialize_with="option_as_f64")]
    pub trail_percent: Option<f64>,
    /// The dollar value away from the high water mark for trailing stop orders.
    #[serde(deserialize_with="option_as_f64")]
    pub trail_price: Option<f64>,
    /// The highest (lowest) market price seen since the trailing stop order was 
    /// submitted.
    #[serde(deserialize_with="option_as_f64")]
    pub hwm: Option<f64>,
}

/******************************************************************************
 * TESTS **********************************************************************
 ******************************************************************************/

#[cfg(test)]
mod tests {
   use crate::data::{Order, Response};

    #[test]
   fn test_deserialize_trade() {
       let txt = r#"{
           "T": "t",
           "i": 96921,
           "S": "AAPL",
           "x": "D",
           "p": 126.55,
           "s": 1,
           "t": "2021-02-22T15:51:44.208Z",
           "c": [
             "@",
             "I"
           ],
           "z": "C"
         }"#;
       let deserialized = serde_json::from_str::<Response>(txt);
       assert!(deserialized.is_ok());
   }
   #[test]
   fn test_deserialize_quote() {
       let txt = r#"{
           "T": "q",
           "S": "AMD",
           "bx": "U",
           "bp": 87.66,
           "bs": 1,
           "ax": "Q",
           "ap": 87.68,
           "as": 4,
           "t": "2021-02-22T15:51:45.335689322Z",
           "c": [
             "R"
           ],
           "z": "C"
         }"#;
         let deserialized = serde_json::from_str::<Response>(txt);
         assert!(deserialized.is_ok());
   }
   #[test]
   fn test_deserialize_bar() {
       let txt = r#"{
           "T": "b",
           "S": "SPY",
           "o": 388.985,
           "h": 389.13,
           "l": 388.975,
           "c": 389.12,
           "v": 49378,
           "t": "2021-02-22T19:15:00Z"
         }"#;
         let deserialized = serde_json::from_str::<Response>(txt);
         assert!(deserialized.is_ok());
   }

   #[test]
   fn test_deserialize_order() {
       let txt = r#"{
            "id":"81859481-60e1-48d2-ba43-8279af711b9e",
            "client_order_id":"a50ffe4e-e631-446e-ad57-ba7fa5f1718c",
            "created_at":"2021-11-08T20:51:49.909525Z",
            "updated_at":"2021-11-08T20:51:49.909525Z",
            "submitted_at":"2021-11-08T20:51:49.903435Z",
            "filled_at":null,
            "expired_at":null,
            "canceled_at":null,
            "failed_at":null,
            "replaced_at":null,
            "replaced_by":null,
            "replaces":null,
            "asset_id":"d9b3d190-0046-4aba-b668-a9c8f9f6787d",
            "symbol":"BTI",
            "asset_class":"us_equity",
            "notional":null,
            "qty":"30",
            "filled_qty":"0",
            "filled_avg_price":null,
            "order_class":"simple",
            "order_type":"market",
            "type":"market",
            "side":"buy",
            "time_in_force":"day",
            "limit_price":null,
            "stop_price":null,
            "status":"accepted",
            "extended_hours":false,
            "legs":null,
            "trail_percent":null,
            "trail_price":null,
            "hwm":null
        }"#;
      let deserialized = serde_json::from_str::<Order>(txt);
      println!("{:?}", deserialized);
      assert!(deserialized.is_ok());
   }
}