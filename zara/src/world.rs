use crate::utils::{GameTime, EnvironmentC};

use std::cell::Cell;
use std::rc::Rc;

/// Contains runtime environment data and game time
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