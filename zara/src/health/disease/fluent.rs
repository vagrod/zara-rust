use crate::health::disease::{StageDescription, StageBuilder, StageLevel};

use std::ops::Deref;

impl StageBuilder {
    fn as_stage_self_heal(&self) -> &dyn StageSelfHeal { self }
    fn as_vitals_node(&self) -> &dyn StageVitalsNode { self }
    fn as_vitals_values(&self) -> &dyn StageVitalsValues { self }
    fn as_stage_end(&self) -> &dyn StageEnd { self }
}

pub trait StageInit {
    fn build_for(&self, level: StageLevel) -> &dyn StageSelfHeal;
}

pub trait StageSelfHeal {
    fn self_heal(&self, hours: f32) -> &dyn StageVitalsNode;
    fn no_self_heal(&self) -> &dyn StageVitalsNode;
}

pub trait StageVitalsNode {
    fn vitals(&self) -> &dyn StageVitalsValues;
}

pub trait StageVitalsValues {
    fn with_target_body_temp(&self, value: f32) -> &dyn StageVitalsValues;
    fn with_target_heart_rate(&self, value: f32) -> &dyn StageVitalsValues;
    fn with_target_blood_pressure(&self, top: f32, bottom: f32) -> &dyn StageVitalsValues;

    fn will_reach_target_in(&self, hours: f32) -> &dyn StageEnd;
}

pub trait StageEnd {
    fn build(&self) -> StageDescription;
}



impl StageInit for StageBuilder {
    fn build_for(&self, level: StageLevel) -> &dyn StageSelfHeal {
        self.level.replace(level);

        self.as_stage_self_heal()
    }
}

impl StageSelfHeal for StageBuilder {
    fn self_heal(&self, hours: f32) -> &dyn StageVitalsNode {
        self.self_heal.replace(Some(hours));

        self.as_vitals_node()
    }

    fn no_self_heal(&self) -> &dyn StageVitalsNode {
        self.as_vitals_node()
    }
}

impl StageVitalsNode for StageBuilder {
    fn vitals(&self) -> &dyn StageVitalsValues {
        self.as_vitals_values()
    }
}

impl StageVitalsValues for StageBuilder {
    fn with_target_body_temp(&self, value: f32) -> &dyn StageVitalsValues {
        self.target_body_temp.set(value);

        self.as_vitals_values()
    }

    fn with_target_heart_rate(&self, value: f32) -> &dyn StageVitalsValues {
        self.target_heart_rate.set(value);

        self.as_vitals_values()
    }

    fn with_target_blood_pressure(&self, top: f32, bottom: f32) -> &dyn StageVitalsValues {
        self.target_pressure_top.set(top);
        self.target_pressure_bottom.set(bottom);

        self.as_vitals_values()
    }

    fn will_reach_target_in(&self, hours: f32) -> &dyn StageEnd {
        self.duration_hours.set(hours);

        self.as_stage_end()
    }
}

impl StageEnd for StageBuilder {
    fn build(&self) -> StageDescription {
        let mut self_heal = Option::None;

        if self.self_heal.borrow().deref().is_some() {
            self_heal = Option::Some(self.self_heal.borrow().deref().unwrap())
        }

        StageDescription {
            level: *self.level.borrow().deref(),
            self_heal,
            duration_hours: self.duration_hours.get(),
            target_body_temp: self.target_body_temp.get(),
            target_heart_rate: self.target_heart_rate.get(),
            target_pressure_top: self.target_pressure_top.get(),
            target_pressure_bottom: self.target_pressure_bottom.get(),
        }
    }
}