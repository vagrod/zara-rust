use std::time::{Duration};
use std::cell::Cell;
use rand::Rng;

use event::{Dispatcher, Listener};
use core::ops;

pub mod event;

/// Structure for storing all needed frame data for controllers
/// including events dispatcher
pub struct FrameC<'a, E: Listener + 'static> {
    pub data: &'a FrameSummaryC,
    pub events: &'a mut Dispatcher<E>
}

/// Structure for storing frame meta info
pub struct FrameSummaryC {
    /// Game time snapshot for this frame
    pub game_time: GameTimeC,
    /// Player status snapshot for this frame
    pub player: PlayerStatusC,
    /// Environment snapshot for this frame
    pub environment: EnvironmentC,
    /// Health snapshot for this frame
    pub health: HealthC,
    /// How many game seconds passed since last call
    pub game_time_delta: f32,
}

/// Structure that holds game time.
///
/// Can be converted from and to `Duration`.
///
/// # Properties
/// - `day`: day of game time (whole number)
/// - `hour`: day of game time whole number)
/// - `minute`: day of game time (whole number)
/// - `second`: day of game time (with floating point)
/// - `duration`: `Duration` that corresponds to the above values
pub struct GameTime {
    /// Day of the game time (whole number)
    pub day : Cell<u64>,
    /// Hour of the game time (whole number)
    pub hour : Cell<u64>,
    /// Minute of the game time (whole number)
    pub minute : Cell<u64>,
    /// Second of the game time (with floating point)
    pub second : Cell<f64>,
    /// `Duration` that corresponds to the values contained in other fields
    pub duration: Cell<Duration>
}

impl GameTime {
    /// Creates new zero game time.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use zara::utils;
    ///
    /// let gt = utils::GameTime::new();
    /// ```
    pub fn new() -> Self {
        GameTime {
            day: Cell::new(0),
            hour : Cell::new(0),
            minute: Cell::new(0),
            second: Cell::new(0.),
            duration: Cell::new(Duration::new(0, 0))
        }
    }

    /// Creates new `GameTime` object from a given `Duration` object
    ///
    /// # Parameters
    /// - `d`: `Duration` object to create new game time from
    ///
    /// # Examples
    /// Basic usage:
    ///
    /// ```
    /// use zara::utils;
    ///
    /// let game_time = utils::GameTime::from_duration(duration);
    /// ```
    pub fn from_duration(d: Duration) -> GameTime {
        let gt = GameTime::new();

        gt.update_from_duration(d);

        return gt;
    }

    /// Creates new `GameTime` object from its simple representation
    pub fn from_contract(gt: GameTimeC) -> Self {
        GameTime::from_duration(gt.to_duration())
    }

    /// Creates `GameTimeC` contract from this `GameTime` instance
    pub fn to_contract(&self) -> GameTimeC {
        GameTimeC {
            day: self.day.get(),
            hour: self.hour.get(),
            minute: self.minute.get(),
            second: self.second.get()
        }
    }

    /// Adds given `Duration` value to this game time
    ///
    /// # Parameters
    /// - `d`: `Duration` object to add
    ///
    /// # Examples
    /// Basic usage:
    ///
    /// ```
    /// game_time.add_duration(duration);
    /// ```
    pub fn add_duration(&self, d: Duration) {
        let new_values = self.duration.get() + d;

        self.update_from_duration(new_values);
    }

    /// Adds given number of seconds to this game time
    ///
    /// # Parameters
    /// - `value`: seconds to add
    ///
    /// # Examples
    /// Basic usage:
    ///
    /// ```
    /// game_time.add_seconds(amount);
    /// ```
    pub fn add_seconds(&self, value: f32) {
        let new_seconds = self.duration.get().as_secs_f64() + value as f64;

        self.update_from_seconds(new_seconds);
    }

    /// Updates this game time to match a given `GameTime`
    ///
    /// # Parameters
    /// - `new_values`: `GameTime` object to match
    ///
    /// # Examples
    /// Basic usage:
    ///
    /// ```
    /// game_time.update_from(duration);
    /// ```
    pub fn update_from(&self, new_values: &GameTime) {
        self.second.set(new_values.second.get());
        self.minute.set(new_values.minute.get());
        self.hour.set(new_values.hour.get());
        self.day.set(new_values.day.get());
    }

