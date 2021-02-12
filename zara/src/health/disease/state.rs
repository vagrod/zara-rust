use crate::health::{StageLevel, Health};
use crate::health::disease::{ActiveStage, LerpDataNodeC, DiseaseDeltasC, ActiveDisease, Disease, LerpDataC, StageDescription};
use crate::utils::GameTimeC;
use crate::state::ActiveDiseaseStateContract;

use std::time::Duration;
use std::collections::BTreeMap;
use std::rc::Rc;
use std::cell::{RefCell, Cell};
use std::fmt;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Default)]
pub struct StageDescriptionStateContract {
    pub level: StageLevel,
    pub self_heal_chance: Option<usize>,
    pub chance_of_death: Option<usize>,
    pub reaches_peak_in_hours: f32,
    pub is_endless: bool,
    pub target_body_temp: f32,
    pub target_heart_rate: f32,
    pub target_pressure_top: f32,
    pub target_pressure_bottom: f32,
    pub target_fatigue_delta: f32,
    pub target_food_drain: f32,
    pub target_water_drain: f32,
    pub target_stamina_drain: f32
}
impl fmt::Display for StageDescriptionStateContract {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Stage description state ({})", self.level)
    }
}
impl Ord for StageDescriptionStateContract {
    fn cmp(&self, other: &Self) -> Ordering {
        self.level.cmp(&other.level)
    }
}
impl PartialOrd for StageDescriptionStateContract {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for StageDescriptionStateContract {
    fn eq(&self, other: &Self) -> bool {
        const EPS: f32 = 0.0001;

        self.level == other.level &&
        self.self_heal_chance == other.self_heal_chance &&
        self.chance_of_death == other.chance_of_death &&
        self.is_endless == other.is_endless &&
        f32::abs(self.reaches_peak_in_hours - other.reaches_peak_in_hours) < EPS &&
        f32::abs(self.target_body_temp - other.target_body_temp) < EPS &&
        f32::abs(self.target_heart_rate - other.target_heart_rate) < EPS &&
        f32::abs(self.target_pressure_top - other.target_pressure_top) < EPS &&
        f32::abs(self.target_pressure_bottom - other.target_pressure_bottom) < EPS &&
        f32::abs(self.target_fatigue_delta - other.target_fatigue_delta) < EPS &&
        f32::abs(self.target_food_drain - other.target_food_drain) < EPS &&
        f32::abs(self.target_water_drain - other.target_water_drain) < EPS &&
        f32::abs(self.target_stamina_drain - other.target_stamina_drain) < EPS
    }
}
impl Eq for StageDescriptionStateContract { }
impl Hash for StageDescriptionStateContract {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.is_endless.hash(state);
        self.self_heal_chance.hash(state);
        self.chance_of_death.hash(state);
        self.is_endless.hash(state);

        state.write_u32(self.reaches_peak_in_hours as u32);
        state.write_u32(self.target_body_temp as u32);
        state.write_u32(self.target_heart_rate as u32);
        state.write_u32(self.target_pressure_top as u32);
        state.write_u32(self.target_pressure_bottom as u32);
        state.write_u32(self.target_fatigue_delta as u32);
        state.write_u32(self.target_food_drain as u32);
        state.write_u32(self.target_water_drain as u32);
        state.write_u32(self.target_stamina_drain as u32);
    }
}

#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Default)]
pub struct ActiveStageStateContract {
    pub key: StageLevel,
    pub info: StageDescriptionStateContract,
    pub start_time: Duration,
    pub peak_time: Duration,
    pub duration: Duration
}

#[derive(Clone, Debug, Default)]
pub struct LerpDataNodeStateContract {
    pub start_time: f32,
    pub end_time: f32,
    pub body_temp_data: Vec<LerpDataStateContract>,
    pub heart_rate_data: Vec<LerpDataStateContract>,
    pub pressure_top_data: Vec<LerpDataStateContract>,
    pub pressure_bottom_data: Vec<LerpDataStateContract>,
    pub fatigue_data: Vec<LerpDataStateContract>,
    pub stamina_data: Vec<LerpDataStateContract>,
    pub food_data: Vec<LerpDataStateContract>,
    pub water_data: Vec<LerpDataStateContract>,
    pub is_endless: bool,
    pub is_for_inverted: bool
}
impl fmt::Display for LerpDataNodeStateContract {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Lerp data node state")
    }
}
impl Ord for LerpDataNodeStateContract {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.start_time < other.start_time {
            return Ordering::Less;
        }
        if self.start_time > other.start_time {
            return Ordering::Greater;
        }

