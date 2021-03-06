use crate::utils::event::{Event, MessageQueue};
use crate::utils::{GameTimeC, HealthC};
use crate::health::disease::{DiseaseMonitor, ActiveDisease};
use crate::health::injury::{ActiveInjury};
use crate::health::side::{SideEffectsMonitor};
use crate::health::medagent::{MedicalAgentsMonitor, CurveType};
use crate::health::medagent::fluent::{AgentStart};
use crate::inventory::items::{InventoryItem, ConsumableC, ApplianceC};
use crate::body::BodyPart;

use std::collections::{HashMap, BTreeMap};
use std::cell::{RefCell, Cell, RefMut};
use std::rc::Rc;
use std::sync::Arc;
use std::convert::TryFrom;
use std::fmt;
use std::hash::{Hash, Hasher};

mod update;
mod status_methods;
mod monitors;

pub(crate) mod state;

pub mod disease;
pub mod injury;
pub mod side;
pub mod medagent;

/// Node that describes and controls player's health. It contains
/// vitals data, active disease, active injuries, registered medical
/// agents, registered disease monitors and side effects controllers,
/// plus you can change here values that control various regain rates
pub struct Health {
    /// How fast stamina recovers (percents per game second)
    /// 
    /// # Links
    /// See [this wiki article](https://github.com/vagrod/zara-rust/wiki/Changing-regain-rates) for more info.
    pub stamina_regain_rate: Cell<f32>,
    /// How fast blood recovers (percents per game second)
    /// 
    /// # Links
    /// See [this wiki article](https://github.com/vagrod/zara-rust/wiki/Changing-regain-rates) for more info.
    pub blood_regain_rate: Cell<f32>,
    /// How fast oxygen recovers (percents per game second)
    /// 
    /// # Links
    /// See [this wiki article](https://github.com/vagrod/zara-rust/wiki/Changing-regain-rates) for more info.
    pub oxygen_regain_rate: Cell<f32>,
    /// All active or scheduled diseases
    pub diseases: Arc<RefCell<HashMap<String, Rc<ActiveDisease>>>>,
    /// All active or scheduled injuries
    pub injuries: Arc<RefCell<HashMap<InjuryKey, Rc<ActiveInjury>>>>,
    /// Registered medical agents
    /// 
    /// # Links
    /// See [this wiki article](https://github.com/vagrod/zara-rust/wiki/Medical-Agents) for more info.
    pub medical_agents: Arc<MedicalAgentsMonitor>,
    /// Stores all registered disease monitors.
    ///
    /// # Important
    /// Do not alter this collection manually. Use
    /// [`register_disease_monitor`] and [`unregister_disease_monitor`] methods instead.
    ///
    /// [`register_disease_monitor`]: #method.register_disease_monitor
    /// [`unregister_disease_monitor`]: #method.unregister_disease_monitor
    /// 
    /// # Links
    /// See [this wiki article](https://github.com/vagrod/zara-rust/wiki/Disease-Monitors) for more info.
    pub disease_monitors: Rc<RefCell<HashMap<usize, Box<dyn DiseaseMonitor>>>>,
    /// Stores all registered side effects monitors.
    ///
    /// # Important
    /// Do not alter this collection manually. Use
    /// [`register_side_effect_monitor`] and [`unregister_side_effect_monitor`] methods instead.
    ///
    /// [`register_side_effect_monitor`]: #method.register_side_effect_monitor
    /// [`unregister_side_effect_monitor`]: #method.unregister_side_effect_monitor
    /// 
    /// # Links
    /// See [this wiki article](https://github.com/vagrod/zara-rust/wiki/Side-effects-Monitors) for more info.
    pub side_effects: Rc<RefCell<HashMap<usize, Box<dyn SideEffectsMonitor>>>>,

    // Health state fields
    /// Body temperature (degrees C)
    body_temperature: Cell<f32>,
    /// Heart rate (bpm)
    heart_rate: Cell<f32>,
    /// Top body pressure (mmHg)
    top_pressure: Cell<f32>,
    /// Bottom body pressure (mmHg)
    bottom_pressure: Cell<f32>,
    /// Blood level (0..100)
    blood_level: Cell<f32>,
    /// Food level (0..100)
    food_level: Cell<f32>,
    /// Water level (0..100)
    water_level: Cell<f32>,
    /// Stamina level (0..100)
    stamina_level: Cell<f32>,
    /// Fatigue level (0..100)
    fatigue_level: Cell<f32>,
    /// Oxygen level (0..100)
    oxygen_level: Cell<f32>,
    /// Is character alive
    is_alive: Cell<bool>,
    /// Has any injury active blood loss
    has_blood_loss: Cell<bool>,

    /// Messages queued for sending on the next frame
    message_queue: RefCell<BTreeMap<usize, Event>>
}

/// Compound injury key that consists of a "injury name"-"body part" pair
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InjuryKey {
    /// Inventory item name (key)
    pub injury: String,
    /// Body part
    pub body_part: BodyPart
}
impl fmt::Display for InjuryKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} on {}", self.injury, self.body_part)
    }
}

