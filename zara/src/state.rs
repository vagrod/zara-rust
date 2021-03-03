use crate::ZaraController;
use crate::utils::event::Listener;
use crate::body::state::BodyStateContract;
use crate::health::state::HealthStateContract;
use crate::inventory::state::InventoryStateContract;

use std::time::Duration;
use std::fmt;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

/// Zara state contract. It **does not** include state of diseases, injuries, side effects monitors,
/// disease monitors, inventory monitors or inventory items. For those objects you may need to
/// implement custom methods for saving and restoring their states.
///
/// It contains game time, state of all medical agents, environment snapshot, saved payer status,
/// health vitals and levels, clothes and body appliances, cached inventory weight and related
/// internal fields.
#[derive(Clone, Debug, Default)]
pub struct ZaraControllerStateContract {
    /// Environment node status snapshot
    pub environment: EnvironmentStateContract,
    /// Player status snapshot
    pub player_status: PlayerStatusContract,
    /// Body node status snapshot
    pub body: BodyStateContract,
    /// Health node status snapshot
    pub health: HealthStateContract,
    /// Inventory node status snapshot (not including items itself)
    pub inventory: InventoryStateContract,

    /// State of an update counter
    pub update_counter: f32,
    /// State of a queue counter
    pub queue_counter: f32,
    /// State of a game time when `update` was last called
    pub last_update_game_time: Duration,
    /// State of a game time when controller was last updated
    pub last_frame_game_time: Duration,
    /// Paused state value
    pub is_paused: bool
}
impl fmt::Display for ZaraControllerStateContract {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Zara core state @{} game secs", self.environment.game_time.as_secs_f32())
    }
}
impl Ord for ZaraControllerStateContract {
    fn cmp(&self, other: &Self) -> Ordering {
        self.environment.game_time.cmp(&other.environment.game_time)
    }
}
impl Eq for ZaraControllerStateContract { }
impl PartialOrd for ZaraControllerStateContract {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for ZaraControllerStateContract {
    fn eq(&self, other: &Self) -> bool {
        const EPS: f32 = 0.0001;

        self.environment == other.environment &&
        self.player_status == other.player_status &&
        self.body == other.body &&
        self.health == other.health &&
        self.inventory == other.inventory &&
        self.last_update_game_time == other.last_update_game_time &&
        self.last_frame_game_time == other.last_frame_game_time &&
        self.is_paused == other.is_paused &&
        f32::abs(self.update_counter - other.update_counter) < EPS &&
        f32::abs(self.queue_counter - other.queue_counter) < EPS
    }
}
impl Hash for ZaraControllerStateContract {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.environment.hash(state);
        self.player_status.hash(state);
        self.body.hash(state);
        self.health.hash(state);
        self.inventory.hash(state);
        self.last_update_game_time.hash(state);
        self.last_frame_game_time.hash(state);
        self.is_paused.hash(state);

        state.write_u32((self.update_counter*100_f32) as u32);
        state.write_u32((self.queue_counter*100_f32) as u32);
    }
}

/// Describes captured state of an active disease
#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ActiveDiseaseStateContract {
    /// Captured state of the `needs_treatment` field
    pub needs_treatment: bool,
    /// Captured state of the `will_self_heal_on` field
    pub will_self_heal_on: crate::health::StageLevel,
    /// Captured state of the `total_duration` field
    pub total_duration: Duration,
    /// Captured state of the `initial_data` field
    pub initial_data: Vec<crate::health::disease::state::StageDescriptionStateContract>,
    /// Captured state of the `stages` field
    pub stages: Vec<crate::health::disease::state::ActiveStageStateContract>,
    /// Captured state of the `lerp_data` field
    pub lerp_data: Option<crate::health::disease::state::LerpDataNodeStateContract>,
    /// Captured state of the `last_deltas` field
    pub last_deltas: crate::health::disease::state::DiseaseDeltasStateContract,
    /// Captured state of the `is_inverted` field
    pub is_inverted: bool,
    /// Captured state of the `activation_time` field
    pub activation_time: Duration,
    /// Captured state of the `will_end` field
    pub will_end: bool,
    /// Captured state of the `end_time` field
    pub end_time: Option<Duration>
}

/// Describes captured state of an active injury
#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ActiveInjuryStateContract {
    /// Captured state of the `needs_treatment` field
    pub needs_treatment: bool,
    /// Captured state of the `is_fracture` field
    pub is_fracture: bool,
    /// Captured state of the `body_part` field
    pub body_part: crate::body::BodyPart,
    /// Captured state of the `will_self_heal_on` field
    pub will_self_heal_on: crate::health::StageLevel,
    /// Captured state of the `total_duration` field
    pub total_duration: Duration,
    /// Captured state of the `initial_data` field
    pub initial_data: Vec<crate::health::injury::state::StageDescriptionStateContract>,
    /// Captured state of the `stages` field
    pub stages: Vec<crate::health::injury::state::ActiveStageStateContract>,
    /// Captured state of the `lerp_data` field
    pub lerp_data: Option<crate::health::injury::state::LerpDataNodeStateContract>,
    /// Captured state of the `last_deltas` field
    pub last_deltas: crate::health::injury::state::InjuryDeltasStateContract,
    /// Captured state of the `is_inverted` field
    pub is_inverted: bool,
    /// Captured state of the `activation_time` field
    pub activation_time: Duration,
    /// Captured state of the `will_end` field
    pub will_end: bool,
    /// Captured state of the `end_time` field
    pub end_time: Option<Duration>
}