        Ordering::Equal
    }
}
impl PartialOrd for LerpDataNodeStateContract {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for LerpDataNodeStateContract {
    fn eq(&self, other: &Self) -> bool {
        const EPS: f32 = 0.0001;

        self.is_endless == other.is_endless &&
        self.is_for_inverted == other.is_for_inverted &&
        self.body_temp_data == other.body_temp_data &&
        self.heart_rate_data == other.heart_rate_data &&
        self.pressure_top_data == other.pressure_top_data &&
        self.pressure_bottom_data == other.pressure_bottom_data &&
        self.fatigue_data == other.fatigue_data &&
        self.stamina_data == other.stamina_data &&
        self.food_data == other.food_data &&
        self.water_data == other.water_data &&
        f32::abs(self.start_time - other.start_time) < EPS &&
        f32::abs(self.end_time - other.end_time) < EPS
    }
}
impl Eq for LerpDataNodeStateContract { }
impl Hash for LerpDataNodeStateContract {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.is_endless.hash(state);
        self.is_for_inverted.hash(state);
        self.body_temp_data.hash(state);
        self.heart_rate_data.hash(state);
        self.pressure_top_data.hash(state);
        self.pressure_bottom_data.hash(state);
        self.fatigue_data.hash(state);
        self.stamina_data.hash(state);
        self.food_data.hash(state);
        self.water_data.hash(state);

        state.write_u32(self.start_time as u32);
        state.write_u32(self.end_time as u32);
    }
}

#[derive(Clone, Debug, Default)]
pub struct LerpDataStateContract {
    pub start_time: f32,
    pub end_time: f32,
    pub start_value: f32,
    pub end_value: f32,
    pub duration: f32,
    pub is_endless: bool
}
impl fmt::Display for LerpDataStateContract {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Lerp data state")
    }
}
impl Ord for LerpDataStateContract {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.start_time < other.start_time {
            return Ordering::Less;
        }
        if self.start_time > other.start_time {
            return Ordering::Greater;
        }

        Ordering::Equal
    }
}
impl PartialOrd for LerpDataStateContract {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for LerpDataStateContract {
    fn eq(&self, other: &Self) -> bool {
        const EPS: f32 = 0.0001;

        self.is_endless == other.is_endless &&
        f32::abs(self.start_time - other.start_time) < EPS &&
        f32::abs(self.end_time - other.end_time) < EPS &&
        f32::abs(self.start_value - other.start_value) < EPS &&
        f32::abs(self.end_value - other.end_value) < EPS &&
        f32::abs(self.duration - other.duration) < EPS
    }
}
impl Eq for LerpDataStateContract { }
impl Hash for LerpDataStateContract {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.is_endless.hash(state);

        state.write_u32(self.start_time as u32);
        state.write_u32(self.end_time as u32);
        state.write_u32(self.start_value as u32);
        state.write_u32(self.end_value as u32);
        state.write_u32(self.duration as u32);
    }
}

#[derive(Clone, Debug, Default)]
pub struct DiseaseDeltasStateContract {
    pub body_temperature_delta: f32,
    pub heart_rate_delta: f32,
    pub pressure_top_delta: f32,
    pub pressure_bottom_delta: f32,
    pub fatigue_delta: f32,
    pub stamina_drain: f32,
    pub oxygen_drain: f32,
    pub food_drain: f32,
    pub water_drain: f32
}
impl fmt::Display for DiseaseDeltasStateContract {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Disease deltas state")
    }
}
impl Eq for DiseaseDeltasStateContract { }
impl PartialEq for DiseaseDeltasStateContract {
    fn eq(&self, other: &Self) -> bool {
        const EPS: f32 = 0.0001;

        f32::abs(self.body_temperature_delta - other.body_temperature_delta) < EPS &&
        f32::abs(self.heart_rate_delta - other.heart_rate_delta) < EPS &&
        f32::abs(self.pressure_top_delta - other.pressure_top_delta) < EPS &&
        f32::abs(self.pressure_bottom_delta - other.pressure_bottom_delta) < EPS &&
        f32::abs(self.fatigue_delta - other.fatigue_delta) < EPS &&
        f32::abs(self.stamina_drain - other.stamina_drain) < EPS &&
        f32::abs(self.oxygen_drain - other.oxygen_drain) < EPS &&
        f32::abs(self.food_drain - other.food_drain) < EPS &&
        f32::abs(self.water_drain - other.water_drain) < EPS
    }
}
impl Hash for DiseaseDeltasStateContract {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32(self.body_temperature_delta as u32);
        state.write_u32(self.heart_rate_delta as u32);
        state.write_u32(self.pressure_top_delta as u32);
        state.write_u32(self.pressure_bottom_delta as u32);
        state.write_u32(self.fatigue_delta as u32);
        state.write_u32(self.stamina_drain as u32);
        state.write_u32(self.oxygen_drain as u32);
        state.write_u32(self.food_drain as u32);
        state.write_u32(self.water_drain as u32);
    }
}

