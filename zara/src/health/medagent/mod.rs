use crate::error::MedicalAgentErr;
use crate::health::Health;
use crate::health::medagent::lerp::{MultiKeyedLerp, KeyFrame};
use crate::utils::GameTimeC;
use crate::utils::event::{Event, MessageQueue};

use std::collections::{HashMap, BTreeMap};
use std::cell::{Cell, RefCell, RefMut};
use std::sync::Arc;
use std::time::Duration;
use std::fmt;
use std::cmp::Ordering;
use std::hash::{Hasher, Hash};

mod lerp;

pub(crate) mod state;
pub mod fluent;

/// Describes medical agent activation curve type
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum CurveType {
    /// Will activate fully in a first third
    Immediately,
    /// Will activate fully in a second half of the time
    MostActiveInSecondHalf,
    /// Will be activating linearly
    Linearly
}
impl fmt::Display for CurveType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Default for CurveType {
    fn default() -> Self {
        CurveType::Linearly
    }
}

impl Health {
    /// Registers new medical agent.
    ///
    /// # Parameters
    /// - `agents`: medical agents to register. Use [`MedicalAgentBuilder`](crate::zara::health::MedicalAgentBuilder)
    ///     to create one.
    ///
    /// # Examples
    ///
    ///```
    /// use crate::zara::health::MedicalAgentBuilder;
    /// use crate::zara::health::medagent::CurveType;
    ///
    /// person.health.register_medical_agents(
    ///     vec![
    ///         MedicalAgentBuilder::start()
    ///             .for_agent("Agent Name")
    ///                 .activates(CurveType::Immediately)
    ///                 //.. and so on
    ///         // .build()
    ///     ]
    ///  );
    ///```
    pub fn register_medical_agents (&self, agents: Vec<MedicalAgent>) {
        let mut b = self.medical_agents.agents.borrow_mut();

        for agent in agents {
            b.insert(agent.name.to_string(), agent);
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Default)]
pub struct MedicalAgentGroup {
    items: Vec<String>
}
impl fmt::Display for MedicalAgentGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Medagent group: {} items", self.items.len())
    }
}
impl MedicalAgentGroup {
    pub fn new(items: Vec<String>) -> Self {
        MedicalAgentGroup {
            items
        }
    }
    pub fn contains(&self, item_name: &String) -> bool { self.items.contains(item_name) }
}

#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
struct AgentDoseKey {
    pub item: String,
    pub timestamp: i32
}

#[derive(Default, Debug, Clone)]
struct AgentDose {
    pub lerp: MultiKeyedLerp,
    pub start_time: f32,
    pub end_time: f32,
    pub duration: f32
}

struct AgentUpdateResult {
    is_active: bool
}

impl AgentUpdateResult {
    pub fn empty() -> Self {
        AgentUpdateResult {
            is_active: false
        }
    }
}

/// Describes medical agent
#[derive(Clone, Debug, Default)]
pub struct MedicalAgent {
    /// Unique name of a medical agent
    pub name: String,
    /// Group of items associated with this agent
    pub group: MedicalAgentGroup,
    /// Type of activation curve
    pub activation_curve: CurveType,
    /// Duration of a single dose, in game minutes
    pub duration_minutes: f32,

    // Private fields
    percent_of_activity: Cell<f32>,
    percent_of_presence: Cell<f32>,
    is_active: Cell<bool>,
    last_dose_end_time: RefCell<Option<GameTimeC>>,
    doses: RefCell<HashMap<AgentDoseKey, AgentDose>>,

    /// Messages queued for sending on the next frame
    message_queue: RefCell<BTreeMap<usize, Event>>
}
impl fmt::Display for MedicalAgent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.group)
    }
}
impl Ord for MedicalAgent {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.duration_minutes < other.duration_minutes{
            return Ordering::Less;
        }
        if self.duration_minutes > other.duration_minutes{
            return Ordering::Greater;
        }

