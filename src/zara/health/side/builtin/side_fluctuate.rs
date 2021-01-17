use crate::health::side::{SideEffectsMonitor, SideEffectDeltasC};
use crate::utils::{FrameSummaryC};
use crate::health::side::builtin::DynamicVitalsSideEffect;

use std::cell::Cell;

/// DynamicVitalsSideEffect implementation

impl DynamicVitalsSideEffect {
    pub fn new() -> Self {
        DynamicVitalsSideEffect {
            first_iteration: Cell::new(true),
            counter: Cell::new(0.),
            half_duration: Cell::new(60. * 5.),
            direction: Cell::new(-1.),
            body_temperature_ceiling: Cell::new(0.0),
            heart_rate_ceiling: Cell::new(0.0),
            top_pressure_ceiling: Cell::new(0.0),
            bottom_pressure_ceiling: Cell::new(0.0)
        }
    }
}
impl SideEffectsMonitor for DynamicVitalsSideEffect {
    fn check(&self, frame_data: &FrameSummaryC) -> SideEffectDeltasC {
        let direction = self.direction.get();
        let mut is_new_cycle = false;
        let is_first = self.first_iteration.get();

        if !is_first {
            self.counter.set(self.counter.get() + frame_data.game_time_delta * direction);
        }

        let c= self.counter.get();
        let d = self.half_duration.get();

        if is_first {
            self.first_iteration.set(false);
            self.init_new_cycle();
        }

        let p = crate::utils::clamp_01(c / d);

        if c >= d {
            // Reached the top
            self.direction.set(direction * -1.);
        } else if c <= 0. {
            // Reached the bottom
            self.direction.set(direction * -1.);
            is_new_cycle = true;
        } else {
            if (0.3..0.7).contains(&p) && crate::utils::roll_dice(5) {
                self.direction.set(direction * -1.);
            }
        }

        let result = SideEffectDeltasC {
            body_temp_bonus: crate::utils::lerp(0., self.body_temperature_ceiling.get(), p),
            heart_rate_bonus: crate::utils::lerp(0., self.heart_rate_ceiling.get(), p),
            top_pressure_bonus: crate::utils::lerp(0., self.top_pressure_ceiling.get(), p),
            bottom_pressure_bonus: crate::utils::lerp(0., self.bottom_pressure_ceiling.get(), p),
            ..Default::default()
        };

        if is_new_cycle && !is_first {
            self.init_new_cycle();
        }

        return result;
    }
}

impl DynamicVitalsSideEffect {
    fn init_new_cycle(&self){
        const BODY_TEMP_CEILING_MAX: f32 = 0.35;
        const HEART_RATE_CEILING_MAX: f32 = 6.;
        const PRESSURE_TOP_CEILING_MAX: f32 = 3.;
        const PRESSURE_BOTTOM_CEILING_MAX: f32 = 7.;

        self.body_temperature_ceiling.set(
            crate::utils::range(BODY_TEMP_CEILING_MAX / 2., BODY_TEMP_CEILING_MAX));
        self.heart_rate_ceiling.set(
            crate::utils::range(HEART_RATE_CEILING_MAX / 2., HEART_RATE_CEILING_MAX));
        self.top_pressure_ceiling.set(
            crate::utils::range(PRESSURE_TOP_CEILING_MAX / 2., PRESSURE_TOP_CEILING_MAX));
        self.bottom_pressure_ceiling.set(
            crate::utils::range(PRESSURE_BOTTOM_CEILING_MAX / 2., PRESSURE_BOTTOM_CEILING_MAX));
    }
}