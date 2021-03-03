use crate::health::StageLevel;
use crate::body::{BodyPart, BodyAppliance};

use std::time::{Duration};
use std::cell::Cell;
use rand::Rng;

use event::{Dispatcher, Listener};
use core::ops;
use std::fmt;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

pub mod event;

/// Structure for storing all needed frame data for controllers
/// including events dispatcher
pub struct FrameC<'a, E: Listener + 'static> {
    /// Snapshot of the current frame state
    pub data: &'a FrameSummaryC,
    /// Events dispatcher
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
/// 
/// # Links
/// See [this wiki article](https://github.com/vagrod/zara-rust/wiki/Game-Time) for more info.
#[derive(Default)]
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
impl fmt::Display for GameTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}d {}h {}m {:.1}s", self.day.get(), self.hour.get(), self.minute.get(), self.second.get())
    }
}
impl GameTime {
    /// Creates new zero game time.
    ///
    /// # Examples
    /// ```
    /// use zara::utils;
    ///
    /// let gt = utils::GameTime::new();
    /// ```
    /// 
    /// # Links
    /// See [this wiki article](https://github.com/vagrod/zara-rust/wiki/Game-Time) for more info.
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
    /// ```
    /// use zara::utils;
    ///
    /// let game_time = utils::GameTime::from_duration(duration);
    /// ```
    pub fn from_duration(d: Duration) -> GameTime {
        let gt = GameTime::new();

        gt.update_from_duration(d);

        gt
    }

    /// Creates new `GameTime` object from its simple representation
    /// 
    /// # Examples
    /// ```
    /// use zara::utils;
    ///
    /// let game_time = utils::GameTime::from_contract(game_time_contract);
    /// ```
    pub fn from_contract(gt: GameTimeC) -> Self {
        GameTime::from_duration(gt.to_duration())
    }

    /// Creates `GameTimeC` contract from this `GameTime` instance
    /// 
    /// # Examples
    /// ```
    /// let game_time_contract = game_time.to_contract();
    /// ```
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

/// Structure for storing simple game time slice.ActiveDiseaseC
/// 
/// # Links
/// See [this wiki article](https://github.com/vagrod/zara-rust/wiki/Game-Time) for more info.
#[derive(Copy, Clone, Debug, Default)]
pub struct GameTimeC {
    /// Day value
    pub day: u64,
    /// Hour value
    pub hour: u64,
    /// Minute value
    pub minute: u64,
    /// Second value
    pub second: f64
}
impl Ord for GameTimeC {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_duration().cmp(&other.to_duration())
    }
}
impl Eq for GameTimeC { }
impl PartialOrd for GameTimeC {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for GameTimeC {
    fn eq(&self, other: &Self) -> bool {
        const EPS: f64 = 0.0001;

        self.day == other.day &&
        self.hour == other.hour &&
        self.minute == other.minute &&
        f64::abs(self.second - other.second) < EPS
    }
}
impl fmt::Display for GameTimeC {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}d {}h {}m {:.1}s", self.day, self.hour, self.minute, self.second)
    }
}
impl Hash for GameTimeC {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.day.hash(state);
        self.hour.hash(state);
        self.minute.hash(state);

        state.write_u32((self.second*100_f64) as u32);
    }
}
impl GameTimeC {
    /// Creates empty (zero) `GameTimeC`
    /// 
    /// # Examples
    /// ```
    /// use zara::utils;
    /// 
    /// let game_time = utils::GameTimeC::empty();
    /// ```
    pub fn empty() -> Self {
        GameTimeC {
            day: 0,
            hour: 0,
            minute: 0,
            second: 0.
        }
    }

    /// Creates new `GameTimeC` with given time values
    /// 
    /// # Examples
    /// ```
    /// use zara::utils;
    /// 
    /// let game_time = utils::GameTimeC::new(0, 5, 52, 34.);
    /// ```
    pub fn new(day: u64, hour: u64, minute: u64, second: f64) -> Self {
        GameTimeC {
            day,
            minute,
            hour,
            second
        }
    }

