use crate::health::Health;

/// Contains health state-related "business" functions

impl Health {

    pub fn is_tired(&self) -> bool { self.fatigue_level.get() >= 70. }
    pub fn is_exhausted(&self) -> bool { self.fatigue_level.get() >= 90. }
    pub fn is_no_strength(&self) -> bool { self.stamina_level.get() <= 0. }

}