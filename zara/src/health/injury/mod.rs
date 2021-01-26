use crate::utils::event::{MessageQueue, Event};
use crate::utils::{GameTimeC};
use crate::health::StageLevel;
use crate::health::injury::fluent::{StageInit};
use crate::inventory::items::{InventoryItem, ApplianceC};
use crate::body::{BodyParts};

use std::rc::Rc;
use std::cell::{Cell, RefCell, RefMut};
use std::collections::{BTreeMap, HashMap};
use std::time::Duration;

mod crud;
mod fluent;
mod lerp;
mod chain;

/// Macro for declaring a simple injury
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

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InjuryKey {
    pub injury: String,
    pub body_part: BodyParts
}
impl InjuryKey {
    pub fn new(injury: String, body_part: BodyParts) -> Self {
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
    /// ## Parameters
    /// - `game_time`: game time when this call happened
    /// - `item`: appliance item description
    /// - `body_part`: part of the body where this appliance was applied
    /// - `active_stage`: instance of the active stage of an injury
    /// - `injury`: injury object itself. You can call `invert` or `invert_back` to start or stop
    ///     "curing" the injury
    /// - `inventory_items`: all inventory items. Used item is still in this list at the
    ///     moment of this call
    fn on_appliance_taken(&self, game_time: &GameTimeC, item: &ApplianceC, body_part: BodyParts,
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
    pub fn get_is_active(&self, game_time: &GameTimeC) -> bool {
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
    pub fn get_percent_active(&self, game_time: &GameTimeC) -> usize {
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
    pub body_part: BodyParts,
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
    /// - `injury`: instance of an object with the [`Injury`](crate::health::injury::Injury) trait
    /// - `body_part`: body part associated with this injury
    /// - `activation_time`: game time when this injury will start to be active. Use the
    ///     current game time to activate immediately
    pub fn new(injury: Box<dyn Injury>, body_part: BodyParts, activation_time: GameTimeC) -> Self {
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
        let is_fracture= injury.get_is_fracture();

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

    /// Gets if this injury will end (is it finite)
    pub fn get_will_end(&self) -> bool { self.will_end.get() }

    /// Gets if this injury is now healing (is inverted)
    pub fn get_is_healing(&self) -> bool { self.is_inverted.get() }

    /// Gets the end time of this injury, if it is finite
    pub fn get_end_time(&self) -> Option<GameTimeC> {
        let b = self.end_time.borrow();

        match b.as_ref() {
            Some(o) => Some(o.copy()),
            None => None
        }
    }

    /// Gets a copy of active injury stage data for a given time
    pub fn get_active_stage(&self, game_time: &GameTimeC) -> Option<ActiveStage> {
        for (_, stage) in self.stages.borrow().iter() {
            if stage.get_is_active(game_time) { return Some(stage.copy()) }
        }

        return None;
    }

    /// Gets active stage level for a given game time
    pub fn get_active_level(&self, game_time: &GameTimeC) -> Option<StageLevel> {
        self.get_active_stage(game_time).map(|st| st.info.level)
    }

    /// Returns a copy of a game time structure containing data of when this injury was activated
    pub fn get_activation_time(&self) -> GameTimeC { self.activation_time.borrow().copy() }

    /// Returns a copy of stage data by its level
    pub fn get_stage(&self, level: StageLevel) -> Option<ActiveStage> {
        for (l, stage) in self.stages.borrow().iter() {
            if level as i32 == *l as i32 { return Some(stage.copy()) }
        }

        return None;
    }

    /// Gets whether injury is active or not for a given time
    pub fn get_is_active(&self, game_time: &GameTimeC) -> bool {
        let activation_secs = self.activation_time.borrow().as_secs_f32();
        let game_time_secs = game_time.as_secs_f32();

        return if self.will_end.get() {
            let b = self.end_time.borrow();
            let border_secs = match b.as_ref() {
                Some(t) => t.as_secs_f32(),
                None => game_time_secs
            };

            game_time_secs >= activation_secs && game_time_secs <= border_secs
        } else {
            game_time_secs >= activation_secs
        }
    }

    /// Returns `true` if this injury already passed and is no longer relevant, for a given game time
    pub fn get_is_old(&self, game_time: &GameTimeC) -> bool {
        let gt = game_time.as_secs_f32();
        return match self.end_time.borrow().as_ref() {
            Some(t) => gt > t.as_secs_f32(),
            None => false
        }
    }

    /// Is called by Zara from the health engine when person takes an appliance
    pub fn on_appliance_taken(&self, game_time: &GameTimeC, item: &ApplianceC, body_part: BodyParts,
                       inventory_items: &HashMap<String, Box<dyn InventoryItem>>) {
        if !self.get_is_active(game_time) { return; }

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
    }

    /// Resumes stopped by the [`stop_blood_loss`] call blood drain
    ///
    /// [`stop_blood_loss`]: #method.stop_blood_loss
    pub fn resume_blood_loss(&self) {
        self.blood_loss_stop.set(false);
    }

    /// Gets if blood loss has been temporary stopped by the [`stop_blood_loss`] call
    ///
    /// [`stop_blood_loss`]: #method.stop_blood_loss
    pub fn get_is_blood_stopped(&self) -> bool {
        self.blood_loss_stop.get()
    }
}

impl MessageQueue for ActiveInjury {
    fn has_messages(&self) -> bool {
        self.message_queue.borrow().len() > 0
    }

    fn queue_message(&self, message: Event) {
        let mut q = self.message_queue.borrow_mut();
        let id = q.len();

        q.insert(id, message);
    }

    fn get_message_queue(&self) -> RefMut<'_, BTreeMap<usize, Event>> {
        self.message_queue.borrow_mut()
    }
}