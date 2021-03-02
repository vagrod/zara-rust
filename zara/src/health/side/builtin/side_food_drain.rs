use crate::health::side::builtin::{FoodDrainOverTimeSideEffect, FoodDrainOverTimeSideEffectStateContract};
use crate::health::side::{SideEffectsMonitor, SideEffectDeltasC};
use crate::utils::FrameSummaryC;

use std::cell::Cell;
use std::any::Any;

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
    /// Returns a state snapshot contract for this `FoodDrainOverTimeSideEffect` instance
    pub fn get_state(&self) -> FoodDrainOverTimeSideEffectStateContract {
        FoodDrainOverTimeSideEffectStateContract {
            drain_amount: self.drain_amount.get()
        }
    }
    /// Restores the state from the given state contract
    /// 
    /// # Parameters
    /// - `state`: captured earlier state
    pub fn restore_state(&self, state: &FoodDrainOverTimeSideEffectStateContract) {
        self.drain_amount.set(state.drain_amount);
    }
}

impl SideEffectsMonitor for FoodDrainOverTimeSideEffect {
    fn check(&self, frame_data: &FrameSummaryC) -> SideEffectDeltasC {
        SideEffectDeltasC {
            food_level_bonus: -self.drain_amount.get() * frame_data.game_time_delta,
            ..Default::default()
        }
    }

    fn as_any(&self) -> &dyn Any { self }
}