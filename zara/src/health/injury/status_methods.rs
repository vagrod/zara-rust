use crate::health::injury::{ActiveInjury, ActiveStage};
use crate::utils::GameTimeC;
use crate::health::StageLevel;

impl ActiveInjury {
    /// Gets if this injury will end (is it finite)
    pub fn will_end(&self) -> bool { self.will_end.get() }

    /// Gets if this injury is now healing (is inverted)
    pub fn is_healing(&self) -> bool { self.is_inverted.get() }

    /// Gets the end time of this injury, if it is finite
    pub fn end_time(&self) -> Option<GameTimeC> {
        self.end_time.borrow().as_ref().map(|x| x.clone())
    }

    /// Gets a copy of active injury stage data for a given time
    pub fn get_active_stage(&self, game_time: &GameTimeC) -> Option<ActiveStage> {
        for (_, stage) in self.stages.borrow().iter() {
            if stage.is_active(game_time) { return Some(stage.copy()) }
        }

        None
    }

    /// Gets active stage level for a given game time
    pub fn active_level(&self, game_time: &GameTimeC) -> Option<StageLevel> {
        self.get_active_stage(game_time).map(|st| st.info.level)
    }

    /// Returns a copy of a game time structure containing data of when this injury was activated
    pub fn activation_time(&self) -> GameTimeC { self.activation_time.borrow().clone() }

    /// Returns a copy of stage data by its level
    pub fn get_stage(&self, level: StageLevel) -> Option<ActiveStage> {
        for (l, stage) in self.stages.borrow().iter() {
            if level as i32 == *l as i32 { return Some(stage.copy()) }
        }

        None
    }

    /// Gets whether injury is active or not for a given time
    pub fn is_active(&self, game_time: &GameTimeC) -> bool {
        let activation_secs = self.activation_time.borrow().as_secs_f32();
        let game_time_secs = game_time.as_secs_f32();

        if self.will_end.get() {
            let b = self.end_time.borrow();
            let border_secs = match b.as_ref() {
                Some(t) => t.as_secs_f32(),
                None => game_time_secs
            };

            game_time_secs >= activation_secs && game_time_secs <= border_secs
        } else {
            game_time_secs >= activation_secs
        }
    }

    /// Returns `true` if this injury already passed and is no longer relevant, for a given game time
    pub fn is_old(&self, game_time: &GameTimeC) -> bool {
        let gt = game_time.as_secs_f32();
        match self.end_time.borrow().as_ref() {
            Some(t) => gt > t.as_secs_f32(),
            None => false
        }
    }

    /// Gets if blood loss has been temporary stopped by the [`stop_blood_loss`] call
    ///
    /// [`stop_blood_loss`]: #method.stop_blood_loss
    pub fn is_blood_stopped(&self) -> bool { self.blood_loss_stop.get() }
}