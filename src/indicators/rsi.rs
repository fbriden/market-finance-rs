use crate::Bar;
use super::averages::ExponentialMovingAverage;
use super::traits::{ TechnicalIndicator, UpdatableIndicator };

type Precision = f64;

/// Relative Strength Index indicator
#[derive(Clone, Debug)]
pub struct RelativeStrengthIndex {
   // EMA for the previous N time periods the price went up
   average_up: ExponentialMovingAverage,

   // EMA for the previous N time periods the price went down
   average_down: ExponentialMovingAverage,

   /// The previous relative strength index value.  Used to calculate the current value
   previous_value: Precision,

   /// The current relative strength index value
   current_value: Precision,
}
impl RelativeStrengthIndex {
   /// Creates a new Relative Strength Index indicator for the given
   /// time periods.  Uses an EMA for smoothing
   ///
   /// # Arguments
   ///
   /// * `periods` the number of time periods for the calculation - often 14
   pub fn new(periods: u16) -> Self {
      Self {
         average_up: ExponentialMovingAverage::new(periods),
         average_down: ExponentialMovingAverage::new(periods),
         current_value: 0.0,
         previous_value: 0.0
      }
   }

   fn build_up_down(&self) -> (f64, f64) {
      ((self.current_value - self.previous_value).max(0.0), (self.previous_value - self.current_value).max(0.0))
   }

   fn calculate(&self) -> Precision {
      let mut rs = self.average_up.current() / self.average_down.current();
      if rs.is_nan() { rs = 1.0 }
      100.0 - (100.0 / (1.0 + rs))
   }

   fn commit_averages(&mut self, current: f64) -> Precision {
      let values = self.build_up_down();
      if values.0 > 0.0 { self.average_up.commit(&values.0); }
      if values.1 > 0.0 { self.average_down.commit(&values.1); }

      self.previous_value = self.current_value;
      self.update_averages(current)
   }

   fn update_averages(&mut self, current: f64) -> Precision {
      let values = self.build_up_down();
      if values.0 > 0.0 { self.average_up.update(&values.0); }
      if values.1 > 0.0 { self.average_down.update(&values.1); }

      self.current_value = current;
      self.calculate()
   }
}
impl TechnicalIndicator for RelativeStrengthIndex {
   type Output = Precision;

   fn current(self) -> Self::Output { self.calculate() }
}
impl UpdatableIndicator<Bar> for RelativeStrengthIndex {
   type Output = Precision;

   fn commit(&mut self, current: &Bar) -> Self::Output { self.commit_averages(current.close) }

   fn update(&mut self, current: &Bar) -> Self::Output { self.update_averages(current.close) }
}
impl UpdatableIndicator<f64> for RelativeStrengthIndex {
   type Output = Precision;

   fn commit(&mut self, current: &f64) -> Self::Output { self.commit_averages(*current) }

   fn update(&mut self, current: &f64) -> Self::Output { self.update_averages(*current) }
}

#[cfg(test)]
mod test {
   use super::*;

   #[test]
   fn new() {
      // GIVEN - a new EMA with a given number of periods
      let rsi = RelativeStrengthIndex::new(7);

      // WHEN - we get the values
      // THEN - are what we expect
      assert_eq!(0.0, rsi.current_value);
      assert_eq!(0.0, rsi.previous_value);
   }

   #[test]
   fn commit_f64() {
      // GIVEN - a new EMA with a given number of periods & initial value
      let mut rsi = RelativeStrengthIndex::new(7);

      // WHEN - we commit & update with a new value
      rsi.commit(&100.0);

      // THEN - are what we expect
      assert_eq!(50.0, rsi.current());
   }

   #[test]
   fn update_f64_first() {
      // GIVEN - a new EMA with a given number of periods
      let mut rsi = RelativeStrengthIndex::new(7);

      // WHEN - we get the values
      let result = rsi.update(&50.0);

      // THEN - are what we expect
      let current = rsi.current();
      assert_eq!(result, current);
      assert_eq!(50.0, current);
   }

   #[test]
   fn update_f64_next() {
      // GIVEN - a new EMA with a given number of periods
      let mut rsi = RelativeStrengthIndex::new(7);

      // WHEN - we get the values
      rsi.commit(&100.0);
      let result = rsi.update(&80.0);

      // THEN - are what we expect
      let current = rsi.current();
      assert_eq!(result, current);
      assert_eq!(100.0, current);
   }
}