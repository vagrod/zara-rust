use crate::error::MedicalAgentErr;

use std::collections::HashMap;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

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

pub struct MedicalAgentGroup {
    pub items: Vec<String>
}

impl MedicalAgentGroup {
    pub fn new(items: Vec<&str>) -> Self {
        MedicalAgentGroup {
            items: items.into_iter().map(|x| x.to_string()).collect()
        }
    }
}

pub struct MedicalAgent {
    pub name: String,
    pub group: MedicalAgentGroup,

    // Private fields
    percent_of_activity: Cell<f32>,
    percent_of_presence: Cell<f32>,
    is_active: Cell<bool>
}

impl MedicalAgent {
    pub fn new(name: &str, group: MedicalAgentGroup) -> Self {
        MedicalAgent {
            name: name.to_string(),
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

    pub fn get_is_active(&self) -> bool {
        self.is_active.get()
    }
    pub fn get_percent_of_presence(&self) -> usize {
        self.percent_of_presence.get() as usize
    }
    pub fn get_percent_of_activity(&self) -> usize {
        self.percent_of_activity.get() as usize
    }
    pub fn copy(&self) -> Self {
        let mut items = Vec::new();

        for item in &self.group.items {
            items.push(item.to_string());
        }

        MedicalAgent {
            name: self.name.to_string(),
            is_active: Cell::new(self.get_is_active()),
            percent_of_presence: Cell::new(self.get_percent_of_presence() as f32),
            percent_of_activity: Cell::new(self.get_percent_of_activity() as f32),
            group: MedicalAgentGroup {
                items
            }
        }
    }
}

pub struct MedicalAgentsMonitor {
    pub agents: HashMap<String, MedicalAgent>
}
impl MedicalAgentsMonitor {
    pub fn new(agents: Vec<MedicalAgent>) -> Self {
        let mut result = HashMap::new();

        for agent in agents {
            let c = agent.copy();
            result.insert(agent.name, c);
        }

        MedicalAgentsMonitor {
            agents: result
        }
    }

    pub fn get_is_active(&self, agent_name: String) -> Result<bool, MedicalAgentErr> {
        match self.agents.get(&agent_name) {
            Some(agent) => Ok(agent.get_is_active()),
            None => Err(MedicalAgentErr::AgentNotFound)
        }
    }
}