    /// Returns `f32` that describes duration (in game seconds) of this `GameTimeC` instance
    /// 
    /// # Examples
    /// ```
    /// let value = game_time.as_secs_f32();
    /// ```
    pub fn as_secs_f32(&self) -> f32 {
        self.second as f32+
            (self.minute as f32)*60_f32+
            (self.hour as f32)*60_f32*60_f32+
            (self.day as f32)*24_f32*60_f32*60_f32
    }

    /// Returns new `GameTimeC` by adding a given amount of minutes
    /// to the current one
    /// 
    /// # Examples
    /// ```
    /// let new_game_time = game_time.add_minutes(12);
    /// ```
    pub fn add_minutes(&self, amount: u64) -> GameTimeC {
        let d= self.to_duration() + Duration::from_secs(amount*60);

        GameTimeC::from_duration(d)
    }

    /// Returns `Duration` object that describes current `GameTimeC`
    /// 
    /// # Examples
    /// ```
    /// let d = game_time.to_duration();
    /// ```
    pub fn to_duration(&self) -> Duration {
        Duration::from_secs_f64(
            self.second+((self.minute*60+self.hour*60*60+self.day*24*60*60) as f64))
    }

    /// Returns new `GameTimeC` instance based on the given `Duration` object
    /// 
    /// # Examples
    /// ```
    /// use zara::utils;
    /// 
    /// let game_time = utils::GameTimeC::from_duration(d);
    /// ```
    pub fn from_duration(d: Duration) -> Self {
        GameTime::from_duration(d).to_contract()
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

/// Describes clothes group object simplified contract
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Default)]
pub struct ClothesGroupC {
    /// Name of the group
    pub name: String,
    /// Group bonus cold resistance (0..100)
    pub bonus_cold_resistance: usize,
    /// Group bonus water resistance (0..100)
    pub bonus_water_resistance: usize
}
impl fmt::Display for ClothesGroupC {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Group {} data", self.name)
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
    /// Oxygen level (0..100)
    pub oxygen_level: f32,
    /// List of active (or scheduled) diseases
    pub diseases: Vec<ActiveDiseaseC>,
    /// List of active (or scheduled) injuries
    pub injuries: Vec<ActiveInjuryC>
}
impl HealthC {
    /// Return "healthy" contract instance, with all vitals set to 
    /// values that describes a healthy individual
    /// 
    /// # Examples
    /// ```
    /// use zara::utile;
    /// 
    /// let o = utils::HealthC::healthy();
    /// ```
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
            oxygen_level: 100.,
            diseases: Vec::new(),
            injuries: Vec::new()
        }
    }
}

/// Structure for storing active disease simplified contract
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Default)]
pub struct ActiveDiseaseC {
    /// Name of a disease
    pub name: String,
    /// Time when this disease becomes active
    pub scheduled_time: GameTimeC,
    /// Time when this disease ends (if ends)
    pub end_time: Option<GameTimeC>,
    /// Current disease active level
    pub current_level: StageLevel,
    /// Current disease active level progression (0..100)
    pub current_level_percent: usize,
    /// Is this disease active now
    pub is_active: bool,
    /// Is this disease inverted (healing) now
    pub is_healing: bool,
    /// Do this disease needs treatment (or will self-heal)
    pub needs_treatment: bool
}
impl fmt::Display for ActiveDiseaseC {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} @{}, active={}", self.name, self.scheduled_time, self.is_active)
    }
}

/// Structure for storing active injury simplified contract
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Default)]
pub struct ActiveInjuryC {
    /// Name of the injury
    pub name: String,
    /// Time when this injury becomes active
    pub scheduled_time: GameTimeC,
    /// Time when this injury ends (if ends)
    pub end_time: Option<GameTimeC>,
    /// Current injury active level
    pub current_level: StageLevel,
    /// Current injury active level progression (0..100)
    pub current_level_percent: usize,
    /// Is this injury active now
    pub is_active: bool,
    /// Is this injury inverted (healing) now
    pub is_healing: bool,
    /// Do this injury needs treatment (or will self-heal)
    pub needs_treatment: bool,
    /// Was blood forcibly stopped for this injury
    pub is_blood_stopped: bool,
    /// Body part where this injury resides
    pub body_part: BodyPart,
    /// Is this injury a fracture
    pub is_fracture: bool
}
impl fmt::Display for ActiveInjuryC {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} on {} @{}, fracture={}, active={}", self.name, self.body_part, self.scheduled_time,
               self.is_fracture, self.is_active)
    }
}

