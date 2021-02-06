use crate::ZaraController;
use crate::utils::event::Listener;
use crate::body::state::BodyStateContract;
use crate::health::state::HealthStateContract;

use std::time::Duration;

pub struct ZaraControllerStateContract {
    pub environment: EnvironmentStateContract,
    pub player_status: PlayerStatusContract,
    pub body: BodyStateContract,
    pub health: HealthStateContract,

    pub update_counter: f32,
    pub queue_counter: f32,
    pub last_update_game_time: Duration,
    pub last_frame_game_time: Duration,
    pub is_paused: bool
}

pub struct EnvironmentStateContract {
    pub game_time: Duration,
    pub wind_speed: f32,
    pub temperature: f32,
    pub rain_intensity: f32
}

pub struct PlayerStatusContract {
    pub is_walking: bool,
    pub is_running: bool,
    pub is_swimming: bool,
    pub is_underwater: bool
}

impl<E: Listener + 'static> ZaraController<E> {
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

            update_counter: self.update_counter.get(),
            queue_counter: self.queue_counter.get(),
            last_update_game_time: self.last_update_game_time.get(),
            last_frame_game_time: self.last_frame_game_time.get(),
            is_paused: self.is_paused.get()
        }
    }
    pub fn restore_state(&self, state: ZaraControllerStateContract) {
        self.update_counter.set(state.update_counter);
        self.queue_counter.set(state.queue_counter);
        self.last_update_game_time.set(state.last_update_game_time);
        self.last_frame_game_time.set(state.last_frame_game_time);
        self.is_paused.set(state.is_paused);

        self.environment.rain_intensity.set(state.environment.rain_intensity);
        self.environment.temperature.set(state.environment.rain_intensity);
        self.environment.wind_speed.set(state.environment.rain_intensity);
        self.environment.game_time.update_from_duration(state.environment.game_time);

        self.player_state.is_walking.set(state.player_status.is_walking);
        self.player_state.is_running.set(state.player_status.is_running);
        self.player_state.is_swimming.set(state.player_status.is_swimming);
        self.player_state.is_underwater.set(state.player_status.is_underwater);

        self.body.restore_state(state.body);
        self.health.restore_state(state.health);
    }
}