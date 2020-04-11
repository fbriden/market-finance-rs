/// A single 'bar' of information for OHLC (+volume) pricing data over a unit
/// of time.
/// 
/// The unit of time is defined by whatever function returns the Bar, but is
/// generally one day.
#[derive(Debug)]
pub struct Bar {
   /// A time during the unit of time - usually the time at open.
   pub timestamp: u64,

   /// The price at the start of the unit of time
   pub open: f64,

   /// The highest price seen during the unit of time
   pub high: f64,

   /// The lowest price seen during the unit of time
   pub low: f64,

   /// The price at the end of the unit of time
   pub close: f64,

   /// The optional volume traded during the unit of time
   pub volume: Option<u64>
}