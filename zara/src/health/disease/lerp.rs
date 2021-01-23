use crate::health::disease::{ActiveDisease, DiseaseDeltasC, LerpDataNodeC, LerpDataC, ActiveStage};
use crate::utils::{lerp, clamp_01, GameTimeC, HealthC};
use crate::health::Health;
use std::cell::Cell;

impl LerpDataNodeC {
    fn new() -> Self {
        LerpDataNodeC {
            start_time: 0.,
            end_time: 0.,
            is_endless: false,
            body_temp_data: Vec::new(),
            heart_rate_data: Vec::new(),
            pressure_top_data: Vec::new(),
            pressure_bottom_data: Vec::new()
        }
    }
}

impl ActiveDisease {
    fn generate_lerp_data(&self, current_health: &Health, game_time: &GameTimeC) {
        self.lerp_data.replace(None);

        let mut lerp_data = LerpDataNodeC::new();
        let mut has_endless_child = false;

        let healthy = HealthC::healthy();
        let gt = game_time.to_duration().as_secs_f32();
        let last_start_body_temp = Cell::new(0.);
        let last_start_heart_rate = Cell::new(0.);
        let last_start_pressure_top = Cell::new(0.);
        let last_start_pressure_bottom = Cell::new(0.);

        lerp_data.start_time = gt;

        let mut process_stage = |stage: &ActiveStage| -> bool {
            let start = stage.start_time.to_duration().as_secs_f32();
            let end = stage.peak_time.to_duration().as_secs_f32();
            let duration = end - start;

            if gt > end { return false; }
            if stage.info.is_endless { has_endless_child = true; }

            let start_time= if gt > start { gt } else { start };
            let gt_progress = gt - start;
            let p = clamp_01(gt_progress/duration);

            if lerp_data.end_time < end { lerp_data.end_time = end; }

            // Body Temperature
            if stage.info.target_body_temp > 0. {
                let end_value = stage.info.target_body_temp - healthy.body_temperature;
                let ld = LerpDataC {
                    start_time,
                    end_time: end,
                    start_value: lerp(last_start_body_temp.get(), end_value, p),
                    end_value,
                    duration: end - start_time,
                    is_endless: stage.info.is_endless
                };

                last_start_body_temp.set(ld.end_value);
                lerp_data.body_temp_data.push(ld);
            }
            // Heart Rate
            if stage.info.target_heart_rate > 0. {
                let end_value = stage.info.target_heart_rate - healthy.heart_rate;
                let ld = LerpDataC {
                    start_time,
                    end_time: end,
                    start_value: lerp(last_start_heart_rate.get(), end_value, p),
                    end_value,
                    duration: end - start_time,
                    is_endless: stage.info.is_endless
                };

                last_start_heart_rate.set(ld.end_value);
                lerp_data.heart_rate_data.push(ld);
            }
            // Pressure Top
            if stage.info.target_pressure_top > 0. {
                let end_value = stage.info.target_pressure_top - healthy.top_pressure;
                let ld = LerpDataC {
                    start_time,
                    end_time: end,
                    start_value: lerp(last_start_pressure_top.get(), end_value, p),
                    end_value,
                    duration: end - start_time,
                    is_endless: stage.info.is_endless
                };

                last_start_pressure_top.set(ld.end_value);
                lerp_data.pressure_top_data.push(ld);
            }
            // Pressure Bottom
            if stage.info.target_pressure_bottom > 0. {
                let end_value = stage.info.target_pressure_bottom - healthy.bottom_pressure;
                let ld = LerpDataC {
                    start_time,
                    end_time: end,
                    start_value: lerp(last_start_pressure_bottom.get(), end_value, p),
                    end_value,
                    duration: end - start_time,
                    is_endless: stage.info.is_endless
                };

                last_start_pressure_bottom.set(ld.end_value);
                lerp_data.pressure_bottom_data.push(ld);
            }

            return true;
        };

        if !self.is_inverted.get() {
            // Leave "last_start_xxxxxx" at zeros here, it's correct for not-inverted chain
            for (_, stage) in self.stages.borrow().iter() {
                if !process_stage(stage) { continue; }
            }
        } else {
            // Must change lerp start values to reflect current ones
            last_start_body_temp.set(current_health.body_temperature.get() - healthy.body_temperature);
            last_start_heart_rate.set(current_health.heart_rate.get() - healthy.heart_rate);
            last_start_pressure_top.set(current_health.top_pressure.get() - healthy.top_pressure);
            last_start_pressure_bottom.set(current_health.bottom_pressure.get() - healthy.bottom_pressure);

            for (_, stage) in self.stages.borrow().iter().rev() {
                if !process_stage(stage) { continue; }
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

        let gt = game_time.to_duration().as_secs_f32();

        if (gt >= ld.start_time && ld.is_endless) || (gt >= ld.start_time && gt <= ld.end_time)
        {
            return true;
        }

        return false;
    }

    /// Gets disease vitals delta for a given time
    pub fn get_vitals_deltas(&self, current_health: &Health, game_time: &GameTimeC) -> DiseaseDeltasC {
        let mut result = DiseaseDeltasC::empty();

        if !self.has_lerp_data_for(game_time) {
            self.generate_lerp_data(current_health, game_time);

            // Could not calculate lerps for some reason
            if !self.has_lerp_data_for(game_time) { return DiseaseDeltasC::empty(); }
        }

        let b = self.lerp_data.borrow();
        let lerp_data = match b.as_ref() {
            Some(o) => o,
            None => return DiseaseDeltasC::empty()
        };
        let gt = game_time.to_duration().as_secs_f32();

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
                    let mut p = clamp_01((gt - d.start_time) / d.duration);
                    if d.is_endless { p = 1.; }
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
                    let mut p = clamp_01((gt - d.start_time) / d.duration);
                    if d.is_endless { p = 1.; }
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
                    let mut p = clamp_01((gt - d.start_time) / d.duration);
                    if d.is_endless { p = 1.; }
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
                    let mut p = clamp_01((gt - d.start_time) / d.duration);
                    if d.is_endless { p = 1.; }
                    result.pressure_bottom_delta = lerp(d.start_value, d.end_value, p);
                },
                None => { }
            }
        }

        return result;
    }
}