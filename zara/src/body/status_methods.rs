use crate::body::Body;
use crate::utils::{GameTimeC, ClothesGroupC};

impl Body {
    /// Is player sleeping now
    /// 
    /// # Examples
    /// ```
    /// let value = person.body.is_sleeping();
    /// ```
    /// 
    /// # Links
    /// See [this wiki article](https://github.com/vagrod/zara-rust/wiki/Sleeping) for more info.
    pub fn is_sleeping(&self) -> bool { self.is_sleeping.get() }

    /// Cached warmth level value
    /// 
    /// # Examples
    /// ```
    /// let value = person.body.warmth_level();
    /// ```
    /// 
    /// # Links
    /// See [this wiki article](https://github.com/vagrod/zara-rust/wiki/Warmth-level) for more info.
    pub fn warmth_level(&self) -> f32 { self.warmth_level.get() }

    /// Cached wetness level value
    ///
    /// # Examples
    /// ```
    /// let value = person.body.wetness_level();
    /// ```
    /// 
    /// # Links
    /// See [this wiki article](https://github.com/vagrod/zara-rust/wiki/Wetness-level) for more info.
    pub fn wetness_level(&self) -> f32 { self.wetness_level.get() }

    /// Last time slept (if any)
    /// 
    /// # Examples
    /// ```
    /// if let Some(sleep_time) = person.body.last_sleep_time() {
    ///     // ...
    /// }
    /// ```
    /// 
    /// # Links
    /// See [this wiki article](https://github.com/vagrod/zara-rust/wiki/Sleeping) for more info.
    pub fn last_sleep_time(&self) -> Option<GameTimeC> {
        match self.last_sleep_time.borrow().as_ref()
        {
            Some(t) => Some(t.clone()),
            _ => None
        }
    }

    /// Duration of the last sleep (game hours)
    /// 
    /// # Examples
    /// ```
    /// let value = person.body.last_sleep_duration();
    /// ```
    /// 
    /// # Links
    /// See [this wiki article](https://github.com/vagrod/zara-rust/wiki/Sleeping) for more info.
    pub fn last_sleep_duration(&self) -> f32 { self.last_sleep_duration.get() }

    /// Returns copy of matched clothes group description contract.
    /// 
    /// # Examples
    /// ```
    /// if let Some(group) = person.body.clothes_group() {
    ///     // ...
    /// }
    /// ```
    /// 
    /// # Links
    /// See [this wiki article](https://github.com/vagrod/zara-rust/wiki/Clothes-groups) for more info.
    pub fn clothes_group(&self) -> Option<ClothesGroupC> {
        self.clothes_group.borrow().clone()
    }

    /// Returns total 0..100 bonus cold resistance value calculated as a sum of all active clothes
    /// cold resistance values plus cold resistance bonus from a matched clothes group, if any.
    ///
    /// # Examples
    /// ```
    /// let value = person.body.total_cold_resistance();
    /// ```
    /// 
    /// # Links
    /// See [this wiki article](https://github.com/vagrod/zara-rust/wiki/Clothes#get-the-total-resistance-levels) for more info.
    /// 
    /// ## Notes
    /// This value is not cached.
    pub fn total_cold_resistance(&self) -> usize {
        let mut result = 0;

        for (_, data) in self.clothes_data.borrow().iter() {
            result += data.cold_resistance;
        }

        if let Some(g) = self.clothes_group.borrow().as_ref() {
            result += g.bonus_cold_resistance;
        }

        result
    }

    /// Returns total 0..100 bonus water resistance value calculated as a sum of all active clothes
    /// water resistance values plus water resistance bonus from a matched clothes group, if any
    ///
    /// # Examples
    /// ```
    /// let value = person.body.total_water_resistance();
    /// ```
    /// 
    /// # Links
    /// See [this wiki article](https://github.com/vagrod/zara-rust/wiki/Clothes#get-the-total-resistance-levels) for more info.
    /// 
    /// ## Notes
    /// This value is not cached.
    pub fn total_water_resistance(&self) -> usize {
        let mut result = 0;

        for (_, data) in self.clothes_data.borrow().iter() {
            result += data.water_resistance;
        }

        if let Some(g) = self.clothes_group.borrow().as_ref() {
            result += g.bonus_water_resistance;
        }

        result
    }
}