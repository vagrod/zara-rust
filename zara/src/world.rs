use crate::utils::{GameTime, EnvironmentC};

use std::cell::Cell;
use std::rc::Rc;
use std::fmt;
use std::hash::{Hash, Hasher};

/// Contains runtime environment data and game time
#[derive(Clone, Default)]
pub struct EnvironmentData {
    /// Game time for this Zara instance
    pub game_time: Rc<GameTime>,

    /// Wind speed (m/s)
    pub wind_speed: Cell<f32>,
    /// Temperature, degrees C
    pub temperature: Cell<f32>,
    /// Rain intensity, 0..1
    pub rain_intensity: Cell<f32>
}
impl fmt::Display for EnvironmentData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, temp {:.1}C, wind {:.1} m/s, rain {:.1}", self.game_time,
               self.temperature.get(), self.wind_speed.get(), self.rain_intensity.get())
    }
}
impl Eq for EnvironmentData { }
impl PartialEq for EnvironmentData {
    fn eq(&self, other: &Self) -> bool {
        const EPS: f32 = 0.0001;

        self.game_time.to_contract() == other.game_time.to_contract() &&
        f32::abs(self.temperature.get() - other.temperature.get()) < EPS &&
        f32::abs(self.wind_speed.get() - other.wind_speed.get()) < EPS &&
        f32::abs(self.rain_intensity.get() - other.rain_intensity.get()) < EPS
    }
}
impl Hash for EnvironmentData {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.game_time.to_contract().hash(state);

        state.write_u32(self.temperature.get() as u32);
        state.write_u32(self.wind_speed.get() as u32);
        state.write_u32(self.rain_intensity.get() as u32);
    }
}
impl EnvironmentData {
    /// Creates new `EnvironmentData`.
    ///
    /// To create `EnvironmentData` with a specific set of parameters, use [`from_description`] method.
    ///
    /// [`from_description`]: #method.from_description
    /// # Examples
    /// ```
    /// use zara::world::EnvironmentData;
    ///
    /// let env = EnvironmentData::new();
    /// ```
    pub fn new() -> Self {
        EnvironmentData {
            game_time: Rc::new(GameTime::new()),
            wind_speed : Cell::new(0.),
            rain_intensity: Cell::new(0.),
            temperature: Cell::new(0.)
        }
    }

    /// Creates new `EnvironmentData` from a given `EnvironmentC` object.
    /// To create default `EnvironmentData`, use [`new`] method.
    ///
    /// [`new`]: #method.new
    /// # Parameters
    /// - `ed`: environment description with initial values for newly created `EnvironmentData`
    ///
    /// # Examples
    /// ```
    /// use zara::world::EnvironmentData;
    ///
    /// let env = EnvironmentData::from_description(env_desc);
    /// ```
    pub fn from_description(ed: EnvironmentC) -> EnvironmentData {
        let e = EnvironmentData::new();

        e.wind_speed.set(ed.wind_speed);
        e.temperature.set(ed.temperature);
        e.rain_intensity.set(ed.rain_intensity);

        return e;
    }
}