//! # Market Finance
//! 
//! A collection of data structures, algorithms and functions for use with a variety of financial markets.

mod traits;
pub use traits::{ Timestamped };

mod interval;
pub use interval::Interval;

mod bar;
pub use bar::Bar;

mod quote;
pub use quote::{ Quote, TradingSession };