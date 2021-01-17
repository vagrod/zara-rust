use crate::health::side::builtin::FatigueSideEffects;
use crate::health::side::{SideEffectsMonitor, SideEffectDeltasC};
use crate::utils::FrameSummaryC;

use std::time::Duration;

impl FatigueSideEffects {
    pub fn new() -> Self {
        FatigueSideEffects {

        }
    }
}
impl SideEffectsMonitor for FatigueSideEffects {
    fn check(&self, frame_data: &FrameSummaryC) -> SideEffectDeltasC {
        const MAX_HOURS_UNTIL_FULLY_EXHAUSTED: f32 = 9.*60.*60.; // game seconds

        let sleep_time: Duration = frame_data.player.last_slept.to_duration();
        let elapsed = frame_data.game_time.to_duration() - sleep_time;
        let p = crate::utils::clamp_01(elapsed.as_secs_f32() / MAX_HOURS_UNTIL_FULLY_EXHAUSTED);

        SideEffectDeltasC {
            fatigue_bonus: crate::utils::lerp(0., 100., p),
            ..Default::default()
        }
    }
}