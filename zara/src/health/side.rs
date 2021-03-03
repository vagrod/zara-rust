use crate::utils::FrameSummaryC;

use std::any::Any;
use std::fmt;
use std::hash::{Hash, Hasher};

pub mod builtin;

/// Trait that must be implemented by all side effects monitors
/// 
/// # Links
/// See [this wiki article](https://github.com/vagrod/zara-rust/wiki/Side-effects-Monitors) for more info.
pub trait SideEffectsMonitor {
    /// Being called once a `UPDATE_INTERVAL` real seconds.
    ///
    /// # Parameters
    /// - `frame_data`: summary containing all environmental data, game time, health snapshot and etc.
    ///
    /// # Returns
    /// [`SideEffectDeltasC`](crate::health::side::SideEffectDeltasC) structure containing deltas
    /// that will be added to the `healthy player state`, and NOT THE CURRENT health state
    /// 
    /// # Links
    /// See [this wiki article](https://github.com/vagrod/zara-rust/wiki/Side-effects-Monitors) for more info.
    fn check(&self, frame_data: &FrameSummaryC) -> SideEffectDeltasC;

    /// For downcasting
    fn as_any(&self) -> &dyn Any;
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
/// 
/// # Links
/// See [this wiki article](https://github.com/vagrod/zara-rust/wiki/Side-effects-Monitors) for more info.
#[derive(Copy, Clone, Debug, Default)]
pub struct SideEffectDeltasC {
    /// Delta that will be added to the healthy value (absolute delta)
    pub body_temp_bonus: f32,
    /// Delta that will be added to the healthy value (absolute delta)
    pub heart_rate_bonus: f32,
    /// Delta that will be added to the healthy value (absolute delta)
    pub top_pressure_bonus: f32,
    /// Delta that will be added to the healthy value (absolute delta)
    pub bottom_pressure_bonus: f32,
    /// Delta relative to the current food value (relative delta)
    pub food_level_bonus: f32,
    /// Delta relative to the current water value (relative delta)
    pub water_level_bonus: f32,
    /// Delta relative to the current stamina value (relative delta)
    pub stamina_bonus: f32,
    /// Delta that will be added to the healthy oxygen value (relative delta)
    pub oxygen_level_bonus: f32,
    /// Delta that will be added to the healthy fatigue value (absolute delta)
    pub fatigue_bonus: f32
}
impl fmt::Display for SideEffectDeltasC {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Side Effect deltas")
    }
}
impl Eq for SideEffectDeltasC { }
impl PartialEq for SideEffectDeltasC {
    fn eq(&self, other: &Self) -> bool {
        const EPS: f32 = 0.0001;

        f32::abs(self.body_temp_bonus - other.body_temp_bonus) < EPS &&
        f32::abs(self.heart_rate_bonus - other.heart_rate_bonus) < EPS &&
        f32::abs(self.top_pressure_bonus - other.top_pressure_bonus) < EPS &&
        f32::abs(self.bottom_pressure_bonus - other.bottom_pressure_bonus) < EPS &&
        f32::abs(self.food_level_bonus - other.food_level_bonus) < EPS &&
        f32::abs(self.water_level_bonus - other.water_level_bonus) < EPS &&
        f32::abs(self.stamina_bonus - other.stamina_bonus) < EPS &&
        f32::abs(self.oxygen_level_bonus - other.oxygen_level_bonus) < EPS &&
        f32::abs(self.fatigue_bonus - other.fatigue_bonus) < EPS
    }
}
impl Hash for SideEffectDeltasC {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_i32((self.body_temp_bonus*10_000_f32) as i32);
        state.write_i32((self.heart_rate_bonus*10_000_f32) as i32);
        state.write_i32((self.top_pressure_bonus*10_000_f32) as i32);
        state.write_i32((self.bottom_pressure_bonus*10_000_f32) as i32);
        state.write_i32((self.food_level_bonus*10_000_f32) as i32);
        state.write_i32((self.water_level_bonus*10_000_f32) as i32);
        state.write_i32((self.stamina_bonus*10_000_f32) as i32);
        state.write_i32((self.oxygen_level_bonus*10_000_f32) as i32);
        state.write_i32((self.fatigue_bonus*10_000_f32) as i32);
    }
}