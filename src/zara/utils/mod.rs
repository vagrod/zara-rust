use std::time::{Duration};
use std::cell::Cell;

use event::{Dispatcher, Listener};

pub mod event;

/// Structure for storing all needed frame data for controllers
/// including events dispatcher
pub struct FrameC<'a, E: Listener + 'static> {
    pub data: &'a SummaryC,
    pub events: &'a mut Dispatcher<E>
}

/// Structure for storing frame meta info
pub struct SummaryC {
    /// Game time snapshot for this frame
    pub game_time: GameTimeC,
    /// Player status snapshot for this frame
    pub player: PlayerStatusC,
    /// Environment snapshot for this frame
    pub environment: EnvironmentC,
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
pub struct GameTimeC {
    pub day: u64,
    pub hour: u64,
    pub minute: u64,
    pub second: f64
}

/// Describes initial environment information
pub struct EnvironmentC {
    /// Wind speed value (m/s)
    pub wind_speed: f32
}

impl EnvironmentC {
    /// Creates new environment description object.
    ///
    /// To create an empty (default) environment description,
    /// use [`empty`] method.
    ///
    /// [`empty`]: #method.empty
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use zara::utils;
    ///
    /// let env = utils::EnvironmentC::new(wind_speed);
    /// ```
    pub fn new(wind_speed: f32) -> EnvironmentC {
        EnvironmentC {
            wind_speed
        }
    }

    /// Creates empty (default) environment description object.
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
    /// let env = utils::EnvironmentC::empty();
    /// ```
    pub fn empty() -> EnvironmentC{
        EnvironmentC::new(0.)
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
    pub is_walking: bool
}