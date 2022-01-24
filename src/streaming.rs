//! # Streaming
//! 
//! Alpaca’s API offers WebSocket streaming for account and order updates 
//! which follows the RFC6455 WebSocket protocol.
//! 
//! **Important Note**
//! Note: The trade_updates stream coming from `wss://paper-api.alpaca.markets/stream`
//! uses Binary frames which differs from the Text frames that comes from the 
//! `wss://data.alpaca.markets/stream`stream. This is why the data streaming
//! api and this one differ as well.
//! 
//! To connect to the WebSocket follow the standard opening handshake as 
//! defined by the RFC specification to `wss://paper-api.alpaca.markets/stream`
//! or `wss://api.alpaca.markets/stream`. Alpaca’s streaming service supports 
//! both JSON and MessagePack codecs.
//! 
//! Once the connection is authorized, the client can listen to one or more 
//! streams to get updates on particular changes. These are the streams the 
//! client can choose to listen to.
//! 
//! So far, the client can chose to listen to:
//!   * trade_updates (the only one at the time being)
//! 
//! ## Conversation Protocol
//! Even though the format of the messages differ, the communication protocol
//! of this service is somewhat similar to that of the realtime data api.
//!  
//! In order to listen to streams, the client first needs to authenticate and
//! can then sends a listen message to the server in order to select the 
//! streams it wants to listen to.
//! 
//! ### Authentication
//! 
//! The WebSocket client can be authenticated using the same API key when 
//! making HTTP requests. Upon connecting to the WebSocket client must send 
//! an authentication message over the WebSocket connection with the API key, 
//! and secret key as its payload:
//! 
//! ```json
//! {
//!     "action": "authenticate",
//!     "data": {
//!         "key_id": "{YOUR_API_KEY_ID}",
//!         "secret_key": "{YOUR_API_SECRET_KEY}"
//!     }
//! }
//! ```
//! 
//! The server will then authorize the connection and respond with either a 
//! successful response:
//! 
//! ```json
//! {
//!     "stream": "authorization",
//!     "data": {
//!         "status": "authorized",
//!         "action": "authenticate"
//!     }
//! }
//! ```
//! 
//! or an unathorized response:
//! 
//! ```json
//! {
//!     "stream": "authorization",
//!     "data": {
//!         "status": "unauthorized",
//!         "action": "authenticate"
//!     }
//! }
//! ```
//! 
//! In the case the socket connection is not authorized yet, a new message 
//! under the authorization stream is issued in response to the listen request.
//! 
//! ```json
//! {
//!     "stream": "authorization",
//!     "data": {
//!         "status": "unauthorized",
//!         "action": "listen"
//!     }
//! }
//! ```
//! 
//! ### Selecting streams to listen to
//! 
//! In order to listen to streams, the client sends a listen message to the 
//! server as follows.
//!  
//! ```json
//! {
//!     "action": "listen",
//!     "data": {
//!         "streams": ["trade_updates"]
//!     }
//! }
//! ```
//! 
//! The server acknowledges by replying a message in the listening stream.
//! ```json
//! {
//!     "stream": "listening",
//!     "data": {
//!         "streams": ["trade_updates"]
//!     }
//! }
//! ```
//! 
//! If some of the requested streams are not available, they will not appear 
//! in the streams list in the acknowledgement. Note that the streams field 
//! in the listen message is to tell the set of streams to listen, so if you 
//! want to stop receiving updates from the stream, you must send an empty 
//! list of streams values as a listen message. Similarly, if you want to add 
//! more streams to get updates in addition to the ones you are already doing 
//! so, you must send all the stream names not only the new ones.
//! 
//! In order to maintain the state of their brokerage accounts at Alpaca, 
//! along with requesting from the REST API, clients can also listen to the 
//! trade streams for their accounts. This will ensure any running algorithms 
//! will always have the most up-to-date picture of any accounts they are 
//! trading with at Alpaca.
//! 
//! **Note:** to request with MessagePack, add the header: 
//! `Content-Type: application/msgpack` (this is not done so far)
//! 
//! ### Order Updates
//! 
//! Updates with regards to orders placed at Alpaca are dispatched over the 
//! WebSocket connection under the event trade_updates, and include any data 
//! pertaining to orders that are executed with Alpaca. This includes order 
//! fills, partial fills, as well as cancellations and rejections of orders. 
//! Clients may listen to this stream by sending a listen message:
//! 
//! ```json
//! {
//!     "action": "listen",
//!     "data": {
//!         "streams": ["trade_updates"]
//!     }
//! }
//! ```
//! 
//! Any listen messages received by the server will be acknowledged via a 
//! message on the listening stream. The message’s data payload will include 
//! the list of streams the client is currently listening to:
//! 
//! ```json
//! {
//!     "stream": "listening",
//!     "data": {
//!         "streams": ["trade_updates"]
//!     }
//! }
//! ```
//! 
//! The fields present in a message sent over the trade_updates stream depend 
//! on the type of event they are communicating. All messages contain an 
//! event type and an order field, which is the same as the order object that 
//! is returned from the REST API. Potential event types and additional 
//! fields that will be in their messages are listed below.
//! 
//! #### Common events:
//! 
//! These are the events that are the expected results of actions you may 
//! have taken by sending API requests.
//! 
//!     * new: Sent when an order has been routed to exchanges for execution.
//!     * fill: Sent when your order has been completely filled.
//!         * timestamp: The time at which the order was filled.
//!         * price: The average price per share at which the order was filled.
//!         * position_qty: The size of your total position, after this fill 
//!             event, in shares. Positive for long positions, negative for 
//!             short positions.
//!     * partial_fill: Sent when a number of shares less than the total remaining
//!         quantity on your order has been filled.
//!         * timestamp: The time at which the shares were filled.
//!         * price: The average price per share at which the shares were filled.
//!         * position_qty: The size of your total position, after this fill event, in shares. Positive for long positions, negative for short positions.
//!     * canceled: Sent when your requested cancelation of an order is processed.
//!         * timestamp: The time at which the order was canceled.
//!     * expired: Sent when an order has reached the end of its lifespan, as determined by the order’s time in force value.
//!         * timestamp: The time at which the order expired.
//!     * done_for_day: Sent when the order is done executing for the day, and will not receive further updates until the next trading day.
//!     * replaced: Sent when your requested replacement of an order is processed.
//!         * timestamp: The time at which the order was replaced.
//! 
//! #### Rarer events:
//! 
//! These are events that may rarely be sent due to unexpected circumstances 
//! on the exchanges. It is unlikely you will need to design your code around 
//! them, but you may still wish to account for the possibility that they 
//! will occur.
//! 
//!     * rejected: Sent when your order has been rejected.
//!         * timestamp: The time at which the rejection occurred.
//!     * pending_new: Sent when the order has been received by Alpaca and 
//!           routed to the exchanges, but has not yet been accepted for 
//!           execution.
//!     * stopped: Sent when your order has been stopped, and a trade is 
//!           guaranteed for the order, usually at a stated price or better, 
//!           but has not yet occurred.
//!     * pending_cancel: Sent when the order is awaiting cancelation. Most 
//!           cancelations will occur without the order entering this state.
//!     * pending_replace: Sent when the order is awaiting replacement.
//!     * calculated: Sent when the order has been completed for the day 
//!           - it is either “filled” or “done_for_day” - but remaining 
//!           settlement calculations are still pending.
//!     * suspended: Sent when the order has been suspended and is not 
//!           eligible for trading.
//!     * order_replace_rejected: Sent when the order replace has been rejected.
//!     * order_cancel_rejected: Sent when the order cancel has been rejected.
//! 
//! Example
//! 
//! An example message sent over the trade_updates stream would look like:
//!  
//! ```json
//! {
//!     "stream": "trade_updates",
//!     "data": {
//!         "event": "fill",
//!         "price": "179.08",
//!         "timestamp": "2018-02-28T20:38:22Z",
//!         "position_qty": "100",
//!         "order": {
//!             "id": "7b7653c4-7468-494a-aeb3-d5f255789473",
//!             "client_order_id": "7b7653c4-7468-494a-aeb3-d5f255789473",
//!             "asset_id": "904837e3-3b76-47ec-b432-046db621571b",
//!             "symbol": "AAPL",
//!             "exchange": "NASDAQ",
//!             "asset_class": "us_equity",
//!             "side": "buy",
//!             ...
//!         }
//!     }
//! }
//! ```

