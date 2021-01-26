use crate::utils::event::{MessageQueue, Event};
use crate::health::injury::{ActiveInjury, ActiveStage, StageLevel};
use crate::utils::{GameTimeC, clamp_bottom};
use crate::error::{ChainInvertErr, ChainInvertBackErr};

use std::time::Duration;
use std::collections::{BTreeMap};
use std::convert::TryFrom;

impl ActiveInjury {
    /// Inverts injury stages so that injury goes from the current state to its beginning.
    ///
    /// Use this to start the "curing" process
    ///
    /// ## Note
    /// Will not do anything if `invert` was already called. Call [`invert_back`] to change
    /// direction of passing stages again.
    ///
    /// [`invert_back`]: #method.invert_back
    ///
    /// ## Returns
    /// Ok on success.
    ///
    /// # Parameters
    /// - `game_time`: the time when inversion occurs
    pub fn invert(&self, game_time: &GameTimeC) -> Result<(), ChainInvertErr> {
        if self.is_inverted.get() { return Err(ChainInvertErr::AlreadyInverted); }
        if !self.get_is_active(game_time) { return Err(ChainInvertErr::DiseaseNotActiveAtGivenTime); }
        let active_stage = match self.get_active_stage(game_time) {
            Some(o) => o,
            None => return Err(ChainInvertErr::NoActiveStageAtGivenTime)
        };
        let mut stages = BTreeMap::new();
        let gt = game_time.as_secs_f32();
        let pt = active_stage.peak_time.as_secs_f32();

        // First of all, we'll calculate bound to the left and to the right of the given
        // "rotation point" -- gt
        let level_int = active_stage.info.level as i32;
        let d = if gt > pt { 0. } else { pt - gt }; // case for "endless" stages
        let new_start_time = clamp_bottom(gt - d, 0.);
        let new_peak_time = new_start_time + active_stage.info.reaches_peak_in_hours*60.*60.;
        let mut chain_start_time = new_start_time;

        // Add this calculated stage to the list.
        stages.insert(active_stage.info.level, ActiveStage {
            info: active_stage.info.copy(),
            duration: Duration::from_secs_f32(new_peak_time-new_start_time),
            start_time: GameTimeC::from_duration(Duration::from_secs_f64(new_start_time as f64)),
            peak_time: GameTimeC::from_duration(Duration::from_secs_f64(new_peak_time as f64)),
        });

        let mut t = new_start_time;
        // With this stage timing calculated we'll add all stages "to the left".
        // Now calculating them is very easy.
        for l in (level_int+1)..(StageLevel::Critical as i32+1) {
            let b = self.initial_data.borrow();
            let ind = match b.iter().position(|x| (x.level as i32) == l) {
                Some(i) => i,
                None => continue
            };
            let mut info = match b.get(ind) {
                Some(i) => i.copy(),
                None => continue
            };
            let start_time = clamp_bottom(t - info.reaches_peak_in_hours*60.*60.,0.);
            let peak_time = t;
            let level = match StageLevel::try_from(l) {
                Ok(l) => l,
                _ => continue
            };

            info.is_endless = false;
            stages.insert(level, ActiveStage {
                info,
                duration: Duration::from_secs_f32(clamp_bottom(peak_time-start_time, 0.)),
                start_time: GameTimeC::from_duration(Duration::from_secs_f64(start_time as f64)),
                peak_time: GameTimeC::from_duration(Duration::from_secs_f64(peak_time as f64)),
            });

            t = start_time;

            if chain_start_time > t { chain_start_time = t; }
        }

        // Same thing with stages "to the right"
        t = new_peak_time;
        let mut l = level_int-1;
        while l > 0 {
            let b = self.initial_data.borrow();
            let ind = match b.iter().position(|x| (x.level as i32) == l) {
                Some(o) => o,
                None => continue
            };
            let mut info = match b.get(ind) {
                Some(o) => *o,
                None => continue
            };
            let start_time = t;
            let peak_time = start_time + info.reaches_peak_in_hours*60.*60.;
            let level = match StageLevel::try_from(l) {
                Ok(l) => l,
                _ => continue
            };

            info.is_endless = false;
            stages.insert(level, ActiveStage {
                info,
                duration: Duration::from_secs_f32(clamp_bottom(peak_time-start_time, 0.)),
                start_time: GameTimeC::from_duration(Duration::from_secs_f64(start_time as f64)),
                peak_time: GameTimeC::from_duration(Duration::from_secs_f64(peak_time as f64)),
            });

            t = peak_time;
            l -= 1;

            if chain_start_time > t { chain_start_time = t; }
        }

        self.stages.replace(stages);
        self.activation_time.replace(GameTimeC::from_duration(Duration::from_secs_f32(chain_start_time)));
        self.end_time.replace(Some(GameTimeC::from_duration(Duration::from_secs_f32(t))));
        self.will_end.set(true);
        self.is_inverted.set(true);

        self.queue_message(Event::InjuryInverted(self.injury.get_name()));

        return Ok(());
    }

