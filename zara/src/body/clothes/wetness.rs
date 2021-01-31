use crate::body::Body;
use crate::utils::{clamp, clamp_01, lerp};

impl Body {
    pub(crate) fn update_wetness_level_if_needed(&self, game_time_delta: f32, player_in_water: bool,
                                                 rain_intensity: f32, world_temp: f32, wind_speed: f32) {
        let eps = 0.0001;

        if f32::abs(self.cached_world_temp.get() - world_temp) > eps ||
            f32::abs(self.cached_wind_speed.get() - wind_speed) > eps ||
            f32::abs(self.cached_rain_intensity.get() - rain_intensity) > eps ||
            self.cached_player_in_water.get() != player_in_water
        {
            self.cached_world_temp.set(world_temp);
            self.cached_wind_speed.set(wind_speed);
            self.cached_rain_intensity.set(rain_intensity);
            self.cached_player_in_water.set(player_in_water);
        }

        // Always calculate the value
        self.recalculate_wetness_level(game_time_delta);
    }

    pub fn recalculate_wetness_level(&self, game_time_delta: f32) {
        if self.cached_player_in_water.get() {
            self.wetness_level.set(100.);
        } else {
            if self.cached_rain_intensity.get() > 0.001 {
                if self.wetness_level.get() >= 100. { return; }

                const RAIN_WETNESS_GAIN_RATE: f32 = 0.193; // max percent per real second

                // Wetness increase
                let mut wet_rate = self.cached_rain_intensity.get() * RAIN_WETNESS_GAIN_RATE;

                // Check for clothes water resistance
                let water_resistance = self.total_water_resistance() as f32;

                wet_rate *= 1. - water_resistance / 100.;

                let new_value = self.wetness_level.get() + wet_rate * game_time_delta;

                self.wetness_level.set(clamp(new_value, 0., 100.));
            } else {
                // Drying
                if self.wetness_level.get() <= 0. { return; }

                const HOT_DRY_RATE: f32    = 0.075;   // percent per real second
                const NORMAL_DRY_RATE: f32 = 0.0325;  // percent per real second
                const COLD_DRY_RATE: f32   = 0.011;   // percent per real second
                const FREEZE_DRY_RATE: f32 = 0.0065;  // percent per real second

                const HOT_TEMPERATURE: f32    = 30.;  // C (and higher)
                const COLD_TEMPERATURE: f32   = 10.;  // C (and higher)
                const FREEZE_TEMPERATURE: f32 = -80.; // C (and higher)

                const WIND_SPEED_FOR_MAX_DRYING: f32 = 7.; // m/s
                const MAX_WIND_DRYING_RATE: f32 = 0.0422;   // percent per real second

                let current_rate = match self.cached_world_temp.get() {
                    t if t <= FREEZE_TEMPERATURE => {
                        FREEZE_DRY_RATE
                    },
                    t if t <= COLD_TEMPERATURE && t > FREEZE_TEMPERATURE => {
                        COLD_DRY_RATE
                    },
                    t if t >= HOT_TEMPERATURE => {
                        HOT_DRY_RATE
                    }
                    _ => {
                        NORMAL_DRY_RATE
                    }
                };

                let wind_percent = self.cached_wind_speed.get() / WIND_SPEED_FOR_MAX_DRYING;
                let wind_bonus = lerp(0., MAX_WIND_DRYING_RATE, clamp_01(wind_percent));
                let drying_rate = current_rate + wind_bonus;
                let new_value = self.wetness_level.get() - drying_rate * game_time_delta;

                self.wetness_level.set(clamp(new_value, 0., 100.));
            }
        }
    }
}