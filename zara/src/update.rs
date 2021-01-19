use crate::ZaraController;
use crate::utils::{FrameC, EnvironmentC, HealthC, GameTimeC, FrameSummaryC, PlayerStatusC, ActiveDiseaseC};
use crate::utils::event::{Listener};

use std::time::Duration;

/// How frequently should Zara update all its controllers,
/// recalculate values and check monitors (real seconds)
/// when player is awake
const UPDATE_INTERVAL: f32 = 1.;
/// How frequently should Zara update all its controllers,
/// recalculate values and check monitors (real seconds)
/// when player is sleeping
const SLEEPING_UPDATE_INTERVAL: f32 = UPDATE_INTERVAL / 5.;

impl<E: Listener + 'static> ZaraController<E> {
    /// Progresses Zara controller state.
    ///
    /// This method should be called every frame.
    ///
    /// # Parameters
    /// - `frame_time`: time, `in seconds`, since last `update` call.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// zara_controller.update(time_delta);
    /// ```
    pub fn update(&self, frame_time: f32) {
        let elapsed = self.update_counter.get() + frame_time;
        let mut ceiling = UPDATE_INTERVAL;
        let game_time_duration = self.environment.game_time.duration.get();

        // When sleeping, our checks are more frequent
        if self.body.is_sleeping.get() {
            ceiling = SLEEPING_UPDATE_INTERVAL;

            // When sleeping, we need to check sleeping state every frame, because
            // otherwise wake up game time will be way off
            self.body.sleep_check(
                &mut self.dispatcher.borrow_mut(),
                &game_time_duration,
                (game_time_duration - self.last_frame_game_time.get()).as_secs_f32()
            );
        }

        if elapsed >= ceiling {
            // Retrieve the summary for sub-controllers
            let summary = &self.get_summary();

            // Form the frame data structure
            let mut frame_data = &mut FrameC {
                events: &mut self.dispatcher.borrow_mut(),
                data: summary
            };

            // Update all sub-controllers
            self.health.update(&mut frame_data);
            self.inventory.update(&mut frame_data);
            self.body.update(&mut frame_data);

            // Reset the counter and set last update game time
            self.last_update_game_time.set(game_time_duration);
            self.update_counter.set(0.);
        } else {
            self.update_counter.set(elapsed);
        }

        // Set last frame game time
        self.last_frame_game_time.set(Duration::from(game_time_duration));
    }

    /// Gets all the info needed for all the controllers to process one frame
    fn get_summary(&self) -> FrameSummaryC {
        let game_time_duration = self.environment.game_time.duration.get();
        let time_delta = game_time_duration - self.last_update_game_time.get();
        let mut active_diseases: Vec<ActiveDiseaseC> = Vec::new();
        let current_secs = game_time_duration.as_secs_f64();

        // Collect active diseases data
        for (_key, active) in self.health.diseases.borrow().iter() {
            active_diseases.push(ActiveDiseaseC {
                name: active.disease.get_name(),
                is_active: current_secs >= active.activation_time.to_duration().as_secs_f64(),
                scheduled_time: active.activation_time.copy()
            });
        };

        // Determine last sleep time
        let mut last_slept: GameTimeC = GameTimeC::empty();
        {
            let borrowed_time = self.body.last_sleep_time.borrow();

            if borrowed_time.is_some() {
                last_slept = borrowed_time.as_ref().unwrap().copy();
            }
        }

        FrameSummaryC {
            game_time : GameTimeC {
                day: self.environment.game_time.day.get(),
                hour: self.environment.game_time.hour.get(),
                minute: self.environment.game_time.minute.get(),
                second: self.environment.game_time.second.get()
            },
            player: PlayerStatusC {
                is_walking: self.player_state.is_walking.get(),
                is_running: self.player_state.is_running.get(),
                is_swimming: self.player_state.is_swimming.get(),
                is_underwater: self.player_state.is_underwater.get(),
                is_sleeping: self.body.is_sleeping.get(),
                last_slept_duration: self.body.last_sleep_duration.get(),
                last_slept
            },
            environment: EnvironmentC {
                wind_speed: self.environment.wind_speed.get(),
                rain_intensity: self.environment.rain_intensity.get(),
                temperature: self.environment.temperature.get()
            },
            health: HealthC {
                body_temperature: self.health.body_temperature.get(),
                blood_level: self.health.blood_level.get(),
                heart_rate: self.health.heart_rate.get(),
                water_level: self.health.water_level.get(),
                food_level: self.health.food_level.get(),
                top_pressure: self.health.top_pressure.get(),
                bottom_pressure: self.health.bottom_pressure.get(),
                stamina_level: self.health.stamina_level.get(),
                fatigue_level: self.health.fatigue_level.get(),

                diseases: active_diseases
            },
            game_time_delta: time_delta.as_secs_f32()
        }
    }
}