use crate::health::side::builtin::{RunningSideEffects, RunningSideEffectsStateContract};
use crate::health::side::{SideEffectsMonitor, SideEffectDeltasC};
use crate::utils::{FrameSummaryC, clamp_bottom};

use std::cell::Cell;
use std::any::Any;

impl RunningSideEffects
{
    /// Creates new `RunningSideEffects` instance.
    ///
    /// # Parameters
    /// - `stamina_drain`: stamina drain when running, 0..100 percents per game second
    /// - `water_drain`: water level drain speed when running, 0..100 percents per game second
    /// 
    /// # Examples
    /// ```
    /// use zara::health::side:::buitin;
    /// let o = buitin::RunningSideEffects::new(0.22, 0.009);
    /// ```
    /// 
    /// # Links
    /// See [this wiki article](https://github.com/vagrod/zara-rust/wiki/Built-in-side-effects) for more info.
    pub fn new(stamina_drain: f32, water_drain: f32) -> Self {
        RunningSideEffects {
            running_state: Cell::new(false),
            sleeping_state: Cell::new(false),
            running_time: Cell::new(0.),
            gained_fatigue: Cell::new(0.),
            stamina_drain_amount: Cell::new(stamina_drain),
            water_drain_amount: Cell::new(water_drain)
        }
    }
    /// Returns a state snapshot contract for this `RunningSideEffects` instance
    /// 
    /// # Examples
    /// ```
    /// let state = monitor.get_state();
    /// ```
    /// 
    /// # Links
    /// See [this wiki article](https://github.com/vagrod/zara-rust/wiki/State-Management) for more info.
    pub fn get_state(&self) -> RunningSideEffectsStateContract {
        RunningSideEffectsStateContract {
            stamina_drain_amount: self.stamina_drain_amount.get(),
            water_drain_amount: self.water_drain_amount.get(),
            running_state: self.running_state.get(),
            sleeping_state: self.sleeping_state.get(),
            running_time: self.running_time.get(),
            gained_fatigue: self.gained_fatigue.get()
        }
    }
    /// Restores the state from the given state contract
    /// 
    /// # Parameters
    /// - `state`: captured earlier state
    /// 
    /// # Examples
    /// ```
    /// monitor.restore_state(state);
    /// ```
    /// 
    /// # Links
    /// See [this wiki article](https://github.com/vagrod/zara-rust/wiki/State-Management) for more info.
    pub fn restore_state(&self, state: &RunningSideEffectsStateContract) {
        self.stamina_drain_amount.set(state.stamina_drain_amount);
        self.water_drain_amount.set(state.water_drain_amount);
        self.running_state.set(state.running_state);
        self.sleeping_state.set(state.sleeping_state);
        self.running_time.set(state.running_time);
        self.gained_fatigue.set(state.gained_fatigue);
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
                stamina_bonus: -self.stamina_drain_amount.get() * frame_data.game_time_delta,
                water_level_bonus: -self.water_drain_amount.get() * frame_data.game_time_delta,
                fatigue_bonus: self.gained_fatigue.get(),

                ..Default::default()
            }
        } else {
            // Lerp back
            if self.running_state.get() == true {
                self.running_state.set(false);
            }

            let running_time = clamp_bottom(self.running_time.get() - frame_data.game_time_delta, 0.);

            const EPS: f32 = 0.000001;

            if running_time > EPS {
                self.running_time.set(running_time);

                let p = crate::utils::clamp_01(running_time / TIME_TO_REACH_RUNNING_EXHAUST);

                return SideEffectDeltasC {
                    body_temp_bonus: crate::utils::lerp(0., MAX_BODY_TEMP_IMPACT, p),
                    heart_rate_bonus: crate::utils::lerp(0., MAX_HEART_RATE_IMPACT, p),
                    top_pressure_bonus: crate::utils::lerp(0., MAX_TOP_PRESSURE_IMPACT, p),
                    bottom_pressure_bonus: crate::utils::lerp(0., MAX_BOTTOM_PRESSURE_IMPACT, p),
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

    fn as_any(&self) -> &dyn Any { self }
}