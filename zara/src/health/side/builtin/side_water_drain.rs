use crate::health::side::builtin::{WaterDrainOverTimeSideEffect, WaterDrainOverTimeSideEffectStateContract};
use crate::health::side::{SideEffectsMonitor, SideEffectDeltasC};
use crate::utils::FrameSummaryC;

use std::cell::Cell;

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
    pub fn get_state(&self) -> WaterDrainOverTimeSideEffectStateContract {
        WaterDrainOverTimeSideEffectStateContract {
            drain_amount: self.drain_amount.get()
        }
    }
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
}