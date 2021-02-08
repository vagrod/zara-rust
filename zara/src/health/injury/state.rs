use crate::health::{StageLevel, Health, InjuryKey};
use crate::health::injury::{ActiveStage, LerpDataNodeC, LerpDataC, StageDescription, Injury, ActiveInjury, InjuryDeltasC};
use crate::utils::GameTimeC;
use crate::state::ActiveInjuryStateContract;

use std::time::Duration;
use std::collections::BTreeMap;
use std::rc::Rc;
use std::cell::{RefCell, Cell};

pub struct StageDescriptionStateContract {
    pub level: StageLevel,
    pub self_heal_chance: Option<usize>,
    pub chance_of_death: Option<usize>,
    pub reaches_peak_in_hours: f32,
    pub is_endless: bool,
    pub target_blood_drain: f32,
    pub target_stamina_drain: f32
}

pub struct ActiveStageStateContract {
    pub key: StageLevel,
    pub info: StageDescriptionStateContract,
    pub start_time: Duration,
    pub peak_time: Duration,
    pub duration: Duration
}

pub struct LerpDataNodeStateContract {
    pub start_time: f32,
    pub end_time: f32,
    pub stamina_data: Vec<LerpDataStateContract>,
    pub blood_data: Vec<LerpDataStateContract>,
    pub is_endless: bool,
    pub is_for_inverted: bool
}

pub struct LerpDataStateContract {
    pub start_time: f32,
    pub end_time: f32,
    pub start_value: f32,
    pub end_value: f32,
    pub duration: f32,
    pub is_endless: bool
}

pub struct InjuryDeltasStateContract {
    pub stamina_drain: f32,
    pub blood_drain: f32
}

impl StageDescription {
    pub(crate) fn get_state(&self) -> StageDescriptionStateContract {
        StageDescriptionStateContract {
            is_endless: self.is_endless,
            level: self.level,
            reaches_peak_in_hours: self.reaches_peak_in_hours,
            chance_of_death: self.chance_of_death.clone(),
            target_stamina_drain: self.target_stamina_drain,
            self_heal_chance: self.self_heal_chance.clone(),
            target_blood_drain: self.target_blood_drain
        }
    }
}

impl LerpDataNodeC {
    pub(crate) fn get_state(&self) -> LerpDataNodeStateContract {
        LerpDataNodeStateContract {
            start_time: self.start_time,
            end_time: self.end_time,
            is_endless: self.is_endless,
            is_for_inverted: self.is_for_inverted,
            blood_data: self.blood_data.iter().map(|x| x.get_state()).collect(),
            stamina_data: self.stamina_data.iter().map(|x| x.get_state()).collect()
        }
    }
}

impl LerpDataC {
    pub(crate) fn get_state(&self) -> LerpDataStateContract {
        LerpDataStateContract {
            start_time: self.start_time,
            end_time: self.end_time,
            duration: self.duration,
            start_value: self.start_value,
            end_value: self.end_value,
            is_endless: self.is_endless
        }
    }
}

impl Health {
    /// Adds new active injury based on the previously saved state
    ///
    /// # Parameters
    /// - `disease_data`: saved injury state (from `ActiveInjury.get_state` method call)
    /// - `injury`: injury instance
    ///
    /// ## Notes
    /// Borrows `injuries` collection
    pub fn restore_injury(&self, injury_data: &ActiveInjuryStateContract, injury: Box<dyn Injury>) {
        let mut b = self.injuries.borrow_mut();
        let treatment = injury.get_treatment();
        let name = injury.get_name().to_string();
        let body_part = injury_data.body_part.clone();
        let i = ActiveInjury {
            injury: Rc::new(injury),
            needs_treatment: injury_data.needs_treatment,
            will_self_heal_on: injury_data.will_self_heal_on,
            total_duration: injury_data.total_duration,
            is_fracture: injury_data.is_fracture,
            body_part: injury_data.body_part.clone(),

            stages: RefCell::new(BTreeMap::new()),
            last_deltas: RefCell::new(InjuryDeltasC::empty()),
            initial_data: RefCell::new(Vec::new()),
            end_time: RefCell::new(None),
            lerp_data: RefCell::new(None),
            is_inverted: Cell::new(false),
            activation_time: RefCell::new(GameTimeC::empty()),
            will_end: Cell::new(false),
            treatment: Rc::new(treatment),
            blood_loss_stop: Cell::new(false),
            message_queue: RefCell::new(BTreeMap::new())
        };

        i.set_state(injury_data);

        b.insert(InjuryKey{
            injury: name,
            body_part
        }, Rc::new(i));
    }
}

