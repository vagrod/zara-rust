use crate::health::Health;
use crate::health::medagent::state::MedicalAgentsMonitorStateContract;

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

    pub(crate) fn restore_state(&self, state: HealthStateContract) {
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
        self.medical_agents.set_state(state.medical_agents);
    }

}