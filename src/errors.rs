//! This module provides a namespace to declare the errors that can occur 
//! in this crate.

use tokio_tungstenite::tungstenite as tungstenite;

use crate::data::{AlpacaError, AuthDataBuilderError, SubscriptionDataBuilderError};

/// Error types that can occur while working with this crate
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("error with the websocket {0}")]
    Websocket(#[from] tungstenite::Error),
    #[error("error with Alpaca {0}")]
    Alpaca(#[from] AlpacaError),
    #[error("error in the conversion from/to JSON")]
    Json(#[from] serde_json::Error),
    #[error("BUG: {0}")]
    AuthDataBuilder(#[from] AuthDataBuilderError),
    #[error("BUG: {0}")]
    SubscriptionDataBuilder(#[from] SubscriptionDataBuilderError),
}
