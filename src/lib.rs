//! # Market Finance
//! 
//! A collection of data structures, algorithms and functions for use with a variety of financial markets.
//!
//! Technical indicators are currently beta and not enabled by default.

mod traits;
pub use traits::{ Timestamped };

#[cfg(feature = "indicators")]
pub mod indicators;

mod interval;
pub use interval::Interval;

mod bar;
pub use bar::Bar;

mod quote;
pub use quote::{ Quote, TradingSession };