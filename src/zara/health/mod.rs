use crate::utils::{ConsumableC, GameTimeC, HealthC};
use crate::health::disease::{DiseaseMonitor, ActiveDisease};
use crate::health::side::{SideEffectsMonitor};

use std::collections::HashMap;
use std::cell::{RefCell, Cell};
use std::rc::Rc;

mod update;
mod disease_methods;
mod status_methods;

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
    /// All active or scheduled diseases
    pub diseases: Rc<RefCell<HashMap<String, Rc<ActiveDisease>>>>,

    /// Stores all registered disease monitors
    monitors: Rc<RefCell<Vec<Box<dyn DiseaseMonitor>>>>,
    /// Stores all registered side effects monitors
    side_effects: Rc<RefCell<Vec<Box<dyn SideEffectsMonitor>>>>
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
            monitors: Rc::new(RefCell::new(Vec::new())),
            side_effects: Rc::new(RefCell::new(Vec::new())),
            diseases: Rc::new(RefCell::new(HashMap::new())),

            // Healthy values by default
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

    /// Registers new disease monitor instance
    ///
    /// # Parameters
    /// - `monitor`: an instance of an object that implements
    /// [`DiseaseMonitor`](crate::health::disease::DiseaseMonitor) trait
    pub fn register_disease_monitor(&self, monitor: Box<dyn DiseaseMonitor>){
        self.monitors.borrow_mut().insert(0, monitor);
    }

    /// Registers new side effects monitor instance
    ///
    /// # Parameters
    /// - `monitor`: an instance of an object that implements
    /// [`SideEffectsMonitor`](crate::health::side::SideEffectsMonitor) trait
    pub fn register_side_effect_monitor(&self, monitor: Box<dyn SideEffectsMonitor>){
        self.side_effects.borrow_mut().insert(0, monitor);
    }

    /// Called by zara controller when item is consumed
    /// as food or water
    pub fn on_item_consumed(&self, game_time: &GameTimeC, item: &ConsumableC){
        println!("consumed {0} (from health): is food {1}", item.name, item.is_food);

        // Notify disease monitors
        for monitor in self.monitors.borrow().iter() {
            monitor.on_consumed(self, game_time, item);
        }
    }

}