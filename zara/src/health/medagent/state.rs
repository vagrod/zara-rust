use crate::health::medagent::{MedicalAgentsMonitor, MedicalAgent, CurveType, MedicalAgentGroup, AgentDose, AgentDoseKey};
use crate::utils::GameTimeC;
use crate::health::medagent::lerp::{MultiKeyedLerp, KeyFrame};

use std::time::Duration;

pub struct MedicalAgentsMonitorStateContract {
    pub active_count: usize,
    pub agents: Vec<MedicalAgentStateContract>
}

pub struct MedicalAgentStateContract {
    pub name: String,
    pub group: MedicalAgentGroupStateContract,
    pub activation_curve: CurveType,
    pub duration_minutes: f32,
    pub percent_of_activity: f32,
    pub percent_of_presence: f32,
    pub is_active: bool,
    pub last_dose_end_time: Option<Duration>,
    pub doses: Vec<AgentDoseStateContract>,
}

pub struct AgentDoseStateContract {
    pub item: String,
    pub timestamp: i32,
    pub start_time: f32,
    pub end_time: f32,
    pub duration: f32,
    pub lerp: MultiKeyedLerpStateContract
}

pub struct MultiKeyedLerpStateContract {
    keyframes: Vec<KeyFrameStateContract>
}

pub struct KeyFrameStateContract {
    pub time: f32,
    pub value: f32
}

pub struct MedicalAgentGroupStateContract {
    pub items: Vec<String>
}

impl MedicalAgentGroup {
    pub(crate) fn get_state(&self) -> MedicalAgentGroupStateContract {
        MedicalAgentGroupStateContract {
            items: self.items.iter().map(|x| x.to_string()).collect()
        }
    }
}

impl AgentDose {
    pub(crate) fn get_state(&self, key: &AgentDoseKey) -> AgentDoseStateContract {
        AgentDoseStateContract {
            item: key.item.to_string(),
            timestamp: key.timestamp,
            start_time: self.start_time,
            end_time: self.end_time,
            duration: self.duration,
            lerp: self.lerp.get_state()
        }
    }
}

impl MultiKeyedLerp {
    pub(crate) fn get_state(&self) -> MultiKeyedLerpStateContract {
        MultiKeyedLerpStateContract{
            keyframes: self.keyframes.iter().map(|x| x.get_state()).collect()
        }
    }
}

impl KeyFrame {
    pub(crate) fn get_state(&self) -> KeyFrameStateContract {
        KeyFrameStateContract{
            time: self.time,
            value: self.value
        }
    }
}

impl MedicalAgent {
    pub(crate) fn get_state(&self) -> MedicalAgentStateContract {
        MedicalAgentStateContract {
            doses: self.doses.borrow().iter().map(|(k, x)| x.get_state(k)).collect(),
            name : self.name.to_string(),
            is_active: self.is_active.get(),
            last_dose_end_time: match self.last_dose_end_time.borrow().as_ref() {
                Some(t) => Some(t.to_duration()),
                None => None
            },
            group: self.group.get_state(),
            percent_of_presence: self.percent_of_presence.get(),
            percent_of_activity: self.percent_of_activity.get(),
            activation_curve: self.activation_curve,
            duration_minutes: self.duration_minutes
        }
    }
    pub(crate) fn set_state(&self, state: &MedicalAgentStateContract) {
        self.is_active.set(state.is_active);
        self.percent_of_presence.set(state.percent_of_presence);
        self.percent_of_activity.set(state.percent_of_activity);
        self.last_dose_end_time.replace(match state.last_dose_end_time {
            Some(t) => Some(GameTimeC::from_duration(t)),
            None => None
        });

        let mut b = self.doses.borrow_mut();

        b.clear();

        for dose in &state.doses {
            b.insert(AgentDoseKey {
                item: dose.item.to_string(),
                timestamp: dose.timestamp
            }, AgentDose {
                duration: dose.duration,
                end_time: dose.end_time,
                start_time: dose.start_time,
                lerp: MultiKeyedLerp::new(dose.lerp.keyframes.iter().map(|x| KeyFrame::new(x.time, x.value)).collect())
            });
        }
    }
}

impl MedicalAgentsMonitor {
    pub(crate) fn get_state(&self) -> MedicalAgentsMonitorStateContract {
        MedicalAgentsMonitorStateContract {
            active_count: self.active_count.get(),
            agents: self.agents.borrow().iter().map(|(_, x)| x.get_state()).collect()
        }
    }
    pub(crate) fn set_state(&self, state: &MedicalAgentsMonitorStateContract) {
        self.active_count.set(state.active_count);

        let mut b = self.agents.borrow_mut();

        b.clear();

        for agent in &state.agents {
            let a = MedicalAgent::new(agent.name.to_string(), agent.activation_curve, agent.duration_minutes,
                                     MedicalAgentGroup::new(agent.group.items.clone()));
            a.set_state(&agent);
            b.insert(a.name.to_string(), a);
        }
    }
}