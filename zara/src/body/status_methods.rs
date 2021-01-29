use crate::body::Body;
use crate::utils::GameTimeC;

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

    /// Does character have a complete clothes set -- one of the registered clothes groups.
    ///
    /// Returns name of the set if does.
    pub fn has_complete_clothes_set(&self) -> Option<String> {
        match self.clothes_group.borrow().as_ref() {
            Some(g) => Some(g.name.to_string()),
            _ => None
        }
    }

    /// Returns 0..100 bonus cold resistance value described by the complete clothes set.
    /// 0 if no clothes group matched.
    pub fn bonus_cold_resistance(&self) -> usize {
        match self.clothes_group.borrow().as_ref() {
            Some(g) => g.bonus_cold_resistance,
            _ => 0
        }
    }

    /// Returns 0..100 bonus water resistance value described by the complete clothes set.
    /// 0 if no clothes group matched.
    pub fn bonus_water_resistance(&self) -> usize {
        match self.clothes_group.borrow().as_ref() {
            Some(g) => g.bonus_water_resistance,
            _ => 0
        }
    }
}