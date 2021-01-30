use crate::body::Body;

impl Body {

    pub(crate) fn update_warmth_level_if_needed(&self, world_temp: f32, wind_speed: f32) {
        let eps = 0.0001;

        if f32::abs(self.cached_world_temp.get() - world_temp) > eps ||
            f32::abs(self.cached_wind_speed.get() - wind_speed) > eps {
            self.cached_world_temp.set(world_temp);
            self.cached_wind_speed.set(wind_speed);

            self.recalculate_warmth_level();
        }
    }

    pub fn recalculate_warmth_level(&self) {
        const COMFORT_TEMPERATURE_NAKED: f32 = 22.; // degrees C
        const MAXIMUM_WETNESS_TEMPERATURE_DECREASE: f32 = 10.; // degrees C
        const MAXIMUM_WIND_TEMPERATURE_DECREASE: f32 = 15.; // degrees C

        let temp = self.cached_world_temp.get();
        let wetness_temperature_bonus = -(self.wetness_level.get() / 100.) * MAXIMUM_WETNESS_TEMPERATURE_DECREASE;
        let wind_speed = self.cached_wind_speed.get();
        let wind_coldness = (wind_speed * (temp / 35.) - wind_speed) / 35.; // -1..+1 scale
        let mut wind_temperature_bonus = wind_coldness * MAXIMUM_WIND_TEMPERATURE_DECREASE;

        if wind_temperature_bonus > 0. {
            wind_temperature_bonus = 0.; // only cold wind counts
        }

        let final_temp = temp + wetness_temperature_bonus + wind_temperature_bonus;
        let cold_resistance = self.total_cold_resistance() as f32;
        let level = (final_temp * (1. - cold_resistance / 100.)) -
            (COMFORT_TEMPERATURE_NAKED - cold_resistance / 2.) + final_temp * (cold_resistance / 100.);

        self.warmth_level.set(level);
    }

}