//! This module provides a namespace to declare the errors that can occur 
//! in this crate.
use reqwest::Response;
use tokio_tungstenite::tungstenite as tungstenite;
use serde::{Deserialize, Serialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use crate::realtime::{AuthDataBuilderError, SubscriptionDataBuilderError};

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
    #[error("error with Alpaca's history API {0}")]
    History(#[from] HistoryError),
    #[error("error with Alpaca's order API {0}")]
    Order(#[from] OrderError),
    #[error("error in the conversion from/to JSON")]
    Json(#[from] serde_json::Error),
    #[error("BUG: {0}")]
    AuthDataBuilder(#[from] AuthDataBuilderError),
    #[error("BUG: {0}")]
    SubscriptionDataBuilder(#[from] SubscriptionDataBuilderError),
    #[error("http error {0}")]
    HttpError(#[from] reqwest::Error),
    /// Should never occur
    #[error("BUG: Unexpected http status ({0})")]
    Unexpected(u16),
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

/// Basically, Alpaca has reused the standard meaning of HTTP statuses but
/// this error type adds some 'business' information on top of it
 #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize_repr, Deserialize_repr, thiserror::Error)]
 #[repr(u16)]
pub enum HistoryError {
    /// Invalid value for query parameter
    #[error("invalid value for query parameter")]
    #[serde(rename="400")]
    BadRequest = 400,
    /// Unauthorized
    #[error("unauthorized")]
    #[serde(rename="403")]
    Forbidden = 403,
    /// Not Found
    #[error("not found")]
    #[serde(rename="404")]
    NotFound = 404,
    /// Invalid query parameter
    #[error("invalid query parameter")]
    #[serde(rename="422")]
    Unprocessable = 422,
    /// Rate limit exceeded
    #[error("rate limit exceeded")]
    #[serde(rename="429")]
    TooManyRequests = 429,
}

/// Attempts to convert an HTTP error into an history error. 
/// Basically, Alpaca has reused the standard meaning of HTTP statuses but
/// this error type adds some 'business' information on top of it
pub(crate) fn maybe_convert_to_hist_error(e: reqwest::Error) -> Error {
    if let Some(status) = e.status() {
        match status.as_u16() {
            400 => Error::History(HistoryError::BadRequest),
            403 => Error::History(HistoryError::Forbidden),
            404 => Error::History(HistoryError::NotFound),
            422 => Error::History(HistoryError::Unprocessable),
            429 => Error::History(HistoryError::TooManyRequests),
            _   => Error::HttpError(e)
        }
    } else {
        Error::HttpError(e)
    }
}
pub(crate) async fn status_code_to_hist_error<T>(rsp: Response) -> Result<T, Error> 
    where T: for<'de> Deserialize<'de>
{
    match rsp.status().as_u16() {
        200 => Ok(rsp.json::<T>().await?),
        400 => Err(Error::History(HistoryError::BadRequest)),
        403 => Err(Error::History(HistoryError::Forbidden)),
        404 => Err(Error::History(HistoryError::NotFound)),
        422 => Err(Error::History(HistoryError::Unprocessable)),
        429 => Err(Error::History(HistoryError::TooManyRequests)),
        s   => Err(Error::Unexpected(s))
    }
}

/*******************************************************************************
 * ORDER API SPECIFIC STUFFS
 ******************************************************************************/

/// Basically, Alpaca has reused the standard meaning of HTTP statuses but
/// this error type adds some 'business' information on top of it
 #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize_repr, Deserialize_repr, thiserror::Error)]
 #[repr(u16)]
pub enum OrderError {
    /// Buying power or shares is not sufficient
    #[error("Buying power or shares is not sufficient")]
    #[serde(rename="403")]
    Forbidden = 403,
    /// Order is not found
    #[error("Order is not found")]
    #[serde(rename="404")]
    NotFound = 404,
    /// Input parameters are not recognized
    #[error("Input parameters are not recognized")]
    #[serde(rename="422")]
    Unprocessable = 422,
    /// Failed to cancel order
    #[error("Failed to cancel order")]
    #[serde(rename="500")]
    InternalError,
}

/// Attempts to convert an HTTP error into an order error. 
/// Basically, Alpaca has reused the standard meaning of HTTP statuses but
/// this error type adds some 'business' information on top of it
pub(crate) fn maybe_convert_to_order_error(e: reqwest::Error) -> Error {
    if let Some(status) = e.status() {
        match status.as_u16() {
            403 => Error::Order(OrderError::Forbidden),
            404 => Error::Order(OrderError::NotFound),
            422 => Error::Order(OrderError::Unprocessable),
            500 => Error::Order(OrderError::InternalError),
            _   => Error::HttpError(e)
        }
    } else {
        Error::HttpError(e)
    }
}
pub(crate) async fn status_code_to_order_error<T>(rsp: Response) -> Result<T, Error> 
    where T: for<'de> Deserialize<'de>
{
    match rsp.status().as_u16() {
        200 => Ok(rsp.json::<T>().await?),
        204 => Ok(rsp.json::<T>().await?),
        207 => Ok(rsp.json::<T>().await?),
        403 => Err(Error::Order(OrderError::Forbidden)),
        404 => Err(Error::Order(OrderError::NotFound)),
        422 => Err(Error::Order(OrderError::Unprocessable)),
        500 => Err(Error::Order(OrderError::InternalError)),
        s   => Err(Error::Unexpected(s)),
    }
}