impl ActiveInjury {
    pub fn get_state(&self) -> ActiveInjuryStateContract {
        ActiveInjuryStateContract {
            needs_treatment: self.needs_treatment,
            is_fracture: self.is_fracture,
            body_part: self.body_part.clone(),
            activation_time: self.activation_time.borrow().to_duration(),
            will_end: self.will_end.get(),
            end_time: match self.end_time.borrow().as_ref() {
                Some(t) => Some(t.to_duration()),
                None => None
            },
            will_self_heal_on: self.will_self_heal_on,
            is_inverted: self.is_inverted.get(),
            total_duration: self.total_duration,

            lerp_data: match self.lerp_data.borrow().as_ref() {
                Some(l) => Some(l.get_state()),
                None => None
            },
            initial_data: self.initial_data.borrow().iter().map(|x| x.get_state()).collect(),
            last_deltas: self.last_deltas.borrow().get_state(),
            stages: self.stages.borrow().iter().map(|(k,x)| x.get_state(k)).collect()
        }
    }

    pub(crate) fn set_state(&self, state: &ActiveInjuryStateContract) {
        self.activation_time.replace(GameTimeC::from_duration(state.activation_time));
        self.will_end.set(state.will_end);

        self.end_time.replace(match state.end_time {
            Some(d) => Some(GameTimeC::from_duration(d)),
            None => None
        });
        self.is_inverted.set(state.is_inverted);

        self.initial_data.replace(state.initial_data.iter().map(|x| StageDescription{
            is_endless: x.is_endless,
            self_heal_chance: x.self_heal_chance.clone(),
            chance_of_death: x.chance_of_death.clone(),
            level: x.level.clone(),
            target_stamina_drain: x.target_stamina_drain,
            reaches_peak_in_hours: x.reaches_peak_in_hours,
            target_blood_drain: x.target_blood_drain
        }).collect());

        {
            let mut b = self.stages.borrow_mut();

            b.clear();

            for stage in &state.stages {
                b.insert(stage.key.clone(), ActiveStage{
                    start_time: GameTimeC::from_duration(stage.start_time),
                    peak_time: GameTimeC::from_duration(stage.peak_time),
                    duration: stage.duration,
                    info: StageDescription {
                        reaches_peak_in_hours: stage.info.reaches_peak_in_hours,
                        target_stamina_drain: stage.info.target_stamina_drain,
                        is_endless: stage.info.is_endless,
                        level: stage.info.level.clone(),
                        chance_of_death: stage.info.chance_of_death.clone(),
                        self_heal_chance: stage.info.self_heal_chance.clone(),
                        target_blood_drain: stage.info.target_blood_drain
                    }
                });
            }
        }

        self.last_deltas.replace(InjuryDeltasC {
            stamina_drain: state.last_deltas.stamina_drain,
            blood_drain: state.last_deltas.blood_drain
        });

        match &state.lerp_data {
            Some(l) => Some(LerpDataNodeC{
                start_time: l.start_time,
                end_time: l.end_time,
                is_for_inverted: l.is_for_inverted,
                is_endless: l.is_endless,
                blood_data: l.blood_data.iter().map(|x| LerpDataC {
                    start_time: x.start_time,
                    end_time: x.end_time,
                    duration: x.duration,
                    start_value: x.start_value,
                    end_value: x.end_value,
                    is_endless: x.is_endless
                }).collect(),
                stamina_data: l.stamina_data.iter().map(|x| LerpDataC {
                    start_time: x.start_time,
                    end_time: x.end_time,
                    duration: x.duration,
                    start_value: x.start_value,
                    end_value: x.end_value,
                    is_endless: x.is_endless
                }).collect()
            }),
            None => None
        };
    }
}

impl InjuryDeltasC {
    pub(crate) fn get_state(&self) -> InjuryDeltasStateContract {
        InjuryDeltasStateContract {
            blood_drain: self.blood_drain,
            stamina_drain: self.stamina_drain
        }
    }
}

impl ActiveStage {
    pub(crate) fn get_state(&self, key: &StageLevel) -> ActiveStageStateContract {
        ActiveStageStateContract {
            key: key.clone(),
            start_time: self.start_time.to_duration(),
            peak_time: self.peak_time.to_duration(),
            duration: self.duration,
            info: self.info.get_state()
        }
    }
}