/// Describes initial environment information
/// 
/// # Links
/// See [this wiki article](https://github.com/vagrod/zara-rust/wiki/Environment) for more info.
#[derive(Clone, Debug, Default)]
pub struct EnvironmentC {
    /// Wind speed value (m/s)
    pub wind_speed: f32,
    /// Temperature, degrees C
    pub temperature : f32,
    /// Rain intensity, 0..1
    pub rain_intensity : f32
}
impl fmt::Display for EnvironmentC {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "World: temp {:.1}C, wind {:.1} m/s, rain {:.1}", self.temperature, self.wind_speed, self.rain_intensity)
    }
}
impl Eq for EnvironmentC { }
impl PartialEq for EnvironmentC {
    fn eq(&self, other: &Self) -> bool {
        const EPS: f32 = 0.0001;

        f32::abs(self.wind_speed - other.wind_speed) < EPS &&
        f32::abs(self.temperature - other.temperature) < EPS &&
        f32::abs(self.rain_intensity - other.rain_intensity) < EPS
    }
}
impl Hash for EnvironmentC {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_i32((self.temperature*10_000_f32) as i32);
        state.write_u32((self.wind_speed*10_000_f32) as u32);
        state.write_u32((self.rain_intensity*10_000_f32) as u32);
    }
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
    /// ```
    /// use zara::utils;
    ///
    /// let env = utils::EnvironmentC::new(25., 3., 0.12);
    /// ```
    /// 
    /// # Links
    /// See [this wiki article](https://github.com/vagrod/zara-rust/wiki/Environment) for more info.
    pub fn new(temperature: f32, wind_speed: f32, rain_intensity: f32) -> EnvironmentC {
        EnvironmentC {
            wind_speed,
            temperature,
            rain_intensity
        }
    }

    /// Creates default environment description object.
    ///
    /// To create environment description with default values (26 degrees C, no rain, no wind),
    /// use [`new`] method.
    ///
    /// [`new`]: #method.new
    ///
    /// # Examples
    /// ```
    /// use zara::utils;
    ///
    /// let env = utils::EnvironmentC::default();
    /// ```
    /// 
    /// # Links
    /// See [this wiki article](https://github.com/vagrod/zara-rust/wiki/Environment) for more info.
    pub fn default() -> EnvironmentC { EnvironmentC::new(26., 0., 0.) }
}

