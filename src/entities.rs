//! This module provides the definition of the entity objects used in 
//! Alpaca's API v2.

extern crate serde;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/******************************************************************************
 * DATA POINTS ****************************************************************
 ******************************************************************************/
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
 * ORDERS *********************************************************************
 ******************************************************************************/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderClass {
    /// Class of the orders that are generated when closing a position
    #[serde(rename="")]
    Closure,
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
pub struct OrderData {
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
    #[serde(deserialize_with="crate::utils::option_as_f64")]
    pub qty: Option<f64>,
    /// Filled quantity
    #[serde(deserialize_with="crate::utils::number_as_f64")]
    pub filled_qty: f64,
    /// Filled average price
    #[serde(deserialize_with="crate::utils::option_as_f64")]
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
    #[serde(deserialize_with="crate::utils::option_as_f64")]
    pub limit_price: Option<f64>,
    /// Stop price
    #[serde(deserialize_with="crate::utils::option_as_f64")]
    pub stop_price: Option<f64>,
    /// The current status of the order in its lifecycle
    pub status: OrderStatus,
    /// If true, eligible for execution outside regular trading hours.
    pub extended_hours: bool,
    /// When querying non-simple order_class orders in a nested style, an array 
    /// of Order entities associated with this order. Otherwise, null.
    pub legs: Option<Vec<OrderData>>,
    /// The percent value away from the high water mark for trailing stop orders.
    #[serde(deserialize_with="crate::utils::option_as_f64")]
    pub trail_percent: Option<f64>,
    /// The dollar value away from the high water mark for trailing stop orders.
    #[serde(deserialize_with="crate::utils::option_as_f64")]
    pub trail_price: Option<f64>,
    /// The highest (lowest) market price seen since the trailing stop order was 
    /// submitted.
    #[serde(deserialize_with="crate::utils::option_as_f64")]
    pub hwm: Option<f64>,
}

/// A notification wrt the status of a cancelation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancellationData {
  /// The order whose cancelation has been requested.
  pub id: String,
  /// The cancelation status
  pub status: CancelationStatus 
}
/// Basically an http status code which is interpreted in the context of an 
/// order cancelation request
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
 #[repr(u16)]
pub enum CancelationStatus {
  /// Cancelation succeeded
  Success = 200,
  /// The request has been sucessfully processed but there is no reply info.
  NoContent = 204,
  /// The order was not found
  NotFound = 404,
  /// The order cannot be canceled
  Unprocessable = 422
}

/// The side of a position (is it a long position or a short one ?)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PositionSide {
    /// This is a long position (bought security before selling)
    #[serde(rename="long")]
    Long, 
    /// This is a short position (sold security before buying)
    #[serde(rename="short")]
    Short
}

/// The description of a position
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionData {
    /// Asset ID
    pub asset_id: String,
    /// Symbol name of the asset
    pub symbol: String,
    /// Exchange name of the asset (ErisX for crypto)
    pub exchange: String,
    /// Asset class name
    pub asset_class: String,
    /// Average entry price of the position
    #[serde(deserialize_with="crate::utils::number_as_f64")]
    pub avg_entry_price: f64,
    /// The number of shares
    #[serde(deserialize_with="crate::utils::number_as_f64")]
    pub qty: f64,
    /// Is it a short or a long position ?
    pub side: PositionSide,
    /// Total dollar amount of the position
    #[serde(deserialize_with="crate::utils::number_as_f64")]
    pub market_value: f64,
    /// Total cost basis in dollar
    #[serde(deserialize_with="crate::utils::number_as_f64")]
    pub cost_basis: f64,
    /// Unrealized profit/loss in dollars
    #[serde(deserialize_with="crate::utils::number_as_f64")]
    pub unrealized_pl: f64,
    /// Unrealized profit/loss percent (by a factor of 1)
    #[serde(deserialize_with="crate::utils::number_as_f64")]
    pub unrealized_plpc: f64,
    /// Unrealized profit/loss in dollars for the day
    #[serde(deserialize_with="crate::utils::number_as_f64")]
    pub unrealized_intraday_pl: f64,
    /// Unrealized profit/loss percent (by a factor of 1)
    #[serde(deserialize_with="crate::utils::number_as_f64")]
    pub unrealized_intraday_plpc: f64,
    /// Current asset price per share
    #[serde(deserialize_with="crate::utils::number_as_f64")]
    pub current_price: f64,
    /// Last day’s asset price per share based on the closing value of the 
    /// last trading day
    #[serde(deserialize_with="crate::utils::number_as_f64")]
    pub lastday_price: f64,
    /// Percent change from last day price (by a factor of 1)
    #[serde(deserialize_with="crate::utils::number_as_f64")]
    pub change_today: f64
}

/// A notification wrt the status of a position closure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClosureData {
  /// The symbol whose position is being closed
  pub symbol: String,
  /// The closure status
  pub status: ClosureStatus 
}
/// Basically an http status code which is interpreted in the context of an 
/// position closure
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
 #[repr(u16)]
pub enum ClosureStatus {
  /// Position closure succeeded
  Success = 200,
  /// The request has been sucessfully processed but there is no reply info.
  NoContent = 204,
  /// The action is forbidden
  Forbidden = 403,
  /// The position was not found
  NotFound = 404,
  /// The position cannot be closed
  Unprocessable = 422
}

