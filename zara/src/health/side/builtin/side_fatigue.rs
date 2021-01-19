use crate::health::side::builtin::FatigueSideEffects;
use crate::health::side::{SideEffectsMonitor, SideEffectDeltasC};
use crate::utils::FrameSummaryC;

use std::time::Duration;
use std::cell::Cell;

impl FatigueSideEffects {
    pub fn new(hours_until_exhausted: usize) -> Self {
        FatigueSideEffects {
            hours_until_exhausted: Cell::new(hours_until_exhausted)
        }
    }
}
impl SideEffectsMonitor for FatigueSideEffects {
    fn check(&self, frame_data: &FrameSummaryC) -> SideEffectDeltasC {
        let max_hours_until_fully_exhausted: f32 = self.hours_until_exhausted.get() as f32; // game hours
        let max_hours_until_fully_exhausted_secs: f32 = max_hours_until_fully_exhausted *60.*60.; // game seconds

        let sleep_time: Duration = frame_data.player.last_slept.to_duration();
        let elapsed = frame_data.game_time.to_duration() - sleep_time;
        let p_added = crate::utils::clamp_01(elapsed.as_secs_f32() / max_hours_until_fully_exhausted_secs);
        let mut p_left = 1.; // if player haven't slept yet, no left fatigue

        if frame_data.player.last_slept_duration > 0.001
        {
            // He already slept
            p_left = crate::utils::clamp_01(frame_data.player.last_slept_duration as f32 / max_hours_until_fully_exhausted);
        }

        let left_fatigue = crate::utils::lerp(100., 0., p_left);
        let added_fatigue= crate::utils::lerp(0., 100., p_added);

        SideEffectDeltasC {
            fatigue_bonus: crate::utils::clamp(left_fatigue + added_fatigue, 0., 100.),
            ..Default::default()
        }
    }
}