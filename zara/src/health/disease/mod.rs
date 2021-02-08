use crate::utils::event::{MessageQueue, Event};
use crate::health::{Health, StageLevel};
use crate::utils::{FrameSummaryC, GameTimeC};
use crate::health::disease::fluent::{StageInit};
use crate::inventory::items::{InventoryItem, ConsumableC, ApplianceC};
use crate::body::BodyPart;

use std::rc::Rc;
use std::cell::{Cell, RefCell, RefMut};
use std::collections::{BTreeMap, HashMap};
use std::time::Duration;

pub(crate) mod state;

mod crud;
mod fluent;
mod lerp;
mod chain;
mod status_methods;

/// Macro for declaring a disease. Use [`StageBuilder`](crate::zara::health::disease::StageBuilder)
/// to create a stage
///
/// # Examples
///
/// ```
/// use zara::health::disease::StageBuilder;
/// use zara::health::StageLevel;
///
/// zara::disease!(Flu, "Flu",
///     Some(Box::new(FluTreatment)),
///     vec! [
///         StageBuilder::start()
///             .build_for(StageLevel::InitialStage)
///                 .self_heal(15)
///                 .vitals()
///                     .with_target_body_temp(37.6)
///                     .with_target_heart_rate(85.)
///                     .with_target_blood_pressure(130., 90.)
///                     .will_reach_target_in(0.1)
///                     .will_end()
///                 .drains()
///                     .stamina(0.2)
///                     .food_level(0.05)
///                     .water_level(0.1)
///                 .affects_fatigue(5.)
///                 .no_death_probability()
///             .build(),
///             // and so on...
///     ]
/// );
/// ```
#[macro_export]
macro_rules! disease(
    ($t:ty, $nm:expr, $trt:expr, $st:expr) => (
        impl zara::health::disease::Disease for $t {
            fn get_name(&self) -> String { format!($nm) }
            fn get_stages(&self) -> Vec<zara::health::disease::StageDescription> {
                $st as Vec<zara::health::disease::StageDescription>
            }
            fn get_treatment(&self) -> Option<Box<dyn zara::health::disease::DiseaseTreatment>> {
                $trt
            }
        }
    );
);

/// Builds a disease stage.
///
/// # Examples
/// Start with `start` method and call `build` when you're done.
/// ```
/// use zara::health::disease::StageBuilder;
/// use zara::health::StageLevel;
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
    target_pressure_bottom: Cell<f32>,
    target_fatigue_delta: Cell<f32>,
    target_stamina_drain: Cell<f32>,
    target_food_drain: Cell<f32>,
    target_water_drain: Cell<f32>,
    chance_of_death: RefCell<Option<usize>>
}

impl StageBuilder {
    pub fn start() -> Box<dyn StageInit> {
        Box::new(
            StageBuilder {
                level: RefCell::new(StageLevel::Undefined),
                self_heal_chance: RefCell::new(None),
                chance_of_death: RefCell::new(None),
                is_endless: Cell::new(false),
                reaches_peak_in_hours: Cell::new(0.),
                target_body_temp: Cell::new(0.),
                target_heart_rate: Cell::new(0.),
                target_pressure_top: Cell::new(0.),
                target_pressure_bottom: Cell::new(0.),
                target_fatigue_delta: Cell::new(0.),
                target_stamina_drain: Cell::new(0.),
                target_food_drain: Cell::new(0.),
                target_water_drain: Cell::new(0.)
            }
        )
    }
}

/// Here you can describe any disease treatment logic based on the consumed items (food/pills/etc)
/// or an appliance (bandage, injection, etc)
pub trait DiseaseTreatment {
    /// Called on all active diseases when player eats something
    ///
    /// # Parameters
    /// - `game_time`: game time when this call happened
    /// - `item`: consumable item description
    /// - `active_stage`: instance of the active stage of a disease
    /// - `disease`: disease object itself. You can call `invert` or `invert_back` to start or stop
    ///     "curing" the disease
    ///  - `inventory_items`: all inventory items. Consumed item is still in this list at the
    ///     moment of this call
    fn on_consumed(&self, game_time: &GameTimeC, item: &ConsumableC, active_stage: &ActiveStage,
                   disease: &ActiveDisease, inventory_items: &HashMap<String, Box<dyn InventoryItem>>);

