use crate::error::MedicalAgentErr;
use crate::health::Health;
use crate::health::medagent::lerp::{MultiKeyedLerp, KeyFrame};

use std::collections::HashMap;
use std::cell::{Cell, RefCell};
use std::sync::Arc;
use crate::utils::GameTimeC;

mod lerp;

pub mod fluent;

/// Describes medical agent activation curve type
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum CurveType {
    Immediately,
    FasterInSecondHalf,
    Linearly
}

/// Macro for declaring medical agent items group
#[macro_export]
macro_rules! med_group(
    ($items:expr) => (
        MedicalAgentGroup::new($items)
    );
);

/// Macro for declaring a named medical agent
#[macro_export]
macro_rules! med_agent(
    ($nm:expr, $g:expr) => (
        MedicalAgent::new($nm, $g)
    );
);

impl Health {
    /// Registers new medical agent.
    ///
    /// ## Parameters
    /// - `agent`: medical agent to register. Use [`MedicalAgentBuilder`](crate::health::MedicalAgentBuilder)
    ///     to create one.
    ///
    /// ## Examples
    ///
    ///```
    /// use crate::zara::health::MedicalAgentBuilder;
    /// use crate::zara::health::medagent::CurveType;
    ///
    /// person.health.register_medical_agent(
    ///     MedicalAgentBuilder::start()
    ///         .for_agent("Agent Name")
    ///         .activates(CurveType::Immediately)
    ///         //.. and so on
    ///     // .build()
    ///  );
    ///```
    ///
    pub fn register_medical_agent(&self, agent: MedicalAgent) {
        self.medical_agents.agents.borrow_mut().insert(agent.name.to_string(), agent);
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
    pub fn contains(&self, item_name: &String) -> bool {
        self.items.contains(item_name)
    }
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

pub struct MedicalAgent {
    pub name: String,
    pub group: MedicalAgentGroup,
    pub activation_curve: CurveType,
    pub peak_time_minutes: f32,

    // Private fields
    percent_of_activity: Cell<f32>,
    percent_of_presence: Cell<f32>,
    is_active: Cell<bool>,
    doses: RefCell<HashMap<AgentDoseKey, AgentDose>>
}

impl MedicalAgent {
    pub fn new(name: String, activation_curve: CurveType, peak_time_minutes: f32, group: MedicalAgentGroup) -> Self {
        MedicalAgent {
            name: name.to_string(),
            activation_curve,
            peak_time_minutes,
            group,
            is_active: Cell::new(false),
            percent_of_activity: Cell::new(0.),
            percent_of_presence: Cell::new(0.),
            doses: RefCell::new(HashMap::new())
        }
    }

    pub fn on_consumed(&self, game_time: &GameTimeC, item_name: String) {
        self.add_dose_if_needed(game_time, item_name);
    }
    pub fn on_appliance_taken(&self, game_time: &GameTimeC, item_name: String) {
        self.add_dose_if_needed(game_time, item_name);
    }

    fn add_dose_if_needed(&self, game_time: &GameTimeC, item_name: String) {
        if self.group.contains(&item_name) {
            let gt = game_time.as_secs_f32();
            let duration_secs = self.peak_time_minutes*60.;

            let mut frames = MedicalAgent::generate_frames(gt, duration_secs, self.activation_curve);
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

            self.doses.borrow_mut().insert(key, dose);
        }
    }

    pub fn is_active(&self) -> bool { self.is_active.get() }
    pub fn percent_of_presence(&self) -> usize { self.percent_of_presence.get() as usize }
    pub fn percent_of_activity(&self) -> usize { self.percent_of_activity.get() as usize }

    pub fn generate_frames(gt: f32, duration_secs: f32, curve: CurveType) -> Vec<KeyFrame> {
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
            CurveType::FasterInSecondHalf => {
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
    pub agents: Arc<RefCell<HashMap<String, MedicalAgent>>>
}
impl MedicalAgentsMonitor {
    pub fn new() -> Self {
        MedicalAgentsMonitor {
            agents: Arc::new(RefCell::new(HashMap::new()))
        }
    }

    pub fn is_active(&self, agent_name: String) -> Result<bool, MedicalAgentErr> {
        match self.agents.borrow().get(&agent_name) {
            Some(agent) => Ok(agent.is_active()),
            None => Err(MedicalAgentErr::AgentNotFound)
        }
    }
}