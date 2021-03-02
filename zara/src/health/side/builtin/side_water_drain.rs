use crate::health::side::builtin::{WaterDrainOverTimeSideEffect, WaterDrainOverTimeSideEffectStateContract};
use crate::health::side::{SideEffectsMonitor, SideEffectDeltasC};
use crate::utils::FrameSummaryC;

use std::cell::Cell;
use std::any::Any;

impl WaterDrainOverTimeSideEffect {
    /// Creates new `WaterDrainOverTimeSideEffect` instance.
    ///
    /// # Parameters
    /// - `drain_amount`: drain speed, 0..100 percents per game second
    pub fn new(drain_amount: f32) -> Self {
        WaterDrainOverTimeSideEffect {
            drain_amount: Cell::new(drain_amount)
        }
    }
    /// Returns a state snapshot contract for this `WaterDrainOverTimeSideEffect` instance
    pub fn get_state(&self) -> WaterDrainOverTimeSideEffectStateContract {
        WaterDrainOverTimeSideEffectStateContract {
            drain_amount: self.drain_amount.get()
        }
    }
    /// Restores state snapshot
    /// 
    /// # Parameters
    /// - `state`: captured earlier state
    pub fn restore_state(&self, state: &WaterDrainOverTimeSideEffectStateContract) {
        self.drain_amount.set(state.drain_amount);
    }
}

impl SideEffectsMonitor for WaterDrainOverTimeSideEffect {
    fn check(&self, frame_data: &FrameSummaryC) -> SideEffectDeltasC {
        SideEffectDeltasC {
            water_level_bonus: -self.drain_amount.get() * frame_data.game_time_delta,
            ..Default::default()
        }
    }

    fn as_any(&self) -> &dyn Any { self }
}