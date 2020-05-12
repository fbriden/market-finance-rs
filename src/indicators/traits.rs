pub trait TechnicalIndicator {
   type Output;

   /// Gets the current indicator value.
   fn current(self) -> Self::Output;
}

pub trait UpdatableIndicator<Input> {
   type Output;

   /// 'Commits' the current indicator value by copying it to the previous
   /// value
   ///
   /// This is used for historical calculations.
   ///
   /// Returns the current indicator value
   fn commit(&mut self, current: &Input) -> Self::Output;

   /// Updates the current value without affecting the previous value.
   ///
   /// This is used for realtime updates where the previous calculations
   /// are not updated.
   ///
   /// Returns the current indicator value
   fn update(&mut self, current: &Input) -> Self::Output;
}