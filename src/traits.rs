#[cfg(feature = "formatting")]
use chrono::{ DateTime, NaiveDateTime, Utc };

///  A collection of methods for a structure that has a single timestamp
pub trait Timestamped {
   #[cfg(feature = "formatting")]
   /// Converts the timestamp into a chrono DateTime structure for easier date / time formatting
   fn datetime(&self) -> DateTime<Utc> {
      let millis = (self.timestamp_millis() % 1_000) * 1_000_000;
      DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(self.timestamp_seconds() as i64, millis as u32), Utc)
   }

   /// Gets the timestamp as the number milliseconds that have elapsed since the EPOCH
   fn timestamp_millis(&self) -> i64;

   /// Gets the timestamp as the number of seconds that have elapsed since the EPOCH
   fn timestamp_seconds(&self) -> i64 { self.timestamp_millis() / 1_000 }
}