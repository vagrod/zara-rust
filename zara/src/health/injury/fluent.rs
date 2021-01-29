use crate::health::{StageLevel};
use crate::health::injury::{StageDescription, StageBuilder};

impl StageBuilder {
    fn as_stage_self_heal(&self) -> &dyn StageSelfHeal { self }
    fn as_drains_node(&self) -> &dyn StageDrainsNode { self }
    fn as_after_no_drains(&self) -> &dyn StageAfterNoDrains { self }
    fn as_drains_values(&self) -> &dyn StageDrainsValues { self }
    fn as_stage_death_chance(&self) -> &dyn StageDeathChance { self }
    fn as_stage_targets(&self) -> &dyn StageTargets { self }
    fn as_stage_duration(&self) -> &dyn StageDuration { self }
    fn as_stage_end(&self) -> &dyn StageEnd { self }
}

pub trait StageInit {
    /// Starts stage building process
    ///
    /// # Parameters
    /// - `level`: level of the stage we will be building
    fn build_for(&self, level: StageLevel) -> &dyn StageSelfHeal;
}

pub trait StageSelfHeal {
    /// Will this stage have a chance of triggering self-healing
    ///
    /// # Parameters
    /// - `probability`: 0..100 chance of self-heal for this stage
    fn self_heal(&self, probability: usize) -> &dyn StageDrainsNode;
    /// This stage has no self-healing probability
    fn no_self_heal(&self) -> &dyn StageDrainsNode;
}

pub trait StageDrainsNode {
    /// Describe how this stage affects other parameters
    fn drains(&self) -> &dyn StageDrainsValues;
    /// This stage has no effect on other parameters
    fn no_drains(&self) -> &dyn StageAfterNoDrains;
}

pub trait StageDrainsValues {
    /// Set the static drain rate for the stamina for this stage. 0..100 percents per game second.
    ///
    /// # Parameters
    /// - `value`: max drain value for this stage (0..100 percents per game second)
    fn stamina(&self, value: f32) -> &dyn StageDrainsValues;
    /// Set the static drain rate for the food level for this stage. 0..100 percents per game second.
    ///
    /// # Parameters
    /// - `value`: max drain value for this stage (0..100 percents per game second)
    fn blood_level(&self, value: f32) -> &dyn StageDrainsValues;
    /// This stage is not deadly
    fn no_death_probability(&self) -> &dyn StageTargets;
    /// This stage will have death probability
    fn deadly(&self) -> &dyn StageDeathChance;
}

pub trait StageAfterNoDrains {
    /// This stage is not deadly
    fn no_death_probability(&self) -> &dyn StageTargets;
    /// This stage will have death probability
    fn deadly(&self) -> &dyn StageDeathChance;
}

pub trait StageDeathChance {
    /// Set chance of death for this stage. This chance will be tested on every Zara pass
    /// (every real second) while this stage is active
    fn with_chance_of_death(&self, value: usize) -> &dyn StageTargets;
}

pub trait StageTargets {
    /// In what time this stage should reach those vitals values (in game time hours)
    ///
    /// # Parameters
    /// - `hours`: number of game hours
    fn will_reach_target_in(&self, hours: f32) -> &dyn StageDuration;
}

pub trait StageDuration {
    /// Tells that injury will move on when `will_reach_target_in` time ends.
    ///
    /// Choosing `will_end` on the last stage will cause injury to disappear after last stage ends
    fn will_end(&self) -> &dyn StageEnd;
    /// Tells that injury will stay on this stage as long as this injury is not removed and
    /// will not move on to the next stage.
    ///
    /// Usually `will_last_forever` appears on a last injury stage.
    fn will_last_forever(&self) -> &dyn StageEnd;
}

pub trait StageEnd {
    /// Builds injury stage object with all the information provided
    fn build(&self) -> StageDescription;
}



impl StageInit for StageBuilder {
    fn build_for(&self, level: StageLevel) -> &dyn StageSelfHeal {
        self.level.replace(level);

        self.as_stage_self_heal()
    }
}

impl StageSelfHeal for StageBuilder {
    fn self_heal(&self, probability: usize) -> &dyn StageDrainsNode {
        self.self_heal_chance.replace(Some(probability));

        self.as_drains_node()
    }

    fn no_self_heal(&self) -> &dyn StageDrainsNode {
        self.as_drains_node()
    }
}

impl StageAfterNoDrains for StageBuilder {
    fn no_death_probability(&self) -> &dyn StageTargets {
        self.chance_of_death.replace(None);

        self.as_stage_targets()
    }

    fn deadly(&self) -> &dyn StageDeathChance {
        self.as_stage_death_chance()
    }
}

impl StageDrainsNode for StageBuilder {
    fn drains(&self) -> &dyn StageDrainsValues {
        self.as_drains_values()
    }

    fn no_drains(&self) -> &dyn StageAfterNoDrains {
        self.target_stamina_drain.set(0.00001);
        self.target_blood_drain.set(0.00001);

        self.as_after_no_drains()
    }
}

impl StageDrainsValues for StageBuilder {
    fn stamina(&self, value: f32) -> &dyn StageDrainsValues {
        self.target_stamina_drain.set(value);

        self.as_drains_values()
    }

    fn blood_level(&self, value: f32) -> &dyn StageDrainsValues {
        self.target_blood_drain.set(value);

        self.as_drains_values()
    }

    fn no_death_probability(&self) -> &dyn StageTargets {
        self.chance_of_death.replace(None);

        self.as_stage_targets()
    }

    fn deadly(&self) -> &dyn StageDeathChance {
        self.as_stage_death_chance()
    }
}

impl StageDeathChance for StageBuilder {
    fn with_chance_of_death(&self, value: usize) -> &dyn StageTargets {
        self.chance_of_death.replace(Some(value));

        self.as_stage_targets()
    }
}

impl StageTargets for StageBuilder {
    fn will_reach_target_in(&self, hours: f32) -> &dyn StageDuration {
        self.reaches_peak_in_hours.set(hours);

        self.as_stage_duration()
    }
}

impl StageDuration for StageBuilder {
    fn will_end(&self) -> &dyn StageEnd {
        self.as_stage_end()
    }

    fn will_last_forever(&self) -> &dyn StageEnd {
        self.is_endless.set(true);

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
            target_stamina_drain: self.target_stamina_drain.get(),
            target_blood_drain: self.target_blood_drain.get()
        }
    }
}