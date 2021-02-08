use crate::ZaraController;
use crate::utils::{FrameC, EnvironmentC, HealthC, FrameSummaryC, PlayerStatusC, ActiveDiseaseC, ActiveInjuryC};
use crate::utils::event::{Listener, Event, MessageQueue};
use crate::error::ZaraUpdateErr;
use crate::health::StageLevel;

use std::time::Duration;
use std::collections::BTreeMap;
use std::cell::RefMut;

/// How frequently should Zara update all its controllers,
/// recalculate values and check monitors (real seconds)
/// when player is awake
const UPDATE_INTERVAL: f32 = 1.;
/// How frequently should Zara update all its controllers,
/// recalculate values and check monitors (real seconds)
/// when player is sleeping
const SLEEPING_UPDATE_INTERVAL: f32 = UPDATE_INTERVAL / 5.;
/// How frequently should Zara process message queue (real seconds)
const MESSAGE_QUEUE_CHECK_PERIOD: f32 = UPDATE_INTERVAL / 3.;

impl<E: Listener + 'static> ZaraController<E> {
    /// Progresses Zara controller state.
    ///
    /// This method should be called every frame.
    ///
    /// # Parameters
    /// - `frame_time`: time, **in seconds**`, since last `update` call.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// person.update(time_delta);
    /// ```
    pub fn update(&self, frame_time: f32) -> Result<(), ZaraUpdateErr>{
        if !self.health.is_alive() { return Err(ZaraUpdateErr::CharacterIsDead); }
        if self.is_paused() { return Err(ZaraUpdateErr::InstancePaused); }

        let elapsed = self.update_counter.get() + frame_time;
        let elapsed_for_queue = self.queue_counter.get() + frame_time;
        let mut ceiling = UPDATE_INTERVAL;
        let game_time_duration = self.environment.game_time.duration.get();

        if elapsed_for_queue >= MESSAGE_QUEUE_CHECK_PERIOD {
            self.queue_counter.set(0.);

            // Send pending events
            self.process_health_events();
            self.process_inventory_events();
            self.process_body_events();
        } else {
            self.queue_counter.set(elapsed_for_queue);
        }

        // When sleeping, our checks are more frequent
        if self.body.is_sleeping() {
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

        Ok(())
    }

    /// Gets all the info needed for all the controllers and monitors to process one frame
    fn get_summary(&self) -> FrameSummaryC {
        let game_time_duration = self.environment.game_time.duration.get();
        let time_delta = game_time_duration - self.last_update_game_time.get();
        let mut active_diseases: Vec<ActiveDiseaseC> = Vec::new();
        let mut active_injuries: Vec<ActiveInjuryC> = Vec::new();
        let game_time_contract = &self.environment.game_time.to_contract();

        // Collect active diseases data
        for (_, disease) in self.health.diseases.borrow().iter() {
            match disease.get_active_stage(game_time_contract) {
                Some(st) => {
                    active_diseases.push(ActiveDiseaseC {
                        name: disease.disease.get_name(),
                        is_active: true,
                        scheduled_time: disease.activation_time(),
                        end_time: disease.end_time(),
                        current_level: st.info.level,
                        current_level_percent: st.percent_active(game_time_contract),
                        is_healing: disease.is_healing(),
                        needs_treatment: disease.needs_treatment
                    });
                },
                None => {
                    active_diseases.push(ActiveDiseaseC {
                        name: disease.disease.get_name(),
                        is_active: false,
                        scheduled_time: disease.activation_time(),
                        end_time: disease.end_time(),
                        current_level: StageLevel::Undefined,
                        current_level_percent: 0,
                        is_healing: false,
                        needs_treatment: disease.needs_treatment
                    });
                }
            }
        };

        // Collect active injuries data
        for (_, injury) in self.health.injuries.borrow().iter() {
            match injury.get_active_stage(game_time_contract) {
                Some(st) => {
                    active_injuries.push(ActiveInjuryC {
                        name: injury.injury.get_name(),
                        is_active: true,
                        scheduled_time: injury.activation_time(),
                        end_time: injury.end_time(),
                        current_level: st.info.level,
                        current_level_percent: st.percent_active(game_time_contract),
                        is_healing: injury.is_healing(),
                        needs_treatment: injury.needs_treatment,
                        is_blood_stopped: injury.is_blood_stopped(),
                        body_part: injury.body_part,
                        is_fracture: injury.is_fracture
                    });
                },
                None => {
                    active_injuries.push(ActiveInjuryC {
                        name: injury.injury.get_name(),
                        is_active: false,
                        scheduled_time: injury.activation_time(),
                        end_time: injury.end_time(),
                        current_level: StageLevel::Undefined,
                        current_level_percent: 0,
                        is_healing: false,
                        needs_treatment: injury.needs_treatment,
                        is_blood_stopped: injury.is_blood_stopped(),
                        body_part: injury.body_part,
                        is_fracture: injury.is_fracture
                    });
                }
            }
        };

        // Determine last sleep time
        let last_slept =
            match self.body.last_sleep_time().as_ref() {
                Some(t) => Some(t.copy()),
                None => { None }
            };

        FrameSummaryC {
            game_time: self.environment.game_time.to_contract(),
            game_time_delta: time_delta.as_secs_f32(),
            player: PlayerStatusC {
                is_walking: self.player_state.is_walking.get(),
                is_running: self.player_state.is_running.get(),
                is_swimming: self.player_state.is_swimming.get(),
                is_underwater: self.player_state.is_underwater.get(),
                is_sleeping: self.body.is_sleeping(),
                last_slept_duration: self.body.last_sleep_duration(),
                last_slept,
                warmth_level: self.body.warmth_level(),
                wetness_level: self.body.wetness_level(),
                clothes: self.body.clothes.borrow().clone(),
                clothes_group: self.body.clothes_group(),
                appliances: self.body.appliances.borrow().clone(),
                total_water_resistance: self.body.total_water_resistance(),
                total_cold_resistance: self.body.total_cold_resistance(),
                inventory_weight: self.inventory.get_weight()
            },
            environment: EnvironmentC {
                wind_speed: self.environment.wind_speed.get(),
                rain_intensity: self.environment.rain_intensity.get(),
                temperature: self.environment.temperature.get()
            },
            health: HealthC {
                body_temperature: self.health.body_temperature(),
                blood_level: self.health.blood_level(),
                heart_rate: self.health.heart_rate(),
                water_level: self.health.water_level(),
                food_level: self.health.food_level(),
                top_pressure: self.health.top_pressure(),
                bottom_pressure: self.health.bottom_pressure(),
                stamina_level: self.health.stamina_level(),
                fatigue_level: self.health.fatigue_level(),
                oxygen_level: self.health.oxygen_level(),

                diseases: active_diseases,
                injuries: active_injuries
            }
        }
    }

    fn process_body_events(&self) {
        if self.body.has_messages() {
            self.process_events(self.body.get_message_queue());
        }
    }

    fn process_inventory_events(&self) {
        if self.inventory.has_messages() {
            self.process_events(self.inventory.get_message_queue());
        }
    }

    fn process_health_events(&self) {
        if self.health.has_messages() {
            self.process_events(self.health.get_message_queue());
        }
    }

    fn process_events(&self, mut q: RefMut<BTreeMap<usize, Event>>) {
        if q.len() == 0 { return }

        let mut dispatcher = self.dispatcher.borrow_mut();
        let mut key = 0;

        loop {
            match q.get(&key) {
                Some(event) => {
                    dispatcher.dispatch(event.clone());

                    q.remove(&key);
                },
                None => break
            }

            key += 1;
        }
    }
}