use chrono::{DateTime, Utc};
use futures::{SinkExt, StreamExt, stream::{SplitSink, SplitStream}};
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async, tungstenite::Message};

use crate::{entities::OrderData, errors::Error};

/// Header used to send the key-id authentication
pub const APCA_API_KEY_ID: &str = "APCA-API-KEY-ID";
/// Header used to send the secret-key for authentication
pub const APCA_API_SECRET_KEY: &str = "APCA-API-SECRET-KEY";

/// Base URL to interact with live trading api
pub const LIVE_TRADING_URL: &str = "wss://api.alpaca.markets/stream";
/// Base URL to interact with paper trading api
pub const PAPER_TRADING_URL: &str = "wss://paper-api.alpaca.markets/stream";


type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

/// This is the object you'll want to create in order to interact with Alpaca's
/// market data api. The object allows both server to client and client to 
/// server communication (these responsibilities can be split for independant/
/// asynchronous processing).
pub struct Client {
    /// The portion of the client devoted to the client to server communication
    write : ClientSender,
    /// The portion of the client devoted to the server to client communication
    read  : ClientReceiver,
}
impl Client {
  pub async fn paper() -> Result<Self, Error> {
    Self::new(false).await
  }
  pub async fn live() -> Result<Self, Error> {
    Self::new(true).await
  }
  /// Creates a client that fetches data from the given source.
  pub async fn new(live: bool) -> Result<Self, Error> {
      // --- Connect to websocket
      let url = if live { LIVE_TRADING_URL } else { PAPER_TRADING_URL };
      let (socket, _rsp) = connect_async(url).await?;
      let (write, read)  = socket.split();
      let write          = ClientSender::new(write);
      let read           = ClientReceiver::new(read);
      //
      Ok(Self {write, read})
  }

