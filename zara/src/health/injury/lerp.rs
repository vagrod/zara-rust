use crate::health::injury::{ActiveInjury, InjuryDeltasC, LerpDataNodeC, LerpDataC, ActiveStage, StageLevel, StageDescription};
use crate::utils::{lerp, clamp_01, GameTimeC};

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
            stamina_data: Vec::new(),
            blood_data: Vec::new()
        }
    }
}

impl ActiveInjury {
    fn generate_lerp_data(&self, game_time: &GameTimeC) {
        let inverted = self.is_inverted.get();
        let gt = game_time.as_secs_f32();
        let last_deltas = self.last_deltas.borrow();
        let mut has_endless_child = false;
        let mut last_start_stamina_delta = last_deltas.stamina_drain;
        let mut last_start_blood_delta = last_deltas.blood_drain;

        // Creating our lerp data object
        let mut lerp_data = LerpDataNodeC::new();
        lerp_data.is_for_inverted = self.is_inverted.get();
        lerp_data.start_time = gt;

        // Clear the old structure
        match self.lerp_data.borrow_mut().as_mut() {
            Some(m) => {
                m.stamina_data.clear();
                m.blood_data.clear();
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
                chance_of_death: None,
                target_stamina_drain: 0.,
                target_blood_drain: 0.
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
            
            let start_time = if gt > start { gt } else { start };

            // Determine the next chain stage if any, only for the inverted chain.
            // Inverted chain lerp takes "start value" parameter of the next stage as its "end value".
            let mut next_stage : Option<&ActiveStage> = None;
            if inverted {
                let next_level = StageLevel::try_from(stage.info.level as i32 - 1)
                    .unwrap_or(StageLevel::Undefined);
                if next_level != StageLevel::Undefined {
                    next_stage = stages.get(&next_level);
                } else {
                    // Need to lerp to zeros (to "healthy" state) when reached
                    // last stage in the inverted chain
                    next_stage = Some(&healthy_stage);
                }
            }

            if lerp_data.end_time < end { lerp_data.end_time = end; }

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
            // Blood
            if stage.info.target_blood_drain > 0. {
                let end_value = match next_stage {
                    Some(st) => st.info.target_blood_drain,
                    None => stage.info.target_blood_drain
                };
                let ld = LerpDataC {
                    start_time,
                    end_time: end,
                    start_value: last_start_blood_delta,
                    end_value,
                    duration: end - start_time,
                    is_endless: stage.info.is_endless
                };

                last_start_blood_delta = ld.end_value;
                lerp_data.blood_data.push(ld);
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

        false
    }

    /// Gets injury drain delta for a given time
    pub(crate) fn get_drains_deltas(&self, game_time: &GameTimeC) -> InjuryDeltasC {
        let mut result = InjuryDeltasC::empty();

        if !self.has_lerp_data_for(game_time) {
            self.generate_lerp_data(game_time);

            // Could not calculate lerps for some reason
            if !self.has_lerp_data_for(game_time) { return InjuryDeltasC::empty(); }
        }

        let b = self.lerp_data.borrow();
        let lerp_data = match b.as_ref() {
            Some(o) => o,
            None => return InjuryDeltasC::empty()
        };
        let gt = game_time.as_secs_f32();

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
        { // Blood
            if self.blood_loss_stop.get() {
                result.blood_drain = 0.;
            } else {
                let mut ld = None;
                for data in lerp_data.blood_data.iter() {
                    if (gt >= data.start_time && data.is_endless) || (gt >= data.start_time && gt <= data.end_time) {
                        ld = Some(data);
                        break;
                    }
                }
                match ld {
                    Some(d) => {
                        let p = clamp_01((gt - d.start_time) / d.duration);
                        result.blood_drain = lerp(d.start_value, d.end_value, p);
                    },
                    None => { }
                }
            }
        }

        self.last_deltas.replace(result.copy());

        result
    }
}