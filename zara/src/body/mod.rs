use crate::utils::{FrameC, GameTimeC, ClothesGroupC};
use crate::utils::event::{Dispatcher, Listener, Event, MessageQueue};
use crate::body::clothes::{ClothesGroup, ClothesItem};
use crate::body::clothes::fluent::ClothesGroupStart;

use std::cell::{Cell, RefCell, RefMut};
use std::time::Duration;
use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;
use std::fmt;
use std::hash::{Hash, Hasher};

mod status_methods;
mod body_appliance;

pub(crate) mod state;
pub mod clothes;

/// Node that controls player body information. Containing clothes, 
/// body appliances, player warmth and wetness levels
pub struct Body {
    /// Clothes that character is wearing now.
    ///
    /// # Important
    /// Do not alter this collection manually, use `ZaraController.put_on_clothes` and
    /// `ZaraController.take_off_clothes`, otherwise clothes will not be correctly synchronized
    /// between controllers
    /// 
    /// # Links
    /// See [this wiki article](https://github.com/vagrod/zara-rust/wiki/Clothes) for more info.
    pub clothes: Arc<RefCell<Vec<String>>>,
    /// Body appliances that character is wearing now.
    ///
    /// # Important
    /// Currently active body appliances. Do not alter this collection manually, 
    /// use `ZaraController.take_appliance` and `ZaraController.remove_appliance`
    /// 
    /// # Links
    /// See [this wiki article](https://github.com/vagrod/zara-rust/wiki/Body-Appliances) for more info.
    pub appliances: Arc<RefCell<Vec<BodyAppliance>>>,

    /// Game time when player slept last time
    last_sleep_time: RefCell<Option<GameTimeC>>,
    /// For how long player slept last time (game hours)
    last_sleep_duration: Cell<f32>,
    /// Is player sleeping now
    is_sleeping: Cell<bool>,
    /// Registered clothes groups
    clothes_groups: Arc<RefCell<HashMap<String, ClothesGroup>>>,
    /// Current matched clothes group
    clothes_group: RefCell<Option<ClothesGroupC>>,
    /// Active clothes resistance levels data
    clothes_data: RefCell<HashMap<String, ClothesItemC>>,
    /// Warmth level value
    warmth_level: Cell<f32>,
    /// Wetness level value
    wetness_level: Cell<f32>,
    
    // Counters and caches
    sleeping_counter: Cell<f64>,
    cached_world_temp: Cell<f32>,
    cached_wind_speed: Cell<f32>,
    cached_player_in_water: Cell<bool>,
    cached_rain_intensity: Cell<f32>,

    /// Messages queued for sending on the next frame
    message_queue: RefCell<BTreeMap<usize, Event>>
}

struct ClothesItemC {
    cold_resistance: usize,
    water_resistance: usize,
}

/// Body appliance data
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct BodyAppliance {
    /// Unique name of an appliance inventory item
    pub item_name: String,
    /// Body part where this appliance is located
    pub body_part: BodyPart
}
impl fmt::Display for BodyAppliance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} on {}", self.item_name, self.body_part)
    }
}
impl Hash for BodyAppliance {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.item_name.hash(state);
        self.body_part.hash(state);
    }
}

/// Used to describe a new clothes group. Use `start` method to begin.
///
/// # Examples
/// ```
/// use zara::body::ClothesGroupBuilder;
///
/// ClothesGroupBuilder.start()
///     .with_name("Water Resistant Suit")
///         .bonus_cold_resistance(5)
///         .bonus_water_resistance(16)
///         .includes(
///             vec![
///                 ("Jacket", Box::new(JacketClothes)),
///                 ("Pants", Box::new(PantsClothes))
///             ]
///         )
///     .build()
/// ```
/// 
/// # Links
/// See [this wiki article](https://github.com/vagrod/zara-rust/wiki/Clothes-groups) for more info.
pub struct ClothesGroupBuilder {
    pub(crate) name: RefCell<String>,
    pub(crate) bonus_cold_resistance: Cell<usize>,
    pub(crate) bonus_water_resistance: Cell<usize>,
    pub(crate) items: RefCell<HashMap<String, ClothesItem>>
}
impl ClothesGroupBuilder {
    /// Starts building process for a new clothes group.
    /// 
    /// # Examples
    /// ```
    /// use zara::body;
    /// 
    /// let o = body::ClothesGroupBuilder::start(); // and continue with `.with_name`
    /// ```
    /// 
    /// # Links
    /// See [this wiki article](https://github.com/vagrod/zara-rust/wiki/Clothes-groups) for more info.
    pub fn start() -> Box<dyn ClothesGroupStart> {
        Box::new(ClothesGroupBuilder {
            name: RefCell::new(String::new()),
            bonus_cold_resistance: Cell::new(0),
            bonus_water_resistance: Cell::new(0),
            items: RefCell::new(HashMap::new())
        })
    }
}