    /// Inverts injury stages back so that injury goes from the current state to its end.
    /// Use this to cancel the "curing" process and make injury getting "worse" again.
    ///
    /// ## Note
    /// Will not do anything if `invert_back` was already called. Call [`invert`] to change
    /// direction of passing stages again.
    ///
    /// [`invert`]: #method.invert
    ///
    /// ## Returns
    /// Ok on success.
    ///
    /// # Parameters
    /// - `game_time`: the time when inversion occurs
    pub fn invert_back(&self, game_time: &GameTimeC) -> Result<(), ChainInvertBackErr> {
        if !self.is_inverted.get() { return Err(ChainInvertBackErr::AlreadyInvertedBack); }
        if !self.get_is_active(game_time) {
            return Err(ChainInvertBackErr::DiseaseNotActiveAtGivenTime);
        }
        let active_stage = match self.get_active_stage(game_time) {
            Some(o) => o,
            None => return Err(ChainInvertBackErr::NoActiveStageAtGivenTime)
        };

        let mut stages = BTreeMap::new();
        let gt = game_time.as_secs_f32();
        let mut will_end = true;
        let pt = active_stage.peak_time.as_secs_f32();

        // First of all, we'll calculate bound to the left and to the right of the given
        // "rotation point" -- gt
        let level_int = active_stage.info.level as i32;
        let d = if gt > pt { 0. } else { pt - gt }; // case for "endless" stages
        let new_start_time = clamp_bottom(gt - d, 0.);
        let new_peak_time = new_start_time + active_stage.info.reaches_peak_in_hours*60.*60.;
        let mut chain_start_time = new_start_time;

        // Add this calculated stage to the list.
        stages.insert(active_stage.info.level, ActiveStage {
            info: active_stage.info.copy(),
            duration: Duration::from_secs_f32(clamp_bottom(new_peak_time-new_start_time, 0.)),
            start_time: GameTimeC::from_duration(Duration::from_secs_f64(new_start_time as f64)),
            peak_time: GameTimeC::from_duration(Duration::from_secs_f64(new_peak_time as f64)),
        });

        // With this stage timing calculated we'll add all stages "to the left".
        // Now calculating them is very easy.
        let mut t = new_start_time;
        let mut l = level_int-1;
        while l > 0 {
            let b = self.initial_data.borrow();
            let ind = match b.iter().position(|x| (x.level as i32) == l) {
                Some(o) => o,
                None => continue
            };
            let info = match b.get(ind) {
                Some(o) => o,
                None => continue
            };

            if info.is_endless { will_end = false; }

            let start_time = clamp_bottom(t - info.reaches_peak_in_hours*60.*60., 0.);
            let peak_time = t;
            let level = match StageLevel::try_from(l) {
                Ok(l) => l,
                _ => continue
            };

            stages.insert(level, ActiveStage {
                info: info.copy(),
                duration: Duration::from_secs_f32(clamp_bottom(peak_time-start_time, 0.)),
                start_time: GameTimeC::from_duration(Duration::from_secs_f64(start_time as f64)),
                peak_time: GameTimeC::from_duration(Duration::from_secs_f64(peak_time as f64)),
            });

            t = start_time;
            l -= 1;

            if chain_start_time > t { chain_start_time = t; }
        }

        t = new_peak_time;
        // Same thing with stages "to the right"
        for l in (level_int+1)..(StageLevel::Critical as i32+1) {
            let b = self.initial_data.borrow();
            let ind = match b.iter().position(|x| (x.level as i32) == l) {
                Some(o) => o,
                None => continue
            };
            let info = match b.get(ind) {
                Some(o) => o,
                None => continue
            };

            if info.is_endless { will_end = false; }

            let start_time = t;
            let peak_time = t + info.reaches_peak_in_hours*60.*60.;
            let level = match StageLevel::try_from(l) {
                Ok(l) => l,
                _ => continue
            };

            stages.insert(level, ActiveStage {
                info: info.copy(),
                duration: Duration::from_secs_f32(clamp_bottom(peak_time-start_time, 0.)),
                start_time: GameTimeC::from_duration(Duration::from_secs_f64(start_time as f64)),
                peak_time: GameTimeC::from_duration(Duration::from_secs_f64(peak_time as f64)),
            });

            t = peak_time;

            if chain_start_time > t { chain_start_time = t; }
        }

        // Not stable yet unfortunately, will uncomment when it become stable
        // let new_end_time = will_end.then_some(GameTimeC::from_duration(Duration::from_secs_f32(t)));
        let new_end_time = if will_end {
            Some(GameTimeC::from_duration(Duration::from_secs_f32(t)))
        } else {
            None
        };

        self.stages.replace(stages);
        self.activation_time.replace(GameTimeC::from_duration(Duration::from_secs_f32(chain_start_time)));
        self.end_time.replace(new_end_time);
        self.will_end.set(will_end);
        self.is_inverted.set(false);

        self.queue_message(Event::InjuryResumed(self.injury.get_name()));

        return Ok(());
    }
}