        Ordering::Equal
    }
}
impl Eq for MedicalAgent { }
impl PartialOrd for MedicalAgent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for MedicalAgent {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name &&
        self.activation_curve == other.activation_curve &&
        self.duration_minutes == other.duration_minutes &&
        self.group == other.group
    }
}
impl Hash for MedicalAgent {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.activation_curve.hash(state);
        self.group.hash(state);

        state.write_u32((self.duration_minutes*10_000_f32) as u32);
    }
}
impl MedicalAgent {
    pub fn new(name: String, activation_curve: CurveType, duration_minutes: f32, group: MedicalAgentGroup) -> Self {
        MedicalAgent {
            name: name.to_string(),
            activation_curve,
            duration_minutes,
            group,
            is_active: Cell::new(false),
            percent_of_activity: Cell::new(0.),
            percent_of_presence: Cell::new(0.),
            last_dose_end_time: RefCell::new(None),
            doses: RefCell::new(HashMap::new()),
            message_queue: RefCell::new(BTreeMap::new()),
        }
    }

    pub(crate) fn on_consumed(&self, game_time: &GameTimeC, item_name: String) {
        self.add_dose_if_needed(game_time, item_name);
    }
    pub(crate) fn on_appliance_taken(&self, game_time: &GameTimeC, item_name: String) {
        self.add_dose_if_needed(game_time, item_name);
    }

    fn update(&self, game_time: &GameTimeC) -> AgentUpdateResult {
        let mut doses_to_remove = Vec::new();
        let gt = game_time.as_secs_f32();
        {
            let doses = self.doses.borrow();
            if doses.len() == 0 { return AgentUpdateResult::empty(); }

            for (key, dose) in doses.iter() {
                if dose.end_time < gt { doses_to_remove.push(key.clone()); }
            }
        }

        // Clean up old stuff
        let mut doses = self.doses.borrow_mut();
        for key in doses_to_remove {
            doses.remove(&key);
        }

        let mut max_percent_of_activity = 0.;
        let mut start_time = f32::MAX;
        let mut end_time = 0.;
        for (_, dose) in doses.iter() {
            if start_time > dose.start_time { start_time = dose.start_time; }
            if dose.end_time > end_time { end_time = dose.end_time; }
            if let Some(value) = dose.lerp.evaluate(gt) {
                if max_percent_of_activity < value { max_percent_of_activity = value; }
            }
        }
        // Sanity check
        let is_active;
        if end_time >= start_time && (gt >= start_time && gt <= end_time) {
            let duration = end_time - start_time;
            let elapsed = gt - start_time;

            is_active = true;
            self.percent_of_presence.set((elapsed / duration) * 100.);
        } else {
            is_active = false;
            self.percent_of_presence.set(0.);
            self.last_dose_end_time.replace(None);
        }

        if !self.is_active.get() && is_active {
            self.queue_message(Event::MedicalAgentActivated(self.name.to_string()));
        }
        if self.is_active.get() && !is_active {
            self.queue_message(Event::MedicalAgentDeactivated(self.name.to_string()));
        }

        self.is_active.set(is_active);
        self.percent_of_activity.set(max_percent_of_activity);

        AgentUpdateResult {
            is_active
        }
    }

    fn add_dose_if_needed(&self, game_time: &GameTimeC, item_name: String) {
        if self.group.contains(&item_name) {
            let gt = game_time.as_secs_f32();
            let duration_secs = self.duration_minutes*60.;

            let frames = MedicalAgent::generate_frames(gt, duration_secs, self.activation_curve);
            let key = AgentDoseKey {
                item: item_name.to_string(),
                timestamp: gt as i32
            };
            let dose = AgentDose {
                start_time: gt,
                end_time: gt + duration_secs,
                duration: duration_secs,
                lerp: MultiKeyedLerp::new(frames)
            };

            self.last_dose_end_time.replace(Some(
                GameTimeC::from_duration(Duration::from_secs_f32(dose.end_time))
            ));
            self.doses.borrow_mut().insert(key, dose);
            self.queue_message(Event::MedicalAgentDoseReceived(self.name.to_string(), item_name.to_string()));
        }
    }

