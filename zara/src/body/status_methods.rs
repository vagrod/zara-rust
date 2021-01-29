use crate::body::Body;
use crate::utils::{GameTimeC, ClothesGroupC};

impl Body {
    /// Is player sleeping now
    pub fn is_sleeping(&self) -> bool { self.is_sleeping.get() }

    /// Last time slept (if any)
    pub fn last_sleep_time(&self) -> Option<GameTimeC> {
        match self.last_sleep_time.borrow().as_ref()
        {
            Some(t) => Some(t.copy()),
            _ => None
        }
    }

    /// Duration of the last sleep (zero if none)
    pub fn last_sleep_duration(&self) -> f64 { self.last_sleep_duration.get() }

    /// Returns copy of matched clothes group description.
    pub fn clothes_group(&self) -> Option<ClothesGroupC> {
        match self.clothes_group.borrow().as_ref() {
            Some(g) => Some(g.copy()),
            _ => None
        }
    }

    /// Returns total 0..100 bonus cold resistance value calculated as a sum of all active clothes
    /// cold resistance values plus cold resistance bonus from a matched clothes group, if any.
    ///
    /// ## Note
    /// This value is not cached.
    pub fn total_cold_resistance(&self) -> usize {
        let mut result = 0;

        for (_, data) in self.clothes_data.borrow().iter() {
            result += data.cold_resistance;
        }

        match self.clothes_group.borrow().as_ref() {
            Some(g) => result += g.bonus_cold_resistance,
            _ => { }
        }

        return result;
    }

    /// Returns total 0..100 bonus water resistance value calculated as a sum of all active clothes
    /// cold resistance values plus water resistance bonus from a matched clothes group, if any
    ///
    /// ## Note
    /// This value is not cached.
    pub fn total_water_resistance(&self) -> usize {
        let mut result = 0;

        for (_, data) in self.clothes_data.borrow().iter() {
            result += data.water_resistance;
        }

        match self.clothes_group.borrow().as_ref() {
            Some(g) => result += g.bonus_water_resistance,
            _ => { }
        }

        return result;
    }
}