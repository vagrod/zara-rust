use crate::utils::{FrameC, ConsumableC, GameTimeC, HealthC};
use crate::utils::event::{Listener};
use crate::health::disease::{Disease, DiseaseMonitor, ActiveDisease};
use crate::health::side::{SideEffectsMonitor, SideEffectDeltasC};

use std::collections::HashMap;
use std::cell::{RefCell, Cell};
use std::rc::Rc;

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

    /// This method is called every `UPDATE_INTERVAL` real seconds
    ///
    /// # Parameters
    /// - `frame`: summary information for this frame
    pub fn update<E: Listener + 'static>(&self, frame: &mut FrameC<E>){
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

        let mut snapshot = HealthC::healthy();

        // Apply monitors deltas
        self.apply_deltas(&mut snapshot, &side_effects_summary);

        // TODO: collect and apply disease effects

        // Apply the resulted health snapshot
        self.apply_health_snapshot(&snapshot);
    }

    fn apply_deltas(&self, snapshot: &mut HealthC, deltas: &SideEffectDeltasC){
        snapshot.body_temperature += deltas.body_temp_bonus;
        snapshot.heart_rate += deltas.heart_rate_bonus;
        snapshot.top_pressure += deltas.top_pressure_bonus;
        snapshot.bottom_pressure += deltas.bottom_pressure_bonus;
        snapshot.water_level += deltas.water_level_bonus;
        snapshot.stamina_level += deltas.stamina_bonus;
        snapshot.fatigue_level += deltas.fatigue_bonus;
    }

    fn apply_health_snapshot(&self, snapshot: &HealthC) {
        self.body_temperature.set(snapshot.body_temperature);
        self.heart_rate.set(snapshot.heart_rate);
        self.top_pressure.set(snapshot.top_pressure);
        self.bottom_pressure.set(snapshot.bottom_pressure);
        self.water_level.set(snapshot.water_level);
        self.stamina_level.set(snapshot.stamina_level);
        self.fatigue_level.set(snapshot.fatigue_level);
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

    /// Spawns a new disease. If disease is already scheduled or active, nothing will happen, and
    /// `false` will be returned
    ///
    /// # Parameters
    /// - `disease`: instance of an object with the [`Disease`](crate::health::disease::Disease) trait
    /// - `activation_time`: game time when this disease will start to be active. Use the
    ///     current game time to activate immediately (on the next `update` pass)
    ///
    /// # Returns
    /// `bool`: `true` on success.
    ///
    /// # Notes
    /// This method borrows the `diseases` collection
    pub fn spawn_disease(&self, disease: Box<dyn Disease>, activation_time: GameTimeC) -> bool {
        println!("Spawn disease call");

        let mut b = self.diseases.borrow_mut();
        let disease_name = disease.get_name();

        if b.contains_key(&disease_name) {
            return false;
        }

        b.insert(disease_name, Rc::new(ActiveDisease::new(
            disease,
            activation_time
        )));

        return true;
    }

    /// Removes active disease if exists. Returns `false` if not.
    ///
    /// # Parameters
    /// - `disease_name`: unique name of the disease
    ///
    /// # Returns
    /// `bool`: `true` on success
    ///
    /// # Notes
    /// This method borrows the `diseases` collection
    pub fn remove_disease(&self, disease_name: &String) -> bool {
        println!("Remove disease call {}", disease_name);

        let mut b = self.diseases.borrow_mut();

        if !b.contains_key(disease_name) {
            return false;
        }

        b.remove(disease_name);

        return true;
    }

}