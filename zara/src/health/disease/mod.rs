use crate::health::{Health};
use crate::utils::{FrameSummaryC, ConsumableC, GameTimeC};
use crate::health::disease::fluent::{StageInit};

mod crud;
mod fluent;

use std::rc::Rc;
use std::cell::{Cell, RefCell};

pub struct StageBuilder {
    level: RefCell<StageLevel>,
    self_heal: RefCell<Option<f32>>,
    duration_hours: Cell<f32>,
    target_body_temp: Cell<f32>,
    target_heart_rate: Cell<f32>,
    target_pressure_top: Cell<f32>,
    target_pressure_bottom: Cell<f32>
}

#[derive(Copy, Clone)]
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
                self_heal: RefCell::new(None),
                duration_hours: Cell::new(0.),
                target_body_temp: Cell::new(0.),
                target_heart_rate: Cell::new(0.),
                target_pressure_top: Cell::new(0.),
                target_pressure_bottom: Cell::new(0.)
            }
        )
    }
}

pub struct StageDescription {
    pub level: StageLevel,
    pub self_heal: Option<f32>,
    pub duration_hours: f32,
    pub target_body_temp: f32,
    pub target_heart_rate: f32,
    pub target_pressure_top: f32,
    pub target_pressure_bottom: f32
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
}

/// Describes an active disease that can be also scheduled
pub struct ActiveDisease {
    /// Disease instance linked to this `ActiveDisease`
    pub disease: Rc<Box<dyn Disease>>,
    /// When this disease will become active
    pub activation_time: GameTimeC
}
impl ActiveDisease {
    /// Creates new active disease object
    ///
    /// # Parameters
    /// - `disease`: instance of an object with the [`Disease`](crate::health::disease::Disease) trait
    /// - `activation_time`: game time when this disease will start to be active. Use the
    ///     current game time to activate immediately
    pub fn new(disease: Box<dyn Disease>, activation_time: GameTimeC) -> Self {
        ActiveDisease {
            disease: Rc::new(disease),
            activation_time
        }
    }
}