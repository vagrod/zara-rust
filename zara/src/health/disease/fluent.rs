use crate::health::disease::{StageDescription, StageBuilder, StageLevel};

impl StageBuilder {
    fn as_stage_self_heal(&self) -> &dyn StageSelfHeal { self }
    fn as_vitals_node(&self) -> &dyn StageVitalsNode { self }
    fn as_vitals_values(&self) -> &dyn StageVitalsValues { self }
    fn as_drains_node(&self) -> &dyn StageDrainsNode { self }
    fn as_drains_values(&self) -> &dyn StageDrainsValues { self }
    fn as_stage_death_chance(&self) -> &dyn StageDeathChance { self }
    fn as_stage_end(&self) -> &dyn StageEnd { self }
}

pub trait StageInit {
    /// Starts stage building process
    ///
    /// ## Parameters
    /// - `level`: level of the stage we will be building
    fn build_for(&self, level: StageLevel) -> &dyn StageSelfHeal;
}

pub trait StageSelfHeal {
    /// Will this stage have a chance of triggering self-healing
    ///
    /// ## Parameters
    /// - `probability`: 0..100 chance of self-heal for this stage
    fn self_heal(&self, probability: usize) -> &dyn StageVitalsNode;
    /// This stage has no self-healing probability
    fn no_self_heal(&self) -> &dyn StageVitalsNode;
}

pub trait StageVitalsNode {
    /// Describe how this stage affects vitals
    fn vitals(&self) -> &dyn StageVitalsValues;
}

pub trait StageVitalsValues {
    /// Set the target body temperature for this stage
    ///
    /// ## Parameters
    /// - `value`: absolute value (like 37.3)
    fn with_target_body_temp(&self, value: f32) -> &dyn StageVitalsValues;
    /// Set the target heart rate for this stage
    ///
    /// ## Parameters
    /// - `value`: absolute value (like 84.)
    fn with_target_heart_rate(&self, value: f32) -> &dyn StageVitalsValues;
    /// Set the target blood pressure for this stage
    ///
    /// ## Parameters
    /// - `top`: absolute value (like 120.)
    /// - `bottom`: absolute value (like 70.)
    fn with_target_blood_pressure(&self, top: f32, bottom: f32) -> &dyn StageVitalsValues;

    /// In what time this stage should reach those vitals values (in game time hours)
    ///
    /// ## Parameters
    /// - `hours`: number of game hours
    fn will_reach_target_in(&self, hours: f32) -> &dyn StageVitalsValues;
    /// Tells that disease will move on when `will_reach_target_in` time ends.
    ///
    /// Choosing `will_end` on the last stage will cause disease to disappear after last stage ends
    fn will_end(&self) -> &dyn StageDrainsNode;
    /// Tells that disease will stay on this stage as long as this disease is not removed and
    /// will not move on to the next stage.
    ///
    /// Usually `will_last_forever` appears on a last disease stage.
    fn will_last_forever(&self) -> &dyn StageDrainsNode;
}

pub trait StageDrainsNode {
    /// Describe how this stage affects other parameters
    fn drains(&self) -> &dyn StageDrainsValues;
    /// This stage has no effect on other parameters
    fn no_drains(&self) -> &dyn StageDeathChance;
}

pub trait StageDrainsValues {
    /// Set the static drain rate for the stamina for this stage. 0..100 percents per game second.
    ///
    /// ## Parameters
    /// - `value`: constant drain value (0..100 percents per game second)
    fn stamina(&self, value: f32) -> &dyn StageDrainsValues;
    /// Set the static drain rate for the food level for this stage. 0..100 percents per game second.
    ///
    /// ## Parameters
    /// - `value`: constant drain value (0..100 percents per game second)
    fn food_level(&self, value: f32) -> &dyn StageDrainsValues;
    /// Set the static drain rate for the water level for this stage. 0..100 percents per game second.
    ///
    /// ## Parameters
    /// - `value`: constant drain value (0..100 percents per game second)
    fn water_level(&self, value: f32) -> &dyn StageDrainsValues;

