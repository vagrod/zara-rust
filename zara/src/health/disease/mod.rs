use crate::health::{Health};
use crate::utils::{FrameSummaryC, ConsumableC, GameTimeC, HealthC,
                   lerp, clamp_01};
use crate::health::disease::fluent::{StageInit};

mod crud;
mod fluent;

use std::rc::Rc;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::time::Duration;

/// Macro for declaring a disease
#[macro_export]
macro_rules! disease(
    ($t:ty, $nm:expr, $st:expr) => (
        impl zara::health::disease::Disease for $t {
            fn get_name(&self) -> String { String::from($nm) }
            fn get_stages(&self) -> Vec<zara::health::disease::StageDescription> {
                $st as Vec<zara::health::disease::StageDescription>
            }
        }
    );
);

/// Builds a disease stage.
///
/// # Examples
/// Start with `start` method and call `build` when you're done.
/// ```
/// use zara::health::disease::{StageBuilder, StageLevel};
///
/// StageBuilder::start()
///     .build_for(StageLevel::InitialStage)
///         .self_heal(15)
///         .vitals(); // and so on...
/// //  .build();
/// ```
pub struct StageBuilder {
    level: RefCell<StageLevel>,
    self_heal_chance: RefCell<Option<usize>>,
    reaches_peak_in_hours: Cell<f32>,
    is_endless: Cell<bool>,
    target_body_temp: Cell<f32>,
    target_heart_rate: Cell<f32>,
    target_pressure_top: Cell<f32>,
    target_pressure_bottom: Cell<f32>
}

/// Disease stage level of seriousness
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum StageLevel {
    HealthyStage,
    InitialStage,
    Progressing,
    Worrying,
    Critical
}

impl StageBuilder {
    pub fn start() -> Box<dyn StageInit> {
        Box::new(
            StageBuilder {
                level: RefCell::new(StageLevel::HealthyStage),
                self_heal_chance: RefCell::new(None),
                is_endless: Cell::new(false),
                reaches_peak_in_hours: Cell::new(0.),
                target_body_temp: Cell::new(0.),
                target_heart_rate: Cell::new(0.),
                target_pressure_top: Cell::new(0.),
                target_pressure_bottom: Cell::new(0.)
            }
        )
    }
}

/// Describes disease stage
#[derive(Copy, Clone)]
pub struct StageDescription {
    /// Level of seriousness (order)
    pub level: StageLevel,
    /// Will self-heal
    pub self_heal_chance: Option<usize>,
    /// In what time will reach peak values
    pub reaches_peak_in_hours: f32,
    /// How long this stage will last
    pub is_endless: bool,
    /// Stage's target body temperature
    pub target_body_temp: f32,
    /// Stage's target heart rate
    pub target_heart_rate: f32,
    /// Stage's target body pressure (top)
    pub target_pressure_top: f32,
    /// Stage's target body pressure (bottom)
    pub target_pressure_bottom: f32
}

impl StageDescription {
    pub fn copy(&self) -> StageDescription {
        StageDescription {
            level: self.level,
            self_heal_chance: if self.self_heal_chance.is_some() { Some(self.self_heal_chance.unwrap()) } else { None },
            reaches_peak_in_hours: self.reaches_peak_in_hours,
            is_endless: self.is_endless,
            target_body_temp: self.target_body_temp,
            target_heart_rate: self.target_heart_rate,
            target_pressure_top: self.target_pressure_top,
            target_pressure_bottom: self.target_pressure_bottom
        }
    }
}

/// Describes active stages
pub struct ActiveStage {
    /// Stage data
    pub info: StageDescription,
    /// When this stage should start
    pub start_time: GameTimeC,
    /// When this stage reaches its peak
    pub peak_time: GameTimeC,
}

/// Describes deltas calculated by the active diseases
pub struct DiseaseDeltasC {
    pub body_temperature_delta: f32,
    pub heart_rate_delta: f32,
    pub pressure_top_delta: f32,
    pub pressure_bottom_delta: f32
}

