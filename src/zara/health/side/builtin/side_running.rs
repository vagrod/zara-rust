use crate::health::side::{SideEffectsMonitor, SideEffectDeltasC};
use crate::utils::{FrameSummaryC};
use crate::health::side::builtin::RunningSideEffects;

use std::cell::Cell;

/// RunningSideEffects implementation

impl RunningSideEffects {
    pub fn new() -> Self {
        RunningSideEffects {
            running_state: Cell::new(false),
            running_time: Cell::new(0.)
        }
    }
}
impl SideEffectsMonitor for RunningSideEffects {
    fn check(&self, frame_data: &FrameSummaryC) -> SideEffectDeltasC {
        const TIME_TO_REACH_RUNNING_EXHAUST: f32 = 60. * 5.; // game seconds
        const MAX_HEART_RATE_IMPACT: f32 = 45.;
        const MAX_BODY_TEMP_IMPACT: f32 = 0.3;
        const MAX_TOP_PRESSURE_IMPACT: f32 = 24.;
        const MAX_BOTTOM_PRESSURE_IMPACT: f32 = 16.;

        if frame_data.player.is_running {
            if self.running_state.get() == false {
                self.running_state.set(true);
                self.running_time.set(0.);
            }

            self.running_time.set(crate::utils::clamp_to(
                self.running_time.get() + frame_data.game_time_delta,
                TIME_TO_REACH_RUNNING_EXHAUST)
            );

            let p = crate::utils::clamp_01(self.running_time.get() / TIME_TO_REACH_RUNNING_EXHAUST);

            return SideEffectDeltasC {
                body_temp_bonus: crate::utils::lerp(0., MAX_BODY_TEMP_IMPACT, p),
                heart_rate_bonus: crate::utils::lerp(0., MAX_HEART_RATE_IMPACT, p),
                top_pressure_bonus: crate::utils::lerp(0., MAX_TOP_PRESSURE_IMPACT, p),
                bottom_pressure_bonus: crate::utils::lerp(0., MAX_BOTTOM_PRESSURE_IMPACT, p),
                stamina_bonus: -crate::utils::lerp(0., 100., p),

                ..Default::default()
            }
        } else {
            if self.running_state.get() == true {
                self.running_state.set(false);
                self.running_time.set(0.);
            }
        }

        Default::default() // No effects otherwise
    }
}