impl StageDescription {
    pub(crate) fn get_state(&self) -> StageDescriptionStateContract {
        StageDescriptionStateContract {
            is_endless: self.is_endless,
            level: self.level,
            reaches_peak_in_hours: self.reaches_peak_in_hours,
            chance_of_death: self.chance_of_death.clone(),
            target_water_drain: self.target_water_drain,
            target_food_drain: self.target_food_drain,
            target_stamina_drain: self.target_stamina_drain,
            target_fatigue_delta: self.target_fatigue_delta,
            target_pressure_top: self.target_pressure_top,
            target_pressure_bottom: self.target_pressure_bottom,
            target_heart_rate: self.target_heart_rate,
            target_body_temp: self.target_body_temp,
            self_heal_chance: self.self_heal_chance.clone()
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
            water_data: self.water_data.iter().map(|x| x.get_state()).collect(),
            food_data: self.food_data.iter().map(|x| x.get_state()).collect(),
            stamina_data: self.stamina_data.iter().map(|x| x.get_state()).collect(),
            fatigue_data: self.fatigue_data.iter().map(|x| x.get_state()).collect(),
            pressure_top_data: self.pressure_top_data.iter().map(|x| x.get_state()).collect(),
            pressure_bottom_data: self.pressure_bottom_data.iter().map(|x| x.get_state()).collect(),
            heart_rate_data: self.heart_rate_data.iter().map(|x| x.get_state()).collect(),
            body_temp_data: self.body_temp_data.iter().map(|x| x.get_state()).collect(),
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
    /// Adds new active disease based on the previously saved state
    ///
    /// # Parameters
    /// - `disease_data`: saved disease state (from `ActiveDisease.get_state` method call)
    /// - `disease`: disease instance
    ///
    /// ## Notes
    /// Borrows `diseases` collection
    pub fn restore_disease(&self, disease_data: &ActiveDiseaseStateContract, disease: Box<dyn Disease>) {
        let mut b = self.diseases.borrow_mut();
        let treatment = disease.get_treatment();
        let name = disease.get_name().to_string();
        let d = ActiveDisease {
            disease: Rc::new(disease),
            needs_treatment: disease_data.needs_treatment,
            will_self_heal_on: disease_data.will_self_heal_on,
            total_duration: disease_data.total_duration,

            stages: RefCell::new(BTreeMap::new()),
            last_deltas: RefCell::new(DiseaseDeltasC::empty()),
            initial_data: RefCell::new(Vec::new()),
            end_time: RefCell::new(None),
            lerp_data: RefCell::new(None),
            is_inverted: Cell::new(false),
            activation_time: RefCell::new(GameTimeC::empty()),
            will_end: Cell::new(false),
            treatment: Rc::new(treatment),
            message_queue: RefCell::new(BTreeMap::new())
        };

        d.set_state(disease_data);

        b.insert(name, Rc::new(d));
    }
}

impl ActiveDisease {
    pub fn get_state(&self) -> ActiveDiseaseStateContract {
        ActiveDiseaseStateContract {
            needs_treatment: self.needs_treatment,
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

    pub(crate) fn set_state(&self, state: &ActiveDiseaseStateContract) {
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
            target_body_temp: x.target_body_temp,
            target_heart_rate: x.target_heart_rate,
            target_pressure_top: x.target_pressure_top,
            target_pressure_bottom: x.target_pressure_bottom,
            target_fatigue_delta: x.target_fatigue_delta,
            target_stamina_drain: x.target_stamina_drain,
            target_food_drain: x.target_food_drain,
            target_water_drain: x.target_water_drain,
            reaches_peak_in_hours: x.reaches_peak_in_hours
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
                        target_water_drain: stage.info.target_water_drain,
                        target_food_drain: stage.info.target_food_drain,
                        target_stamina_drain: stage.info.target_stamina_drain,
                        target_fatigue_delta: stage.info.target_fatigue_delta,
                        target_pressure_bottom: stage.info.target_pressure_bottom,
                        target_pressure_top: stage.info.target_pressure_top,
                        target_heart_rate: stage.info.target_heart_rate,
                        target_body_temp: stage.info.target_body_temp,
                        is_endless: stage.info.is_endless,
                        level: stage.info.level.clone(),
                        chance_of_death: stage.info.chance_of_death.clone(),
                        self_heal_chance: stage.info.self_heal_chance.clone(),
                    }
                });
            }
        }