impl DiseaseDeltasC {
    pub fn empty() -> Self {
        DiseaseDeltasC {
            body_temperature_delta: 0.,
            heart_rate_delta: 0.,
            pressure_top_delta: 0.,
            pressure_bottom_delta: 0.
        }
    }
    pub fn negative() -> Self {
        DiseaseDeltasC {
            body_temperature_delta: -1000.,
            heart_rate_delta: -1000.,
            pressure_top_delta: -1000.,
            pressure_bottom_delta: -1000.
        }
    }
    pub fn cleanup(&mut self){
        if self.heart_rate_delta < -900. { self.heart_rate_delta = 0.; }
        if self.body_temperature_delta < -900. { self.body_temperature_delta = 0.; }
        if self.pressure_top_delta < -900. { self.pressure_top_delta = 0.; }
        if self.pressure_bottom_delta < -900. { self.pressure_bottom_delta = 0.; }
    }
}

impl ActiveStage {
    /// Checks if stage if active for a given time
    pub fn get_is_active(&self, game_time: &GameTimeC) -> bool {
        let start = self.start_time.to_duration().as_secs_f32();
        let peak = self.peak_time.to_duration().as_secs_f32();
        let gt = game_time.to_duration().as_secs_f32();

        return if self.info.is_endless {
            gt >= start
        } else {
            gt >= start && gt <= peak
        }
    }

    pub fn copy(&self) -> ActiveStage {
        ActiveStage {
            info: self.info.copy(),
            peak_time: self.peak_time.copy(),
            start_time: self.start_time.copy(),
        }
    }
}

/// Trait for disease monitors
pub trait DiseaseMonitor {
    /// Being called once a `UPDATE_INTERVAL` real seconds.
    ///
    /// # Parameters
    /// - `health`: health controller object. It can be used to call `spawn_disease` for example
    /// - `frame_data`: summary containing all environmental data, game time, health snapshot and etc.
    fn check(&self, health: &Health, frame_data: &FrameSummaryC);

    /// Being called when player consumes food or water
    ///
    /// # Parameters
    /// - `health`: health controller object. It can be used to call `spawn_disease` for example
    /// - `item`: consumable item summary info
    fn on_consumed(&self, health: &Health, game_time: &GameTimeC, item: &ConsumableC);
}

/// Trait that must be implemented by all diseases
pub trait Disease {
    /// Gets the unique name of this disease kind
    fn get_name(&self) -> String;
    /// Gets all disease stages. Use [`StageBuilder`](zara::health::disease::StageBuilder) to
    /// describe a stage
    fn get_stages(&self) -> Vec<StageDescription>;
}

struct LerpDataNodeC {
    start_time: f32,
    end_time: f32,
    body_temp_data: Vec<LerpDataC>,
    heart_rate_data: Vec<LerpDataC>,
    pressure_top_data: Vec<LerpDataC>,
    pressure_bottom_data: Vec<LerpDataC>,
    is_endless: bool
}
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

struct LerpDataC {
    start_time: f32,
    end_time: f32,
    start_value: f32,
    end_value: f32,
    duration: f32,
    is_endless: bool
}

/// Describes an active disease that can be also scheduled
pub struct ActiveDisease {
    /// Disease instance linked to this `ActiveDisease`
    pub disease: Rc<Box<dyn Disease>>,
    /// When this disease will become active
    pub activation_time: GameTimeC,
    /// Do this disease have an end
    pub will_end: bool,
    /// Disease end time, if applicable
    pub end_time: Option<GameTimeC>,
    /// Disease needs treatment or will self-heal
    pub needs_treatment: bool,

