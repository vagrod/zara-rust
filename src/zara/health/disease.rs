use super::super::health::{Health};
use super::super::utils::{SummaryC};
use crate::utils::ConsumableC;

/// Trait for disease monitors
pub trait DiseaseMonitor {
    /// Being called once a `UPDATE_INTERVAL` real seconds.
    ///
    /// # Parameters
    /// - `health`: health controller object. It can be used to call `spawn_disease` for example
    /// - `frame_data`: summary containing all environmental data, game time and etc.
    fn check(&self, health: &Health, frame_data: &SummaryC);

    /// Being called when player consumes food or water
    ///
    /// # Parameters
    /// - `health`: health controller object. It can be used to call `spawn_disease` for example
    /// - `item`: consumable item summary info
    fn on_consumed(&self, health: &Health, item: &ConsumableC);
}