/// Simplified player state for a single frame
#[derive(Clone, Debug, Default)]
pub struct PlayerStatusC {
    /// Is player walking now
    pub is_walking: bool,
    /// Is player running now
    pub is_running: bool,
    /// Is player swimming now
    pub is_swimming: bool,
    /// Is player under the water now
    pub is_underwater: bool,
    /// Is player sleeping now
    pub is_sleeping: bool,
    /// Last time player slept (if any)
    pub last_slept: Option<GameTimeC>,
    /// For how long player slept last time
    pub last_slept_duration: f32,
    /// Player's current warmth level (-5..+5 is a comfort zone)
    pub warmth_level: f32,
    /// Player's current wetness level (0..100)
    pub wetness_level: f32,
    /// Clothes player is wearing now
    pub clothes: Vec<String>,
    /// Body appliances player is wearing now
    pub appliances: Vec<BodyAppliance>,
    /// Current clothes group (is any)
    pub clothes_group: Option<ClothesGroupC>,
    /// Total calculated water resistance value (0..100)
    pub total_water_resistance: usize,
    /// Total calculated cold resistance value (0..100)
    pub total_cold_resistance: usize,
    /// Player's current inventory weight
    pub inventory_weight: f32
}
impl fmt::Display for PlayerStatusC {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Player status ({} clothes, {} appliances, inventory {:.0}g)", self.clothes.len(),
               self.appliances.len(), self.inventory_weight)
    }
}
impl Eq for PlayerStatusC { }
impl PartialEq for PlayerStatusC {
    fn eq(&self, other: &Self) -> bool {
        const EPS: f32 = 0.0001;

        self.is_walking == other.is_walking &&
        self.is_running == other.is_running &&
        self.is_swimming == other.is_swimming &&
        self.is_underwater == other.is_underwater &&
        self.is_sleeping == other.is_sleeping &&
        self.last_slept == other.last_slept &&
        self.clothes == other.clothes &&
        self.appliances == other.appliances &&
        self.clothes_group == other.clothes_group &&
        self.total_water_resistance == other.total_water_resistance &&
        self.total_cold_resistance == other.total_cold_resistance &&
        f32::abs(self.last_slept_duration - other.last_slept_duration) < EPS &&
        f32::abs(self.warmth_level - other.warmth_level) < EPS &&
        f32::abs(self.wetness_level - other.wetness_level) < EPS &&
        f32::abs(self.inventory_weight - other.inventory_weight) < EPS
    }
}
impl Hash for PlayerStatusC {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.is_walking.hash(state);
        self.is_running.hash(state);
        self.is_swimming.hash(state);
        self.is_underwater.hash(state);
        self.is_sleeping.hash(state);
        self.last_slept.hash(state);
        self.clothes.hash(state);
        self.appliances.hash(state);
        self.clothes_group.hash(state);
        self.total_water_resistance.hash(state);
        self.total_cold_resistance.hash(state);

        state.write_u32((self.last_slept_duration*10_000_f32) as u32);
        state.write_i32((self.warmth_level*10_000_f32) as i32);
        state.write_u32((self.wetness_level*10_000_f32) as u32);
        state.write_u32((self.inventory_weight*1_000_f32) as u32);
    }
}

/// Classic linear lerp
/// 
/// # Examples
/// ```
/// let value = zara::utils::lerp(0., 5., 0.8);
/// ```
pub fn lerp(first: f32, second: f32, by: f32) -> f32 {
    first * (1. - by) + second * by
}

/// Clamp both ways
/// 
/// # Examples
/// ```
/// let value = zara::utils::clamp(101., 0., 100.);
/// ```
pub fn clamp(value: f32, floor: f32, ceiling: f32) -> f32 {
    if value > ceiling {
        return ceiling;
    }

    if value < floor {
        return floor;
    }

    value
}

/// Clamps ceiling
/// 
/// # Examples
/// ```
/// let value = zara::utils::clamp_to(101., 100.);
/// ```
pub fn clamp_to(value: f32, ceiling: f32) -> f32 {
    if value > ceiling {
        return ceiling;
    }

    value
}

/// Clamps floor
/// 
/// # Examples
/// ```
/// let value = zara::utils::clamp_bottom(-5., 0.);
/// ```
pub fn clamp_bottom(value: f32, floor: f32) -> f32 {
    if value < floor {
        return floor;
    }

    value
}


/// Clamps 0..1
/// 
/// # Examples
/// ```
/// let value = zara::utils::clamp_01(2.3);
/// ```
pub fn clamp_01(value: f32) -> f32 {
    if value > 1. {
        return 1.;
    }
    if value < 0. {
        return 0.;
    }

    value
}

/// Will return `true` is a given probability is satisfied
/// 
/// # Examples
/// ```
/// if zara::utils::roll_dice(65) {
///     // ...
/// }
/// ```
pub fn roll_dice(probability: usize) -> bool {
    if probability == 0 { return false; }
    if probability >= 100 { return true; }

    let mut rng = rand::thread_rng();
    let r = rng.gen_range(0..100);

    r < probability
}

/// Will return a random number between these two
/// 
/// # Examples
/// ```
/// let value = zara::utils::range(0., 100.);
/// ```
pub fn range(a: f32, b: f32) -> f32 {
    let mut rng = rand::thread_rng();

    rng.gen_range(a..b)
}