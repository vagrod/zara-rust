use crate::health::Health;

impl Health {

    /// Is character alive
    pub fn is_alive(&self) -> bool { self.is_alive.get() }
    /// Is player tired (`fatigue_level` more than 70%)
    pub fn is_tired(&self) -> bool { self.fatigue_level.get() >= 70. }
    /// Is player tired (`fatigue_level` more than 90%)
    pub fn is_exhausted(&self) -> bool { self.fatigue_level.get() >= 90. }
    /// Player has low stamina (`stamina_level` 5% and less)
    pub fn is_no_strength(&self) -> bool { self.stamina_level.get() <= 5. }
    /// Player has low oxygen level
    pub fn is_low_oxygen(&self) -> bool { self.oxygen_level.get() <= 5. }
    /// Player has low food level
    pub fn is_low_food(&self) -> bool { self.food_level.get() <= 5. }
    /// Player has low water level
    pub fn is_low_water(&self) -> bool { self.water_level.get() <= 5. }
    /// Player has low blood level
    pub fn is_low_blood(&self) -> bool { self.blood_level.get() <= 5. }
    /// Player has active non-zero blood loss from some injury
    pub fn is_blood_loss(&self) -> bool { self.has_blood_loss.get() }
    /// Current body temperature (degrees C)
    pub fn body_temperature(&self) -> f32 { self.body_temperature.get() }
    /// Current heart rate (bpm)
    pub fn heart_rate(&self) -> f32 { self.heart_rate.get() }
    /// Current top blood pressure (mmHg)
    pub fn top_pressure(&self) -> f32 { self.top_pressure.get() }
    /// Current bottom blood pressure (mmHg)
    pub fn bottom_pressure(&self) -> f32 { self.bottom_pressure.get() }
    /// Current blood level (0..100 percents)
    pub fn blood_level(&self) -> f32 { self.blood_level.get() }
    /// Current food level (0..100 percents)
    pub fn food_level(&self) -> f32 { self.food_level.get() }
    /// Current water level (0..100 percents)
    pub fn water_level(&self) -> f32 { self.water_level.get() }
    /// Current stamina level (0..100 percents)
    pub fn stamina_level(&self) -> f32 { self.stamina_level.get() }
    /// Current fatigue level (0..100 percents)
    pub fn fatigue_level(&self) -> f32 { self.fatigue_level.get() }
    /// Current oxygen level (0..100 percents)
    pub fn oxygen_level(&self) -> f32 { self.oxygen_level.get() }

}