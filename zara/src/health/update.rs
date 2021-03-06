use crate::health::{Health, StageLevel, InjuryKey};
use crate::health::side::{SideEffectDeltasC};
use crate::health::disease::{DiseaseDeltasC};
use crate::utils::{HealthC, FrameC, GameTimeC, FrameSummaryC};
use crate::utils::event::{Event, Listener, Dispatcher, MessageQueue};
use crate::health::injury::{InjuryDeltasC};

use std::cell::RefMut;
use std::collections::BTreeMap;

struct ProcessDiseasesResult {
    deltas: DiseaseDeltasC
}

struct ProcessInjuriesResult {
    deltas: InjuryDeltasC,
    blood_loss: bool
}

impl Health {
    /// This method is called every `UPDATE_INTERVAL` real seconds
    ///
    /// # Parameters
    /// - `frame`: summary information for this frame
    pub(crate) fn update<E: Listener + 'static>(&self, frame: &mut FrameC<E>) {
        // Update disease monitors
        for (_, monitor) in self.disease_monitors.borrow().iter() {
            monitor.check(self, &frame.data);
        }

        // Update medical agents
        self.medical_agents.update(&frame.data.game_time);
        if self.medical_agents.has_messages() {
            self.flush_queue(self.medical_agents.get_message_queue());
        }

        let mut snapshot = HealthC::healthy();

        // Stamina, blood, oxygen, food and water are relative
        snapshot.stamina_level = self.stamina_level.get();
        snapshot.food_level = self.food_level.get();
        snapshot.water_level = self.water_level.get();
        snapshot.blood_level = self.blood_level.get();
        snapshot.oxygen_level = self.oxygen_level.get();

        // For pretty picture, freeze fatigue value when sleeping
        if frame.data.player.is_sleeping {
           snapshot.fatigue_level = frame.data.health.fatigue_level;
        }

        // Retrieve side effects deltas
        let side_effects_summary = self.process_side_effects(&frame.data);

        // Apply side effects deltas
        self.apply_deltas(&mut snapshot, &side_effects_summary);

        // Process diseases and get vitals deltas from them
        let diseases_result = self.process_diseases(&frame.data.game_time, frame.data.game_time_delta);

        // Apply disease deltas
        self.apply_disease_deltas(&mut snapshot, &diseases_result.deltas);

        // Process injuries and get drain deltas from them
        let injuries_result = self.process_injuries(&frame.data.game_time, frame.data.game_time_delta);

        // Apply injuries deltas
        self.apply_injury_deltas(&mut snapshot, &injuries_result.deltas);

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
        // Will always regain oxygen. Side effects must "fight" it
        {
            let value = snapshot.oxygen_level + self.oxygen_regain_rate.get() * frame.data.game_time_delta;
            snapshot.oxygen_level = crate::utils::clamp(value, 0., 100.);
        }

        // Apply the resulted health snapshot
        self.apply_health_snapshot(&snapshot);

        self.has_blood_loss.set(injuries_result.blood_loss);

