/// The trading session where a quote has occurred
#[derive(Debug)]
pub enum TradingSession {
   /// The period of trading before the regular market session
   PreMarket,

   /// The period of trading during the regular market session
   Regular,

   /// The period of trading immediately after the regular market session
   AfterHours,

   /// Undefined right now - for future use
   Other
}

/// A symbol's quote at a period in time
#[derive(Debug)]
pub struct Quote {
   /// The symbol for the quote
   pub symbol: String,

   /// The date / time of the quote
   pub timestamp: u64,

   /// The trading session of the quote - pre market / regular hours / after hours
   pub session: TradingSession,

   /// The price of the quote
   pub price: f64,

   /// The volume (daily or transactional) of the symbol
   pub volume: u64
}
