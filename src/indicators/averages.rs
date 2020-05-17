use crate::Bar;
use super::traits::{ TechnicalIndicator, UpdatableIndicator };

type Precision = f64;

/// Exponential Moving Average
///
/// Uses an alpha of 2 / (1 + number of periods)
#[derive(Clone, Copy, Debug)]
pub struct ExponentialMovingAverage {
   /// The degree of weighting decrease - always between 0 and 1
   alpha: Precision,

   // The current value of the moving average
   current_value: Precision,

   /// The previous value in the series
   previous_avg: Precision,
}
impl ExponentialMovingAverage {
   /// Create a new EMA for a given number of time periods.
   pub fn new(period_count: u16) -> Self {
      Self {
         alpha: 2.0 / (Precision::from(period_count) + 1.0),
         current_value: 0.0,
         previous_avg: 0.0
      }
   }

   fn calculate(&mut self, value: Precision) -> Precision {
      self.current_value = self.alpha * value + (1.0 - self.alpha) * self.previous_avg;
      self.current_value
   }
}
impl TechnicalIndicator for ExponentialMovingAverage {
   type Output = Precision;

   fn current(self) -> Self::Output { self.current_value }
}
impl UpdatableIndicator<Bar> for ExponentialMovingAverage {
   type Output = Precision;

   fn commit(&mut self, current: &Bar) -> Self::Output {
      self.previous_avg = self.calculate(current.close);
      self.calculate(current.close)
   }

   fn update(&mut self, current: &Bar) -> Self::Output { self.calculate(current.close) }
}
impl UpdatableIndicator<f64> for ExponentialMovingAverage {
   type Output = Precision;

   fn commit(&mut self, current: &f64) -> Self::Output {
      self.previous_avg = self.calculate(*current);
      self.calculate(*current)
   }

   fn update(&mut self, current: &f64) -> Self::Output { self.calculate(*current) }
}


#[cfg(test)]
mod test {
   use super::*;

   #[test]
   fn new() {
      // GIVEN - a new EMA with a given number of periods
      let ema = ExponentialMovingAverage::new(7);

      // WHEN - we get the values
      // THEN - are what we expect
      assert_eq!(0.0, ema.current_value);
      assert_eq!(0.0, ema.previous_avg);
      assert_eq!(0.25, ema.alpha);
   }

   #[test]
   fn commit_f64() {
      // GIVEN - a new EMA with a given number of periods & initial value
      let mut ema = ExponentialMovingAverage::new(7);

      // WHEN - we commit & update with a new value
      ema.commit(&100.0);

      // THEN - are what we expect
      assert_eq!(43.75, ema.current());
   }

   #[test]
   fn update_f64_first() {
      // GIVEN - a new EMA with a given number of periods
      let mut ema = ExponentialMovingAverage::new(7);

      // WHEN - we we update
      let result = ema.update(&100.0);

      // THEN - are what we expect
      let current = ema.current();
      assert_eq!(result, current);
      assert_eq!(25.0, current);
   }

   #[test]
   fn update_f64_next() {
      // GIVEN - a new EMA with a given number of periods
      let mut ema = ExponentialMovingAverage::new(7);

      // WHEN - we we update
      ema.commit(&100.0);
      let result = ema.update(&50.0);

      // THEN - are what we expect
      let current = ema.current();
      assert_eq!(result, current);
      assert_eq!(31.25, current);
   }
}