/// Describes captured state of an environment
#[derive(Clone, Debug, Default)]
pub struct EnvironmentStateContract {
    /// Captured state of the `game_time` field
    pub game_time: Duration,
    /// Captured state of the `wind_speed` field
    pub wind_speed: f32,
    /// Captured state of the `temperature` field
    pub temperature: f32,
    /// Captured state of the `rain_intensity` field
    pub rain_intensity: f32
}
impl fmt::Display for EnvironmentStateContract {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} secs, temp {:.1}C, wind {:.1} m/s, rain {:.1}", self.game_time.as_secs_f32(),
               self.temperature, self.wind_speed, self.rain_intensity)
    }
}
impl Ord for EnvironmentStateContract {
    fn cmp(&self, other: &Self) -> Ordering {
        self.game_time.cmp(&other.game_time)
    }
}
impl Eq for EnvironmentStateContract { }
impl PartialOrd for EnvironmentStateContract {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for EnvironmentStateContract {
    fn eq(&self, other: &Self) -> bool {
        const EPS: f32 = 0.0001;

        self.game_time == other.game_time &&
        f32::abs(self.temperature - other.temperature) < EPS &&
        f32::abs(self.wind_speed - other.wind_speed) < EPS &&
        f32::abs(self.rain_intensity - other.rain_intensity) < EPS
    }
}
impl Hash for EnvironmentStateContract {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.game_time.hash(state);

        state.write_i32((self.temperature*10_000_f32) as i32);
        state.write_u32((self.wind_speed*10_000_f32) as u32);
        state.write_u32((self.rain_intensity*10_000_f32) as u32);
    }
}

/// Describes captured state of a player status
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Default)]
pub struct PlayerStatusContract {
    /// Captured state of the `is_walking` field
    pub is_walking: bool,
    /// Captured state of the `is_running` field
    pub is_running: bool,
    /// Captured state of the `is_swimming` field
    pub is_swimming: bool,
    /// Captured state of the `is_underwater` field
    pub is_underwater: bool
}

impl<E: Listener + 'static> ZaraController<E> {
    /// Gets Zara state snapshot, **not** including active diseases, active injuries,
    /// disease/inventory/side effects monitors and inventory items.
    ///
    /// For diseases and injuries, you need to call `get_state` for every active disease or
    /// injury, and when needed call `restore_disease` and `restore_injury` on `health` node.
    ///
    /// It will capture current game time, state of all medical agents, environment snapshot, saved
    /// payer status, health vitals and levels, clothes and body appliances, cached inventory weight
    /// and related internal fields.
    /// 
    /// # Examples
    /// ```
    /// let state = person.get_state();
    /// ```
    /// 
    /// # Links
    /// See [this wiki article](https://github.com/vagrod/zara-rust/wiki/State-Management) for more info.
    pub fn get_state(&self) -> ZaraControllerStateContract {
        ZaraControllerStateContract {
            environment: EnvironmentStateContract {
                game_time: self.environment.game_time.duration.get(),
                wind_speed: self.environment.wind_speed.get(),
                temperature: self.environment.temperature.get(),
                rain_intensity: self.environment.rain_intensity.get()
            },
            player_status: PlayerStatusContract {
                is_walking: self.player_state.is_walking.get(),
                is_running: self.player_state.is_running.get(),
                is_swimming: self.player_state.is_swimming.get(),
                is_underwater: self.player_state.is_underwater.get()
            },
            body: self.body.get_state(),
            health: self.health.get_state(),
            inventory: self.inventory.get_state(),

            update_counter: self.update_counter.get(),
            queue_counter: self.queue_counter.get(),
            last_update_game_time: self.last_update_game_time.get(),
            last_frame_game_time: self.last_frame_game_time.get(),
            is_paused: self.is_paused.get()
        }
    }

    /// Restores previously captured state. This will **not** restore active diseases, injuries,
    /// disease/inventory/side effects monitors or inventory items.
    ///
    /// It will restore current game time, state of all medical agents, environment snapshot, saved
    /// payer status, health vitals and levels, clothes and body appliances, cached inventory weight
    /// and related internal fields.
    ///
    /// To restore a disease or injury, call `restore_disease` or `restore_injury` on a `health` node.
    /// 
    /// # Examples
    /// ```
    /// person.restore_state(state);
    /// ```
    /// 
    /// # Links
    /// See [this wiki article](https://github.com/vagrod/zara-rust/wiki/State-Management) for more info.
    pub fn restore_state(&self, state: &ZaraControllerStateContract) {
        self.update_counter.set(state.update_counter);
        self.queue_counter.set(state.queue_counter);
        self.last_update_game_time.set(state.last_update_game_time);
        self.last_frame_game_time.set(state.last_frame_game_time);
        self.is_paused.set(state.is_paused);

        self.environment.rain_intensity.set(state.environment.rain_intensity);
        self.environment.temperature.set(state.environment.temperature);
        self.environment.wind_speed.set(state.environment.wind_speed);
        self.environment.game_time.update_from_duration(state.environment.game_time);

        self.player_state.is_walking.set(state.player_status.is_walking);
        self.player_state.is_running.set(state.player_status.is_running);
        self.player_state.is_swimming.set(state.player_status.is_swimming);
        self.player_state.is_underwater.set(state.player_status.is_underwater);

        self.body.restore_state(&state.body);
        self.health.restore_state(&state.health);
        self.inventory.restore_state(&state.inventory)
    }
}