    /// Updates all fields inside this game time to match the given `Duration`
    ///
    /// # Parameters
    /// - `d`: `Duration` object to apply to this game time
    ///
    /// # Examples
    /// Basic usage:
    ///
    /// ```
    /// game_time.update_from_duration(duration);
    /// ```
    pub fn update_from_duration(&self, d: Duration){
        let secs_passed_f64 = d.as_secs_f64();

        self.update_from_seconds(secs_passed_f64);
    }

    /// Updates all fields inside this game time to match the given seconds amount
    ///
    /// # Parameters
    /// - `new_seconds`: amount of seconds
    ///
    /// # Examples
    /// Basic usage:
    ///
    /// ```
    /// game_time.update_from_seconds(amount);
    /// ```
    pub fn update_from_seconds(&self, new_seconds: f64){
        let second = new_seconds % 60_f64;
        let secs_passed = new_seconds;
        let minutes_passed = ((secs_passed / 60_f64) as u64) as u64;
        let minute = minutes_passed % 60_u64;
        let hours_passed = ((minutes_passed / 60_u64) as u64) as u64;
        let hour = hours_passed % 24_u64;
        let day = ((hours_passed / 24_u64) as u64) as u64;

        self.day.set(day);
        self.hour.set(hour);
        self.minute.set(minute);
        self.second.set(second);
        self.duration.set(Duration::from_secs_f64(new_seconds));
    }

}

/// Structure for storing simple game time slice
/// #[derive(Copy)]
pub struct GameTimeC {
    pub day: u64,
    pub hour: u64,
    pub minute: u64,
    pub second: f64
}

impl GameTimeC {
    pub fn empty() -> Self {
        GameTimeC {
            day: 0,
            hour: 0,
            minute: 0,
            second: 0.
        }
    }

    pub fn new(day: u64, hour: u64, minute: u64, second: f64) -> Self {
        GameTimeC {
            day,
            minute,
            hour,
            second
        }
    }

    /// Returns new `GameTimeC` by adding a given amount of minutes
    /// to the current one
    pub fn add_minutes(&self, amount: u64) -> GameTimeC {
        let d= self.to_duration() + Duration::from_secs(amount*60);

        GameTimeC::from_duration(d)
    }

    /// Returns `Duration` object that describes current `GameTimeC`
    pub fn to_duration(&self) -> Duration {
        Duration::from_secs_f64(
            self.second+((self.minute*60+self.hour*60*60+self.day*24*60*60) as f64))
    }

    pub fn from_duration(d: Duration) -> Self {
        GameTime::from_duration(d).to_contract()
    }

    /// Creates a copy of `GameTimeC`
    pub fn copy(&self) -> GameTimeC {
        GameTimeC {
            day: self.day,
            hour: self.hour,
            minute: self.minute,
            second: self.second
        }
    }
}

impl ops::Add<GameTimeC> for GameTimeC {
    type Output = GameTimeC;

    fn add(self, _rhs: GameTimeC) -> GameTimeC {
        let d = self.to_duration() + _rhs.to_duration();

        GameTime::from_duration(d).to_contract()
    }
}

impl ops::Sub<GameTimeC> for GameTimeC {
    type Output = GameTimeC;

    fn sub(self, _rhs: GameTimeC) -> GameTimeC {
        let d = self.to_duration() - _rhs.to_duration();

        GameTime::from_duration(d).to_contract()
    }
}

/// Structure for storing health snapshot
pub struct HealthC {
    /// Body temperature (degrees C)
    pub body_temperature: f32,
    /// Heart rate (bpm)
    pub heart_rate: f32,
    /// Top body pressure (mmHg)
    pub top_pressure: f32,
    /// Bottom body pressure (mmHg)
    pub bottom_pressure: f32,
    /// Blood level (0..100)
    pub blood_level: f32,
    /// Food level (0..100)
    pub food_level: f32,
    /// Water level (0..100)
    pub water_level: f32,
    /// Stamina level (0..100)
    pub stamina_level: f32,
    /// Fatigue level (0..100)
    pub fatigue_level: f32,
    /// List of active (or scheduled) diseases
    pub diseases: Vec<ActiveDiseaseC>
}
impl HealthC {
    pub fn healthy() -> Self {
        HealthC {
            blood_level: 100.,
            body_temperature: 36.6,
            top_pressure: 120.,
            bottom_pressure: 70.,
            food_level: 100.,
            water_level: 100.,
            heart_rate: 64.,
            stamina_level: 100.,
            fatigue_level: 0.,
            diseases: Vec::new()
        }
    }
}

