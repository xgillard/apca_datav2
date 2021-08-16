//! This module provides a namespace to declare the errors that can occur 
//! in this crate.

use tokio_tungstenite::tungstenite as tungstenite;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use crate::data::{AuthDataBuilderError, SubscriptionDataBuilderError};

/*******************************************************************************
 * GENERIC STUFFS
 ******************************************************************************/

/// Error types that can occur while working with this crate
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("error with the websocket {0}")]
    Websocket(#[from] tungstenite::Error),
    #[error("error with Alpaca's realtime API {0}")]
    Realtime(#[from] RealtimeError),
    #[error("error in the conversion from/to JSON")]
    Json(#[from] serde_json::Error),
    #[error("BUG: {0}")]
    AuthDataBuilder(#[from] AuthDataBuilderError),
    #[error("BUG: {0}")]
    SubscriptionDataBuilder(#[from] SubscriptionDataBuilderError),
    #[error("http error {0}")]
    HttpError(#[from] reqwest::Error),
}

/*******************************************************************************
 * REALTIME SPECIFIC STUFFS
 ******************************************************************************/

/// Encapsulates the realtime specific protocol errors
#[derive(Debug, thiserror::Error, Clone, Serialize, Deserialize)]
#[error("{message}")]
pub struct RealtimeError {
    /// Code identifying the problem
    #[serde(rename="code")]
    pub code: RealtimeErrorCode,
    /// Human readable explanation of the failure
    #[serde(rename="msg")]
    message: String,
}
/// Encapsulates the protocol errors codes
#[derive(Debug, Clone, Copy, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum RealtimeErrorCode {
    /// The message you sent to the server did not follow the specification
    /// ```[{"T":"error","code":400,"msg":"invalid syntax"}]```
    #[serde(rename="400")]
    InvalidSyntax = 400,
    /// You have attempted to subscribe or unsubscribe before authentication
    /// ```[{"T":"error","code":401,"msg":"not authenticated"}]```
    #[serde(rename="401")]
    NotAuthenticated = 401,
    /// You have provided invalid authentication credentials.
    /// ```[{"T":"error","code":402,"msg":"auth failed"}]```
    #[serde(rename="402")]
    AuthFailed = 402,
    /// You have already successfully authenticated during your current session.
    /// ```[{"T":"error","code":403,"msg":"already authenticated"}]```
    #[serde(rename="403")]
    AlreadyAuthenticated = 403,
    /// You failed to successfully authenticate after connecting. 
    /// You have a few seconds to authenticate after connecting.
    /// ```[{"T":"error","code":404,"msg":"auth timeout"}]```
    #[serde(rename="404")]
    AuthTimeout  = 404,
    /// The symbol subscription request you sent would put you over the limit 
    /// set by your subscription package. If this happens your symbol 
    /// subscriptions are the same as they were before you sent the request 
    /// that failed.
    /// ```[{"T":"error","code":405,"msg":"symbol limit exceeded"}]```
    #[serde(rename="405")]
    SymbolLimitExceeded = 405,
    /// You already have an ongoing authenticated session.
    /// ```[{"T":"error","code":406,"msg":"connection limit exceeded"}]```
    #[serde(rename="406")]
    ConnectionLimitExceeded = 406,
    /// You may receive this if you are too slow to process the messages sent 
    /// by the server. Please note that this is not guaranteed to arrive 
    /// before you are disconnected to avoid keeping slow connections active 
    /// forever
    /// ```[{"T":"error","code":407,"msg":"slow client"}]```
    #[serde(rename="407")]
    SlowClient = 407,
    /// Your account does not have access to Data v2.
    /// ```[{"T":"error","code":408,"msg":"v2 not enabled"}]```
    #[serde(rename="408")]
    DataV2NotEnabled = 408,
    /// You have attempted to access a data source not available in your 
    /// subscription package.
    /// ```[{"T":"error","code":409,"msg":"insufficient subscription"}]```
    #[serde(rename="409")]
    InsufficientSubscription = 409,
    /// An unexpected error occurred on our end and we are investigating the issue.
    /// ```[{"T":"error","code":500,"msg":"internal error"}```
    #[serde(rename="500")]
    InternalError = 500,
}


/*******************************************************************************
 * HISTORICAL API SPECIFIC STUFFS
 ******************************************************************************/
