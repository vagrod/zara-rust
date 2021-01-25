use crate::utils::event::{Event, MessageQueue};
use crate::utils::{GameTimeC, HealthC};
use crate::health::disease::{DiseaseMonitor, ActiveDisease};
use crate::health::injury::ActiveInjury;
use crate::health::side::{SideEffectsMonitor};
use crate::inventory::items::{InventoryItem, ConsumableC, ApplianceC};
use crate::body::BodyParts;

use std::collections::{HashMap, BTreeMap};
use std::cell::{RefCell, Cell, RefMut};
use std::rc::Rc;
use std::sync::Arc;
use std::convert::TryFrom;

mod update;
mod status_methods;
mod monitors;

pub mod disease;
pub mod injury;
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
    /// All active or scheduled injuries
    pub injuries: Arc<RefCell<HashMap<String, Rc<ActiveInjury>>>>,

    /// Stores all registered disease monitors
    disease_monitors: Rc<RefCell<HashMap<usize, Box<dyn DiseaseMonitor>>>>,
    /// Stores all registered side effects monitors
    side_effects: Rc<RefCell<HashMap<usize, Box<dyn SideEffectsMonitor>>>>,
    /// Is character alive
    is_alive: Cell<bool>,

    /// Messages queued for sending on the next frame
    message_queue: RefCell<BTreeMap<usize, Event>>
}

/// Disease or injury stage level of seriousness
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum StageLevel {
    Undefined = -1,
    InitialStage = 1,
    Progressing = 2,
    Worrying = 3,
    Critical = 4
}
impl TryFrom<i32> for StageLevel {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == StageLevel::InitialStage as i32 => Ok(StageLevel::InitialStage),
            x if x == StageLevel::Progressing as i32 => Ok(StageLevel::Progressing),
            x if x == StageLevel::Worrying as i32 => Ok(StageLevel::Worrying),
            x if x == StageLevel::Critical as i32 => Ok(StageLevel::Critical),
            _ => Err(()),
        }
    }
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
            injuries: Arc::new(RefCell::new(HashMap::new())),
            stamina_regain_rate: Cell::new(0.1),
            blood_regain_rate: Cell::new(0.006),
            message_queue: RefCell::new(BTreeMap::new()),

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
    pub fn on_consumed(&self, game_time: &GameTimeC, item: &ConsumableC, inventory_items: &HashMap<String, Box<dyn InventoryItem>>){
        // Notify disease monitors
        for (_, monitor) in self.disease_monitors.borrow().iter() {
            monitor.on_consumed(self, game_time, item, inventory_items);
        }

        // Notify diseases
        for (_, disease) in self.diseases.borrow().iter() {
            if disease.get_is_active(game_time) {
                disease.on_consumed(game_time, item, inventory_items);
            }
        }
    }

    /// Called by zara controller when appliance item is taken
    pub fn on_appliance_taken(&self, game_time: &GameTimeC, item: &ApplianceC, body_part: BodyParts, inventory_items: &HashMap<String, Box<dyn InventoryItem>>){
        // Notify diseases
        for (_, disease) in self.diseases.borrow().iter() {
            if disease.get_is_active(game_time) {
                disease.on_appliance_taken(game_time, item, body_part, inventory_items);
            }
        }

        // Notify injuries
        for (_, injury) in self.injuries.borrow().iter() {
            if injury.get_is_active(game_time) {
                injury.on_appliance_taken(game_time, item, body_part, inventory_items);
            }
        }
    }
}

impl MessageQueue for Health {
    fn has_messages(&self) -> bool {
        self.message_queue.borrow().len() > 0
    }

    fn queue_message(&self, message: Event) {
        let mut q = self.message_queue.borrow_mut();
        let id = q.len();

        q.insert(id, message);
    }

    fn get_message_queue(&self) -> RefMut<'_, BTreeMap<usize, Event>> {
        self.message_queue.borrow_mut()
    }
}