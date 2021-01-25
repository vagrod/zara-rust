use crate::ZaraController;
use crate::utils::{FrameC, EnvironmentC, HealthC, GameTimeC, FrameSummaryC, PlayerStatusC, ActiveDiseaseC, ActiveInjuryC};
use crate::utils::event::{Listener, Event};
use crate::error::ZaraUpdateErr;

use std::time::Duration;
use crate::health::StageLevel;

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
    pub fn update(&self, frame_time: f32) -> Result<(), ZaraUpdateErr>{
        if !self.is_alive.get() { return Err(ZaraUpdateErr::CharacterIsDead); }

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
            let health_result;

                // Form the frame data structure
            let mut frame_data = &mut FrameC {
                events: &mut self.dispatcher.borrow_mut(),
                data: summary
            };

            // Update all sub-controllers
            health_result = self.health.update(&mut frame_data);
            self.inventory.update(&mut frame_data);
            self.body.update(&mut frame_data);

            // Reset the counter and set last update game time
            self.last_update_game_time.set(game_time_duration);
            self.update_counter.set(0.);

            if !health_result.is_alive {
                self.is_alive.set(false);
                frame_data.events.dispatch(Event::DeathFromDisease(health_result.disease_caused_death))
            }
        } else {
            self.update_counter.set(elapsed);
        }

        // Set last frame game time
        self.last_frame_game_time.set(Duration::from(game_time_duration));

        Ok(())
    }

    /// Gets all the info needed for all the controllers to process one frame
    fn get_summary(&self) -> FrameSummaryC {
        let game_time_duration = self.environment.game_time.duration.get();
        let time_delta = game_time_duration - self.last_update_game_time.get();
        let mut active_diseases: Vec<ActiveDiseaseC> = Vec::new();
        let mut active_injuries: Vec<ActiveInjuryC> = Vec::new();
        let game_time_contract = &self.environment.game_time.to_contract();

        // Collect active diseases data
        for (_key, disease) in self.health.diseases.borrow().iter() {
            match disease.get_active_stage(game_time_contract) {
                Some(st) => {
                    active_diseases.push(ActiveDiseaseC {
                        name: disease.disease.get_name(),
                        is_active: true,
                        scheduled_time: disease.get_activation_time(),
                        end_time: disease.get_end_time(),
                        current_level: st.info.level,
                        current_level_percent: st.get_percent_active(game_time_contract),
                        is_healing: disease.get_is_healing(),
                        needs_treatment: disease.needs_treatment
                    });
                },
                _ => {
                    active_diseases.push(ActiveDiseaseC {
                        name: disease.disease.get_name(),
                        is_active: false,
                        scheduled_time: disease.get_activation_time(),
                        end_time: disease.get_end_time(),
                        current_level: StageLevel::Undefined,
                        current_level_percent: 0,
                        is_healing: false,
                        needs_treatment: disease.needs_treatment
                    });
                }
            }
        };

        // Collect active injuries data
        for (_key, injury) in self.health.injuries.borrow().iter() {
            match injury.get_active_stage(game_time_contract) {
                Some(st) => {
                    active_injuries.push(ActiveInjuryC {
                        name: injury.injury.get_name(),
                        is_active: true,
                        scheduled_time: injury.get_activation_time(),
                        end_time: injury.get_end_time(),
                        current_level: st.info.level,
                        current_level_percent: st.get_percent_active(game_time_contract),
                        is_healing: injury.get_is_healing(),
                        needs_treatment: injury.needs_treatment,
                        is_blood_stopped: injury.get_is_blood_stopped()
                    });
                },
                _ => {
                    active_injuries.push(ActiveInjuryC {
                        name: injury.injury.get_name(),
                        is_active: false,
                        scheduled_time: injury.get_activation_time(),
                        end_time: injury.get_end_time(),
                        current_level: StageLevel::Undefined,
                        current_level_percent: 0,
                        is_healing: false,
                        needs_treatment: injury.needs_treatment,
                        is_blood_stopped: injury.get_is_blood_stopped()
                    });
                }
            }
        };

        // Determine last sleep time
        let mut last_slept: GameTimeC = GameTimeC::empty();
        {
            let borrowed_time = self.body.last_sleep_time.borrow();
            match borrowed_time.as_ref() {
                Some(t) => last_slept = t.copy(),
                None => { }
            }
        }

        FrameSummaryC {
            game_time : self.environment.game_time.to_contract(),
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

                diseases: active_diseases,
                injuries: active_injuries
            },
            game_time_delta: time_delta.as_secs_f32()
        }
    }
}