    /// Disease stages for lerping
    stages: RefCell<HashMap<StageLevel, ActiveStage>>,
    lerp_data: RefCell<Option<LerpDataNodeC>>
}
impl ActiveDisease {
    /// Creates new active disease object
    ///
    /// # Parameters
    /// - `disease`: instance of an object with the [`Disease`](crate::health::disease::Disease) trait
    /// - `activation_time`: game time when this disease will start to be active. Use the
    ///     current game time to activate immediately
    pub fn new(disease: Box<dyn Disease>, activation_time: GameTimeC) -> Self {
        let mut stages: HashMap<StageLevel, ActiveStage> = HashMap::new();
        let mut time_elapsed= activation_time.to_duration();
        let mut will_end = true;
        let mut self_heal = false;

        for stage in disease.get_stages().iter() {
            if stage.self_heal_chance.is_some() {
                if crate::utils::roll_dice(stage.self_heal_chance.unwrap()) {
                    self_heal = true;
                }
            }

            let start_time = GameTimeC::from_duration(time_elapsed);
            let peak_duration = Duration::from_secs_f32(stage.reaches_peak_in_hours*60.*60.);
            let peak_time = GameTimeC::from_duration(time_elapsed + peak_duration);

            stages.insert(stage.level, ActiveStage {
                info: *stage,
                start_time,
                peak_time
            });

            if stage.is_endless && will_end {
                will_end = false;
            }

            time_elapsed = time_elapsed + peak_duration;
        }

        let end_time = if will_end { Some(GameTimeC::from_duration(time_elapsed)) } else { None };

        ActiveDisease {
            disease: Rc::new(disease),
            activation_time,
            stages: RefCell::new(stages),
            will_end,
            end_time,
            needs_treatment: !self_heal,
            lerp_data: RefCell::new(None) // will be calculated on first get_vitals_deltas
        }
    }

