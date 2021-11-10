//! The positions API provides information about an account’s current open 
//! positions. The response will include information such as cost basis, 
//! shares traded, and market value, which will be updated live as price 
//! information is updated. Once a position is closed, it will no longer be 
//! queryable through this API.

use crate::{entities::{ClosureData, OrderData, PositionData}, errors::{Error, maybe_convert_to_position_error, status_code_to_position_error}, rest::Client};

/// Path to the positions endpoints
static ENDPOINT: &str = "/v2/positions";

impl Client {
  /// Retrieves a list of the account’s open positions. 
  pub async fn list_open_positions(&self) -> Result<Vec<PositionData>, Error> {
    let url = format!("{}/{}", self.env_url(), ENDPOINT);
    let rsp = self.get_authenticated(&url)
      .send().await
      .map_err(maybe_convert_to_position_error)?;
    status_code_to_position_error(rsp).await
  }
  /// Retrieves the account’s open position for the given symbol.
  pub async fn get_open_position(&self, symbol: &str) -> Result<PositionData, Error> {
    let url = format!("{}/{}/{}", self.env_url(), ENDPOINT, symbol);
    let rsp = self.get_authenticated(&url)
      .send().await
      .map_err(maybe_convert_to_position_error)?;
    status_code_to_position_error(rsp).await
  }
  /// Closes (liquidates) all of the account’s open long and short positions. 
  /// A response will be provided for each order that is attempted to be 
  /// cancelled. If an order is no longer cancelable, the server will respond 
  /// with status 500 and reject the request.
  /// 
  /// # Param
  /// - cancel_orders: If true is specified, cancel all open orders before 
  ///     liquidating all positions.
  pub async fn close_all_positions(&self, cancel_orders: bool) -> Result<Vec<ClosureData>, Error> {
    let url = format!("{}/{}", self.env_url(), ENDPOINT);
    let rsp = self.delete_authenticated(&url)
      .query(&[("cancel_orders", cancel_orders)])
      .send().await
      .map_err(maybe_convert_to_position_error)?;
    status_code_to_position_error(rsp).await
  }

  /// # Params
  /// - symbol: symbol or asset_id
  /// - qty   : the number of shares to liquidate. Can accept up to 9 decimal 
  ///     points. Cannot work with percentage
  /// - percentage: percentage of position to liquidate. Must be between 
  ///     0 and 100. Would only sell fractional if position is originally 
  ///     fractional. Can accept up to 9 decimal points. Cannot work with qty 
  pub async fn close_position(&self, symbol: &str, qty: Option<f64>, percentage: Option<f64>) -> Result<OrderData, Error> {
    let url = format!("{}/{}/{}", self.env_url(), ENDPOINT, symbol);
    let mut params = vec![];
    if let Some(qty) = qty {
      params.push(("qty", qty));
    }
    if let Some(percentage) = percentage {
      params.push(("percentage", percentage));
    }
    let rsp = self.delete_authenticated(&url)
      .query(&params)
      .send().await
      .map_err(maybe_convert_to_position_error)?;
    status_code_to_position_error(rsp).await
  }
}