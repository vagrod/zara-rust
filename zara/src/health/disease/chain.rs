use crate::health::disease::{ActiveDisease, ActiveStage, StageLevel, StageDescription};
use crate::utils::{GameTimeC, clamp_bottom, HealthC};

use std::time::Duration;
use std::collections::HashMap;
use std::convert::TryFrom;

impl ActiveDisease {
    /// Inverts disease stages so that disease goes from the current state to its beginning.
    ///
    /// Use this to start the "curing" process
    ///
    /// ## Note
    /// `HealthyStage` will be added at the end of the stages chain.
    ///
    /// Will not do anything if `invert` was already called. Call [`invert_back`] to change
    /// direction of passing stages again.
    ///
    /// [`invert_back`]:#method.invert_back
    ///
    /// ## Returns
    /// `true` on success.
    ///
    /// # Parameters
    /// - `game_time`: the time when inversion occurs
    pub fn invert(&self, game_time: &GameTimeC) -> bool {
        if self.is_inverted.get() { return false; }
        if !self.get_is_active(game_time) { return false; }
        let active_stage_opt = self.get_active_stage(game_time);
        if !active_stage_opt.is_some() { return false; }

        let mut stages = HashMap::new();
        let gt = game_time.to_duration().as_secs_f32();
        let active_stage = active_stage_opt.unwrap();
        let pt = active_stage.peak_time.to_duration().as_secs_f32();

        // First of all, we'll calculate bound to the left and to the right of the given
        // "rotation point" -- gt
        let level_int = active_stage.info.level as i32;
        let d = if gt > pt { 0. } else { pt - gt }; // case for "endless" stages
        let new_start_time = clamp_bottom(gt - d, 0.);
        let new_peak_time = new_start_time + active_stage.info.reaches_peak_in_hours*60.*60.;

        // Add this calculated stage to the list.
        stages.insert(active_stage.info.level, ActiveStage {
            info: active_stage.info.copy(),
            start_time: GameTimeC::from_duration(Duration::from_secs_f64(new_start_time as f64)),
            peak_time: GameTimeC::from_duration(Duration::from_secs_f64(new_peak_time as f64)),
        });

        let mut t = new_start_time;
        // With this stage timing calculated we'll add all stages "to the left".
        // Now calculating them is very easy.
        for l in (level_int+1)..(StageLevel::Critical as i32+1) {
            let b = self.initial_data.borrow();
            let ind = b.iter().position(|x| (x.level as i32) == l);
            if !ind.is_some() { continue; } // strange case that should never happen, but we know...
            let info_opt = b.get(ind.unwrap());
            if !info_opt.is_some() { continue; } // same
            let mut info = info_opt.unwrap().copy();
            let start_time = clamp_bottom(t - info.reaches_peak_in_hours*60.*60.,0.);
            let peak_time = t;
            let level_res = StageLevel::try_from(l);

            if level_res.is_err() { continue; }

            info.is_endless = false;
            stages.insert(level_res.unwrap(), ActiveStage {
                info,
                start_time: GameTimeC::from_duration(Duration::from_secs_f64(start_time as f64)),
                peak_time: GameTimeC::from_duration(Duration::from_secs_f64(peak_time as f64)),
            });

            t = start_time;
        }

        // Same thing with stages "to the right"
        t = new_peak_time;
        let mut l = level_int-1;
        while l > 0 {
            let b = self.initial_data.borrow();
            let ind = b.iter().position(|x| (x.level as i32) == l);
            if !ind.is_some() { continue; } // strange case that should never happen, but we know...
            let info_opt = b.get(ind.unwrap());
            if !info_opt.is_some() { continue; } // same
            let mut info = info_opt.unwrap().copy();
            let start_time = t;
            let peak_time = start_time + info.reaches_peak_in_hours*60.*60.;
            let level_res = StageLevel::try_from(l);

            if level_res.is_err() { continue; }

            info.is_endless = false;
            stages.insert(level_res.unwrap(), ActiveStage {
                info,
                start_time: GameTimeC::from_duration(Duration::from_secs_f64(start_time as f64)),
                peak_time: GameTimeC::from_duration(Duration::from_secs_f64(peak_time as f64)),
            });

            t = peak_time;
            l -= 1;
        }

        // Add "healthy" node
        let healthy = HealthC::healthy();
        let healthy_stage_duration_sec = 5.*60.;
        stages.insert(StageLevel::HealthyStage, ActiveStage {
            info: StageDescription{
                level: StageLevel::HealthyStage,
                reaches_peak_in_hours: healthy_stage_duration_sec/60./60., // 5 minutes
                is_endless: false,
                self_heal_chance: None,
                target_body_temp: healthy.body_temperature,
                target_heart_rate: healthy.heart_rate,
                target_pressure_top: healthy.top_pressure,
                target_pressure_bottom: healthy.bottom_pressure
            },
            start_time: GameTimeC::from_duration(Duration::from_secs_f64(t as f64)),
            peak_time: GameTimeC::from_duration(Duration::from_secs_f64((t + healthy_stage_duration_sec) as f64)),
        });

        self.stages.replace(stages);
        self.is_inverted.set(true);

        return true;
    }