   /// Called on all active diseases when appliance is taken (bandage, injection, etc)
   ///
   /// # Parameters
   /// - `game_time`: game time when this call happened
   /// - `item`: appliance item description
   /// - `body_part`: part of the body where this appliance was applied
   /// - `active_stage`: instance of the active stage of a disease
   /// - `disease`: disease object itself. You can call `invert` or `invert_back` to start or stop
   ///     "curing" the disease
   /// - `inventory_items`: all inventory items. Consumed item is still in this list at the
   ///     moment of this call
    fn on_appliance_taken(&self, game_time: &GameTimeC, item: &ApplianceC, body_part: BodyPart,
                          active_stage: &ActiveStage, disease: &ActiveDisease,
                          inventory_items: &HashMap<String, Box<dyn InventoryItem>>);
}

/// Describes disease stage
#[derive(Copy, Clone)]
pub struct StageDescription {
    /// Level of seriousness (order)
    pub level: StageLevel,
    /// Probability of disease start self-healing during this stage
    pub self_heal_chance: Option<usize>,
    /// Probability of death from this disease during this stage.
    pub chance_of_death: Option<usize>,
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
    pub target_pressure_bottom: f32,
    /// Target fatigue delta value (0..100 percents) at the end of this stage
    pub target_fatigue_delta: f32,
    /// Target food drain for this stage (0..100 percents per game second)
    pub target_food_drain: f32,
    /// Target water drain for this stage (0..100 percents per game second)
    pub target_water_drain: f32,
    /// Target stamina drain for this stage (0..100 percents per game second)
    pub target_stamina_drain: f32
}

