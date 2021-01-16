use super::utils::{FrameC, ConsumableC};
use super::utils::event::{Listener};
use super::health::disease::{DiseaseMonitor};
use super::health::side::{SideEffectsMonitor};

use std::cell::{RefCell, Cell};
use std::rc::Rc;
use crate::health::side::SideEffectDeltasC;

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

    /// Stores all registered disease monitors
    monitors: Rc<RefCell<Vec<Box<dyn DiseaseMonitor>>>>,
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
        Health {
            monitors: Rc::new(RefCell::new(Vec::new())),
            side_effects: Rc::new(RefCell::new(Vec::new())),

            // Healthy values by default
            blood_level: Cell::new(100.),
            body_temperature: Cell::new(36.6),
            top_pressure: Cell::new(120.),
            bottom_pressure: Cell::new(70.),
            food_level: Cell::new(100.),
            water_level: Cell::new(100.),
            heart_rate: Cell::new(64.),
            stamina_level: Cell::new(100.),
            fatigue_level: Cell::new(0.)
        }
    }

    /// This method is called every `UPDATE_INTERVAL` real seconds
    ///
    /// # Parameters
    /// - `frame`: summary information for this frame
    pub fn update<E: Listener + 'static>(&self, frame: &mut FrameC<E>){
        println!("From health update: wind speed is {}", frame.data.environment.wind_speed);

        // Update disease monitors
        for monitor in self.monitors.borrow().iter() {
            monitor.check(self, &frame.data);
        }

        let mut side_effects_summary: SideEffectDeltasC = Default::default();

        // Collect side effects data
        for side_effect in self.side_effects.borrow().iter() {
            let res = side_effect.check(&frame.data);

            side_effects_summary.body_temp_bonus += res.body_temp_bonus;
            side_effects_summary.heart_rate_bonus += res.heart_rate_bonus;
            side_effects_summary.top_pressure_bonus += res.top_pressure_bonus;
            side_effects_summary.bottom_pressure_bonus += res.bottom_pressure_bonus;
            side_effects_summary.water_level_bonus += res.water_level_bonus;
            side_effects_summary.stamina_bonus += res.stamina_bonus;
            side_effects_summary.fatigue_bonus += res.fatigue_bonus;
        }

        // Apply monitors deltas
        self.apply_deltas(&side_effects_summary);
    }

    fn apply_deltas(&self, _deltas: &SideEffectDeltasC){

    }

    /// Registers new disease monitor instance
    ///
    /// # Parameters
    /// - `monitor`: an instance of an object that implements [`DiseaseMonitor`](crate::health::disease::DiseaseMonitor) trait
    pub fn register_disease_monitor(&self, monitor: Box<dyn DiseaseMonitor>){
        self.monitors.borrow_mut().insert(0, monitor);
    }

    /// Registers new side effects monitor instance
    ///
    /// # Parameters
    /// - `monitor`: an instance of an object that implements [`SideEffectsMonitor`](crate::health::side::SideEffectsMonitor) trait
    pub fn register_side_effect_monitor(&self, monitor: Box<dyn SideEffectsMonitor>){
        self.side_effects.borrow_mut().insert(0, monitor);
    }

    /// Called by zara controller when item is consumed
    /// as food or water
    pub fn on_item_consumed(&self, item: &ConsumableC){
        println!("consumed {0} (from health): is food {1}", item.name, item.is_food);

        // Notify disease monitors
        for monitor in self.monitors.borrow().iter() {
            monitor.on_consumed(self, item);
        }
    }

    /// Spawns a new disease. If disease is already scheduled or active, nothing will happen
    pub fn spawn_disease(&self){
        println!("Spawn disease call");
    }
}