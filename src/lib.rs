//! # Market Data
//! ## Overview
//! Alpaca Data API v2 provides market data through an easy to use HTTP API for 
//! historical data and through websocket for real-time data.
//! 
//! ## Subscription Plans
//! Alpaca Data API v2 provides market data in 2 two different plans: Free 
//! and Unlimited. The Free plan is included in both paper-only and live 
//! trading accounts as the default plan for free. The Free plan consists of 
//! data from IEX (Investors Exchange LLC). For the Unlimited plan, we receive 
//! direct feeds from the CTA (administered by NYSE) and UTP (administered by 
//! Nasdaq) SIPs. These 2 feeds combined offer 100% market volume.

mod utils;
pub mod errors;
pub mod entities;

pub mod rest;

pub mod historical;
pub mod orders;
pub mod positions;
pub mod assets;

pub mod realtime;
pub mod streaming;