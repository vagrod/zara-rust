use crate::health::Health;
use crate::health::side::{SideEffectDeltasC};
use crate::health::disease::{DiseaseDeltasC, StageLevel};
use crate::utils::{HealthC, FrameC, GameTimeC, FrameSummaryC};
use crate::utils::event::{Event, Listener, Dispatcher};

/// Contains code related to the `update` method (calculating and updating health state)

impl Health {
    /// This method is called every `UPDATE_INTERVAL` real seconds
    ///
    /// # Parameters
    /// - `frame`: summary information for this frame
    pub fn update<E: Listener + 'static>(&self, frame: &mut FrameC<E>){
        // Update disease monitors
        for (_, monitor) in self.disease_monitors.borrow().iter() {
            monitor.check(self, &frame.data);
        }

        let mut snapshot = HealthC::healthy();

        // Stamina, blood, food and water are relative
        snapshot.stamina_level = self.stamina_level.get();
        snapshot.food_level = self.food_level.get();
        snapshot.water_level = self.water_level.get();
        snapshot.blood_level = self.blood_level.get();

        // For pretty picture, freeze fatigue value when sleeping
        if frame.data.player.is_sleeping {
           snapshot.fatigue_level = frame.data.health.fatigue_level;
        }

        // Retrieve side effects deltas
        let side_effects_summary = self.process_side_effects(&frame.data);

        // Apply side effects deltas
        self.apply_deltas(&mut snapshot, &side_effects_summary);

        // Process diseases and get vitals deltas from them
        let diseases_deltas = self.process_diseases(&frame.data.game_time, frame.data.game_time_delta);

        // Apply disease deltas
        self.apply_disease_deltas(&mut snapshot, &diseases_deltas);

        // Will always regain stamina. Side effects must "fight" it
        {
            let value = snapshot.stamina_level + self.stamina_regain_rate.get() * frame.data.game_time_delta;
            snapshot.stamina_level = crate::utils::clamp(value, 0., 100.);
        }
        // Will always regain blood. Side effects must "fight" it
        {
            let value = snapshot.blood_level + self.blood_regain_rate.get() * frame.data.game_time_delta;
            snapshot.blood_level = crate::utils::clamp(value, 0., 100.);
        }

        // Apply the resulted health snapshot
        self.apply_health_snapshot(&snapshot);

