use crate::health::Health;
use crate::health::medagent::state::MedicalAgentsMonitorStateContract;

use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Default)]
pub struct HealthStateContract {
    pub stamina_regain_rate: f32,
    pub blood_regain_rate: f32,
    pub oxygen_regain_rate: f32,
    pub medical_agents: MedicalAgentsMonitorStateContract,

    pub body_temperature: f32,
    pub heart_rate: f32,
    pub top_pressure: f32,
    pub bottom_pressure: f32,
    pub blood_level: f32,
    pub food_level: f32,
    pub water_level: f32,
    pub stamina_level: f32,
    pub fatigue_level: f32,
    pub oxygen_level: f32,
    pub is_alive: bool,
    pub has_blood_loss: bool
}
impl fmt::Display for HealthStateContract {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Health state")
    }
}
impl Eq for HealthStateContract { }
impl PartialEq for HealthStateContract {
    fn eq(&self, other: &Self) -> bool {
        const EPS: f32 = 0.0001;

        self.medical_agents == other.medical_agents &&
        self.is_alive == other.is_alive &&
        self.has_blood_loss == other.has_blood_loss &&
        f32::abs(self.stamina_regain_rate - other.stamina_regain_rate) < EPS &&
        f32::abs(self.blood_regain_rate - other.blood_regain_rate) < EPS &&
        f32::abs(self.oxygen_regain_rate - other.oxygen_regain_rate) < EPS &&
        f32::abs(self.body_temperature - other.body_temperature) < EPS &&
        f32::abs(self.heart_rate - other.heart_rate) < EPS &&
        f32::abs(self.top_pressure - other.top_pressure) < EPS &&
        f32::abs(self.bottom_pressure - other.bottom_pressure) < EPS &&
        f32::abs(self.blood_level - other.blood_level) < EPS &&
        f32::abs(self.food_level - other.food_level) < EPS &&
        f32::abs(self.water_level - other.water_level) < EPS &&
        f32::abs(self.stamina_level - other.stamina_level) < EPS &&
        f32::abs(self.fatigue_level - other.fatigue_level) < EPS &&
        f32::abs(self.oxygen_level - other.oxygen_level) < EPS
    }
}
impl Hash for HealthStateContract {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.medical_agents.hash(state);
        self.is_alive.hash(state);
        self.has_blood_loss.hash(state);

        state.write_u32(self.stamina_regain_rate as u32);
        state.write_u32(self.blood_regain_rate as u32);
        state.write_u32(self.oxygen_regain_rate as u32);
        state.write_u32(self.body_temperature as u32);
        state.write_u32(self.heart_rate as u32);
        state.write_u32(self.top_pressure as u32);
        state.write_u32(self.bottom_pressure as u32);
        state.write_u32(self.blood_level as u32);
        state.write_u32(self.food_level as u32);
        state.write_u32(self.water_level as u32);
        state.write_u32(self.stamina_level as u32);
        state.write_u32(self.fatigue_level as u32);
        state.write_u32(self.oxygen_level as u32);
    }
}

impl Health {
    pub(crate) fn get_state(&self) -> HealthStateContract {
        HealthStateContract {
            medical_agents: self.medical_agents.get_state(),

            stamina_regain_rate: self.stamina_regain_rate.get(),
            blood_regain_rate: self.blood_regain_rate.get(),
            oxygen_regain_rate: self.oxygen_regain_rate.get(),

            body_temperature: self.body_temperature.get(),
            heart_rate: self.heart_rate.get(),
            top_pressure: self.top_pressure.get(),
            bottom_pressure: self.bottom_pressure.get(),
            blood_level: self.blood_level.get(),
            food_level: self.food_level.get(),
            water_level: self.water_level.get(),
            stamina_level: self.stamina_level.get(),
            fatigue_level: self.fatigue_level.get(),
            oxygen_level: self.oxygen_level.get(),
            is_alive:  self.is_alive.get(),
            has_blood_loss: self.has_blood_loss.get()
        }
    }

    pub(crate) fn restore_state(&self, state: &HealthStateContract) {
        self.stamina_regain_rate.set(state.stamina_regain_rate);
        self.blood_regain_rate.set(state.blood_regain_rate);
        self.oxygen_regain_rate.set(state.oxygen_regain_rate);
        self.body_temperature.set(state.body_temperature);
        self.heart_rate.set(state.heart_rate);
        self.top_pressure.set(state.top_pressure);
        self.bottom_pressure.set(state.bottom_pressure);
        self.blood_level.set(state.blood_level);
        self.food_level.set(state.food_level);
        self.water_level.set(state.water_level);
        self.stamina_level.set(state.stamina_level);
        self.fatigue_level.set(state.fatigue_level);
        self.oxygen_level.set(state.oxygen_level);
        self.is_alive.set(state.is_alive);
        self.has_blood_loss.set(state.has_blood_loss);
        self.medical_agents.set_state(&state.medical_agents);
    }
}