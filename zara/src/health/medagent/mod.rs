use crate::error::MedicalAgentErr;
use crate::health::Health;

use std::collections::HashMap;
use std::cell::{Cell, RefCell};
use std::sync::Arc;

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
    pub items: Vec<String>
}

impl MedicalAgentGroup {
    pub fn new(items: Vec<String>) -> Self {
        MedicalAgentGroup {
            items
        }
    }
}

pub struct MedicalAgent {
    pub name: String,
    pub group: MedicalAgentGroup,
    pub activation_curve: CurveType,
    pub peak_time_minutes: f32,

    // Private fields
    percent_of_activity: Cell<f32>,
    percent_of_presence: Cell<f32>,
    is_active: Cell<bool>
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
            percent_of_presence: Cell::new(0.)
        }
    }

    pub fn on_consumed(&self, item_name: String) {
        //println!("medagent on consumed");
    }
    pub fn on_appliance_taken(&self, item_name: String) {

    }

    pub fn is_active(&self) -> bool {
        self.is_active.get()
    }
    pub fn percent_of_presence(&self) -> usize {
        self.percent_of_presence.get() as usize
    }
    pub fn percent_of_activity(&self) -> usize {
        self.percent_of_activity.get() as usize
    }
    pub fn copy(&self) -> Self {
        let mut items = Vec::new();

        for item in &self.group.items {
            items.push(item.to_string());
        }

        MedicalAgent {
            name: self.name.to_string(),
            is_active: Cell::new(self.is_active()),
            activation_curve: self.activation_curve,
            peak_time_minutes: self.peak_time_minutes,
            percent_of_presence: Cell::new(self.percent_of_presence() as f32),
            percent_of_activity: Cell::new(self.percent_of_activity() as f32),
            group: MedicalAgentGroup {
                items
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