        // Do the external events
        self.dispatch_events::<E>(frame.events);
    }

    fn dispatch_events<E: Listener + 'static>(&self, events: &mut Dispatcher<E>) {
        const HEART_RATE_LOW_DANGER: f32 = 20.;
        const HEART_RATE_HIGH_DANGER: f32 = 200.;
        const BLOOD_PRESSURE_TOP_LOW_DANGER: f32 = 50.;
        const BLOOD_PRESSURE_TOP_HIGH_DANGER: f32 = 230.;
        const BLOOD_PRESSURE_BOTTOM_LOW_DANGER: f32 = 35.;
        const BLOOD_PRESSURE_BOTTOM_HIGH_DANGER: f32 = 130.;
        const TEMPERATURE_LOW_DANGER: f32 = 33.6;
        const TEMPERATURE_HIGH_DANGER: f32 = 41.2;

        if self.is_no_strength() {
            events.dispatch(Event::StaminaDrained);
        }
        if self.is_low_oxygen() {
            events.dispatch(Event::OxygenDrained);
        }
        if self.is_low_food() {
            events.dispatch(Event::FoodDrained);
        }
        if self.is_low_water() {
            events.dispatch(Event::WaterDrained);
        }
        if self.is_low_blood() {
            events.dispatch(Event::BloodDrained);
        }
        if self.is_exhausted() {
            events.dispatch(Event::Exhausted);
        } else {
            if self.is_tired() {
                events.dispatch(Event::Tired);
            }
        }
        if self.top_pressure.get() <= BLOOD_PRESSURE_TOP_LOW_DANGER ||
           self.bottom_pressure.get() <= BLOOD_PRESSURE_BOTTOM_LOW_DANGER
        {
            events.dispatch(Event::LowBloodPressureDanger);
        }
        if self.top_pressure.get() >= BLOOD_PRESSURE_TOP_HIGH_DANGER ||
            self.bottom_pressure.get() >= BLOOD_PRESSURE_BOTTOM_HIGH_DANGER
        {
            events.dispatch(Event::HighBloodPressureDanger);
        }
        if self.body_temperature.get() <= TEMPERATURE_LOW_DANGER {
            events.dispatch(Event::LowBodyTemperatureDanger);
        }
        if self.body_temperature.get() >= TEMPERATURE_HIGH_DANGER {
            events.dispatch(Event::HighBodyTemperatureDanger);
        }
        if self.heart_rate.get() <= HEART_RATE_LOW_DANGER {
            events.dispatch(Event::LowHeartRateDanger);
        }
        if self.heart_rate.get() >= HEART_RATE_HIGH_DANGER {
            events.dispatch(Event::HighHeartRateDanger);
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
            side_effects_summary.oxygen_level_bonus += res.oxygen_level_bonus;

            // Just for pretty picture
            if !frame_data.player.is_sleeping {
                side_effects_summary.fatigue_bonus += res.fatigue_bonus;
            }
        }

        side_effects_summary
    }

    fn process_diseases(&self, game_time: &GameTimeC, game_time_delta: f32) -> ProcessDiseasesResult {
        // Clean up garbage diseases
        let mut diseases_to_remove = Vec::new();
        {
            let diseases = self.diseases.borrow();
            for (name, disease) in diseases.iter() {
                if disease.is_old(game_time) {
                    self.queue_message(Event::DiseaseExpired(disease.disease.get_name()));
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
            for (disease_name, disease) in diseases.iter() {
                // Move messages from diseases to the main queue for further processing
                if disease.has_messages() {
                    self.flush_queue(disease.get_message_queue());
                }
                if disease.is_active(game_time) {
                    disease_deltas.push(disease.get_vitals_deltas(game_time));

                    let active_stage = disease.get_active_stage(game_time);

                    // Handling death probabilities
                    if let Some(st) = &active_stage {
                        let chance = st.info.chance_of_death.unwrap_or(0);

                        if chance > 0 && !disease.is_healing() {
                            // The further into the stage, the bigger is probability of death
                            if crate::utils::roll_dice(st.percent_active(game_time))
                                && crate::utils::roll_dice(chance)
                            {
                                self.is_alive.set(false);

                                self.queue_message(Event::DeathFromDisease(disease_name.to_string()))
                            }
                        }
                    }

                    // Handling self-heal
                    if !disease.needs_treatment && disease.will_self_heal_on != StageLevel::Undefined && !disease.is_healing() {
                        if let Some(st) = &active_stage {
                            let p = st.percent_active(game_time);
                            let dice = crate::utils::range(50., 99.) as usize;
                            if (st.info.level == disease.will_self_heal_on && p > dice) ||
                                st.info.level as i32 > disease.will_self_heal_on as i32
                            {
                                // Invoke the healing process
                                disease.invert(game_time).ok(); // aren't interested in result
                                self.queue_message(Event::DiseaseSelfHealStarted(disease_name.to_string()));
                            }
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

            // Those are % per game second drains
            result.stamina_drain += d.stamina_drain * game_time_delta; // stamina drain is cumulative
            result.oxygen_drain += d.oxygen_drain * game_time_delta; // oxygen drain is cumulative
            result.food_drain += d.food_drain * game_time_delta; // food drain is cumulative
            result.water_drain += d.water_drain * game_time_delta; // water drain is cumulative
        }

        result.cleanup();

        ProcessDiseasesResult {
            deltas: result
        }
    }

    fn process_injuries(&self, game_time: &GameTimeC, game_time_delta: f32) -> ProcessInjuriesResult {
        let mut blood_loss = false;

        // Clean up garbage injuries
        let mut injuries_to_remove = Vec::new();
        {
            let injuries = self.injuries.borrow();
            for (key, injury) in injuries.iter() {
                if injury.is_old(game_time) {
                    self.queue_message(Event::InjuryExpired(injury.injury.get_name(), key.body_part));
                    injuries_to_remove.push(InjuryKey::new(key.injury.to_string(), key.body_part));
                }
            }
        }
        for injury_key in injuries_to_remove {
            self.remove_injury(injury_key.injury, injury_key.body_part).ok(); // we don't really care here
        }

        let mut result = InjuryDeltasC::for_related();

        // Collect injury deltas
        let mut injury_deltas = Vec::new();
        {
            let injuries = self.injuries.borrow();
            for (_, injury) in injuries.iter() {
                // Move messages from injuries to the main queue for further processing
                if injury.has_messages() {
                    self.flush_queue(injury.get_message_queue());
                }
                if injury.is_active(game_time) {
                    let d = injury.get_drains_deltas(game_time);

                    if !injury.is_blood_stopped() && d.blood_drain > 0. { blood_loss = true; }

                    injury_deltas.push(d);

                    let active_stage = injury.get_active_stage(game_time);

                    // Handling death probabilities
                    if let Some(st) = &active_stage {
                        let chance = st.info.chance_of_death.unwrap_or(0);

                        if chance > 0 && !injury.is_healing() {
                            // The further into the stage, the bigger is probability of death
                            if crate::utils::roll_dice(st.percent_active(game_time))
                                && crate::utils::roll_dice(chance)
                            {
                                self.is_alive.set(false);

                                self.queue_message(Event::DeathFromInjury(
                                    injury.injury.get_name().to_string(),
                                    injury.body_part
                                ));
                            }
                        }
                    }

                    // Handling self-heal
                    if !injury.needs_treatment && injury.will_self_heal_on != StageLevel::Undefined && !injury.is_healing() {
                        if let Some(st) = &active_stage {
                            let p = st.percent_active(game_time);
                            let dice = crate::utils::range(50., 99.) as usize;
                            if (st.info.level == injury.will_self_heal_on && p > dice) ||
                                st.info.level as i32 > injury.will_self_heal_on as i32
                            {
                                // Invoke the healing process
                                injury.invert(game_time).ok(); // aren't interested in result
                                self.queue_message(Event::InjurySelfHealStarted(
                                    injury.injury.get_name().to_string(),
                                    injury.body_part
                                ));
                            }
                        }
                    }
                }
            }
        }

        // Normalize injury deltas
        for d in injury_deltas.iter() {
            // Those are % per game second drains
            result.stamina_drain += d.stamina_drain * game_time_delta; // stamina drain is cumulative
            result.blood_drain += d.blood_drain * game_time_delta; // blood drain is cumulative
        }

        result.cleanup();

        ProcessInjuriesResult {
            deltas: result,
            blood_loss
        }
    }

    fn apply_deltas(&self, snapshot: &mut HealthC, deltas: &SideEffectDeltasC) {
        snapshot.body_temperature += deltas.body_temp_bonus;
        snapshot.heart_rate += deltas.heart_rate_bonus;
        snapshot.top_pressure += deltas.top_pressure_bonus;
        snapshot.bottom_pressure += deltas.bottom_pressure_bonus;
        snapshot.food_level += deltas.food_level_bonus;
        snapshot.water_level += deltas.water_level_bonus;
        snapshot.stamina_level += deltas.stamina_bonus;
        snapshot.oxygen_level += deltas.oxygen_level_bonus;
        snapshot.fatigue_level += deltas.fatigue_bonus;
    }

    fn apply_disease_deltas(&self, snapshot: &mut HealthC, deltas: &DiseaseDeltasC) {
        snapshot.body_temperature += deltas.body_temperature_delta;
        snapshot.heart_rate += deltas.heart_rate_delta;
        snapshot.top_pressure += deltas.pressure_top_delta;
        snapshot.bottom_pressure += deltas.pressure_bottom_delta;
        snapshot.fatigue_level += deltas.fatigue_delta;
        snapshot.food_level -= deltas.food_drain;
        snapshot.water_level -= deltas.water_drain;
        snapshot.stamina_level -= deltas.stamina_drain;
        snapshot.oxygen_level -= deltas.oxygen_drain;
    }

    fn apply_injury_deltas(&self, snapshot: &mut HealthC, deltas: &InjuryDeltasC) {
        snapshot.blood_level -= deltas.blood_drain;
        snapshot.stamina_level -= deltas.stamina_drain;
    }

    fn apply_health_snapshot(&self, snapshot: &HealthC) {
        self.body_temperature.set(snapshot.body_temperature);
        self.heart_rate.set(snapshot.heart_rate);
        self.top_pressure.set(snapshot.top_pressure);
        self.bottom_pressure.set(snapshot.bottom_pressure);
        self.food_level.set(crate::utils::clamp(snapshot.food_level, 0., 100.));
        self.water_level.set(crate::utils::clamp(snapshot.water_level, 0., 100.));
        self.blood_level.set(crate::utils::clamp(snapshot.blood_level, 0., 100.));
        self.stamina_level.set(crate::utils::clamp(snapshot.stamina_level, 0., 100.));
        self.oxygen_level.set(crate::utils::clamp(snapshot.oxygen_level, 0., 100.));
        self.fatigue_level.set(crate::utils::clamp(snapshot.fatigue_level, 0., 100.));
    }

    fn flush_queue(&self, mut q: RefMut<BTreeMap<usize, Event>>) {
        if q.len() == 0 { return }

        let mut key = 0;

        loop {
            match q.get(&key) {
                Some(event) => {
                    self.queue_message(event.clone());

                    q.remove(&key);
                },
                None => break
            }

            key += 1;
        }
    }
}