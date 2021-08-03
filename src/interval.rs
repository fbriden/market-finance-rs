use std::fmt;
use std::str::FromStr;

/// An interval use when requesting periods of quote information.
/// 
/// Since we cannot start the values with numbers (as they are normally represented),
/// we start them with underscores.
/// 
/// `m` is for minutes. `mo` is for months, the rest should be self explanatory
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Interval { _1m, _2m, _5m, _15m, _30m, _60m, _90m, _1d, _5d, _1mo, _3mo, _6mo, _1y, _2y, _5y, _10y, _ytd, _max }
impl Interval {
   pub fn is_intraday(&self) -> bool {
      match self {
         Self::_1m | Self::_2m | Self::_5m | Self::_15m | Self::_30m | Self::_60m | Self::_90m => true,
         _ => false
      }
   }
}
impl fmt::Display for Interval {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result  {
      f.write_str(&format!("{:?}", self)[1..]) // strip the leading underscore
   }
}

impl FromStr for Interval {
   type Err = InvalidIntervalError;

   fn from_str(s: &str) -> Result<Self, InvalidIntervalError> {
      match s {
         "1m" => Ok(Interval::_1m),
         "2m" => Ok(Interval::_2m),
         "5m" => Ok(Interval::_5m),
         "15m" => Ok(Interval::_15m),
         "30m" => Ok(Interval::_30m),
         "60m" => Ok(Interval::_60m),
         "90m" => Ok(Interval::_90m),
         "1d" => Ok(Interval::_1d),
         "5d" => Ok(Interval::_5d),
         "1mo" => Ok(Interval::_1mo),
         "3mo" => Ok(Interval::_3mo),
         "6mo" => Ok(Interval::_6mo),
         "1y" => Ok(Interval::_1y),
         "2y" => Ok(Interval::_2y),
         "5y" => Ok(Interval::_5y),
         "10y" => Ok(Interval::_10y),
         "ytd" => Ok(Interval::_ytd),
         "max" => Ok(Interval::_max),
         _ => Err(InvalidIntervalError)
      }
   }
}

#[derive(Debug, Clone)]
pub struct InvalidIntervalError;

impl fmt::Display for InvalidIntervalError {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "Invalid interval value")
   }
}


#[cfg(test)]
mod tests {
   use super::Interval;
   use std::str::FromStr;

   // Validate that the intervals are all set up correctly
   // and that there are no copy-paste issues

   fn test_interval(interval: Interval, value: &str, is_intraday: bool) {
      assert_eq!(format!("{}", interval), value);
      assert_eq!(interval.is_intraday(), is_intraday);
      assert_eq!(Interval::from_str(value).unwrap(), interval)
   }

   #[test] fn interval_1m() { test_interval(Interval::_1m, "1m", true); }
   #[test] fn interval_2m() { test_interval(Interval::_2m, "2m", true); }
   #[test] fn interval_5m() { test_interval(Interval::_5m, "5m", true); }
   #[test] fn interval_15m() { test_interval(Interval::_15m, "15m", true); }
   #[test] fn interval_30m() { test_interval(Interval::_30m, "30m", true); }
   #[test] fn interval_60m() { test_interval(Interval::_60m, "60m", true); }
   #[test] fn interval_90m() { test_interval(Interval::_90m, "90m", true); }
   #[test] fn interval_1d() { test_interval(Interval::_1d, "1d", false); }
   #[test] fn interval_5d() { test_interval(Interval::_5d, "5d", false); }
   #[test] fn interval_1mo() { test_interval(Interval::_1mo, "1mo", false); }
   #[test] fn interval_3mo() { test_interval(Interval::_3mo, "3mo", false); }
   #[test] fn interval_6mo() { test_interval(Interval::_6mo, "6mo", false); }
   #[test] fn interval_1y() { test_interval(Interval::_1y, "1y", false); }
   #[test] fn interval_2y() { test_interval(Interval::_2y, "2y", false); }
   #[test] fn interval_5y() { test_interval(Interval::_5y, "5y", false); }
   #[test] fn interval_10y() { test_interval(Interval::_10y, "10y", false); }
   #[test] fn interval_ytd() { test_interval(Interval::_ytd, "ytd", false); }
   #[test] fn interval_max() { test_interval(Interval::_max, "max", false); }
}