        // Do the events
        self.dispatch_events::<E>(frame.events);
    }

    fn dispatch_events<E: Listener + 'static>(&self, events: &mut Dispatcher<E>) {
        if self.is_no_strength() {
            events.dispatch(Event::StaminaDrained);
        }
        if self.is_tired() {
            events.dispatch(Event::Tired);
        }
        if self.is_exhausted() {
            events.dispatch(Event::Exhausted);
        }
    }

    fn process_side_effects(&self, frame_data: &FrameSummaryC) -> SideEffectDeltasC {
        let mut side_effects_summary: SideEffectDeltasC = SideEffectDeltasC::default();

        // Collect side effects data
        for (_, side_effect) in self.side_effects.borrow().iter() {
            let res = side_effect.check(frame_data);

            side_effects_summary.body_temp_bonus += res.body_temp_bonus;
            side_effects_summary.heart_rate_bonus += res.heart_rate_bonus;
            side_effects_summary.top_pressure_bonus += res.top_pressure_bonus;
            side_effects_summary.bottom_pressure_bonus += res.bottom_pressure_bonus;
            side_effects_summary.water_level_bonus += res.water_level_bonus;
            side_effects_summary.food_level_bonus += res.food_level_bonus;
            side_effects_summary.stamina_bonus += res.stamina_bonus;

            // Just for pretty picture
            if !frame_data.player.is_sleeping {
                side_effects_summary.fatigue_bonus += res.fatigue_bonus;
            }
        }

        return side_effects_summary;
    }

    fn process_diseases(&self, game_time: &GameTimeC, game_time_delta: f32) -> DiseaseDeltasC {
        // Clean up garbage diseases
        let mut diseases_to_remove = Vec::new();
        {
            let diseases = self.diseases.borrow();
            for (name, disease) in diseases.iter() {
                if disease.get_is_old(game_time) {
                    diseases_to_remove.push(name.clone());
                }
            }
        }
        for disease_name in diseases_to_remove {
            self.remove_disease(&disease_name).ok(); // we don't really care here
        }

        let mut result = DiseaseDeltasC::for_related();

        // Collect disease deltas
        let mut disease_deltas = Vec::new();
        {
            let diseases = self.diseases.borrow();
            for (_, disease) in diseases.iter() {
                if disease.get_is_active(game_time) {
                    disease_deltas.push(disease.get_vitals_deltas(game_time));

                    match disease.get_active_stage(game_time) {
                        Some(st) => {
                            result.stamina_drain += st.info.stamina_drain * game_time_delta; // stamina drain is cumulative
                            result.food_drain += st.info.food_drain * game_time_delta; // food drain is cumulative
                            result.water_drain += st.info.water_drain * game_time_delta; // water drain is cumulative
                        },
                        _ => { }
                    };

                    // Handling self-heal
                    if !disease.needs_treatment && disease.will_self_heal_on != StageLevel::Undefined && !disease.get_is_healing() {
                        let stage = disease.get_active_stage(game_time);
                        match stage {
                            Some(st) => {
                                let p = st.get_percent_active(game_time);
                                let dice = crate::utils::range(50., 99.) as usize;
                                if (st.info.level == disease.will_self_heal_on && p > dice) ||
                                    st.info.level as i32 > disease.will_self_heal_on as i32
                                {
                                    // Invoke the healing process
                                    disease.invert(game_time).ok(); // aren't interested in result
                                }
                            },
                            _ => { }
                        }
                    }
                }
            }
        }

        // Normalize disease deltas
        for d in disease_deltas.iter() {
            result.body_temperature_delta =
                if result.body_temperature_delta < d.body_temperature_delta
                { d.body_temperature_delta } else { result.body_temperature_delta };
            result.heart_rate_delta =
                if result.heart_rate_delta < d.heart_rate_delta
                { d.heart_rate_delta } else { result.heart_rate_delta };
            result.pressure_top_delta =
                if result.pressure_top_delta < d.pressure_top_delta
                { d.pressure_top_delta } else { result.pressure_top_delta };
            result.pressure_bottom_delta =
                if result.pressure_bottom_delta < d.pressure_bottom_delta
                { d.pressure_bottom_delta } else { result.pressure_bottom_delta };
            result.fatigue_delta += d.fatigue_delta; // fatigue is cumulative
        }

        result.cleanup();

        return result;
    }

    fn apply_deltas(&self, snapshot: &mut HealthC, deltas: &SideEffectDeltasC) {
        snapshot.body_temperature += deltas.body_temp_bonus;
        snapshot.heart_rate += deltas.heart_rate_bonus;
        snapshot.top_pressure += deltas.top_pressure_bonus;
        snapshot.bottom_pressure += deltas.bottom_pressure_bonus;
        snapshot.food_level += deltas.food_level_bonus;
        snapshot.water_level += deltas.water_level_bonus;
        snapshot.stamina_level += deltas.stamina_bonus;
        snapshot.fatigue_level += deltas.fatigue_bonus;
    }

    fn apply_disease_deltas(&self, snapshot: &mut HealthC, deltas: &DiseaseDeltasC) {
        snapshot.body_temperature += deltas.body_temperature_delta;
        snapshot.heart_rate += deltas.heart_rate_delta;
        snapshot.top_pressure += deltas.pressure_top_delta;
        snapshot.bottom_pressure += deltas.pressure_bottom_delta;
        snapshot.food_level -= deltas.food_drain;
        snapshot.water_level -= deltas.water_drain;
        snapshot.stamina_level -= deltas.stamina_drain;
        snapshot.fatigue_level += deltas.fatigue_delta;
    }

    fn apply_health_snapshot(&self, snapshot: &HealthC) {
        self.body_temperature.set(snapshot.body_temperature);
        self.heart_rate.set(snapshot.heart_rate);
        self.top_pressure.set(snapshot.top_pressure);
        self.bottom_pressure.set(snapshot.bottom_pressure);
        self.food_level.set(crate::utils::clamp(snapshot.food_level, 0., 100.));
        self.water_level.set(crate::utils::clamp(snapshot.water_level, 0., 100.));
        self.stamina_level.set(crate::utils::clamp(snapshot.stamina_level, 0., 100.));
        self.fatigue_level.set(crate::utils::clamp(snapshot.fatigue_level, 0., 100.));
    }
}