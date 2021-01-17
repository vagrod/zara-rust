use crate::health::Health;
use crate::utils::{HealthC, FrameC};
use crate::utils::event::{Event, Listener};
use crate::health::side::SideEffectDeltasC;

/// Contains code related to the `update` method (calculating and updating health state)

impl Health {
    /// This method is called every `UPDATE_INTERVAL` real seconds
    ///
    /// # Parameters
    /// - `frame`: summary information for this frame
    pub fn update<E: Listener + 'static>(&self, frame: &mut FrameC<E>){
        // Update disease monitors
        for monitor in self.monitors.borrow().iter() {
            monitor.check(self, &frame.data);
        }

        let mut side_effects_summary: SideEffectDeltasC = Default::default();

        // Collect side effects data
        for side_effect in self.side_effects.borrow().iter() {
            let res = side_effect.check(&frame.data);

            side_effects_summary.body_temp_bonus += res.body_temp_bonus;
            side_effects_summary.heart_rate_bonus += res.heart_rate_bonus;
            side_effects_summary.top_pressure_bonus += res.top_pressure_bonus;
            side_effects_summary.bottom_pressure_bonus += res.bottom_pressure_bonus;
            side_effects_summary.water_level_bonus += res.water_level_bonus;
            side_effects_summary.stamina_bonus += res.stamina_bonus;

            // Just for pretty picture
            if !frame.data.player.is_sleeping {
                side_effects_summary.fatigue_bonus += res.fatigue_bonus;
            }
        }

        let mut snapshot = HealthC::healthy();

        // Stamina, blood, food and water are relative
        snapshot.stamina_level = self.stamina_level.get();
        snapshot.food_level = self.food_level.get();
        snapshot.water_level = self.water_level.get();
        snapshot.blood_level = self.blood_level.get();

        // For pretty picture
        if frame.data.player.is_sleeping {
           snapshot.fatigue_level = frame.data.health.fatigue_level;
        }

        // Apply monitors deltas
        self.apply_deltas(&mut snapshot, &side_effects_summary);

        // TODO: collect and apply disease effects

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

        if self.is_no_strength() {
            frame.events.dispatch(Event::StaminaDrained);
        }
        if self.is_tired() {
            frame.events.dispatch(Event::Tired);
        }
        if self.is_exhausted() {
            frame.events.dispatch(Event::Exhausted);
        }
    }

    fn apply_deltas(&self, snapshot: &mut HealthC, deltas: &SideEffectDeltasC){
        snapshot.body_temperature += deltas.body_temp_bonus;
        snapshot.heart_rate += deltas.heart_rate_bonus;
        snapshot.top_pressure += deltas.top_pressure_bonus;
        snapshot.bottom_pressure += deltas.bottom_pressure_bonus;
        snapshot.water_level += deltas.water_level_bonus;
        snapshot.food_level += deltas.food_level_bonus;
        snapshot.stamina_level += deltas.stamina_bonus;
        snapshot.fatigue_level += deltas.fatigue_bonus;
    }

    fn apply_health_snapshot(&self, snapshot: &HealthC) {
        self.body_temperature.set(snapshot.body_temperature);
        self.heart_rate.set(snapshot.heart_rate);
        self.top_pressure.set(snapshot.top_pressure);
        self.bottom_pressure.set(snapshot.bottom_pressure);
        self.water_level.set(crate::utils::clamp(snapshot.water_level, 0., 100.));
        self.stamina_level.set(crate::utils::clamp(snapshot.stamina_level, 0., 100.));
        self.fatigue_level.set(crate::utils::clamp(snapshot.fatigue_level, 0., 100.));
    }
}