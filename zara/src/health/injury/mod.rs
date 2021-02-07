use crate::utils::event::{MessageQueue, Event};
use crate::utils::{GameTimeC};
use crate::health::{StageLevel, InjuryKey};
use crate::health::injury::fluent::{StageInit};
use crate::inventory::items::{InventoryItem, ApplianceC};
use crate::body::{BodyPart};

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

/// Macro for declaring a simple injury
///
/// # Examples
///
/// ```
/// zara::injury!(Cut, "Cut",
///     Some(Box::new(CutTreatment)),
///     vec![
///         StageBuilder::start()
///             .build_for(StageLevel::InitialStage)
///                 .self_heal(20)
///                 .drains()
///                     .stamina(0.2)
///                     .blood_level(0.08)
///                 .deadly()
///                     .with_chance_of_death(0)
///                 .will_reach_target_in(0.3)
///                 .will_end()
///             .build(),
///         // and so on...
///     ]
/// );
/// ```
#[macro_export]
macro_rules! injury(
    ($t:ty, $nm:expr, $trt:expr, $st:expr) => (
        impl zara::health::injury::Injury for $t {
            fn get_name(&self) -> String { format!($nm) }
            fn get_stages(&self) -> Vec<zara::health::injury::StageDescription> {
                $st as Vec<zara::health::injury::StageDescription>
            }
            fn get_treatment(&self) -> Option<Box<dyn zara::health::injury::InjuryTreatment>> {
                $trt
            }
            fn get_is_fracture(&self) -> bool { false }
        }
    );
);

/// Macro for declaring a fracture injury
///
/// # Examples
///
/// ```
/// use zara::health::disease::StageBuilder;
/// use zara::health::StageLevel;
///
/// zara::fracture!(Fracture, "Fracture",
///     Some(Box::new(FractureTreatment)),
///     vec![
///         StageBuilder::start()
///             .build_for(StageLevel::InitialStage)
///                 .no_self_heal()
///                 .drains()
///                     .stamina(0.2)
///                     .blood_level(0.08)
///                 .deadly()
///                     .with_chance_of_death(0)
///                 .will_reach_target_in(0.3)
///                 .will_end()
///             .build(),
///         // and so on...
///     ]
/// );
/// ```
#[macro_export]
macro_rules! fracture(
    ($t:ty, $nm:expr, $trt:expr, $st:expr) => (
        impl zara::health::injury::Injury for $t {
            fn get_name(&self) -> String { format!($nm) }
            fn get_stages(&self) -> Vec<zara::health::injury::StageDescription> {
                $st as Vec<zara::health::injury::StageDescription>
            }
            fn get_treatment(&self) -> Option<Box<dyn zara::health::injury::InjuryTreatment>> {
                $trt
            }
            fn get_is_fracture(&self) -> bool { true }
        }
    );
);

impl InjuryKey {
    pub fn new(injury: String, body_part: BodyPart) -> Self {
        InjuryKey {
            injury,
            body_part
        }
    }
}

/// Builds an injury stage.
///
/// # Examples
/// Start with `start` method and call `build` when you're done.
/// ```
/// use zara::health::injury::{StageBuilder};
/// use zara::health::{StageLevel};
///
/// StageBuilder::start()
///     .build_for(StageLevel::InitialStage)
///         .self_heal(15)
///         .drains(); // and so on...
/// //  .build();
/// ```
pub struct StageBuilder {
    level: RefCell<StageLevel>,
    self_heal_chance: RefCell<Option<usize>>,
    reaches_peak_in_hours: Cell<f32>,
    is_endless: Cell<bool>,
    target_stamina_drain: Cell<f32>,
    target_blood_drain: Cell<f32>,
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
                target_stamina_drain: Cell::new(0.),
                target_blood_drain: Cell::new(0.)
            }
        )
    }
}

/// Here you can describe any injury treatment logic based on the appliances
pub trait InjuryTreatment {
    /// Called on all active injuries when player takes an appliance (bandage, injection, etc)
    ///
    /// # Parameters
    /// - `game_time`: game time when this call happened
    /// - `item`: appliance item description
    /// - `body_part`: part of the body where this appliance was applied
    /// - `active_stage`: instance of the active stage of an injury
    /// - `injury`: injury object itself. You can call `invert` or `invert_back` to start or stop
    ///     "curing" the injury
    /// - `inventory_items`: all inventory items. Used item is still in this list at the
    ///     moment of this call
    fn on_appliance_taken(&self, game_time: &GameTimeC, item: &ApplianceC, body_part: BodyPart,
                          active_stage: &ActiveStage, injury: &ActiveInjury,
                          inventory_items: &HashMap<String, Box<dyn InventoryItem>>);
}

