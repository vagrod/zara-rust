use crate::health::medagent::{MedicalAgentsMonitor, MedicalAgent, CurveType, MedicalAgentGroup, AgentDose, AgentDoseKey};
use crate::utils::GameTimeC;
use crate::health::medagent::lerp::{MultiKeyedLerp, KeyFrame};

use std::time::Duration;
use std::fmt;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct MedicalAgentsMonitorStateContract {
    pub active_count: usize,
    pub agents: Vec<MedicalAgentStateContract>
}

#[derive(Clone, Debug, Default)]
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
impl fmt::Display for MedicalAgentStateContract {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Medagent {} state", self.name)
    }
}
impl Eq for MedicalAgentStateContract { }
impl PartialEq for MedicalAgentStateContract {
    fn eq(&self, other: &Self) -> bool {
        const EPS: f32 = 0.0001;

        self.name == other.name &&
        self.group == other.group &&
        self.activation_curve == other.activation_curve &&
        self.is_active == other.is_active &&
        self.last_dose_end_time == other.last_dose_end_time &&
        self.doses == other.doses &&
        f32::abs(self.duration_minutes - other.duration_minutes) < EPS &&
        f32::abs(self.percent_of_activity - other.percent_of_activity) < EPS &&
        f32::abs(self.percent_of_presence - other.percent_of_presence) < EPS
    }
}
impl Hash for MedicalAgentStateContract {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.group.hash(state);
        self.activation_curve.hash(state);
        self.is_active.hash(state);
        self.last_dose_end_time.hash(state);
        self.doses.hash(state);

        state.write_u32(self.duration_minutes as u32);
        state.write_u32(self.percent_of_activity as u32);
        state.write_u32(self.percent_of_presence as u32);
    }
}

#[derive(Clone, Debug, Default)]
pub struct AgentDoseStateContract {
    pub item: String,
    pub timestamp: i32,
    pub start_time: f32,
    pub end_time: f32,
    pub duration: f32,
    pub lerp: MultiKeyedLerpStateContract
}
impl fmt::Display for AgentDoseStateContract {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Agent dose state @{}", self.timestamp)
    }
}
impl Ord for AgentDoseStateContract {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.start_time < other.start_time {
            return Ordering::Less;
        }
        if self.start_time > other.start_time {
            return Ordering::Greater;
        }

        Ordering::Equal
    }
}
impl Eq for AgentDoseStateContract { }
impl PartialOrd for AgentDoseStateContract {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for AgentDoseStateContract {
    fn eq(&self, other: &Self) -> bool {
        const EPS: f32 = 0.0001;

        self.item == other.item &&
        self.timestamp == other.timestamp &&
        self.lerp == other.lerp &&
        f32::abs(self.start_time - other.start_time) < EPS &&
        f32::abs(self.end_time - other.end_time) < EPS &&
        f32::abs(self.duration - other.duration) < EPS
    }
}
impl Hash for AgentDoseStateContract {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.item.hash(state);
        self.timestamp.hash(state);
        self.lerp.hash(state);

        state.write_u32(self.start_time as u32);
        state.write_u32(self.end_time as u32);
        state.write_u32(self.duration as u32);
    }
}

#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Default)]
pub struct MultiKeyedLerpStateContract {
    keyframes: Vec<KeyFrameStateContract>
}

#[derive(Clone, Debug, Default)]
pub struct KeyFrameStateContract {
    pub time: f32,
    pub value: f32
}
impl fmt::Display for KeyFrameStateContract {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Keyframe @{}={}", self.time, self.value)
    }
}
impl Ord for KeyFrameStateContract {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.time < other.time {
            return Ordering::Less;
        }
        if self.time > other.time {
            return Ordering::Greater;
        }

        Ordering::Equal
    }
}
impl Eq for KeyFrameStateContract { }
impl PartialOrd for KeyFrameStateContract {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for KeyFrameStateContract {
    fn eq(&self, other: &Self) -> bool {
        const EPS: f32 = 0.0001;

        f32::abs(self.time - other.time) < EPS &&
        f32::abs(self.value - other.value) < EPS
    }
}
impl Hash for KeyFrameStateContract {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32(self.time as u32);
        state.write_u32(self.value as u32);
    }
}

#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Default)]
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