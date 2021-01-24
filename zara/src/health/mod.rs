use crate::utils::{ConsumableC, GameTimeC, HealthC};
use crate::health::disease::{DiseaseMonitor, ActiveDisease};
use crate::health::side::{SideEffectsMonitor};

use std::collections::HashMap;
use std::cell::{RefCell, Cell};
use std::rc::Rc;
use std::sync::Arc;

mod update;
mod status_methods;
mod monitors;

pub mod disease;
pub mod side;

/// Describes and controls player's health
pub struct Health {
    // Health state fields
    /// Body temperature (degrees C)
    pub body_temperature: Cell<f32>,
    /// Heart rate (bpm)
    pub heart_rate: Cell<f32>,
    /// Top body pressure (mmHg)
    pub top_pressure: Cell<f32>,
    /// Bottom body pressure (mmHg)
    pub bottom_pressure: Cell<f32>,
    /// Blood level (0..100)
    pub blood_level: Cell<f32>,
    /// Food level (0..100)
    pub food_level: Cell<f32>,
    /// Water level (0..100)
    pub water_level: Cell<f32>,
    /// Stamina level (0..100)
    pub stamina_level: Cell<f32>,
    /// Fatigue level (0..100)
    pub fatigue_level: Cell<f32>,
    /// How fast stamina recovers (percents per game second)
    pub stamina_regain_rate: Cell<f32>,
    /// How fast blood recovers (percents per game second)
    pub blood_regain_rate: Cell<f32>,
    /// All active or scheduled diseases
    pub diseases: Arc<RefCell<HashMap<String, Rc<ActiveDisease>>>>,

    /// Stores all registered disease monitors
    disease_monitors: Rc<RefCell<HashMap<usize, Box<dyn DiseaseMonitor>>>>,
    /// Stores all registered side effects monitors
    side_effects: Rc<RefCell<HashMap<usize, Box<dyn SideEffectsMonitor>>>>,
    /// Is character alive
    is_alive: Cell<bool>
}

impl Health {
    /// Creates new ready-to-use `Health`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use zara::health;
    ///
    /// let h = health::Health::new();
    /// ```
    pub fn new() -> Self {
        let healthy = HealthC::healthy();

        Health {
            disease_monitors: Rc::new(RefCell::new(HashMap::new())),
            side_effects: Rc::new(RefCell::new(HashMap::new())),
            diseases: Arc::new(RefCell::new(HashMap::new())),
            stamina_regain_rate: Cell::new(0.1),
            blood_regain_rate: Cell::new(0.006),

            // Healthy values by default
            is_alive: Cell::new(true),
            blood_level: Cell::new(healthy.blood_level),
            body_temperature: Cell::new(healthy.body_temperature),
            top_pressure: Cell::new(healthy.top_pressure),
            bottom_pressure: Cell::new(healthy.bottom_pressure),
            food_level: Cell::new(healthy.food_level),
            water_level: Cell::new(healthy.water_level),
            heart_rate: Cell::new(healthy.heart_rate),
            stamina_level: Cell::new(healthy.stamina_level),
            fatigue_level: Cell::new(healthy.fatigue_level)
        }
    }

    /// Called by zara controller when item is consumed as food or water
    pub fn on_item_consumed(&self, game_time: &GameTimeC, item: &ConsumableC){
        // Notify disease monitors
        for (_, monitor) in self.disease_monitors.borrow().iter() {
            monitor.on_consumed(self, game_time, item);
        }

        // Notify diseases
        for (_, disease) in self.diseases.borrow().iter() {
            if disease.needs_treatment && disease.get_is_active(game_time) {
                disease.on_consumed(game_time, item);
            }
        }
    }

}