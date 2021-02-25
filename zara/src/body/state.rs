use crate::body::{Body, BodyPart, BodyAppliance, ClothesItemC};
use crate::utils::{ClothesGroupC, GameTimeC};

use std::time::Duration;
use std::fmt;
use std::hash::{Hash, Hasher};

/// State snippet for the `Body` node
#[derive(Clone, Debug, Default)]
pub struct BodyStateContract {
    /// Captured state of the `clothes` field
    pub clothes: Vec<String>,
    /// Captured state of the `appliances` field
    pub appliances: Vec<BodyApplianceStateContract>,
    /// Captured state of the `last_sleep_time` field
    pub last_sleep_time: Option<Duration>,
    /// Captured state of the `last_sleep_duration` field
    pub last_sleep_duration: f32,
    /// Captured state of the `is_sleeping` field
    pub is_sleeping: bool,
    /// Captured state of the `clothes_group` field
    pub clothes_group: Option<ClothesGroupStateContract>,
    /// Captured state of the `clothes_data` field
    pub clothes_data: Vec<ClothesItemStateContract>,
    /// Captured state of the `warmth_level` field
    pub warmth_level: f32,
    /// Captured state of the `wetness_level` field
    pub wetness_level: f32,
    /// Captured state of the `sleeping_counter` field
    pub sleeping_counter: f64,
    /// Captured state of the `cached_world_temp` field
    pub cached_world_temp: f32,
    /// Captured state of the `cached_wind_speed` field
    pub cached_wind_speed: f32,
    /// Captured state of the `cached_player_in_water` field
    pub cached_player_in_water: bool,
    /// Captured state of the `cached_rain_intensity` field
    pub cached_rain_intensity: f32,
}
impl fmt::Display for BodyStateContract {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Body state ({} clothes, {} appliances)", self.clothes.len(), self.appliances.len())
    }
}
impl Eq for BodyStateContract { }
impl PartialEq for BodyStateContract {
    fn eq(&self, other: &Self) -> bool {
        const EPS_32: f32 = 0.0001;
        const EPS_64: f64 = 0.0001;

        self.clothes == other.clothes &&
        self.appliances == other.appliances &&
        self.last_sleep_time == other.last_sleep_time &&
        self.is_sleeping == other.is_sleeping &&
        self.clothes_group == other.clothes_group &&
        self.clothes_data == other.clothes_data &&
        self.cached_player_in_water == other.cached_player_in_water &&
        f32::abs(self.last_sleep_duration - other.last_sleep_duration) < EPS_32 &&
        f32::abs(self.warmth_level - other.warmth_level) < EPS_32 &&
        f32::abs(self.wetness_level - other.wetness_level) < EPS_32 &&
        f32::abs(self.cached_world_temp - other.cached_world_temp) < EPS_32 &&
        f32::abs(self.cached_wind_speed - other.cached_wind_speed) < EPS_32 &&
        f32::abs(self.cached_rain_intensity - other.cached_rain_intensity) < EPS_32 &&
        f64::abs(self.sleeping_counter - other.sleeping_counter) < EPS_64
    }
}
impl Hash for BodyStateContract {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.clothes.hash(state);
        self.appliances.hash(state);
        self.last_sleep_time.hash(state);
        self.is_sleeping.hash(state);
        self.clothes_group.hash(state);
        self.clothes_data.hash(state);
        self.cached_player_in_water.hash(state);

        state.write_u32((self.last_sleep_duration*10_000_f32) as u32);
        state.write_i32((self.warmth_level*10_000_f32) as i32);
        state.write_u32((self.wetness_level*10_000_f32) as u32);
        state.write_i32((self.cached_world_temp*10_000_f32) as i32);
        state.write_u32((self.cached_wind_speed*10_000_f32) as u32);
        state.write_u32((self.cached_rain_intensity*10_000_f32) as u32);
        state.write_u64((self.sleeping_counter*1_000_f64) as u64);
    }
}

/// State snippet for the body appliance item
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Default)]
pub struct BodyApplianceStateContract {
    /// Captured state of the `item_name` field
    pub item_name: String,
    /// Captured state of the `body_part` field
    pub body_part: BodyPart
}

/// State snippet for the clothes group
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Default)]
pub struct ClothesGroupStateContract {
    /// Captured state of the `name` field
    pub name: String,
    /// Captured state of the `bonus_cold_resistance` field
    pub bonus_cold_resistance: usize,
    /// Captured state of the `bonus_water_resistance` field
    pub bonus_water_resistance: usize
}

/// State snippet for the applied clothes item
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Default)]
pub struct ClothesItemStateContract {
    /// Captured state of the `key` field
    pub key: String,
    /// Captured state of the `cold_resistance` field
    pub cold_resistance: usize,
    /// Captured state of the `water_resistance` field
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
            clothes_group: self.clothes_group.borrow().as_ref().map(|x| x.get_state()),
            clothes_data: self.clothes_data.borrow().iter().map(|(k, x)| x.get_state(k.to_string())).collect(),
            last_sleep_time: self.last_sleep_time.borrow().as_ref().map(|x| x.to_duration())
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
            state.clothes_group.as_ref().map(|x|
                ClothesGroupC {
                    name: x.name.to_string(),
                    bonus_water_resistance: x.bonus_water_resistance,
                    bonus_cold_resistance: x.bonus_cold_resistance
                }
            )
        );
        self.last_sleep_time.replace(state.last_sleep_time.map(|x| GameTimeC::from_duration(x)));
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