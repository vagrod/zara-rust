use crate::health::side::{SideEffectsMonitor, SideEffectDeltasC};
use crate::utils::{FrameSummaryC};
use crate::health::side::builtin::RunningSideEffects;

use std::cell::Cell;

/// RunningSideEffects implementation

impl RunningSideEffects {
    pub fn new() -> Self {
        RunningSideEffects {
            running_state: Cell::new(false),
            sleeping_state: Cell::new(false),
            running_time: Cell::new(0.),
            gained_fatigue: Cell::new(0.)
        }
    }
}
impl SideEffectsMonitor for RunningSideEffects {
    fn check(&self, frame_data: &FrameSummaryC) -> SideEffectDeltasC {
        const TIME_TO_REACH_RUNNING_EXHAUST: f32 = 60. * 5.; // game seconds
        const TIME_TO_REACH_FATIGUE_CAP: f32 = 3.*60.*60.; // game seconds
        const MAX_HEART_RATE_IMPACT: f32 = 45.;
        const MAX_BODY_TEMP_IMPACT: f32 = 0.3;
        const MAX_TOP_PRESSURE_IMPACT: f32 = 24.;
        const MAX_BOTTOM_PRESSURE_IMPACT: f32 = 16.;
        const STAMINA_DRAIN: f32 = 0.22; // percents per game second
        const WATER_DRAIN: f32 = 0.009; // percents per game second

        if !frame_data.player.is_sleeping && self.sleeping_state.get() {
            // Woke up
            self.sleeping_state.set(false);
            self.gained_fatigue.set(0.);
        }
        if frame_data.player.is_sleeping && !self.sleeping_state.get() {
            // Went to bed
            self.sleeping_state.set(true);
        }

        if frame_data.player.is_running {
            if self.running_state.get() == false {
                self.running_state.set(true);
                self.running_time.set(0.);
            }

            let running_time = self.running_time.get() + frame_data.game_time_delta;
            let running_time_capped = crate::utils::clamp_to(
                running_time,
                TIME_TO_REACH_RUNNING_EXHAUST);

            self.running_time.set(running_time);

            let p = crate::utils::clamp_01(running_time_capped / TIME_TO_REACH_RUNNING_EXHAUST);
            let p_fatigue = crate::utils::clamp_01(running_time / TIME_TO_REACH_FATIGUE_CAP);

            self.gained_fatigue.set(crate::utils::lerp(0., 100., p_fatigue));

            return SideEffectDeltasC {
                body_temp_bonus: crate::utils::lerp(0., MAX_BODY_TEMP_IMPACT, p),
                heart_rate_bonus: crate::utils::lerp(0., MAX_HEART_RATE_IMPACT, p),
                top_pressure_bonus: crate::utils::lerp(0., MAX_TOP_PRESSURE_IMPACT, p),
                bottom_pressure_bonus: crate::utils::lerp(0., MAX_BOTTOM_PRESSURE_IMPACT, p),
                stamina_bonus: -STAMINA_DRAIN * frame_data.game_time_delta,
                water_level_bonus: -WATER_DRAIN * frame_data.game_time_delta,
                fatigue_bonus: self.gained_fatigue.get(),

                ..Default::default()
            }
        } else {
            if self.running_state.get() == true {
                self.running_state.set(false);
                self.running_time.set(0.);
            }
        }

        SideEffectDeltasC {
            fatigue_bonus: self.gained_fatigue.get(),
            ..Default::default()
        }
    }
}