/*******************************************************************************
 * ASSET API SPECIFIC STUFFS
 ******************************************************************************/
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum AssetStatus {
    #[serde(rename="active")]
    Active,
    #[serde(rename="inactive")]
    Inactive
}
impl AssetStatus {
    pub fn to_str(self) -> &'static str {
        match self {
            AssetStatus::Active   => "active",
            AssetStatus::Inactive => "inactive",
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct AssetData {
    /// Asset ID.
    pub id: String,
    /// “us_equity”
    pub class: String,
    /// AMEX, ARCA, BATS, NYSE, NASDAQ or NYSEARCA
    pub exchange: String,
    /// Symbol of the asset
    pub symbol: String,
    /// active or inactive
    pub status: AssetStatus,
    /// Asset is tradable on Alpaca or not.
    #[serde(rename="tradable", default)]
    pub tradable: bool,
    /// Asset is marginable or not
    #[serde(rename="marginable", default)]
    pub marginable: bool,
    /// Asset is shortable or not.
    #[serde(rename="shortable", default)]
    pub shortable: bool,
    /// Asset is easy-to-borrow or not (filtering for easy_to_borrow = True 
    /// is the best way to check whether the name is currently available to 
    /// short at Alpaca).
    #[serde(rename="easy_to_borrow", default)]
    pub easy_to_borrow: bool,
    /// Asset is fractionable or not.
    #[serde(rename="fractionable", default)]
    pub fractionable: bool,
}

/*******************************************************************************
 * WATCHLIST API SPECIFIC STUFFS
 ******************************************************************************/
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct WatchlistData {
    /// watchlist id
    #[serde(rename="id")]
    pub id: String,
    /// user-defined watchlist name (up to 64 characters)
    #[serde(rename="name")]
    pub name: String,
    /// account ID
    #[serde(rename="account_id")]
    pub account_id: String,
    /// Time when the watchlist was created
    #[serde(rename="created_at")]
    pub created_at: DateTime<Utc>,
    /// Time when the watchlist was last updated
    #[serde(rename="updated_at")]
    pub updated_at: DateTime<Utc>,
    /// the content of this watchlist, in the order as registered by the client
    #[serde(rename="assets", default)]
    pub assets: Vec<AssetData>
}

/******************************************************************************
 * TESTS **********************************************************************
 ******************************************************************************/

#[cfg(test)]
mod tests {
   use crate::entities::{AssetData, OrderData, PositionData};

use super::WatchlistData;

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
      let deserialized = serde_json::from_str::<OrderData>(txt);
      println!("{:?}", deserialized);
      assert!(deserialized.is_ok());
   }

   #[test]
   fn test_deserialize_position() {
      let txt = r#"{
        "asset_id": "904837e3-3b76-47ec-b432-046db621571b",
        "symbol": "AAPL ",
        "exchange": "NASDAQ",
        "asset_class": "us_equity",
        "avg_entry_price": "100.0",
        "qty": "5",
        "side": "long",
        "market_value": "600.0",
        "cost_basis": "500.0",
        "unrealized_pl": "100.0",
        "unrealized_plpc": "0.20",
        "unrealized_intraday_pl": "10.0",
        "unrealized_intraday_plpc": "0.0084",
        "current_price": "120.0",
        "lastday_price": "119.0",
        "change_today": "0.0084"
      }"#;
      let deserialized = serde_json::from_str::<PositionData>(txt);
      println!("{:?}", deserialized);
      assert!(deserialized.is_ok());
   }

   #[test]
   fn test_deserialize_asset() {
      let txt = r#"{
        "id": "904837e3-3b76-47ec-b432-046db621571b",
        "class": "us_equity",
        "exchange": "NASDAQ",
        "symbol": "AAPL",
        "status": "active",
        "tradable": true,
        "marginable": true,
        "shortable": true,
        "easy_to_borrow": true,
        "fractionable": true      
      }"#;
      let deserialized = serde_json::from_str::<AssetData>(txt);
      println!("{:?}", deserialized);
      assert!(deserialized.is_ok());
   }

   #[test]
   fn deserialize_watchlist() {
       let txt = r#"{
                "account_id": "1d5493c9-ea39-4377-aa94-340734c368ae",
                "assets": [
                    {
                        "class": "us_equity",
                        "easy_to_borrow": true,
                        "exchange": "ARCA",
                        "id": "b28f4066-5c6d-479b-a2af-85dc1a8f16fb",
                        "marginable": true,
                        "shortable": true,
                        "status": "active",
                        "symbol": "SPY",
                        "tradable": true
                    },
                    {
                        "class": "us_equity",
                        "easy_to_borrow": false,
                        "exchange": "NASDAQ",
                        "id": "f801f835-bfe6-4a9d-a6b1-ccbb84bfd75f",
                        "marginable": true,
                        "shortable": false,
                        "status": "active",
                        "symbol": "AMZN",
                        "tradable": true
                    }
                ],
                "created_at": "2019-10-30T07:54:42.981322Z",
                "id": "fb306e55-16d3-4118-8c3d-c1615fcd4c03",
                "name": "Monday List",
                "updated_at": "2019-10-30T07:54:42.981322Z"
            }"#;

       let deserialized = serde_json::from_str::<WatchlistData>(txt);
       println!("{:?}", deserialized);
       assert!(deserialized.is_ok());
   }

   #[test]
   fn deserialize_empty_watchlist() {
       let txt = r#"[
            {"id":"0b956f78-3c49-416f-aea9-9cfc944a9df4",
            "account_id":"823c5bc5-46f3-4f24-8d08-e0864d536f4e",
            "created_at":"2021-11-09T17:45:56.000167Z",
            "updated_at":"2021-11-09T17:45:56.000167Z",
            "name":"Primary Watchlist"
            }]"#;
       let rsp = serde_json::from_str::<Vec<WatchlistData>>(txt);
       println!("{:?}", rsp);
       assert!(rsp.is_ok())
   }

}
