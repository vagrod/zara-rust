use crate::health::disease::{ActiveDisease, DiseaseDeltasC, LerpDataNodeC, LerpDataC, ActiveStage, StageLevel, StageDescription};
use crate::utils::{lerp, clamp_01, GameTimeC, HealthC};

use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::time::Duration;

impl LerpDataNodeC {
    fn new() -> Self {
        LerpDataNodeC {
            start_time: 0.,
            end_time: 0.,
            is_endless: false,
            is_for_inverted: false,
            body_temp_data: Vec::new(),
            heart_rate_data: Vec::new(),
            pressure_top_data: Vec::new(),
            pressure_bottom_data: Vec::new(),
            stamina_data: Vec::new(),
            food_data: Vec::new(),
            water_data: Vec::new(),
            fatigue_data: Vec::new()
        }
    }
}

impl ActiveDisease {
    fn generate_lerp_data(&self, game_time: &GameTimeC) {
        let inverted = self.is_inverted.get();
        let healthy = HealthC::healthy();
        let gt = game_time.as_secs_f32();
        let last_deltas = self.last_deltas.borrow();
        let mut has_endless_child = false;
        let mut last_start_body_temp = last_deltas.body_temperature_delta;
        let mut last_start_heart_rate = last_deltas.heart_rate_delta;
        let mut last_start_pressure_top = last_deltas.pressure_top_delta;
        let mut last_start_pressure_bottom = last_deltas.pressure_bottom_delta;
        let mut last_start_fatigue_delta = last_deltas.fatigue_delta;
        let mut last_start_stamina_delta = last_deltas.stamina_drain;
        let mut last_start_food_delta = last_deltas.food_drain;
        let mut last_start_water_delta = last_deltas.water_drain;

        // Creating our lerp data object
        let mut lerp_data = LerpDataNodeC::new();
        lerp_data.is_for_inverted = self.is_inverted.get();
        lerp_data.start_time = gt;

        // Clear the old structure
        match self.lerp_data.borrow_mut().as_mut() {
            Some(m) => {
                m.body_temp_data.clear();
                m.heart_rate_data.clear();
                m.pressure_top_data.clear();
                m.pressure_bottom_data.clear();
                m.fatigue_data.clear();
                m.stamina_data.clear();
                m.food_data.clear();
                m.water_data.clear();
            },
            None => { }
        };
        self.lerp_data.replace(None);

        let healthy_stage = ActiveStage {
            info: StageDescription {
                level: StageLevel::Undefined,
                is_endless: false,
                reaches_peak_in_hours: 0.,
                self_heal_chance: None,
                target_fatigue_delta: 0.,
                target_stamina_drain: 0.,
                target_food_drain: 0.,
                target_water_drain: 0.,
                target_body_temp: healthy.body_temperature,
                target_heart_rate: healthy.heart_rate,
                target_pressure_top: healthy.top_pressure,
                target_pressure_bottom: healthy.bottom_pressure
            },
            duration: Duration::new(0,0),
            start_time: GameTimeC::empty(),
            peak_time: GameTimeC::empty()
        };

        // This is called in both cases -- for original and for inverted chains
        let mut process_stage =
            |stage: &ActiveStage, stages: &BTreeMap<StageLevel, ActiveStage>| -> bool {
            let start = stage.start_time.as_secs_f32();
            let end = stage.peak_time.as_secs_f32();

            if gt > end { return false; } // we are not interested in stages that already passed
            if stage.info.is_endless { has_endless_child = true; }
            
            let start_time= if gt > start { gt } else { start };

            // Determine the next chain stage if any, only for the inverted chain.
            // Inverted chain lerp takes "start value" parameter of the next stage as its "end value".
            let mut next_stage : Option<&ActiveStage> = None;
            if inverted {
                let next_level = StageLevel::try_from(stage.info.level as i32 - 1)
                    .unwrap_or(StageLevel::Undefined);
                if next_level != StageLevel::Undefined {
                    match stages.get(&next_level) {
                        Some(st) => next_stage = Some(st),
                        _ => { }
                    }
                } else {
                    // Need to lerp to zeros (to "healthy" state) when reached
                    // last stage in the inverted chain
                    next_stage = Some(&healthy_stage);
                }
            }

            if lerp_data.end_time < end { lerp_data.end_time = end; }

            // Body Temperature
            if stage.info.target_body_temp > 0. {
                let end_value = match next_stage {
                        Some(st) => st.info.target_body_temp,
                        None => stage.info.target_body_temp
                    } - healthy.body_temperature;
                let ld = LerpDataC {
                    start_time,
                    end_time: end,
                    start_value: last_start_body_temp,
                    end_value,
                    duration: end - start_time,
                    is_endless: stage.info.is_endless
                };

                last_start_body_temp = ld.end_value;
                lerp_data.body_temp_data.push(ld);
            }
            // Heart Rate
            if stage.info.target_heart_rate > 0. {
                let end_value = match next_stage {
                        Some(st) => st.info.target_heart_rate,
                        None => stage.info.target_heart_rate
                    } - healthy.heart_rate;
                let ld = LerpDataC {
                    start_time,
                    end_time: end,
                    start_value: last_start_heart_rate,
                    end_value,
                    duration: end - start_time,
                    is_endless: stage.info.is_endless
                };

                last_start_heart_rate = ld.end_value;
                lerp_data.heart_rate_data.push(ld);
            }
            // Pressure Top
            if stage.info.target_pressure_top > 0. {
                let end_value = match next_stage {
                        Some(st) => st.info.target_pressure_top,
                        None => stage.info.target_pressure_top
                    } - healthy.top_pressure;
                let ld = LerpDataC {
                    start_time,
                    end_time: end,
                    start_value: last_start_pressure_top,
                    end_value,
                    duration: end - start_time,
                    is_endless: stage.info.is_endless
                };

                last_start_pressure_top = ld.end_value;
                lerp_data.pressure_top_data.push(ld);
            }
            // Pressure Bottom
            if stage.info.target_pressure_bottom > 0. {
                let end_value = match next_stage {
                        Some(st) => st.info.target_pressure_bottom,
                        None => stage.info.target_pressure_bottom
                    } - healthy.bottom_pressure;
                let ld = LerpDataC {
                    start_time,
                    end_time: end,
                    start_value: last_start_pressure_bottom,
                    end_value,
                    duration: end - start_time,
                    is_endless: stage.info.is_endless
                };

                last_start_pressure_bottom = ld.end_value;
                lerp_data.pressure_bottom_data.push(ld);
            }
            // Fatigue
            if stage.info.target_fatigue_delta > 0. {
                let end_value = match next_stage {
                    Some(st) => st.info.target_fatigue_delta,
                    None => stage.info.target_fatigue_delta
                };
                let ld = LerpDataC {
                    start_time,
                    end_time: end,
                    start_value: last_start_fatigue_delta,
                    end_value,
                    duration: end - start_time,
                    is_endless: stage.info.is_endless
                };

                last_start_fatigue_delta = ld.end_value;
                lerp_data.fatigue_data.push(ld);
            }
            // Stamina
            if stage.info.target_stamina_drain > 0. {
                let end_value = match next_stage {
                    Some(st) => st.info.target_stamina_drain,
                    None => stage.info.target_stamina_drain
                };
                let ld = LerpDataC {
                    start_time,
                    end_time: end,
                    start_value: last_start_stamina_delta,
                    end_value,
                    duration: end - start_time,
                    is_endless: stage.info.is_endless
                };

                last_start_stamina_delta = ld.end_value;
                lerp_data.stamina_data.push(ld);
            }
            // Food
            if stage.info.target_food_drain > 0. {
                let end_value = match next_stage {
                    Some(st) => st.info.target_food_drain,
                    None => stage.info.target_food_drain
                };
                let ld = LerpDataC {
                    start_time,
                    end_time: end,
                    start_value: last_start_food_delta,
                    end_value,
                    duration: end - start_time,
                    is_endless: stage.info.is_endless
                };

                last_start_food_delta = ld.end_value;
                lerp_data.food_data.push(ld);
            }
            // Water
            if stage.info.target_water_drain > 0. {
                let end_value = match next_stage {
                    Some(st) => st.info.target_water_drain,
                    None => stage.info.target_water_drain
                };
                let ld = LerpDataC {
                    start_time,
                    end_time: end,
                    start_value: last_start_water_delta,
                    end_value,
                    duration: end - start_time,
                    is_endless: stage.info.is_endless
                };

                last_start_water_delta = ld.end_value;
                lerp_data.water_data.push(ld);
            }

            return true;
        };

        if !inverted {
            let b = self.stages.borrow();
            for (_, stage) in b.iter() {
                process_stage(stage, &b);
            }
        } else {
            let b = self.stages.borrow();
            for (_, stage) in b.iter().rev() {
                process_stage(stage, &b);
            }
        }

        lerp_data.is_endless = has_endless_child;

        self.lerp_data.replace(Some(lerp_data));
    }