    /// Inverts disease stages back so that disease goes from the current state to its end.
    /// Use this to cancel the "curing" process and make disease getting "worse" again.
    ///
    /// ## Note
    /// This method will not invert back disease which time marker (passed `game_time` parameter)
    /// is on the `HealthyStage`. `false` will be returned in this case.
    ///
    /// Will not do anything if `invert_back` was already called. Call [`invert`] to change
    /// direction of passing stages again.
    ///
    /// [`invert`]:#method.invert
    ///
    /// ## Returns
    /// `true` on success.
    ///
    /// # Parameters
    /// - `game_time`: the time when inversion occurs
    pub fn invert_back(&self, game_time: &GameTimeC) -> bool {
        if !self.is_inverted.get() { return false; }
        if !self.get_is_active(game_time) { return false; }
        let active_stage_opt = self.get_active_stage(game_time);
        if !active_stage_opt.is_some() { return false; }

        let mut stages = HashMap::new();
        let gt = game_time.to_duration().as_secs_f32();
        let active_stage = active_stage_opt.unwrap();

        // We do not roll back when the disease is healed
        if active_stage.info.level == StageLevel::HealthyStage { return false; }

        let pt = active_stage.peak_time.to_duration().as_secs_f32();

        // First of all, we'll calculate bound to the left and to the right of the given
        // "rotation point" -- gt
        let level_int = active_stage.info.level as i32;
        let d = if gt > pt { 0. } else { pt - gt }; // case for "endless" stages
        let new_start_time = clamp_bottom(gt - d, 0.);
        let new_peak_time = new_start_time + active_stage.info.reaches_peak_in_hours*60.*60.;

        // Add this calculated stage to the list.
        stages.insert(active_stage.info.level, ActiveStage {
            info: active_stage.info.copy(),
            start_time: GameTimeC::from_duration(Duration::from_secs_f64(new_start_time as f64)),
            peak_time: GameTimeC::from_duration(Duration::from_secs_f64(new_peak_time as f64)),
        });

        let mut t = new_peak_time;
        // With this stage timing calculated we'll add all stages "to the right".
        // Now calculating them is very easy.
        for l in (level_int+1)..(StageLevel::Critical as i32+1) {
            let b = self.initial_data.borrow();
            let ind = b.iter().position(|x| (x.level as i32) == l);
            if !ind.is_some() { continue; } // strange case that should never happen, but we know...
            let info_opt = b.get(ind.unwrap());
            if !info_opt.is_some() { continue; } // same
            let info = info_opt.unwrap();

            let start_time = t;
            let peak_time = t + info.reaches_peak_in_hours*60.*60.;
            let level_res = StageLevel::try_from(l);

            if level_res.is_err() { continue; }

            stages.insert(level_res.unwrap(), ActiveStage {
                info: info.copy(),
                start_time: GameTimeC::from_duration(Duration::from_secs_f64(start_time as f64)),
                peak_time: GameTimeC::from_duration(Duration::from_secs_f64(peak_time as f64)),
            });

            t = peak_time;
        }

        // Same thing with stages "to the left"
        t = new_start_time;
        let mut l = level_int-1;
        while l > 0 {
            let b = self.initial_data.borrow();
            let ind = b.iter().position(|x| (x.level as i32) == l);
            if !ind.is_some() { continue; } // strange case that should never happen, but we know...
            let info_opt = b.get(ind.unwrap());
            if !info_opt.is_some() { continue; } // same
            let info = info_opt.unwrap();

            let start_time = clamp_bottom(t - info.reaches_peak_in_hours*60.*60., 0.);
            let peak_time = t;

            let level_res = StageLevel::try_from(l);

            if level_res.is_err() { continue; }

            stages.insert(level_res.unwrap(), ActiveStage {
                info: info.copy(),
                start_time: GameTimeC::from_duration(Duration::from_secs_f64(start_time as f64)),
                peak_time: GameTimeC::from_duration(Duration::from_secs_f64(peak_time as f64)),
            });

            t = start_time;
            l -= 1;
        }

        self.stages.replace(stages);
        self.is_inverted.set(false);

        return true;
    }
}