/// Structure for storing active disease snapshot
pub struct ActiveDiseaseC {
    pub name: String,
    pub scheduled_time: GameTimeC,
    pub is_active: bool
}

/// Describes initial environment information
pub struct EnvironmentC {
    /// Wind speed value (m/s)
    pub wind_speed: f32,
    /// Temperature, degrees C
    pub temperature : f32,
    /// Rain intensity, 0..1
    pub rain_intensity : f32
}

impl EnvironmentC {
    /// Creates new environment description object.
    ///
    /// To create an empty (default) environment description,
    /// use [`empty`] method.
    ///
    /// [`empty`]: #method.empty
    ///
    /// # Parameters
    /// - `temperature`: temperature, degrees C
    /// - `rain_intansity`: rain intensity, 0..1
    /// - `wind_speed`: m/s
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use zara::utils;
    ///
    /// let env = utils::EnvironmentC::new(25., 3., 0.12);
    /// ```
    pub fn new(temperature: f32, wind_speed: f32, rain_intensity: f32) -> EnvironmentC {
        EnvironmentC {
            wind_speed,
            temperature,
            rain_intensity
        }
    }

    /// Creates default environment description object.
    ///
    /// To create environment description with default values,
    /// use [`new`] method.
    ///
    /// [`new`]: #method.new
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use zara::utils;
    ///
    /// let env = utils::EnvironmentC::default();
    /// ```
    pub fn default() -> EnvironmentC {
        EnvironmentC::new(26., 0., 0.)
    }
}

/// Describes consumable contract
pub struct ConsumableC {
    /// Unique name of the item
    pub name: String,
    /// Is this consumable a food
    pub is_food: bool,
    /// Is this consumable a water
    pub is_water: bool,
    /// How many items of this type has been consumed
    pub consumed_count: usize
}

impl ConsumableC {
    pub fn new() -> Self {
        ConsumableC {
            name: String::new(),
            is_food: false,
            is_water: false,
            consumed_count: 0
        }
    }
}

/// Describes a snapshot of the player state for a single frame
pub struct PlayerStatusC {
    pub is_walking: bool,
    pub is_running: bool,
    pub is_swimming: bool,
    pub is_underwater: bool,
    pub is_sleeping: bool,
    pub last_slept: GameTimeC,
    pub last_slept_duration: f64
}

/// Classic linear lerp
pub fn lerp(first: f32, second: f32, by: f32) -> f32 {
    first * (1. - by) + second * by
}

/// Clamp both ways
pub fn clamp(value: f32, floor: f32, ceiling: f32) -> f32 {
    if value > ceiling {
        return ceiling;
    }

    if value < floor {
        return floor;
    }

    return value;
}

/// Clamps ceiling
pub fn clamp_to(value: f32, ceiling: f32) -> f32 {
    if value > ceiling {
        return ceiling;
    }

    return value;
}

/// Clamps ceiling
pub fn clamp_bottom(value: f32, floor: f32) -> f32 {
    if value < floor {
        return floor;
    }

    return value;
}


/// Clamps 0..1
pub fn clamp_01(value: f32) -> f32 {
    if value > 1. {
        return 1.;
    }
    if value < 0. {
        return 0.;
    }

    return value;
}

/// Will return `true` is a given probability is satisfied
pub fn roll_dice(probability: usize) -> bool {
    let mut rng = rand::thread_rng();
    let r = rng.gen_range(0..100);

    return r <= probability;
}

/// Will return a random number between this two
pub fn range(a: f32, b: f32) -> f32 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(a..b);
}

/// Box equality check
pub fn eq<T: ?Sized>(left: &Box<T>, right: &Box<T>) -> bool {
    let left : *const T = left.as_ref();
    let right : *const T = right.as_ref();
    left == right
}