    fn has_lerp_data_for(&self, game_time: &GameTimeC) -> bool {
        let b = self.lerp_data.borrow();
        let ld = match b.as_ref() {
            Some(o) => o,
            None => return false
        };

        if self.is_inverted.get() != ld.is_for_inverted {
            return false;
        }

        let gt = game_time.as_secs_f32();

        if (gt >= ld.start_time && ld.is_endless) || (gt >= ld.start_time && gt <= ld.end_time)
        {
            return true;
        }

        return false;
    }

    /// Gets disease vitals delta for a given time
    pub fn get_vitals_deltas(&self, game_time: &GameTimeC) -> DiseaseDeltasC {
        let mut result = DiseaseDeltasC::empty();

        if !self.has_lerp_data_for(game_time) {
            self.generate_lerp_data(game_time);

            // Could not calculate lerps for some reason
            if !self.has_lerp_data_for(game_time) { return DiseaseDeltasC::empty(); }
        }

        let b = self.lerp_data.borrow();
        let lerp_data = match b.as_ref() {
            Some(o) => o,
            None => return DiseaseDeltasC::empty()
        };
        let gt = game_time.as_secs_f32();

        { // Body Temperature
            let mut ld = None;
            for data in lerp_data.body_temp_data.iter() {
                if (gt >= data.start_time && data.is_endless) || (gt >= data.start_time && gt <= data.end_time) {
                    ld = Some(data);
                    break;
                }
            }
            match ld {
                Some(d) => {
                    let p = clamp_01((gt - d.start_time) / d.duration);
                    result.body_temperature_delta = lerp(d.start_value, d.end_value, p);
                },
                None => { }
            }
        }
        { // Heart Rate
            let mut ld = None;
            for data in lerp_data.heart_rate_data.iter() {
                if (gt >= data.start_time && data.is_endless) || (gt >= data.start_time && gt <= data.end_time) {
                    ld = Some(data);
                    break;
                }
            }
            match ld {
                Some(d) => {
                    let p = clamp_01((gt - d.start_time) / d.duration);
                    result.heart_rate_delta = lerp(d.start_value, d.end_value, p);
                },
                None => { }
            }
        }
        { // Top Pressure
            let mut ld = None;
            for data in lerp_data.pressure_top_data.iter() {
                if (gt >= data.start_time && data.is_endless) || (gt >= data.start_time && gt <= data.end_time) {
                    ld = Some(data);
                    break;
                }
            }
            match ld {
                Some(d) => {
                    let p = clamp_01((gt - d.start_time) / d.duration);
                    result.pressure_top_delta = lerp(d.start_value, d.end_value, p);
                },
                None => { }
            }
        }
        { // Bottom Pressure
            let mut ld = None;
            for data in lerp_data.pressure_bottom_data.iter() {
                if (gt >= data.start_time && data.is_endless) || (gt >= data.start_time && gt <= data.end_time) {
                    ld = Some(data);
                    break;
                }
            }
            match ld {
                Some(d) => {
                    let p = clamp_01((gt - d.start_time) / d.duration);
                    result.pressure_bottom_delta = lerp(d.start_value, d.end_value, p);
                },
                None => { }
            }
        }
        { // Fatigue
            let mut ld = None;
            for data in lerp_data.fatigue_data.iter() {
                if (gt >= data.start_time && data.is_endless) || (gt >= data.start_time && gt <= data.end_time) {
                    ld = Some(data);
                    break;
                }
            }
            match ld {
                Some(d) => {
                    let p = clamp_01((gt - d.start_time) / d.duration);
                    result.fatigue_delta = lerp(d.start_value, d.end_value, p);
                },
                None => { }
            }
        }
        { // Stamina
            let mut ld = None;
            for data in lerp_data.stamina_data.iter() {
                if (gt >= data.start_time && data.is_endless) || (gt >= data.start_time && gt <= data.end_time) {
                    ld = Some(data);
                    break;
                }
            }
            match ld {
                Some(d) => {
                    let p = clamp_01((gt - d.start_time) / d.duration);
                    result.stamina_drain = lerp(d.start_value, d.end_value, p);
                },
                None => { }
            }
        }
        { // Food
            let mut ld = None;
            for data in lerp_data.food_data.iter() {
                if (gt >= data.start_time && data.is_endless) || (gt >= data.start_time && gt <= data.end_time) {
                    ld = Some(data);
                    break;
                }
            }
            match ld {
                Some(d) => {
                    let p = clamp_01((gt - d.start_time) / d.duration);
                    result.food_drain = lerp(d.start_value, d.end_value, p);
                },
                None => { }
            }
        }
        { // Water
            let mut ld = None;
            for data in lerp_data.water_data.iter() {
                if (gt >= data.start_time && data.is_endless) || (gt >= data.start_time && gt <= data.end_time) {
                    ld = Some(data);
                    break;
                }
            }
            match ld {
                Some(d) => {
                    let p = clamp_01((gt - d.start_time) / d.duration);
                    result.water_drain = lerp(d.start_value, d.end_value, p);
                },
                None => { }
            }
        }

        self.last_deltas.replace(result.copy());

        return result;
    }
}