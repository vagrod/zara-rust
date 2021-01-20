use crate::health::{Health};
use crate::utils::{FrameSummaryC, ConsumableC, GameTimeC};
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

/// Describes active stages
pub struct ActiveStage {
    /// Stage data
    pub info: StageDescription,
    /// When this stage should start
    pub start_time: GameTimeC,
    /// When this stage reaches its peak
    pub peak_time: GameTimeC,
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
    stages: Rc<RefCell<HashMap<StageLevel, ActiveStage>>>
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
            stages: Rc::new(RefCell::new(stages)),
            will_end,
            end_time,
            needs_treatment: !self_heal
        }
    }

    /// Is called by Zara from the health engine
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