use super::super::health::{Health};
use super::super::utils::{FrameSummaryC, ConsumableC};
use std::rc::Rc;

/// Trait for disease monitors
pub trait DiseaseMonitor {
    /// Being called once a `UPDATE_INTERVAL` real seconds.
    ///
    /// # Parameters
    /// - `health`: health controller object. It can be used to call `spawn_disease` for example
    /// - `frame_data`: summary containing all environmental data, game time, health snapshot and etc.
    fn check(&self, health: &Health, frame_data: &FrameSummaryC);

    /// Being called when player consumes food or water
    ///
    /// # Parameters
    /// - `health`: health controller object. It can be used to call `spawn_disease` for example
    /// - `item`: consumable item summary info
    fn on_consumed(&self, health: &Health, item: &ConsumableC);
}

/// Trait that must be implemented by all diseases
pub trait Disease {
    fn get_name(&self) -> String;
}

/// Describes an active disease that can be also scheduled
pub struct ActiveDisease {
    /// Disease instance linked to this `ActiveDisease`
    pub disease: Rc<Box<dyn Disease>>
}
impl ActiveDisease {
    /// Creates new active disease
    pub fn new(disease: Box<dyn Disease>) -> Self {
        ActiveDisease {
            disease: Rc::new(disease)
        }
    }
}