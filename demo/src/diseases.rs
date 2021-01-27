use zara::health::{StageLevel};
use zara::body::{BodyParts};
use zara::health::disease::{StageBuilder, DiseaseTreatment, ActiveStage, ActiveDisease};
use zara::inventory::items::{InventoryItem, ConsumableC, ApplianceC};
use zara::utils::{GameTimeC};

use std::collections::HashMap;

pub struct Flu;
zara::disease!(Flu, "Flu", Some(Box::new(Flu)),
    vec![
        StageBuilder::start()
            .build_for(StageLevel::InitialStage)
                .no_self_heal()
                .vitals()
                    .with_target_body_temp(37.6)
                    .with_target_heart_rate(85.)
                    .with_target_blood_pressure(130., 90.)
                    .will_reach_target_in(0.1) // 6min
                    .will_end()
                .drains()
                    .stamina(0.2)
                    .food_level(0.05)
                    .water_level(0.1)
                .affects_fatigue(5.)
                .with_chance_of_death(0)
            .build(),

        StageBuilder::start()
            .build_for(StageLevel::Progressing)
                .no_self_heal()
                .vitals()
                    .with_target_body_temp(38.2)
                    .with_target_heart_rate(89.)
                    .with_target_blood_pressure(126., 84.)
                    .will_reach_target_in(0.2) // 6 + 12min
                    .will_end()
                .drains()
                    .stamina(0.025)
                    .food_level(0.055)
                    .water_level(0.15)
                .affects_fatigue(10.)
                .no_death_probability()
            .build(),

        StageBuilder::start()
            .build_for(StageLevel::Worrying)
                .no_self_heal()
                .vitals()
                    .with_target_body_temp(39.4)
                    .with_target_heart_rate(89.)
                    .with_target_blood_pressure(126., 84.)
                    .will_reach_target_in(0.15) // 18 + 9min
                    .will_end()
                .drains()
                    .stamina(0.029)
                    .food_level(0.059)
                    .water_level(0.19)
                .no_fatigue_effect()
                .no_death_probability()
            .build(),

        StageBuilder::start()
            .build_for(StageLevel::Critical)
                .no_self_heal()
                .vitals()
                    .with_target_body_temp(39.9)
                    .with_target_heart_rate(89.)
                    .with_target_blood_pressure(126., 84.)
                    .will_reach_target_in(0.1) // 27 + 6min
                    .will_last_forever()
                .no_drains()
                .no_death_probability()
            .build() // 33 min total
    ]
);
impl DiseaseTreatment for Flu {
    fn on_consumed(&self, game_time: &GameTimeC, item: &ConsumableC, active_stage: &ActiveStage, disease: &ActiveDisease, inventory_items: &HashMap<String, Box<dyn InventoryItem>>) {
        //println!("from treatment");
    }

    fn on_appliance_taken(&self, game_time: &GameTimeC, item: &ApplianceC, body_part: BodyParts, active_stage: &ActiveStage, disease: &ActiveDisease, inventory_items: &HashMap<String, Box<dyn InventoryItem>>) {
        //println!("from treatment");
    }
}

pub struct Angina;
zara::disease!(Angina, "Angina", None,
    vec![
        StageBuilder::start()
            .build_for(StageLevel::InitialStage)
                .self_heal(3)
                .vitals()
                    .with_target_body_temp(37.5)
                    .with_target_heart_rate(85.)
                    .with_target_blood_pressure(130., 90.)
                    .will_reach_target_in(0.7)
                    .will_end()
                .no_drains()
                .no_death_probability()
            .build(),

        StageBuilder::start()
            .build_for(StageLevel::Progressing)
                .no_self_heal()
                .vitals()
                    .with_target_body_temp(38.2)
                    .with_target_heart_rate(89.)
                    .with_target_blood_pressure(126., 84.)
                    .will_reach_target_in(1.2)
                    .will_end()
                .no_drains()
                .no_death_probability()
            .build()
    ]
);