    /// Tells if this medical agent is active now
    pub fn is_active(&self) -> bool { self.is_active.get() }
    /// Returns medical agent percent of presence in blood (0..100%)
    pub fn percent_of_presence(&self) -> usize { self.percent_of_presence.get() as usize }
    /// Returns medical agent percent of overall activity (0..100%)
    pub fn percent_of_activity(&self) -> usize { self.percent_of_activity.get() as usize }
    /// Returns time when the last dose for this agent was taken
    pub fn last_dose_end_time(&self) -> Option<GameTimeC> {
        match self.last_dose_end_time.borrow().as_ref() {
            Some(t) => Some(t.clone()),
            _ => None
        }
    }

    fn generate_frames(gt: f32, duration_secs: f32, curve: CurveType) -> Vec<KeyFrame> {
        match curve {
            CurveType::Linearly => {
                vec![
                    KeyFrame::new(gt, 0.),
                    KeyFrame::new(gt + duration_secs * 0.5, 100.),
                    KeyFrame::new(gt + duration_secs, 0.)
                ]
            },
            CurveType::Immediately => {
                vec![
                    KeyFrame::new(gt, 0.),
                    KeyFrame::new(gt + duration_secs * 0.25, 100.),
                    KeyFrame::new(gt + duration_secs * 0.85, 100.),
                    KeyFrame::new(gt + duration_secs, 0.),
                ]
            },
            CurveType::MostActiveInSecondHalf => {
                vec![
                    KeyFrame::new(gt, 0.),
                    KeyFrame::new(gt + duration_secs * 0.3, 15.),
                    KeyFrame::new(gt + duration_secs * 0.5, 15.),
                    KeyFrame::new(gt + duration_secs * 0.65, 100.),
                    KeyFrame::new(gt + duration_secs * 0.9, 100.),
                    KeyFrame::new(gt + duration_secs, 0.),
                ]
            }
        }
    }
}

/// Node that controls all the medical agents
pub struct MedicalAgentsMonitor {
    /// All registered medical agents
    ///
    /// # Important
    /// Do not alter this collection manually. Use
    /// [`register_medical_agents`] method instead.
    ///
    /// [`register_medical_agents`]: #method.register_medical_agents
    pub agents: Arc<RefCell<HashMap<String, MedicalAgent>>>,

    active_count: Cell<usize>,

    /// Messages queued for sending on the next frame
    message_queue: RefCell<BTreeMap<usize, Event>>
}
impl fmt::Display for MedicalAgentsMonitor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Agents: {}, active: {}", self.agents.borrow().len(), self.active_count.get())
    }
}
impl MedicalAgentsMonitor {
    pub fn new() -> Self {
        MedicalAgentsMonitor {
            agents: Arc::new(RefCell::new(HashMap::new())),
            active_count: Cell::new(0),
            message_queue: RefCell::new(BTreeMap::new())
        }
    }

    /// Checks if a given medical agent is active now.
    ///
    /// # Parameters
    /// - `agent_name` unique medical agent name
    pub fn is_active(&self, agent_name: &String) -> Result<bool, MedicalAgentErr> {
        match self.agents.borrow().get(agent_name) {
            Some(agent) => Ok(agent.is_active()),
            None => Err(MedicalAgentErr::AgentNotFound)
        }
    }

    pub(crate) fn update(&self, game_time: &GameTimeC) {
        let mut active_count = 0;
        for (_, agent) in self.agents.borrow().iter() {
            let result = agent.update(game_time);

            if agent.has_messages() {
                self.flush_queue(agent.get_message_queue());
            }

            if result.is_active { active_count += 1; }
        }
        self.active_count.set(active_count);
    }

    /// Returns number of active medical agents
    pub fn active_count(&self) -> usize { self.active_count.get() }

    fn flush_queue(&self, mut q: RefMut<BTreeMap<usize, Event>>) {
        if q.len() == 0 { return }

        let mut key = 0;

        loop {
            match q.get(&key) {
                Some(event) => {
                    self.queue_message(event.clone());

                    q.remove(&key);
                },
                None => break
            }

            key += 1;
        }
    }
}

impl MessageQueue for MedicalAgent {
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
impl MessageQueue for MedicalAgentsMonitor {
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