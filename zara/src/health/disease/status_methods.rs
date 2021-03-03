use crate::health::disease::{ActiveDisease, ActiveStage};
use crate::utils::GameTimeC;
use crate::health::StageLevel;

impl ActiveDisease {
    /// Gets if this disease will end (is it finite)
    /// 
    /// # Examples
    /// ```
    /// let value = disease.will_end();
    /// ```
    pub fn will_end(&self) -> bool { self.will_end.get() }

    /// Gets if this disease is now healing (is inverted)
    /// 
    /// # Examples
    /// ```
    /// let value = disease.is_healing();
    /// ```
    pub fn is_healing(&self) -> bool { self.is_inverted.get() }

    /// Gets the end time of this disease, if it is finite
    /// 
    /// # Examples
    /// ```
    /// if let Some(game_time) = disease.end_time() {
    ///     // ...
    /// }
    /// ```
    pub fn end_time(&self) -> Option<GameTimeC> {
        self.end_time.borrow().as_ref().map(|x| x.clone())
    }

    /// Gets a copy of active disease stage data for a given time if exists
    /// 
    /// # Examples
    /// ```
    /// if let Some(stage) = disease.get_active_stage(game_time) {
    ///     // ...
    /// }
    /// ```
    pub fn get_active_stage(&self, game_time: &GameTimeC) -> Option<ActiveStage> {
        for (_, stage) in self.stages.borrow().iter() {
            if stage.is_active(game_time) { return Some(stage.clone()) }
        }

        None
    }

    /// Gets active stage level for a given game time if exists
    /// 
    /// # Examples
    /// ```
    /// if let Some(level) = disease.active_level(game_time) {
    ///     // ...
    /// }
    /// ```
    pub fn active_level(&self, game_time: &GameTimeC) -> Option<StageLevel> {
        self.get_active_stage(game_time).map(|st| st.info.level)
    }

    /// Returns a copy of a game time structure containing data of when 
    /// this disease was activated
    /// 
    /// # Examples
    /// ```
    /// let game_time = disease.activation_time();
    /// ```
    pub fn activation_time(&self) -> GameTimeC { self.activation_time.borrow().clone() }

    /// Returns a copy of stage data by its level if exists
    /// 
    /// # Examples
    /// ```
    /// use zara::health;
    /// 
    /// if let Some(stage) = disease.get_stage(health::StageLevel::Worrying) {
    ///     // ...
    /// }
    /// ```
    pub fn get_stage(&self, level: StageLevel) -> Option<ActiveStage> {
        for (l, stage) in self.stages.borrow().iter() {
            if level as i32 == *l as i32 { return Some(stage.clone()) }
        }

        None
    }

    /// Gets whether disease is active or not for a given time
    /// 
    /// # Examples
    /// ```
    /// let value = disease.is_active(game_time);
    /// ```
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

    /// Returns `true` if this disease already passed and is no longer relevant, 
    /// for a given game time
    /// 
    /// # Examples
    /// ```
    /// let value = disease.is_old(game_time);
    /// ```
    pub fn is_old(&self, game_time: &GameTimeC) -> bool {
        let gt = game_time.as_secs_f32();
        match self.end_time.borrow().as_ref() {
            Some(t) => gt > t.as_secs_f32(),
            None => false
        }
    }
}