/// Disease or injury stage level of seriousness
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum StageLevel {
    Undefined = -1,
    InitialStage = 1,
    Progressing = 2,
    Worrying = 3,
    Critical = 4
}
impl Default for StageLevel {
    fn default() -> Self { StageLevel::Undefined }
}
impl Hash for StageLevel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(*self as usize);
    }
}
impl fmt::Display for StageLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
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
/// Used to describe a new medical agent. Use `start` method to begin.
pub struct MedicalAgentBuilder {
    pub(crate) name: RefCell<String>,
    pub(crate) duration_minutes: Cell<f32>,
    pub(crate) curve_type: RefCell<CurveType>,
    pub(crate) items: RefCell<Vec<String>>
}
impl MedicalAgentBuilder {
    /// Starts building process for a new medical agent
    /// 
    /// # Examples
    /// ```
    /// use zara::health::medagent;
    ///
    /// let medagent = medagent::MedicalAgentBuilder::start()
    ///     .for_agent("Epinephrine")
    ///         .activates(medagent::CurveType::Immediately)
    ///         .and_lasts_for_minutes(23.)
    ///         .includes(
    ///             vec![
    ///                 "Adrenaline Pills",
    ///                 "Big Green Leaves",
    ///                 "Syringe With Epinephrine",
    ///                 // ... and so on
    ///             ]
    ///         )
    ///     .build();
    /// ```
    /// 
    /// # Links
    /// See [this wiki article](https://github.com/vagrod/zara-rust/wiki/Medical-Agents) for more info.
    pub fn start() -> Box<dyn AgentStart> {
        Box::new(MedicalAgentBuilder {
            name: RefCell::new(String::new()),
            curve_type: RefCell::new(CurveType::Linearly),
            duration_minutes: Cell::new(0.),
            items: RefCell::new(Vec::new())
        })
    }
}

impl Health {
    pub(crate) fn new() -> Self {
        let healthy = HealthC::healthy();

        Health {
            disease_monitors: Rc::new(RefCell::new(HashMap::new())),
            side_effects: Rc::new(RefCell::new(HashMap::new())),
            diseases: Arc::new(RefCell::new(HashMap::new())),
            injuries: Arc::new(RefCell::new(HashMap::new())),
            stamina_regain_rate: Cell::new(0.1),
            blood_regain_rate: Cell::new(0.006),
            oxygen_regain_rate: Cell::new(0.05),
            message_queue: RefCell::new(BTreeMap::new()),
            medical_agents: Arc::new(MedicalAgentsMonitor::new()),

            // Healthy values by default
            has_blood_loss: Cell::new(false),
            is_alive: Cell::new(true),
            blood_level: Cell::new(healthy.blood_level),
            body_temperature: Cell::new(healthy.body_temperature),
            top_pressure: Cell::new(healthy.top_pressure),
            bottom_pressure: Cell::new(healthy.bottom_pressure),
            food_level: Cell::new(healthy.food_level),
            oxygen_level: Cell::new(healthy.oxygen_level),
            water_level: Cell::new(healthy.water_level),
            heart_rate: Cell::new(healthy.heart_rate),
            stamina_level: Cell::new(healthy.stamina_level),
            fatigue_level: Cell::new(healthy.fatigue_level)
        }
    }

    /// Called by zara controller when item is consumed as food or water
    pub(crate) fn on_consumed(&self, game_time: &GameTimeC, item: &ConsumableC,
                       inventory_items: &HashMap<String, Box<dyn InventoryItem>>){
        // Affect water- and food levels
        self.food_level.set(crate::utils::clamp(self.food_level.get() + item.food_gain, 0., 100.));
        self.water_level.set(crate::utils::clamp(self.water_level.get() + item.water_gain, 0., 100.));

        // Notify disease monitors
        for (_, monitor) in self.disease_monitors.borrow().iter() {
            monitor.on_consumed(self, game_time, item, inventory_items);
        }

        // Notify diseases
        for (_, disease) in self.diseases.borrow().iter() {
            if disease.is_active(game_time) {
                disease.on_consumed(game_time, item, inventory_items);
            }
        }

        // Notify medical agents
        for (_, agent) in self.medical_agents.agents.borrow().iter() {
            agent.on_consumed(game_time, item.name.to_string())
        }
    }

    /// Called by zara controller when appliance item is taken
    pub(crate) fn on_appliance_taken(&self, game_time: &GameTimeC, item: &ApplianceC,
                                     body_part: BodyPart, inventory_items: &HashMap<String, Box<dyn InventoryItem>>){
        // Notify disease monitors
        for (_, monitor) in self.disease_monitors.borrow().iter() {
            monitor.on_appliance_taken(self, game_time, item, body_part, inventory_items);
        }

        // Notify diseases
        for (_, disease) in self.diseases.borrow().iter() {
            if disease.is_active(game_time) {
                disease.on_appliance_taken(game_time, item, body_part, inventory_items);
            }
        }

        // Notify injuries
        for (_, injury) in self.injuries.borrow().iter() {
            if injury.is_active(game_time) {
                injury.on_appliance_taken(game_time, item, body_part, inventory_items);
            }
        }

        // Notify medical agents
        for (_, agent) in self.medical_agents.agents.borrow().iter() {
            agent.on_appliance_taken(game_time, item.name.to_string())
        }
    }

    /// Sets controller alive state to `false`
    pub(crate) fn declare_dead(&self) { self.is_alive.set(false); }

    /// Removes all diseases.
    ///
    /// # Examples
    /// ```
    /// person.health.clear_diseases();
    /// ```
    /// 
    /// ## Notes
    /// Borrows `diseases` collection
    pub fn clear_diseases(&self) {
        self.diseases.borrow_mut().clear();
    }

    /// Removes all injuries.
    ///
    /// # Examples
    /// ```
    /// person.health.clear_injuries();
    /// ```
    /// 
    /// ## Notes
    /// Borrows `injuries` collection
    pub fn clear_injuries(&self) {
        self.injuries.borrow_mut().clear();
    }
}

impl MessageQueue for Health {
    fn has_messages(&self) -> bool { self.message_queue.borrow().len() > 0 }

    fn queue_message(&self, message: Event) {
        let mut q = self.message_queue.borrow_mut();
        let id = q.len();

        q.insert(id, message);
    }

    fn get_message_queue(&self) -> RefMut<'_, BTreeMap<usize, Event>> {
        self.message_queue.borrow_mut()
    }
}