    /// Choose this if you want this stage to affect fatigue.
    ///
    /// ## Parameters
    /// - `target_delta`: maximum impact on fatigue at the end of this stage (0..100 percents)
    fn affects_fatigue(&self, target_delta: f32) -> &dyn StageDeathChance;
    /// This stage does not affect fatigue
    fn no_fatigue_effect(&self) -> &dyn StageDeathChance;
}

pub trait StageDeathChance {
    /// Set chance of death for this stage. This chance will be tested on every Zara pass
    /// (every real second) while this stage is active
    fn with_chance_of_death(&self, value: usize) -> &dyn StageEnd;
    /// No death chance for this stage
    fn no_death_probability(&self) -> &dyn StageEnd;
}

pub trait StageEnd {
    /// Builds disease stage object with all the information provided
    fn build(&self) -> StageDescription;
}



impl StageInit for StageBuilder {
    fn build_for(&self, level: StageLevel) -> &dyn StageSelfHeal {
        self.level.replace(level);

        self.as_stage_self_heal()
    }
}

impl StageSelfHeal for StageBuilder {
    fn self_heal(&self, probability: usize) -> &dyn StageVitalsNode {
        self.self_heal_chance.replace(Some(probability));

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

    fn will_reach_target_in(&self, hours: f32) -> &dyn StageVitalsValues {
        self.reaches_peak_in_hours.set(hours);

        self.as_vitals_values()
    }

    fn will_end(&self) -> &dyn StageDrainsNode {
        self.as_drains_node()
    }

    fn will_last_forever(&self) -> &dyn StageDrainsNode {
        self.is_endless.set(true);

        self.as_drains_node()
    }
}

impl StageDrainsNode for StageBuilder {
    fn drains(&self) -> &dyn StageDrainsValues {
        self.as_drains_values()
    }

    fn no_drains(&self) -> &dyn StageDeathChance {
        self.target_stamina_drain.set(0.00001);
        self.target_food_drain.set(0.00001);
        self.target_water_drain.set(0.00001);

        self.as_stage_death_chance()
    }
}

impl StageDrainsValues for StageBuilder {
    fn stamina(&self, value: f32) -> &dyn StageDrainsValues {
        self.target_stamina_drain.set(value);

        self.as_drains_values()
    }

    fn food_level(&self, value: f32) -> &dyn StageDrainsValues {
        self.target_food_drain.set(value);

        self.as_drains_values()
    }

    fn water_level(&self, value: f32) -> &dyn StageDrainsValues {
        self.target_water_drain.set(value);

        self.as_drains_values()
    }

    fn affects_fatigue(&self, target_delta: f32) -> &dyn StageDeathChance {
        self.target_fatigue_delta.set(target_delta);

        self.as_stage_death_chance()
    }

    fn no_fatigue_effect(&self) -> &dyn StageDeathChance {
        self.target_fatigue_delta.set(0.000001);

        self.as_stage_death_chance()
    }
}

impl StageDeathChance for StageBuilder {
    fn with_chance_of_death(&self, value: usize) -> &dyn StageEnd {
        self.chance_of_death.replace(Some(value));

        self.as_stage_end()
    }

    fn no_death_probability(&self) -> &dyn StageEnd {
        self.chance_of_death.replace(None);

        self.as_stage_end()
    }
}

impl StageEnd for StageBuilder {
    fn build(&self) -> StageDescription {
        let self_heal_chance = match self.self_heal_chance.borrow().as_ref() {
            Some(c) => Some(*c),
            None => None
        };
        let chance_of_death = match self.chance_of_death.borrow().as_ref() {
            Some(c) => Some(*c),
            None => None
        };

        StageDescription {
            level: *self.level.borrow(),
            self_heal_chance,
            chance_of_death,
            is_endless: self.is_endless.get(),
            reaches_peak_in_hours: self.reaches_peak_in_hours.get(),
            target_body_temp: self.target_body_temp.get(),
            target_heart_rate: self.target_heart_rate.get(),
            target_pressure_top: self.target_pressure_top.get(),
            target_pressure_bottom: self.target_pressure_bottom.get(),
            target_fatigue_delta: self.target_fatigue_delta.get(),
            target_stamina_drain: self.target_stamina_drain.get(),
            target_food_drain: self.target_food_drain.get(),
            target_water_drain: self.target_water_drain.get()
        }
    }
}