    fn generate_lerp_data(&self, game_time: &GameTimeC) {
        self.lerp_data.replace(None);

        let mut lerp_data = LerpDataNodeC::new();
        let healthy = HealthC::healthy();
        let gt = game_time.to_duration().as_secs_f32();
        let mut last_start_body_temp = 0.;
        let mut last_start_heart_rate = 0.;
        let mut last_start_pressure_top = 0.;
        let mut last_start_pressure_bottom = 0.;
        let mut has_endless_child = false;

        lerp_data.start_time = gt;

        for (_, stage) in self.stages.borrow().iter() {
            let start = stage.start_time.to_duration().as_secs_f32();
            let end = stage.peak_time.to_duration().as_secs_f32();
            let duration = end - start;

            if stage.info.is_endless { has_endless_child = true; }

            if gt > end { continue; }

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
                    start_value: lerp(last_start_body_temp, end_value, p),
                    end_value,
                    duration: end - start_time,
                    is_endless: stage.info.is_endless
                };

                last_start_body_temp = ld.end_value;
                lerp_data.body_temp_data.push(ld);
            }
            // Heart Rate
            if stage.info.target_heart_rate > 0. {
                let end_value = stage.info.target_heart_rate - healthy.heart_rate;
                let ld = LerpDataC {
                    start_time,
                    end_time: end,
                    start_value: lerp(last_start_heart_rate, end_value, p),
                    end_value,
                    duration: end - start_time,
                    is_endless: stage.info.is_endless
                };

                last_start_heart_rate = ld.end_value;
                lerp_data.heart_rate_data.push(ld);
            }
            // Pressure Top
            if stage.info.target_pressure_top > 0. {
                let end_value = stage.info.target_pressure_top - healthy.top_pressure;
                let ld = LerpDataC {
                    start_time,
                    end_time: end,
                    start_value: lerp(last_start_pressure_top, end_value, p),
                    end_value,
                    duration: end - start_time,
                    is_endless: stage.info.is_endless
                };

                last_start_pressure_top = ld.end_value;
                lerp_data.pressure_top_data.push(ld);
            }
            // Pressure Bottom
            if stage.info.target_pressure_bottom > 0. {
                let end_value = stage.info.target_pressure_bottom - healthy.bottom_pressure;
                let ld = LerpDataC {
                    start_time,
                    end_time: end,
                    start_value: lerp(last_start_pressure_bottom, end_value, p),
                    end_value,
                    duration: end - start_time,
                    is_endless: stage.info.is_endless
                };

                last_start_pressure_bottom = ld.end_value;
                lerp_data.pressure_bottom_data.push(ld);
            }
        }

        lerp_data.is_endless = has_endless_child;

        self.lerp_data.replace(Some(lerp_data));
    }

    fn has_lerp_data_for(&self, game_time: &GameTimeC) -> bool {
        let ld_opt = self.lerp_data.borrow();

        if !ld_opt.is_some() { return false; }

        let gt = game_time.to_duration().as_secs_f32();
        let ld = ld_opt.as_ref().unwrap();

        if (gt >= ld.start_time && ld.is_endless) || (gt >= ld.start_time && gt <= ld.end_time)
        {
            return true;
        }

        return false;
    }

    /// Gets disease vitals delta for a given time
    pub fn get_vitals_deltas(&self, game_time: &GameTimeC) -> DiseaseDeltasC {
        let mut result = DiseaseDeltasC::empty();

        if !self.has_lerp_data_for(game_time) {
            self.generate_lerp_data(game_time);

            // Could not calculate lerps for some reason
            if !self.has_lerp_data_for(game_time) { return DiseaseDeltasC::empty(); }
        }

        let b = self.lerp_data.borrow();
        let lerp_data = b.as_ref().unwrap();
        let gt = game_time.to_duration().as_secs_f32();

        { // Body Temperature
            let mut ld = None;
            for data in lerp_data.body_temp_data.iter() {
                if (gt >= data.start_time && data.is_endless) || (gt >= data.start_time && gt <= data.end_time) {
                    ld = Some(data);
                    break;
                }
            }
            if ld.is_some() {
                let d = ld.unwrap();
                let mut p = clamp_01((gt - d.start_time) / d.duration);
                if d.is_endless { p = 1.; }
                result.body_temperature_delta = lerp(d.start_value, d.end_value, p);
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
            if ld.is_some() {
                let d = ld.unwrap();
                let mut p = clamp_01((gt - d.start_time) / d.duration);
                if d.is_endless { p = 1.; }
                result.heart_rate_delta = lerp(d.start_value, d.end_value, p);
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
            if ld.is_some() {
                let d = ld.unwrap();
                let mut p = clamp_01((gt - d.start_time) / d.duration);
                if d.is_endless { p = 1.; }
                result.pressure_top_delta = lerp(d.start_value, d.end_value, p);
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
            if ld.is_some() {
                let d = ld.unwrap();
                let mut p = clamp_01((gt - d.start_time) / d.duration);
                if d.is_endless { p = 1.; }
                result.pressure_bottom_delta = lerp(d.start_value, d.end_value, p);
            }
        }

        return result;
    }

    /// Gets a copy of active disease stage data for a given time
    pub fn get_active_stage(&self, game_time: &GameTimeC) -> Option<ActiveStage> {
        for (_, stage) in self.stages.borrow().iter() {
            if stage.get_is_active(game_time) { return Some(stage.copy()) }
        }

        return None;
    }

    /// Returns a copy of stage data if level is found
    fn get_stage(&self, level: StageLevel) -> Option<ActiveStage> {
        for (l, stage) in self.stages.borrow().iter() {
            if level == *l { return Some(stage.copy()) }
        }

        return None;
    }

    /// Is called by Zara from the health engine when person consumes an item
    pub fn on_consumed(&self, game_time: &GameTimeC, item: &ConsumableC) {

    }

    /// Gets whether disease is active or not for the given time
    pub fn get_is_active(&self, game_time: &GameTimeC) -> bool {
        let activation_secs = self.activation_time.to_duration().as_secs_f32();
        let game_time_secs = game_time.to_duration().as_secs_f32();

        if self.will_end {
            let border_secs = self.end_time.as_ref().unwrap().to_duration().as_secs_f32();

            return game_time_secs >= activation_secs && game_time_secs <= border_secs;
        } else {
            return game_time_secs >= activation_secs;
        }
    }
}