impl StageDescription {
    pub fn copy(&self) -> StageDescription {
        StageDescription {
            level: self.level,
            self_heal_chance: match self.self_heal_chance { Some(o) => Some(o), None => None },
            chance_of_death: match self.chance_of_death { Some(o) => Some(o), None => None },
            reaches_peak_in_hours: self.reaches_peak_in_hours,
            is_endless: self.is_endless,
            target_body_temp: self.target_body_temp,
            target_heart_rate: self.target_heart_rate,
            target_pressure_top: self.target_pressure_top,
            target_pressure_bottom: self.target_pressure_bottom,
            target_fatigue_delta: self.target_fatigue_delta,
            target_stamina_drain: self.target_stamina_drain,
            target_food_drain: self.target_food_drain,
            target_water_drain: self.target_water_drain
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
    /// Duration of the stage
    pub duration: Duration
}

/// Describes deltas calculated by the active diseases
pub struct DiseaseDeltasC {
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

impl DiseaseDeltasC {
    pub fn empty() -> Self {
        DiseaseDeltasC {
            body_temperature_delta: 0.,
            heart_rate_delta: 0.,
            pressure_top_delta: 0.,
            pressure_bottom_delta: 0.,
            fatigue_delta: 0.,
            stamina_drain: 0.,
            oxygen_drain: 0.,
            food_drain: 0.,
            water_drain: 0.
        }
    }
    pub(crate) fn for_related() -> Self {
        DiseaseDeltasC {
            body_temperature_delta: -1000.,
            heart_rate_delta: -1000.,
            pressure_top_delta: -1000.,
            pressure_bottom_delta: -1000.,
            fatigue_delta: 0.,
            stamina_drain: 0.,
            food_drain: 0.,
            water_drain: 0.,
            oxygen_drain: 0.
        }
    }
    pub(crate) fn cleanup(&mut self){
        if self.heart_rate_delta < -900. { self.heart_rate_delta = 0.; }
        if self.body_temperature_delta < -900. { self.body_temperature_delta = 0.; }
        if self.pressure_top_delta < -900. { self.pressure_top_delta = 0.; }
        if self.pressure_bottom_delta < -900. { self.pressure_bottom_delta = 0.; }
    }
    pub fn copy(&self) -> DiseaseDeltasC {
        DiseaseDeltasC {
            body_temperature_delta: self.body_temperature_delta,
            heart_rate_delta: self.heart_rate_delta,
            pressure_top_delta: self.pressure_top_delta,
            pressure_bottom_delta: self.pressure_bottom_delta,
            fatigue_delta: self.fatigue_delta,
            stamina_drain: self.stamina_drain,
            food_drain: self.food_drain,
            water_drain: self.water_drain,
            oxygen_drain: self.oxygen_drain
        }
    }
}

impl ActiveStage {
    /// Checks if stage is active for a given time
    pub fn is_active(&self, game_time: &GameTimeC) -> bool {
        let start = self.start_time.as_secs_f32();
        let peak = self.peak_time.as_secs_f32();
        let gt = game_time.as_secs_f32();

        return if self.info.is_endless {
            gt >= start
        } else {
            gt >= start && gt <= peak
        }
    }

    /// Returns percent of activity of this stage. Always in 0..100 range.
    ///
    /// # Parameters
    /// - `game_time`: game time for which to calculate the value
    pub fn percent_active(&self, game_time: &GameTimeC) -> usize {
        let gt = game_time.as_secs_f32();
        let start = self.start_time.as_secs_f32();
        let end = self.peak_time.as_secs_f32();
        let d = end - start;

        if d < 0. { return 0; }
        if gt >= end { return 100; }
        if gt <= start { return 0; }

        let gt_d = gt - start;

        return ((gt_d/d) * 100.) as usize;
    }

    pub fn copy(&self) -> ActiveStage {
        ActiveStage {
            info: self.info.copy(),
            peak_time: self.peak_time.copy(),
            start_time: self.start_time.copy(),
            duration: self.duration.clone()
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
    /// - `game_time`: health controller object. It can be used to call `spawn_disease` for example
    /// - `item`: consumable description
    /// - `inventory_items`: all inventory items. Consumed item is still in this list at the
    ///     moment of this call
    fn on_consumed(&self, health: &Health, game_time: &GameTimeC, item: &ConsumableC,
                   inventory_items: &HashMap<String, Box<dyn InventoryItem>>);

    /// Being called when player takes an appliance (like bandage or injection)
    ///
    /// # Parameters
    /// - `health`: health controller object. It can be used to call `spawn_disease` for example
    /// - `game_time`: health controller object. It can be used to call `spawn_disease` for example
    /// - `item`: appliance description
    /// - `body_part`: body part to which this item was applied
    /// - `inventory_items`: all inventory items. Applied item is still in this list at the
    ///     moment of this call
    fn on_appliance_taken(&self, health: &Health, game_time: &GameTimeC, item: &ApplianceC,
                          body_part: BodyPart, inventory_items: &HashMap<String, Box<dyn InventoryItem>>);
}

/// Trait that must be implemented by all diseases
pub trait Disease {
    /// Gets the unique name of this disease kind
    fn get_name(&self) -> String;
    /// Gets all disease stages. Use [`StageBuilder`](zara::health::disease::StageBuilder) to
    /// describe a stage
    fn get_stages(&self) -> Vec<StageDescription>;
    /// Treatment instance associated with this disease object
    fn get_treatment(&self) -> Option<Box<dyn DiseaseTreatment>>;
}

struct LerpDataNodeC {
    start_time: f32,
    end_time: f32,
    body_temp_data: Vec<LerpDataC>,
    heart_rate_data: Vec<LerpDataC>,
    pressure_top_data: Vec<LerpDataC>,
    pressure_bottom_data: Vec<LerpDataC>,
    fatigue_data: Vec<LerpDataC>,
    stamina_data: Vec<LerpDataC>,
    food_data: Vec<LerpDataC>,
    water_data: Vec<LerpDataC>,
    is_endless: bool,
    is_for_inverted: bool
}

#[derive(Default, Copy, Clone)]
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
    /// Disease needs treatment or will self-heal
    pub needs_treatment: bool,
    /// On which stage level disease will start self-healing (`StageLevel::Undefined` if none)
    pub will_self_heal_on: StageLevel,
    /// Total duration of all stages, from first start to last peak
    pub total_duration: Duration,

    // Private fields
    /// Initial stages data given by user
    initial_data: RefCell<Vec<StageDescription>>,
    /// Disease stages with calculated timings and order
    stages: RefCell<BTreeMap<StageLevel, ActiveStage>>,
    /// Calculated data for lerping
    lerp_data: RefCell<Option<LerpDataNodeC>>,
    /// Calculated on the last frame deltas
    last_deltas: RefCell<DiseaseDeltasC>,
    /// Is disease chain inverted (`invert` was called)
    is_inverted: Cell<bool>,
    /// When this disease will become active
    activation_time: RefCell<GameTimeC>,
    /// Do this disease have an end
    will_end: Cell<bool>,
    /// Disease end time, if applicable
    end_time: RefCell<Option<GameTimeC>>,
    /// Treatment object associated with this disease
    treatment: Rc<Option<Box<dyn DiseaseTreatment>>>,

    /// Messages queued for sending on the next frame
    message_queue: RefCell<BTreeMap<usize, Event>>
}
impl ActiveDisease {
    /// Creates new active disease object
    ///
    /// # Parameters
    /// - `disease`: instance of an object with the [`Disease`](crate::zara::health::disease::Disease) trait
    /// - `activation_time`: game time when this disease will start to be active. Use the
    ///     current game time to activate immediately
    pub fn new(disease: Box<dyn Disease>, activation_time: GameTimeC) -> Self {
        let mut stages: BTreeMap<StageLevel, ActiveStage> = BTreeMap::new();
        let mut time_elapsed= activation_time.to_duration();
        let mut will_end = true;
        let mut self_heal = false;
        let mut self_heal_level = StageLevel::Undefined;
        let initial_data = disease.get_stages();

        for stage in disease.get_stages().iter() {
            match stage.self_heal_chance {
                Some(c) => {
                    if !self_heal && crate::utils::roll_dice(c) {
                        self_heal_level = stage.level;
                        self_heal = true;
                    }
                },
                None => { }
            }

            let start_time = GameTimeC::from_duration(time_elapsed);
            let peak_duration = Duration::from_secs_f32(stage.reaches_peak_in_hours*60.*60.);
            let peak_time = GameTimeC::from_duration(time_elapsed + peak_duration);

            stages.insert(stage.level, ActiveStage {
                info: *stage,
                start_time,
                peak_time,
                duration: peak_duration.clone()
            });

            if stage.is_endless && will_end {
                will_end = false;
            }

            time_elapsed = time_elapsed + peak_duration;
        }

        let end_time = if will_end { Some(GameTimeC::from_duration(time_elapsed)) } else { None };
        let treatment = disease.get_treatment();

        ActiveDisease {
            disease: Rc::new(disease),
            treatment: Rc::new(treatment),
            initial_data: RefCell::new(initial_data),
            is_inverted: Cell::new(false),
            total_duration: time_elapsed,
            activation_time: RefCell::new(activation_time),
            stages: RefCell::new(stages),
            will_end: Cell::new(will_end),
            end_time: RefCell::new(end_time),
            needs_treatment: !self_heal,
            will_self_heal_on: self_heal_level,
            lerp_data: RefCell::new(None), // will be calculated on first get_vitals_deltas
            last_deltas: RefCell::new(DiseaseDeltasC::empty()),
            message_queue: RefCell::new(BTreeMap::new())
        }
    }

    /// Is called by Zara from the health engine when person consumes an item
    pub(crate) fn on_consumed(&self, game_time: &GameTimeC, item: &ConsumableC,
                       inventory_items: &HashMap<String, Box<dyn InventoryItem>>) {
        if !self.is_active(game_time) { return; }

        match self.treatment.as_ref() {
            Some(t) => match self.get_active_stage(game_time) {
                Some(st) => t.on_consumed(game_time, item, &st, &self, inventory_items),
                None => { }
            },
            None => { }
        };
    }

    /// Is called by Zara from the health engine when appliance is taken
    pub(crate) fn on_appliance_taken(&self, game_time: &GameTimeC, item: &ApplianceC, body_part: BodyPart,
                                     inventory_items: &HashMap<String, Box<dyn InventoryItem>>) {
        if !self.is_active(game_time) { return; }

        match self.treatment.as_ref() {
            Some(t) => match self.get_active_stage(game_time) {
                Some(st) => t.on_appliance_taken(game_time, item, body_part, &st, &self, inventory_items),
                None => { }
            },
            None => { }
        };
    }
}

impl MessageQueue for ActiveDisease {
    fn has_messages(&self) -> bool { self.message_queue.borrow().len() > 0 }

    fn queue_message(&self, message: Event) {
        let mut q = self.message_queue.borrow_mut();
        let id = q.len();

        q.insert(id, message);
    }

    fn get_message_queue(&self) -> RefMut<'_, BTreeMap<usize, Event>> {
        self.message_queue.borrow_mut()
    }
}