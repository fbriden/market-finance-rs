use std::cmp::Ordering;
use super::Timestamped;

/// A single 'bar' of information for OHLC (+volume) pricing data over a unit
/// of time.
///
///  * Bars are ordered by the timestamp.
///  * The unit of time is defined by whatever function returns the Bar, but is
/// generally one day.
#[derive(Clone, Copy, Debug)]
pub struct Bar {
   /// A time during the unit of time - usually the time at open.
   pub timestamp: i64,

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
impl Eq for Bar {}
impl Ord for Bar {
   fn cmp(&self, other: &Self) -> Ordering { self.timestamp.cmp(&other.timestamp) }
}
impl PartialOrd for Bar {
   fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}
impl PartialEq for Bar {
   fn eq(&self, other: &Self) -> bool { self.timestamp == other.timestamp }
}
impl Timestamped for Bar {
   /// Gets the timestamp in millisecond accuracy
   fn timestamp_millis(&self) -> i64 { self.timestamp }
}


#[cfg(test)]
mod tests {
   #[cfg(feature = "formatting")]
   use chrono::{ TimeZone, Utc };
   use super::{ Bar, Timestamped };

   #[cfg(feature = "formatting")]
   #[test]
   fn verify_datetime() {
      let dt = Utc.ymd(1970, 1, 1).and_hms_milli(0, 0, 3, 123);
      let bar = Bar { timestamp: 3123, open: 0.0, high: 0.0, low: 0.0, close: 0.0, volume: None };
      assert_eq!(dt, bar.datetime())
   }

   #[test]
   fn verify_timestamp_millis() {
      let bar = Bar { timestamp: 3002, open: 0.0, high: 0.0, low: 0.0, close: 0.0, volume: None };
      assert_eq!(3002, bar.timestamp_millis())
   }

   #[test]
   fn verify_timestamp_seconds() {
      let bar = Bar { timestamp: 3002, open: 0.0, high: 0.0, low: 0.0, close: 0.0, volume: None };
      assert_eq!(3, bar.timestamp_seconds())
   }
}