//! # Watchlist
//!
//! The watchlist API provides CRUD operation for the account’s watchlist. An 
//! account can have multiple watchlists and each is uniquely identified by 
//! id but can also be addressed by user-defined name. Each watchlist is an 
//! ordered list of assets.
//!
use serde::{Serialize, Deserialize};

use crate::{entities::WatchlistData, errors::{Error, maybe_convert_to_watchlist_error, status_code_to_watchlist_error, status_code_to_watchlist_error_noparse}, rest::Client};

/// General endpoint of the watchlist API
static ENDPOINT: &str = "/v2/watchlists";

impl Client {
    /// Returns the list of watchlists registered under the account
    pub async fn list_watchlists(&self) -> Result<Vec<WatchlistData>, Error> {
        let url = format!("{}{}", self.env_url(), ENDPOINT);
        let rsp = self.get_authenticated(&url)
            .send().await
            .map_err(maybe_convert_to_watchlist_error)?;

        status_code_to_watchlist_error(rsp).await
    }

    /// Create a new watchlist with initial set of assets.
    /// 
    /// # Parameters
    /// 
    /// - name arbitrary name string, up to 64 characters
    /// - symbols set of symbol string
    pub async fn create_watchlist(&self, name: &str, symbols: &[&str]) -> Result<WatchlistData, Error> {
        let url = format!("{}{}", self.env_url(), ENDPOINT);
        let req = CreateUpdate {
            name: name.to_string(),
            symbols: symbols.iter().map(|x| x.to_string()).collect()
        };
        let rsp = self.post_authenticated(&url)
            .json(&req)
            .send().await
            .map_err(maybe_convert_to_watchlist_error)?;

        status_code_to_watchlist_error(rsp).await
    }

    /// Returns a watchlist identified by the ID
    pub async fn get_watchlist(&self, id: &str) -> Result<WatchlistData, Error> {
        let url = format!("{}{}/{}", self.env_url(), ENDPOINT, id);
        let rsp = self.get_authenticated(&url)
            .send().await
            .map_err(maybe_convert_to_watchlist_error)?;

        status_code_to_watchlist_error(rsp).await
    }

    /// Update the name and/or content of watchlist
    /// 
    /// # Parameters
    /// 
    /// id the watchlist id
    /// name the new name of the watchlist
    /// symbols the new list of symbol names to replace the watchlist content
    pub async fn update_watchlist(&self, 
            id: &str, 
            name: &str,
            symbols: &[&str]
        ) -> Result<WatchlistData, Error> {
        let url = format!("{}{}/{}", self.env_url(), ENDPOINT, id);
        let req = CreateUpdate{
            name: name.to_string(),
            symbols: symbols.iter().map(|x| x.to_string()).collect()
        };
        let rsp = self.put_authenticated(&url)
            .json(&req)
            .send().await
            .map_err(maybe_convert_to_watchlist_error)?;

        status_code_to_watchlist_error(rsp).await
    }

    ///  Append an asset for the symbol to the end of watchlist asset list
    /// 
    /// # Parameters
    /// 
    /// - id the uuid of the watchlist
    /// - symbol the asset to add to the watchlist
    pub async fn add_asset_to_watchlist(&self, id: &str, symbol: &str) -> Result<WatchlistData, Error> {
        let url = format!("{}{}/{}", self.env_url(), ENDPOINT, id);
        let req = Add { symbol: symbol.to_string() };
        let rsp = self.post_authenticated(&url)
            .json(&req)
            .send().await
            .map_err(maybe_convert_to_watchlist_error)?;

        status_code_to_watchlist_error(rsp).await
    }

    /// Delete a watchlist. This is a permanent deletion
    /// 
    /// # Parameters
    /// 
    /// - id the uuid of the watchlist to delete
    pub async fn delete_watchlist(&self, id: &str) -> Result<(), Error> {
        let url = format!("{}{}/{}", self.env_url(), ENDPOINT, id);
        
        let rsp = self.delete_authenticated(&url)
            .send().await
            .map_err(maybe_convert_to_watchlist_error)?;

        status_code_to_watchlist_error_noparse(rsp).await
    }

    /// Delete one entry for an asset by symbol name
    /// 
    /// # Parameters
    /// 
    /// - id the uuid of the watchlist
    /// - symbol the symbol to remove from watchlist
    pub async fn remove_asset_from_watchlist(&self, id: &str, symbol: &str) -> Result<(), Error> {
        let url = format!("{}{}/{}/{}", self.env_url(), ENDPOINT, id, symbol);
        
        let rsp = self.delete_authenticated(&url)
            .send().await
            .map_err(maybe_convert_to_watchlist_error)?;

        status_code_to_watchlist_error_noparse(rsp).await
    }
}

/// Private : body parameters to create/update a watch list
#[derive(Debug, Serialize, Deserialize)]
struct CreateUpdate {
    name: String,
    symbols: Vec<String>,
}

/// Private : body parameters to add some ticker to watchlist
#[derive(Debug, Serialize, Deserialize)]
struct Add {
    symbol: String,
}