/// Describes injury stage
#[derive(Copy, Clone)]
pub struct StageDescription {
    /// Level of seriousness (order)
    pub level: StageLevel,
    /// Probability of injury start self-healing during this stage
    pub self_heal_chance: Option<usize>,
    /// Probability of death from this injury during this stage.
    pub chance_of_death: Option<usize>,
    /// In what time will reach peak values
    pub reaches_peak_in_hours: f32,
    /// How long this stage will last
    pub is_endless: bool,
    /// Target blood drain for this stage (0..100 percents per game second)
    pub target_blood_drain: f32,
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
            target_stamina_drain: self.target_stamina_drain,
            target_blood_drain: self.target_blood_drain,
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

/// Describes deltas calculated by the active injury
pub struct InjuryDeltasC {
    pub stamina_drain: f32,
    pub blood_drain: f32
}

impl InjuryDeltasC {
    pub fn empty() -> Self {
        InjuryDeltasC {
            stamina_drain: 0.,
            blood_drain: 0.
        }
    }
    pub fn for_related() -> Self {
        InjuryDeltasC {
            stamina_drain: 0.,
            blood_drain: 0.
        }
    }
    pub fn cleanup(&mut self){
        // No "max" logic here (yet?)
    }
    pub fn copy(&self) -> InjuryDeltasC {
        InjuryDeltasC {
            stamina_drain: self.stamina_drain,
            blood_drain: self.blood_drain
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

/// Trait that must be implemented by all injuries
pub trait Injury {
    /// Gets the unique name of this injury kind
    fn get_name(&self) -> String;
    /// Gets all injury stages. Use [`StageBuilder`](zara::health::injury::StageBuilder) to
    /// describe a stage
    fn get_stages(&self) -> Vec<StageDescription>;
    /// Treatment instance associated with this injury object
    fn get_treatment(&self) -> Option<Box<dyn InjuryTreatment>>;
    /// True if injury is a fracture
    fn get_is_fracture(&self) -> bool;
}

struct LerpDataNodeC {
    start_time: f32,
    end_time: f32,
    stamina_data: Vec<LerpDataC>,
    blood_data: Vec<LerpDataC>,
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
pub struct ActiveInjury {
    /// Injury instance linked to this `ActiveDisease`
    pub injury: Rc<Box<dyn Injury>>,
    /// Disease needs treatment or will self-heal
    pub needs_treatment: bool,
    /// On which stage level injury will start self-healing (`StageLevel::Undefined` if none)
    pub will_self_heal_on: StageLevel,
    /// Total duration of all stages, from first start to last peak.
    pub total_duration: Duration,
    /// Body part associated with this injury
    pub body_part: BodyPart,
    /// Is this injury a fracture
    pub is_fracture: bool,

    // Private fields
    /// Initial stages data given by user
    initial_data: RefCell<Vec<StageDescription>>,
    /// Disease stages with calculated timings and order
    stages: RefCell<BTreeMap<StageLevel, ActiveStage>>,
    /// Calculated data for lerping
    lerp_data: RefCell<Option<LerpDataNodeC>>,
    /// Calculated on the last frame deltas
    last_deltas: RefCell<InjuryDeltasC>,
    /// Is disease chain inverted (`invert` was called)
    is_inverted: Cell<bool>,
    /// When this disease will become active
    activation_time: RefCell<GameTimeC>,
    /// Do this disease have an end
    will_end: Cell<bool>,
    /// Disease end time, if applicable
    end_time: RefCell<Option<GameTimeC>>,
    /// Treatment object associated with this disease
    treatment: Rc<Option<Box<dyn InjuryTreatment>>>,
    /// Blood loss stopped from "outside"
    blood_loss_stop: Cell<bool>,

    // Messages queued for sending on the next frame
    message_queue: RefCell<BTreeMap<usize, Event>>
}
impl ActiveInjury {
    /// Creates new active disease object
    ///
    /// # Parameters
    /// - `injury`: instance of an object with the [`Injury`](crate::zara::health::injury::Injury) trait
    /// - `body_part`: body part associated with this injury
    /// - `activation_time`: game time when this injury will start to be active. Use the
    ///     current game time to activate immediately
    pub fn new(injury: Box<dyn Injury>, body_part: BodyPart, activation_time: GameTimeC) -> Self {
        let mut stages: BTreeMap<StageLevel, ActiveStage> = BTreeMap::new();
        let mut time_elapsed= activation_time.to_duration();
        let mut will_end = true;
        let mut self_heal = false;
        let mut self_heal_level = StageLevel::Undefined;
        let initial_data = injury.get_stages();

        for stage in injury.get_stages().iter() {
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
        let treatment = injury.get_treatment();
        let is_fracture = injury.get_is_fracture();

        ActiveInjury {
            injury: Rc::new(injury),
            body_part,
            is_fracture,
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
            lerp_data: RefCell::new(None), // will be calculated on first get_drain_deltas
            last_deltas: RefCell::new(InjuryDeltasC::empty()),
            blood_loss_stop: Cell::new(false),
            message_queue: RefCell::new(BTreeMap::new())
        }
    }


    /// Is called by Zara from the health engine when person takes an appliance
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

    /// Temporary stop blood drain. You can call [`resume_blood_loss`] to resume it
    ///
    /// [`resume_blood_loss`]: #method.resume_blood_loss
    pub fn stop_blood_loss(&self) {
        self.blood_loss_stop.set(true);

        self.queue_message(Event::BloodLossStopped(self.injury.get_name().to_string(), self.body_part));
    }

    /// Resumes stopped by the [`stop_blood_loss`] call blood drain
    ///
    /// [`stop_blood_loss`]: #method.stop_blood_loss
    pub fn resume_blood_loss(&self) {
        self.blood_loss_stop.set(false);

        self.queue_message(Event::BloodLossResumed(self.injury.get_name().to_string(), self.body_part));
    }
}

impl MessageQueue for ActiveInjury {
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