  /// Authenticates the client
  pub async fn authenticate<'a>(&mut self, key: String, secret: String) -> Result<(), Error> {
      self.write.authenticate(key, secret).await
  }
  /// Subscribe for realtime data about certain trades, quotes or bars
  pub async fn listen(&mut self, sub: Vec<MessageStream>) -> Result<(), Error> {
      self.write.listen(sub).await
  }
  /// Returns the stream which is used to receive the responses from the server
  pub fn stream(self) -> impl StreamExt<Item=Response> {
      self.read.stream()
  }
}
// The portion of the client devoted to the client to server communication
pub struct ClientSender {
  write : SplitSink<WsStream, Message>,
}
impl ClientSender {
  /// Creates a new instance from a given write sink
  pub fn new(write: SplitSink<WsStream, Message>) -> Self {
      Self {write}
  }
  /// Authenticates the client
  pub async fn authenticate<'a>(&mut self, key: String, secret: String) -> Result<(), Error> {
      let data = AuthData { key, secret };
      self.action(Request::Authenticate{data}).await
  }
  /// Subscribe for realtime data about certain trades, quotes or bars
  pub async fn listen(&mut self, sub: Vec<MessageStream>) -> Result<(), Error> {
      let data = StreamList {streams: sub};
      self.action(Request::Listen{data}).await
  }
  /// Performs the specified action on the server
  pub async fn action(&mut self, action: Request) -> Result<(), Error> {
      let json = serde_json::to_string(&action)?;
      //self.write.send(Message::Text(json)).await?;
      self.write.send(Message::Binary(json.as_bytes().to_vec())).await?;
      Ok(())
  }
}
/// The portion of the client devoted to the server to client communication.
/// This object is essentially used as a means to obtain an opaquely-types 
/// stream of Responses.
pub struct ClientReceiver {
  read: SplitStream<WsStream>
}
impl ClientReceiver {
  /// Create a new instance from a given message stream
  pub fn new(read: SplitStream<WsStream>) -> Self {
      Self {read}
  }
  /// Returns the stream which is used to receive the responses from the server
  pub fn stream(self) -> impl StreamExt<Item=Response> {
      self.read
      .filter_map(|m| async move {
          if let Ok(Message::Binary(bytes)) = m {
              let text = String::from_utf8_lossy(&bytes);
              let data = serde_json::from_str::<Response>(&text)
                   .unwrap_or_else(|_| panic!("unexpected message '{}'", text));
              Some(data)
          } else {
              None
          }
      })
  }
}

