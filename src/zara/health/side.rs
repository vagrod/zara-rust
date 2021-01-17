use crate::utils::FrameSummaryC;

pub mod builtin;

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
    /// Delta that will be added to the healthy value (absolute delta)
    pub body_temp_bonus: f32,
    /// Delta that will be added to the healthy value (absolute delta)
    pub heart_rate_bonus: f32,
    /// Delta that will be added to the healthy value (absolute delta)
    pub top_pressure_bonus: f32,
    /// Delta that will be added to the healthy value (absolute delta)
    pub bottom_pressure_bonus: f32,
    /// Delta that will be added to the healthy value (absolute delta)
    pub water_level_bonus: f32,
    /// Delta relative to the current stamina value (relative delta)
    pub stamina_bonus: f32,
    /// Delta that will be added to the healthy value (absolute delta)
    pub fatigue_bonus: f32
}