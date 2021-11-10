//! # Assets
//! The assets API serves as the master list of assets available for trade 
//! and data consumption from Alpaca. Assets are sorted by asset class, 
//! exchange and symbol. Some assets are only available for data consumption 
//! via Polygon, and are not tradable with Alpaca. These assets will be 
//! marked with the flag tradable=false.

use crate::{entities::{AssetData, AssetStatus}, errors::{Error, maybe_convert_to_asset_error, status_code_to_asset_error}, rest::Client};

static ENDPOINT: &str = "/v2/assets";

impl Client {
  /// Get a list of assets
  /// 
  /// # Parameters
  /// - status: .g. “active”. By default, all statuses are included.
  /// - asset_class: Defaults to us_equity.
  pub async fn list_assets(&self, status: Option<AssetStatus>, asset_class: Option<&str>) -> Result<Vec<AssetData>, Error> {
    let url = format!("{}/{}", self.env_url(), ENDPOINT);
    let mut params = vec![];
    if let Some(status) = status {
      params.push(("status", status.to_str()));
    }
    if let Some(asset_class) = asset_class {
      params.push(("asset_class", asset_class));
    }
    let rsp = self.get_authenticated(&url)
      .query(&params)
      .send().await
      .map_err(maybe_convert_to_asset_error)?;
    status_code_to_asset_error(rsp).await
  }

  /// Get an asset for the given symbol
  pub async fn get_asset(&self, symbol: &str) -> Result<AssetData, Error> {
    let url = format!("{}/{}/{}", self.env_url(), ENDPOINT, symbol);
    let rsp = self.get_authenticated(&url)
      .send().await
      .map_err(maybe_convert_to_asset_error)?;
    status_code_to_asset_error(rsp).await
  }
}