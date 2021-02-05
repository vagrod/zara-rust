use crate::error::MedicalAgentErr;
use crate::health::Health;
use crate::health::medagent::lerp::{MultiKeyedLerp, KeyFrame};
use crate::utils::GameTimeC;

use std::collections::HashMap;
use std::cell::{Cell, RefCell};
use std::sync::Arc;
use std::time::Duration;

mod lerp;

pub mod fluent;

/// Describes medical agent activation curve type
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum CurveType {
    Immediately,
    MostActiveInSecondHalf,
    Linearly
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
    ///
    pub fn register_medical_agents (&self, agents: Vec<MedicalAgent>) {
        let mut b = self.medical_agents.agents.borrow_mut();

        for agent in agents {
            b.insert(agent.name.to_string(), agent);
        }
    }
}

pub struct MedicalAgentGroup {
    items: Vec<String>
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

pub struct MedicalAgent {
    pub name: String,
    pub group: MedicalAgentGroup,
    pub activation_curve: CurveType,
    pub duration_minutes: f32,

    // Private fields
    percent_of_activity: Cell<f32>,
    percent_of_presence: Cell<f32>,
    is_active: Cell<bool>,
    last_dose_end_time: RefCell<Option<GameTimeC>>,
    doses: RefCell<HashMap<AgentDoseKey, AgentDose>>
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
            doses: RefCell::new(HashMap::new())
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
            match dose.lerp.evaluate(gt) {
                Some(value) => {
                    if max_percent_of_activity < value { max_percent_of_activity = value; }
                },
                _ => { }
            };
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
                item: item_name,
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
        }
    }

    pub fn is_active(&self) -> bool { self.is_active.get() }
    pub fn percent_of_presence(&self) -> usize { self.percent_of_presence.get() as usize }
    pub fn percent_of_activity(&self) -> usize { self.percent_of_activity.get() as usize }
    pub fn last_dose_end_time(&self) -> Option<GameTimeC> {
        match self.last_dose_end_time.borrow().as_ref() {
            Some(t) => Some(t.copy()),
            _ => None
        }
    }

    fn generate_frames(gt: f32, duration_secs: f32, curve: CurveType) -> Vec<KeyFrame> {
        return match curve {
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

pub struct MedicalAgentsMonitor {
    pub agents: Arc<RefCell<HashMap<String, MedicalAgent>>>,

    active_count: Cell<i32>
}
impl MedicalAgentsMonitor {
    pub fn new() -> Self {
        MedicalAgentsMonitor {
            agents: Arc::new(RefCell::new(HashMap::new())),
            active_count: Cell::new(0)
        }
    }

    pub fn is_active(&self, agent_name: String) -> Result<bool, MedicalAgentErr> {
        match self.agents.borrow().get(&agent_name) {
            Some(agent) => Ok(agent.is_active()),
            None => Err(MedicalAgentErr::AgentNotFound)
        }
    }

    pub(crate) fn update(&self, game_time: &GameTimeC) {
        let mut active_count = 0;
        for (_, agent) in self.agents.borrow().iter() {
            let result = agent.update(game_time);

            if result.is_active { active_count += 1; }
        }
        self.active_count.set(active_count);
    }

    pub fn active_count(&self) -> i32 { self.active_count.get() }
}