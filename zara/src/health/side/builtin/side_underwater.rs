use crate::health::side::builtin::UnderwaterSideEffect;
use crate::health::side::{SideEffectsMonitor, SideEffectDeltasC};
use crate::utils::{FrameSummaryC, clamp_bottom};

use std::cell::Cell;

pub struct UnderwaterSideEffectStateContract {
    pub oxygen_drain_amount: f32,
    pub stamina_drain_amount: f32,
    pub sleeping_state: bool,
    pub gained_fatigue: f32,
    pub underwater_state: bool,
    pub time_under_water: f32
}

impl UnderwaterSideEffect {
    /// Creates new `UnderwaterSideEffect` instance.
    ///
    /// # Parameters
    /// - `oxygen_drain`: oxygen drain speed is under water, 0..100 percents per game second
    /// - `stamina_drain`: stamina drain speed is under water, 0..100 percents per game second
    pub fn new(oxygen_drain: f32, stamina_drain: f32) -> Self {
        UnderwaterSideEffect {
            oxygen_drain_amount: Cell::new(oxygen_drain),
            stamina_drain_amount: Cell::new(stamina_drain),
            gained_fatigue: Cell::new(0.),
            sleeping_state: Cell::new(false),
            time_under_water: Cell::new(0.),
            underwater_state: Cell::new(false)
        }
    }
    pub fn get_state(&self) -> UnderwaterSideEffectStateContract {
        UnderwaterSideEffectStateContract {
            oxygen_drain_amount: self.oxygen_drain_amount.get(),
            stamina_drain_amount: self.stamina_drain_amount.get(),
            sleeping_state: self.sleeping_state.get(),
            gained_fatigue: self.gained_fatigue.get(),
            underwater_state: self.underwater_state.get(),
            time_under_water: self.time_under_water.get()
        }
    }
    pub fn restore_state(&self, state: &UnderwaterSideEffectStateContract) {
        self.oxygen_drain_amount.set(state.oxygen_drain_amount);
        self.stamina_drain_amount.set(state.stamina_drain_amount);
        self.sleeping_state.set(state.sleeping_state);
        self.gained_fatigue.set(state.gained_fatigue);
        self.underwater_state.set(state.underwater_state);
        self.time_under_water.set(state.time_under_water);
    }
}

impl SideEffectsMonitor for UnderwaterSideEffect {
    fn check(&self, frame_data: &FrameSummaryC) -> SideEffectDeltasC {
        const TIME_TO_REACH_UNDERWATER_TOPS: f32 = 60. * 5.; // game seconds
        const TIME_TO_REACH_FATIGUE_CAP: f32 = 3.*60.*60.; // game seconds
        const MAX_HEART_RATE_IMPACT: f32 = 79.;
        const MAX_TOP_PRESSURE_IMPACT: f32 = 35.;
        const MAX_BOTTOM_PRESSURE_IMPACT: f32 = 29.;

        if !frame_data.player.is_sleeping && self.sleeping_state.get() {
            // Woke up
            self.sleeping_state.set(false);
            self.gained_fatigue.set(0.);
        }
        if frame_data.player.is_sleeping && !self.sleeping_state.get() {
            // Went to bed
            self.sleeping_state.set(true);
        }

        if frame_data.player.is_underwater && !self.underwater_state.get() {
            self.underwater_state.set(true);
            self.time_under_water.set(0.);
        }
        if !frame_data.player.is_underwater && self.underwater_state.get() {
            self.underwater_state.set(false);
        }

        if frame_data.player.is_underwater {
            let underwater_time = self.time_under_water.get() + frame_data.game_time_delta;
            let underwater_time_capped = crate::utils::clamp_to(
                underwater_time,
                TIME_TO_REACH_UNDERWATER_TOPS);

            self.time_under_water.set(underwater_time);

            let p = crate::utils::clamp_01(underwater_time_capped / TIME_TO_REACH_UNDERWATER_TOPS);
            let p_fatigue = crate::utils::clamp_01(underwater_time / TIME_TO_REACH_FATIGUE_CAP);

            self.gained_fatigue.set(crate::utils::lerp(0., 100., p_fatigue));

            return SideEffectDeltasC {
                oxygen_level_bonus: -self.oxygen_drain_amount.get() * frame_data.game_time_delta,
                stamina_bonus: -self.stamina_drain_amount.get() * frame_data.game_time_delta,
                top_pressure_bonus: crate::utils::lerp(0., MAX_TOP_PRESSURE_IMPACT, p),
                bottom_pressure_bonus: crate::utils::lerp(0., MAX_BOTTOM_PRESSURE_IMPACT, p),
                heart_rate_bonus: crate::utils::lerp(0., MAX_HEART_RATE_IMPACT, p),
                fatigue_bonus: self.gained_fatigue.get(),

                ..Default::default()
            }
        } else {
            // Lerp back
            let underwater_time = clamp_bottom(self.time_under_water.get() - frame_data.game_time_delta, 0.);

            const EPS: f32 = 0.000001;

            if underwater_time > EPS {
                self.time_under_water.set(underwater_time);

                let p = crate::utils::clamp_01(underwater_time / TIME_TO_REACH_UNDERWATER_TOPS);

                return SideEffectDeltasC {
                    top_pressure_bonus: crate::utils::lerp(0., MAX_TOP_PRESSURE_IMPACT, p),
                    bottom_pressure_bonus: crate::utils::lerp(0., MAX_BOTTOM_PRESSURE_IMPACT, p),
                    heart_rate_bonus: crate::utils::lerp(0., MAX_HEART_RATE_IMPACT, p),
                    fatigue_bonus: self.gained_fatigue.get(),

                    ..Default::default()
                }
            }
        }

        SideEffectDeltasC {
            fatigue_bonus: self.gained_fatigue.get(),

            ..Default::default()
        }
    }
}