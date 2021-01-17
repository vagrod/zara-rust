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
        const MAX_HOURS_UNTIL_FULLY_EXHAUSTED: f32 = 9.; // game hours
        const MAX_HOURS_UNTIL_FULLY_EXHAUSTED_SECS: f32 = MAX_HOURS_UNTIL_FULLY_EXHAUSTED*60.*60.; // game seconds

        let sleep_time: Duration = frame_data.player.last_slept.to_duration();
        let elapsed = frame_data.game_time.to_duration() - sleep_time;
        let p_added = crate::utils::clamp_01(elapsed.as_secs_f32() / MAX_HOURS_UNTIL_FULLY_EXHAUSTED_SECS);
        let mut p_left = 1.; // if player did not sleep yet, no left fatigue

        if frame_data.player.last_slept_duration > 0.001
        {
            // He already slept
            p_left = crate::utils::clamp_01(frame_data.player.last_slept_duration as f32 / MAX_HOURS_UNTIL_FULLY_EXHAUSTED);
        }

        let left_fatigue = crate::utils::lerp(100., 0., p_left);
        let added_fatigue= crate::utils::lerp(0., 100., p_added);

        SideEffectDeltasC {
            fatigue_bonus: crate::utils::clamp(left_fatigue + added_fatigue, 0., 100.),
            ..Default::default()
        }
    }
}