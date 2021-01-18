use crate::utils::{ConsumableC, GameTimeC, HealthC};
use crate::health::disease::{DiseaseMonitor, ActiveDisease};
use crate::health::side::{SideEffectsMonitor};

use std::collections::HashMap;
use std::cell::{RefCell, Cell};
use std::rc::Rc;
use std::sync::Arc;

mod update;
mod disease_crud;
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
    /// How fast stamina recovers (percents per game second)
    pub stamina_regain_rate: Cell<f32>,
    /// How fast blood recovers (percents per game second)
    pub blood_regain_rate: Cell<f32>,
    /// All active or scheduled diseases
    pub diseases: Arc<RefCell<HashMap<String, Rc<ActiveDisease>>>>,

    /// Stores all registered disease monitors
    disease_monitors: Rc<RefCell<HashMap<usize, Box<dyn DiseaseMonitor>>>>,
    /// Stores all registered side effects monitors
    side_effects: Rc<RefCell<HashMap<usize, Box<dyn SideEffectsMonitor>>>>
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
    ///
    /// # Returns
    /// `usize`: unique key of this registered instance
    pub fn register_disease_monitor(&self, monitor: Box<dyn DiseaseMonitor>) -> usize {
        let mut b = self.disease_monitors.borrow_mut();
        let key = b.keys().max().unwrap_or(&0) + 1;

        b.insert(key, monitor);

        return key;
    }

    /// Unregisters disease monitor
    ///
    /// # Parameters
    /// - `key`: unique key given as a result of a [`register_disease_monitor`] method.
    ///
    /// [`register_disease_monitor`]:#method.register_disease_monitor
    pub fn unregister_disease_monitor(&self, key: usize) -> bool {
        let mut b = self.disease_monitors.borrow_mut();

        if !b.contains_key(&key)
        {
            return false;
        }

        b.remove(&key);

        return true;
    }

    /// Registers new side effects monitor instance
    ///
    /// # Parameters
    /// - `monitor`: an instance of an object that implements
    ///
    /// # Returns
    /// `usize`: unique key of this registered instance
    /// [`SideEffectsMonitor`](crate::health::side::SideEffectsMonitor) trait
    pub fn register_side_effect_monitor(&self, monitor: Box<dyn SideEffectsMonitor>) -> usize {
        let mut b = self.side_effects.borrow_mut();
        let key = b.keys().max().unwrap_or(&0) + 1;

        b.insert(key, monitor);

        return key;
    }

    /// Unregisters side effects monitor
    ///
    /// # Parameters
    /// - `key`: unique key given as a result of a [`register_side_effect_monitor`] method.
    ///
    /// [`register_side_effect_monitor`]:#method.register_side_effect_monitor
    pub fn unregister_side_effect_monitor(&self, key: usize) -> bool {
        let mut b = self.side_effects.borrow_mut();

        if !b.contains_key(&key)
        {
            return false;
        }

        b.remove(&key);

        return true;
    }

    /// Called by zara controller when item is consumed
    /// as food or water
    pub fn on_item_consumed(&self, game_time: &GameTimeC, item: &ConsumableC){
        println!("consumed {0} (from health): is food {1}", item.name, item.is_food);

        // Notify disease monitors
        for (_, monitor) in self.disease_monitors.borrow().iter() {
            monitor.on_consumed(self, game_time, item);
        }
    }

}