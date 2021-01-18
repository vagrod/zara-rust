use crate::health::side::builtin::FoodDrainOverTimeSideEffect;
use crate::health::side::{SideEffectsMonitor, SideEffectDeltasC};
use crate::utils::FrameSummaryC;

use std::cell::Cell;

impl FoodDrainOverTimeSideEffect {
    /// Creates new `FoodDrainOverTimeSideEffect` instance.
    ///
    /// # Parameters
    /// - `drain_amount`: drain speed, 0..100 percents per game second
    pub fn new(drain_amount: f32) -> Self {
        FoodDrainOverTimeSideEffect {
            drain_amount: Cell::new(drain_amount)
        }
    }
}

impl SideEffectsMonitor for FoodDrainOverTimeSideEffect {
    fn check(&self, frame_data: &FrameSummaryC) -> SideEffectDeltasC {
        SideEffectDeltasC {
            food_level_bonus: -self.drain_amount.get() * frame_data.game_time_delta,
            ..Default::default()
        }
    }
}