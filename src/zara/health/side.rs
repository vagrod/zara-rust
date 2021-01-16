use super::super::utils::FrameSummaryC;
use std::cell::Cell;

/// Trait that must be implemented by all side effects monitors
pub trait SideEffectsMonitor {
    /// Being called once a `UPDATE_INTERVAL` real seconds.
    ///
    /// # Parameters
    /// - `frame_data`: summary containing all environmental data, game time, health snapshot and etc.
    ///
    /// # Returns
    /// [`SideEffectDeltasC`](crate::health::side::SideEffectDeltasC) structure containing deltas
    /// that will be added to the `healthy player state`, and NOT THE CURRENT health state
    fn check(&self, frame_data: &FrameSummaryC) -> SideEffectDeltasC;
}

/// Structure that contains result of a side effects monitor check for one frame.
/// All the values are `deltas` that will be `added` to the respective
/// health parameters of the player. If there are multiple side effects monitors,
/// their deltas will be combined.
///
/// # Examples
///
/// ```
/// use zara::health::side;
///
/// let result = side::SideEffectDeltasC { body_temp_bonus: 0.05, ..Default::default() };
/// ```
#[derive(Default)]
pub struct SideEffectDeltasC {
    pub body_temp_bonus: f32,
    pub heart_rate_bonus: f32,
    pub top_pressure_bonus: f32,
    pub bottom_pressure_bonus: f32,
    pub water_level_bonus: f32,
    pub stamina_bonus: f32,
    pub fatigue_bonus: f32
}

/// Side effects monitor that checks if player is running and increases his
/// heart rate, blood pressure, affects stamina, fatigue and water level
pub struct RunningSideEffects {
    running_state: Cell<bool>,
    running_time: Cell<f32> // game seconds
}

impl RunningSideEffects {
    pub fn new() -> Self {
        RunningSideEffects {
            running_state: Cell::new(false),
            running_time: Cell::new(0.),
        }
    }
}
impl SideEffectsMonitor for RunningSideEffects {
    fn check(&self, frame_data: &FrameSummaryC) -> SideEffectDeltasC {
        const TIME_TO_REACH_RUNNING_EXHAUST: f32 = 60. * 5.; // game seconds
        const MAX_HEART_RATE_IMPACT: f32 = 45.;
        const MAX_TOP_PRESSURE_IMPACT: f32 = 24.;
        const MAX_BOTTOM_PRESSURE_IMPACT: f32 = 16.;

        if frame_data.player.is_running {
            if self.running_state.get() == false {
                self.running_state.set(true);
                self.running_time.set(0.);
            }

            self.running_time.set(crate::utils::clamp_to(self.running_time.get() + frame_data.game_time_delta, TIME_TO_REACH_RUNNING_EXHAUST));

            let p = self.running_time.get() / TIME_TO_REACH_RUNNING_EXHAUST;

            return SideEffectDeltasC {
                body_temp_bonus: crate::utils::lerp(0., MAX_HEART_RATE_IMPACT, p),
                top_pressure_bonus: crate::utils::lerp(0., MAX_TOP_PRESSURE_IMPACT, p),
                bottom_pressure_bonus: crate::utils::lerp(0., MAX_BOTTOM_PRESSURE_IMPACT, p),

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