/// All body parts enum
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum BodyPart {
    Unknown = -1,
    Forehead = 0,
    Nape = 1,
    Eye = 2,
    Ear = 3,
    Nose = 4,
    Throat = 5,
    LeftShoulder = 6,
    RightShoulder = 7,
    LeftForearm = 8,
    RightForearm = 9,
    LeftSpokebone = 10,
    RightSpokebone = 11,
    LeftBrush = 12,
    RightBrush = 13,
    LeftChest = 14,
    RightChest = 15,
    Belly = 16,
    LeftHip = 17,
    RightHip = 18,
    LeftKnee = 19,
    RightKnee = 20,
    LeftShin = 21,
    RightShin = 22,
    LeftFoot = 23,
    RightFoot = 24,
    Back = 25
}
impl Default for BodyPart {
    fn default() -> Self {
        BodyPart::Unknown
    }
}
impl fmt::Display for BodyPart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Body {
    pub(crate) fn new() -> Self {
        Body {
            clothes: Arc::new(RefCell::new(Vec::new())),
            appliances: Arc::new(RefCell::new(Vec::new())),
            last_sleep_time: RefCell::new(Option::None),
            is_sleeping: Cell::new(false),
            sleeping_counter: Cell::new(0.),
            last_sleep_duration: Cell::new(0.),
            clothes_groups: Arc::new(RefCell::new(HashMap::new())),
            message_queue: RefCell::new(BTreeMap::new()),
            clothes_group: RefCell::new(None),
            clothes_data: RefCell::new(HashMap::new()),
            cached_wind_speed: Cell::new(-1000.),
            cached_world_temp: Cell::new(-1000.),
            cached_rain_intensity: Cell::new(0.),
            cached_player_in_water: Cell::new(false),
            warmth_level: Cell::new(0.),
            wetness_level: Cell::new(0.)
        }
    }

    /// This method is called every `UPDATE_INTERVAL` real seconds
    ///
    /// # Parameters
    /// - `frame`: summary information for this frame
    pub(crate) fn update<E: Listener + 'static>(&self, frame: &mut FrameC<E>){
        self.update_warmth_level_if_needed(
            frame.data.environment.temperature,
            frame.data.environment.wind_speed
        );
        self.update_wetness_level_if_needed(
            frame.data.game_time_delta,
            frame.data.player.is_swimming || frame.data.player.is_underwater,
            frame.data.environment.rain_intensity,
            frame.data.environment.temperature,
            frame.data.environment.wind_speed
        );
    }

    /// Is called every frame by Zara controller.
    /// Cannot be called in `update` because we need time precision
    pub(crate) fn sleep_check<E: Listener + 'static>
            (&self, events: &mut Dispatcher<E>, game_time: &Duration, game_time_delta: f32) {
        if self.is_sleeping.get(){
            let left = self.sleeping_counter.get() - game_time_delta as f64;

            if left <= 0.
            {
                self.is_sleeping.set(false);
                self.sleeping_counter.set(0.);
                self.last_sleep_time.replace(Option::Some(GameTimeC::from_duration(*game_time)));

                events.dispatch(Event::WokeUp);
            } else {
                self.sleeping_counter.set(left);
            }
        }
    }

    /// Starts sleeping. `is_sleeping` will be set to `true`, and on wake up `WokeUp` event will
    /// be triggered
    ///
    /// # Parameters
    /// - `game_hours`: for how many game hours should player sleep
    /// 
    /// # Examples
    /// ```
    /// person.body.start_sleeping(5.5);
    /// ```
    /// 
    /// # Links
    /// See [this wiki article](https://github.com/vagrod/zara-rust/wiki/Sleeping) for more info.
    pub fn start_sleeping(&self, game_hours: f32) -> bool {
        self.is_sleeping.set(true);
        self.sleeping_counter.set(game_hours as f64 * 60. * 60.);
        self.last_sleep_duration.set(game_hours);

        self.queue_message(Event::SleepStarted(game_hours));

        true
    }
}

impl MessageQueue for Body {
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