/// In order to interact with the server over the websocket, you'll need to 
/// tell it what you want to do. Basically, the very first thing you'll want to
/// do after connecting is to authenticate (failure to to so within a few 
/// seconds will result in the receipt of an error control message).
///
/// Once authenticated you will have the opportunity to listen and 
/// unsubscribe from messages you want to receive from Alpaca.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag="action")]
pub enum Request {
    #[serde(rename = "authenticate")] 
    Authenticate {
      data: AuthData
    },
    #[serde(rename = "listen")] 
    Listen {
      data: StreamList
    }
}

/// Used to send our credentials to the server and let it identify who we are
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthData {
  #[serde(rename="key_id")]
  key: String,
  #[serde(rename="secret_key")]
  secret: String,
}
/// The messages streams a client may decide to listen to
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum MessageStream {
  #[serde(rename = "trade_updates")]
  TradeUpdates
}


/// The fields present in a message sent over the trade_updates stream depend 
/// on the type of event they are communicating. All messages contain an 
/// event type and an order field, which is the same as the order object that 
/// is returned from the REST API. Potential event types and additional 
/// fields that will be in their messages are listed below.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event")]
pub enum OrderUpdate {
  /// Sent when an order has been routed to exchanges for execution.
  #[serde(rename="new")]
  New{
    /// The order which is impacted by this event
    order: OrderData
  },
  /// Sent when your order has been completely filled. 
  #[serde(rename="fill")]
  Fill{
    /// The order which is impacted by this event
    order: OrderData, 
    /// The time at which the order was filled.
    timestamp: DateTime<Utc>, 
    /// The average price per share at which the order was filled
    #[serde(deserialize_with="crate::utils::number_as_f64")]
    price: f64, 
    /// The size of your total position, after this fill event, in shares.
    /// Positive for long positions, negative for short positions. 
    #[serde(deserialize_with="crate::utils::number_as_f64")]
    position_qty: f64
  },
  /// Sent when a number of shares less than the total remaining quantity on 
  /// your order has been filled. 
  #[serde(rename="partial_fill")]
  PartialFill {
    /// The order which is impacted by this event
    order: OrderData, 
    /// The time at which the shares were filled.
    timestamp: DateTime<Utc>, 
    /// The average price per share at which the shares were filled.
    #[serde(deserialize_with="crate::utils::number_as_f64")]
    price: f64, 
    /// The size of your total position, after this fill event, in shares. 
    /// Positive for long positions, negative for short positions.
    #[serde(deserialize_with="crate::utils::number_as_f64")]
    position_qty: f64
  },
  /// Sent when your requested cancelation of an order is processed. 
  #[serde(rename="canceled")]
  Canceled {
    /// The order which is impacted by this event
    order: OrderData, 
    /// The time at which the order was canceled
    timestamp: DateTime<Utc>, 
  },
  /// Sent when an order has reached the end of its lifespan, as determined by i
  /// the order’s time in force value
  #[serde(rename="expired")]
  Expired {
    /// The order which is impacted by this event
    order: OrderData, 
    /// The time at which the order expired
    timestamp: DateTime<Utc>, 
  },
  /// Sent when the order is done executing for the day, and will not receive 
  /// further updates until the next trading day.
  #[serde(rename="done_for_day")]
  DoneForDay {
    /// The order which is impacted by this event
    order: OrderData, 
  },
  /// Sent when your requested replacement of an order is processed. 
  #[serde(rename="replaced")]
  Replaced {
    // The order which is impacted by this event
    order: OrderData, 
    /// The time at which the order was replaced
    timestamp: DateTime<Utc>, 
  },
  // ---- RARER EVENTS --------------------------------------------------------
  /// Sent when your order has been rejected.
  #[serde(rename="rejected")]
  Rejected {
    // The order which is impacted by this event
    order: OrderData, 
    /// The time at which the order was rejected
    timestamp: DateTime<Utc>, 
  },
  /// Sent when the order has been received by Alpaca and routed to the 
  /// exchanges, but has not yet been accepted for execution.
  #[serde(rename="pending_new")]
  PendingNew {
    // The order which is impacted by this event
    order: OrderData, 
  },
  /// Sent when your order has been stopped, and a trade is guaranteed for 
  /// the order, usually at a stated price or better, but has not yet occurred
  #[serde(rename="stopped")]
  Stopped {
    // The order which is impacted by this event
    order: OrderData, 
  },
  /// Sent when the order is awaiting cancelation. Most cancelations will occur 
  /// without the order entering this state.
  #[serde(rename="pending_cancel")]
  PendingCancel {
    // The order which is impacted by this event
    order: OrderData, 
  },
  /// Sent when the order is awaiting replacement
  #[serde(rename="pending_replace")]
  PendingReplace {
    // The order which is impacted by this event
    order: OrderData, 
  },
  /// Sent when the order has been completed for the day 
  /// - it is either “filled” or “done_for_day” - but remaining settlement 
  /// calculations are still pending
  #[serde(rename="calculated")]
  Calculated {
    // The order which is impacted by this event
    order: OrderData, 
  },
  /// Sent when the order has been suspended and is not eligible for trading.
  #[serde(rename="suspended")]
  Suspended {
    // The order which is impacted by this event
    order: OrderData, 
  },
  /// Sent when the order replace has been rejected
  #[serde(rename="order_replace_rejected")]
  OrderReplaceRejected {
    // The order which is impacted by this event
    order: OrderData, 
  },
  /// Sent when the order cancel has been rejected
  #[serde(rename="order_cancel_rejected")]
  OrderCancelRejected {
    // The order which is impacted by this event
    order: OrderData, 
  }
}

