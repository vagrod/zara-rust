use crate::body::{Body, BodyPart, BodyAppliance, ClothesItemC};
use crate::utils::{ClothesGroupC, GameTimeC};

use std::time::Duration;

pub struct BodyStateContract {
    pub clothes: Vec<String>,
    pub appliances: Vec<BodyApplianceStateContract>,
    pub last_sleep_time: Option<Duration>,
    pub last_sleep_duration: f32,
    pub is_sleeping: bool,
    pub clothes_group: Option<ClothesGroupStateContract>,
    pub clothes_data: Vec<ClothesItemStateContract>,
    pub warmth_level: f32,
    pub wetness_level: f32,
    pub sleeping_counter: f64,
    pub cached_world_temp: f32,
    pub cached_wind_speed: f32,
    pub cached_player_in_water: bool,
    pub cached_rain_intensity: f32,
}

pub struct BodyApplianceStateContract {
    pub item_name: String,
    pub body_part: BodyPart
}

pub struct ClothesGroupStateContract {
    pub name: String,
    pub bonus_cold_resistance: usize,
    pub bonus_water_resistance: usize
}

pub struct ClothesItemStateContract {
    pub key: String,
    pub cold_resistance: usize,
    pub water_resistance: usize,
}

impl BodyAppliance {
    pub(crate) fn get_state(&self) -> BodyApplianceStateContract {
        BodyApplianceStateContract {
            item_name: self.item_name.to_string(),
            body_part: self.body_part
        }
    }
}

impl ClothesGroupC {
    pub(crate) fn get_state(&self) -> ClothesGroupStateContract {
        ClothesGroupStateContract {
            name: self.name.to_string(),
            bonus_cold_resistance: self.bonus_cold_resistance,
            bonus_water_resistance: self.bonus_water_resistance
        }
    }
}

impl ClothesItemC {
    pub(crate) fn get_state(&self, key: String) -> ClothesItemStateContract {
        ClothesItemStateContract {
            key,
            water_resistance: self.water_resistance,
            cold_resistance: self.cold_resistance
        }
    }
}

impl Body {
    pub(crate) fn get_state(&self) -> BodyStateContract {
        BodyStateContract {
            wetness_level: self.wetness_level.get(),
            warmth_level: self.warmth_level.get(),
            cached_player_in_water: self.cached_player_in_water.get(),
            cached_rain_intensity: self.cached_rain_intensity.get(),
            cached_wind_speed: self.cached_wind_speed.get(),
            cached_world_temp: self.cached_world_temp.get(),
            is_sleeping: self.is_sleeping.get(),
            last_sleep_duration: self.last_sleep_duration.get(),
            sleeping_counter: self.sleeping_counter.get(),

            clothes: self.clothes.borrow().iter().map(|x|x.to_string()).collect(),
            appliances: self.appliances.borrow().iter().map(|x| x.get_state()).collect(),
            clothes_group: match self.clothes_group.borrow().as_ref() {
                Some(g) => Some(g.get_state()),
                None => None
            },
            clothes_data: self.clothes_data.borrow().iter().map(|(k, x)| x.get_state(k.to_string())).collect(),
            last_sleep_time: match self.last_sleep_time.borrow().as_ref() {
                Some(t) => Some(t.to_duration()),
                None => None
            }
        }
    }

    pub(crate) fn restore_state(&self, state: &BodyStateContract) {
        self.wetness_level.set(state.wetness_level);
        self.warmth_level.set(state.warmth_level);
        self.cached_player_in_water.set(state.cached_player_in_water);
        self.cached_rain_intensity.set(state.cached_rain_intensity);
        self.cached_wind_speed.set(state.cached_wind_speed);
        self.cached_world_temp.set(state.cached_world_temp);
        self.is_sleeping.set(state.is_sleeping);
        self.last_sleep_duration.set(state.last_sleep_duration);
        self.sleeping_counter.set(state.sleeping_counter);

        self.clothes_group.replace(
            match &state.clothes_group {
                Some(g) => Some(ClothesGroupC {
                    name: g.name.to_string(),
                    bonus_water_resistance: g.bonus_water_resistance,
                    bonus_cold_resistance: g.bonus_cold_resistance
                }),
                None => None
            }
        );
        self.last_sleep_time.replace(
            match state.last_sleep_time {
                Some(t) => Some(GameTimeC::from_duration(t)),
                None => None
            }
        );
        {
            let mut b = self.clothes.borrow_mut();

            b.clear();

            for c in &state.clothes {
                b.push(c.to_string());
            }
        }
        {
            let mut b = self.appliances.borrow_mut();

            b.clear();

            for a in &state.appliances {
                b.push(BodyAppliance{
                    item_name: a.item_name.to_string(),
                    body_part: a.body_part
                });
            }
        }
        {
            let mut b = self.clothes_data.borrow_mut();

            b.clear();

            for d in &state.clothes_data {
                b.insert(d.key.to_string(), ClothesItemC{
                    cold_resistance: d.cold_resistance,
                    water_resistance: d.water_resistance
                });
            }
        }
    }
}