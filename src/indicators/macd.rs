use crate::Bar;
use super::averages::ExponentialMovingAverage;
use super::traits::{ TechnicalIndicator, UpdatableIndicator };

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MACD {
   pub histogram: f64,
   pub signal: f64,
   pub value: f64
}
impl Eq for MACD {}

/// The Moving Average Convergence-Divergence indicator.
///
/// For more information - https://en.wikipedia.org/wiki/MACD
#[derive(Clone, Debug)]
pub struct MovingAverageCD {
   fast_ema: ExponentialMovingAverage,
   slow_ema: ExponentialMovingAverage,
   macd_ema: ExponentialMovingAverage,

   /// The current MACD values
   current: MACD,
}
impl MovingAverageCD {
   /// Creates a new Moving Average Convergence Divergence indicator for the given
   /// time periods.
   ///
   /// # Arguments
   ///
   /// * `fast` the number of time periods for the fast line - often 12
   /// * `slow` the number of time periods for the slow line - often 26
   /// * `signal` the number of time periods for the signal line - often 9
   pub fn new(fast: u16, slow: u16, signal: u16) -> Self {
      Self {
         fast_ema: ExponentialMovingAverage::new(fast),
         slow_ema: ExponentialMovingAverage::new(slow),
         macd_ema: ExponentialMovingAverage::new(signal),
         current: MACD { histogram: 0.0, signal: 0.0, value: 0.0 }
      }
   }

   fn update_current(&mut self, fast_ema: f64, slow_ema: f64, macd_ema: f64) -> MACD {
      self.current = MACD {
         value: fast_ema - slow_ema,
         signal: macd_ema,
         histogram: fast_ema - slow_ema - macd_ema
      };

      self.current
   }
}
impl TechnicalIndicator for MovingAverageCD {
   type Output = MACD;

   fn current(self) -> Self::Output { self.current }
}
impl UpdatableIndicator<Bar> for MovingAverageCD {
   type Output = MACD;

   fn commit(&mut self, current: &Bar) -> Self::Output {
      let fast_ema = self.fast_ema.commit(current);
      let slow_ema = self.slow_ema.commit(current);
      let macd_ema = self.macd_ema.commit(&(fast_ema - slow_ema));

      self.update_current(fast_ema, slow_ema, macd_ema)
   }

   fn update(&mut self, current: &Bar) -> Self::Output {
      let fast_ema = self.fast_ema.update(current);
      let slow_ema = self.slow_ema.update(current);
      let macd_ema = self.macd_ema.update(&(fast_ema - slow_ema));

      self.update_current(fast_ema, slow_ema, macd_ema)
   }
}
impl UpdatableIndicator<f64> for MovingAverageCD {
   type Output = MACD;

   fn commit(&mut self, current: &f64) -> Self::Output {
      let fast_ema = self.fast_ema.commit(current);
      let slow_ema = self.slow_ema.commit(current);
      let macd_ema = self.macd_ema.commit(&(fast_ema - slow_ema));

      self.update_current(fast_ema, slow_ema, macd_ema)
   }
   
   fn update(&mut self, current: &f64) -> Self::Output {
      let fast_ema = self.fast_ema.update(current);
      let slow_ema = self.slow_ema.update(current);
      let macd_ema = self.macd_ema.update(&(fast_ema - slow_ema));

      self.update_current(fast_ema, slow_ema, macd_ema)
   }
}

#[cfg(test)]
mod test {
   use super::*;

   #[test]
   fn new() {
      // GIVEN - a new EMA with a given number of periods
      let macd = MovingAverageCD::new(7, 15, 9);

      // WHEN - we get the values
      // THEN - are what we expect
      let current = macd.current();
      assert_eq!(0.0, current.histogram);
      assert_eq!(0.0, current.signal);
      assert_eq!(0.0, current.value);
   }

   #[test]
   fn update_f64_start() {
      // GIVEN - a new EMA with a given number of periods
      let mut macd = MovingAverageCD::new(7, 15, 9);

      // WHEN - we get the values
      let result = macd.update(&100.0);

      // THEN - are what we expect
      let current = macd.current();
      assert_eq!(10.0, current.histogram);
      assert_eq!(2.5, current.signal);
      assert_eq!(12.5, current.value);
   }

   #[test]
   fn update_f64_next() {
      // GIVEN - a new EMA with a given number of periods
      let mut macd = MovingAverageCD::new(7, 15, 9);

      // WHEN - we get the values
      macd.commit(&100.0);
      let result = macd.update(&50.0);

      // THEN - are what we expect
      let current = macd.current();
      assert_eq!(8.0, current.histogram);
      assert_eq!(6.0625, current.signal);
      assert_eq!(14.0625, current.value);
   }
}