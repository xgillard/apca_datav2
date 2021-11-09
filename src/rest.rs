//! This module encapsulates the technicalities bound to the use of a rest API.
//! As such, it does not provide any business information. It does however help
//! in implementing a seamless interface to Alpaca's services.

use std::{pin::Pin, task::Poll};

use futures::{Future, FutureExt, Stream};
use reqwest::RequestBuilder;

use crate::errors::Error;

/// Header used to send the key-id authentication
pub const APCA_API_KEY_ID: &str = "APCA-API-KEY-ID";
/// Header used to send the secret-key for authentication
pub const APCA_API_SECRET_KEY: &str = "APCA-API-SECRET-KEY";

/// Base URL to interact with live trading api
pub const LIVE_TRADING_URL: &str = "https://api.alpaca.markets";
/// Base URL to interact with paper trading api
pub const PAPER_TRADING_URL: &str = "https://paper-api.alpaca.markets";


/// An authenticated REST client
pub struct Client {
    key: String,
    secret: String,
    client: reqwest::Client,
    env_url: &'static str,
}

impl Client {
  pub fn live(key: String, secret: String) -> Self {
    Self::new(key, secret, true)
  }
  pub fn paper(key: String, secret: String) -> Self {
    Self::new(key, secret, false)
  }
  pub fn new(key: String, secret: String, live: bool) -> Self {
    let env_url = if live { LIVE_TRADING_URL } else { PAPER_TRADING_URL };
    Self {key, secret, client: reqwest::Client::new(), env_url}
  }
  pub fn get_authenticated(&self, url: &str) -> RequestBuilder {
    self.client.get(url)
        .header(APCA_API_KEY_ID,     &self.key)
        .header(APCA_API_SECRET_KEY, &self.secret)        
  }
  pub fn post_authenticated(&self, url: &str) -> RequestBuilder {
    self.client.post(url)
        .header(APCA_API_KEY_ID,     &self.key)
        .header(APCA_API_SECRET_KEY, &self.secret)        
  }
  pub fn patch_authenticated(&self, url: &str) -> RequestBuilder {
    self.client.patch(url)
        .header(APCA_API_KEY_ID,     &self.key)
        .header(APCA_API_SECRET_KEY, &self.secret)        
  }
  pub fn delete_authenticated(&self, url: &str) -> RequestBuilder {
    self.client.delete(url)
        .header(APCA_API_KEY_ID,     &self.key)
        .header(APCA_API_SECRET_KEY, &self.secret)        
  }
  pub fn env_url(&self) -> &'static str {
    self.env_url
  }
}
/******************************************************************************
 ******************************************************************************
 ******************************************************************************/
 
// TODO: If anybody ever reviews this portion of code; is there any better/more
//       idomatic way to accomplish this ?

/// This trait denotes a page (aka a chunk) from a paginated list of item.
/// Basically, it gives a convenient way to transparently access the paged 
/// data along with the next page token which needs to be sent to server in 
/// order to fetch the next chunk.
pub trait Paged {
    type Item;
    /// Splits the page in a data set and an optional next page token
    fn split(self) -> (Vec<Self::Item>, Option<String>);
}
/// This trait basically denotes a factory that creates a future used to fetch
/// the next chunk of data from the server
pub trait FetchNextPage<'a, T: Paged> {
    fn fetch(self: Pin<&Self>, token: Option<String>) -> Pin<Box< dyn Future<Output=Result<T, Error>> + 'a >>;
}

/// A future bound to some given lifetime, returning an Ok(T) or an Error
pub type FailibleFuture<'a, T> = dyn Future<Output=Result<T, Error>> + 'a;

/// A paged stream is a stream that buffers a chunk of data and transparently 
/// fetches the next page whenever whenever needed.
pub struct PagedStream<'a, T, F> 
where T: Paged, 
      T::Item: Unpin,
      F: FetchNextPage<'a, T> + Unpin
{
    source: Pin<Box<F>>,
    data  : Vec<T::Item>,
    fut   : Option<Pin<Box< FailibleFuture<'a, T> >>>
}

impl <'a, T, F> PagedStream<'a, T, F> 
where T: Paged, 
      T::Item: Unpin,
      F: FetchNextPage<'a, T> + Unpin
{
    /// Creates a new paged stream from a given source. The first future is
    /// created by passing a None token.
    pub fn new(source: F) -> Self {
        let source = Box::pin(source);
        let fut    = source.as_ref().fetch(None);

        Self {
            source,
            data: vec![],
            fut : Some(fut),
        }
    }
}

impl <'a, T, F> Stream for PagedStream<'a, T, F> 
where T: Paged, 
      T::Item: Unpin,
      F: FetchNextPage<'a, T> + Unpin
{
    type Item = T::Item;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
        let data = self.data.pop();
        if data.is_some() {
            Poll::Ready(data)
        } else if let Some(fut) = self.fut.as_mut() {
            match fut.poll_unpin(cx) {
                std::task::Poll::Pending => std::task::Poll::Pending,
                std::task::Poll::Ready(data) => {
                    let (data, token) = data.unwrap().split();
                    
                    if token.is_some() {
                        self.fut = Some(self.source.as_ref().fetch(token));
                    } else {
                        self.fut = None;
                    }
                    self.data = data;
                    self.data.reverse();

                    std::task::Poll::Ready(self.data.pop())
                }
            }
        } else {
            Poll::Ready(None)
        }
    }
}