/// Tells the information stream which is impacted by the received message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag="stream")]
#[allow(clippy::large_enum_variant)]
pub enum Response {
    #[serde(rename = "authorization")] 
    Authorization {
      data: AuthorizationData
    },
    #[serde(rename = "listening")] 
    Listening {
      data: StreamList
    },  
    #[serde(rename = "trade_updates")]
    TradeUpdates {
      data: OrderUpdate
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamList {
  pub streams: Vec<MessageStream>
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum AuthorizationStatus {
  #[serde(rename="authorized")]
  Authorized, 
  #[serde(rename="unauthorized")]
  Unauthorized,
}


#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "authenticate")] 
    Authenticate,
    #[serde(rename = "listen")] 
    Listen,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct AuthorizationData {
  pub status: AuthorizationStatus,
  pub action: Action
}

#[cfg(test)]
mod tests {
    use crate::streaming::Response;

  #[test]
  fn deserialize_auth_response() {
    let text = r#"{"stream":"authorization","data":{"action":"authenticate","status":"authorized"}}"#;
    let deserialized = serde_json::from_str::<Response>(text);
    println!("{:?}", deserialized);
    assert!(deserialized.is_ok());
  }
  
  #[test]
  fn deserialize_order_response() {
    let text = r#"{
        "stream": "trade_updates",
        "data": {
            "event": "fill",
            "execution_id": "b0c17642-209c-4a21-9650-915a755dc4ce",
            "order": {
                "asset_class": "us_equity",
                "asset_id": "b6d1aa75-5c9c-4353-a305-9e2caa1925ab",
                "canceled_at": null,
                "client_order_id": "ad1a656c-c524-421b-a1ff-c84bb1b4ae38",
                "created_at": "2021-11-11T17:11:17.353294Z",
                "expired_at": null,
                "extended_hours": false,
                "failed_at": null,
                "filled_at": "2021-11-11T17:11:17.557793Z",
                "filled_avg_price": "333.16",
                "filled_qty": "1",
                "hwm": null,
                "id": "810f77c9-fd3f-4a10-a78c-046c611f26db",
                "legs": null,
                "limit_price": null,
                "notional": null,
                "order_class": "simple",
                "order_type": "market",
                "qty": "1",
                "replaced_at": null,
                "replaced_by": null,
                "replaces": null,
                "side": "buy",
                "status": "filled",
                "stop_price": null,
                "submitted_at": "2021-11-11T17:11:17.347956Z",
                "symbol": "MSFT",
                "time_in_force": "day",
                "trail_percent": null,
                "trail_price": null,
                "type": "market",
                "updated_at": "2021-11-11T17:11:17.594109Z"
            },
            "position_qty": "1",
            "price": "333.16",
            "qty": "1",
            "timestamp": "2021-11-11T17:11:17.557793708Z"
        }
    }"#;
    let deserialized = serde_json::from_str::<Response>(text);
    println!("{:?}", deserialized);
    assert!(deserialized.is_ok());
  }
}
