use crate::health::Health;

/// Contains health state-related "business" functions

impl Health {

    /// Is player tired (`fatigue_level` more than 70%)
    pub fn is_tired(&self) -> bool { self.fatigue_level.get() >= 70. }
    /// Is player tired (`fatigue_level` more than 90%)
    pub fn is_exhausted(&self) -> bool { self.fatigue_level.get() >= 90. }
    /// Player has low stamina (`stamina_level` 5% and less)
    pub fn is_no_strength(&self) -> bool { self.stamina_level.get() <= 5. }

}