        self.last_deltas.replace(DiseaseDeltasC{
            body_temperature_delta: state.last_deltas.body_temperature_delta,
            heart_rate_delta: state.last_deltas.heart_rate_delta,
            pressure_top_delta: state.last_deltas.pressure_top_delta,
            pressure_bottom_delta: state.last_deltas.pressure_bottom_delta,
            fatigue_delta: state.last_deltas.fatigue_delta,
            food_drain: state.last_deltas.food_drain,
            stamina_drain: state.last_deltas.stamina_drain,
            water_drain: state.last_deltas.water_drain,
            oxygen_drain: state.last_deltas.oxygen_drain
        });

        match &state.lerp_data {
            Some(l) => Some(LerpDataNodeC{
                start_time: l.start_time,
                end_time: l.end_time,
                is_for_inverted: l.is_for_inverted,
                is_endless: l.is_endless,
                water_data: l.water_data.iter().map(|x| LerpDataC {
                    start_time: x.start_time,
                    end_time: x.end_time,
                    duration: x.duration,
                    start_value: x.start_value,
                    end_value: x.end_value,
                    is_endless: x.is_endless
                }).collect(),
                food_data: l.food_data.iter().map(|x| LerpDataC {
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
                }).collect(),
                fatigue_data: l.fatigue_data.iter().map(|x| LerpDataC {
                    start_time: x.start_time,
                    end_time: x.end_time,
                    duration: x.duration,
                    start_value: x.start_value,
                    end_value: x.end_value,
                    is_endless: x.is_endless
                }).collect(),
                pressure_top_data: l.pressure_top_data.iter().map(|x| LerpDataC {
                    start_time: x.start_time,
                    end_time: x.end_time,
                    duration: x.duration,
                    start_value: x.start_value,
                    end_value: x.end_value,
                    is_endless: x.is_endless
                }).collect(),
                pressure_bottom_data: l.pressure_bottom_data.iter().map(|x| LerpDataC {
                    start_time: x.start_time,
                    end_time: x.end_time,
                    duration: x.duration,
                    start_value: x.start_value,
                    end_value: x.end_value,
                    is_endless: x.is_endless
                }).collect(),
                heart_rate_data: l.heart_rate_data.iter().map(|x| LerpDataC {
                    start_time: x.start_time,
                    end_time: x.end_time,
                    duration: x.duration,
                    start_value: x.start_value,
                    end_value: x.end_value,
                    is_endless: x.is_endless
                }).collect(),
                body_temp_data: l.body_temp_data.iter().map(|x| LerpDataC {
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

impl DiseaseDeltasC {
    pub(crate) fn get_state(&self) -> DiseaseDeltasStateContract {
        DiseaseDeltasStateContract {
            oxygen_drain: self.oxygen_drain,
            water_drain: self.water_drain,
            stamina_drain: self.stamina_drain,
            food_drain: self.food_drain,
            fatigue_delta: self.fatigue_delta,
            pressure_top_delta: self.pressure_top_delta,
            pressure_bottom_delta: self.pressure_bottom_delta,
            heart_rate_delta: self.heart_rate_delta,
            body_temperature_delta: self.body_temperature_delta
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