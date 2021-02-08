use crate::events::ZaraEventsListener;
use crate::diseases::Flu;
use crate::injuries::Cut;

use zara::health::InjuryKey;
use std::collections::HashMap;

/// Object that manages save/load state for this demo app
pub struct StateObject {
    /// State of ZaraController. Contains most of the state information needed
    pub main_state: Option<zara::state::ZaraControllerStateContract>,

    pub is_captured: bool,

    // Ids of all registered side effects monitors we use in this demo
    pub monitor_vitals: Option<usize>,
    pub monitor_running: Option<usize>,
    pub monitor_fatigue: Option<usize>,
    pub monitor_food: Option<usize>,
    pub monitor_water: Option<usize>,
    pub monitor_underwater: Option<usize>,

    /// States of all spawned by us diseases
    pub diseases: HashMap<String, zara::state::ActiveDiseaseStateContract>,
    /// States of all spawned by us injuries
    pub injuries: HashMap<InjuryKey, zara::state::ActiveInjuryStateContract>,

    // States of all side effects monitors
    pub monitor_vitals_state: Option<zara::health::side::builtin::DynamicVitalsSideEffectStateContract>,
    pub monitor_running_state: Option<zara::health::side::builtin::RunningSideEffectsStateContract>,
    pub monitor_fatigue_state: Option<zara::health::side::builtin::FatigueSideEffectsStateContract>,
    pub monitor_food_state: Option<zara::health::side::builtin::FoodDrainOverTimeSideEffectStateContract>,
    pub monitor_water_state: Option<zara::health::side::builtin::WaterDrainOverTimeSideEffectStateContract>,
    pub monitor_underwater_state: Option<zara::health::side::builtin::UnderwaterSideEffectStateContract>
}
impl StateObject {
    pub fn new() -> Self {
        StateObject {
            is_captured: false,
            main_state: None,
            monitor_vitals: None,
            monitor_running: None,
            monitor_fatigue: None,
            monitor_food: None,
            monitor_water: None,
            monitor_underwater: None,
            monitor_vitals_state: None,
            monitor_running_state: None,
            monitor_fatigue_state: None,
            monitor_food_state: None,
            monitor_water_state: None,
            monitor_underwater_state: None,
            diseases: HashMap::new(),
            injuries: HashMap::new()
        }
    }
    pub fn capture(&mut self, controller: &zara::ZaraController<ZaraEventsListener>) {
        if self.is_captured { return; }

        // Remember the main state
        self.main_state = Some(controller.get_state());

        // Clear old states
        self.diseases.clear();
        self.injuries.clear();

        // Remember states of all diseases
        for (key, disease) in controller.health.diseases.borrow().iter() {
            self.diseases.insert(key.to_string(), disease.get_state());
        }

        // Remember states of all injuries
        for (key, injury) in controller.health.injuries.borrow().iter() {
            self.injuries.insert(key.clone(), injury.get_state());
        }

        // Remember states of all side effect monitors
        let monitors = controller.health.side_effects.borrow();
        self.monitor_fatigue_state = match &self.monitor_fatigue {
            Some(mid) => match &monitors.get(mid) {
                Some(m) => match m.as_any().downcast_ref::<zara::health::side::builtin::FatigueSideEffects>() {
                    Some(o) => Some(o.get_state()),
                    None => None
                }
                None => None
            },
            None => None
        };
        self.monitor_food_state = match &self.monitor_food {
            Some(mid) => match &monitors.get(mid) {
                Some(m) => match m.as_any().downcast_ref::<zara::health::side::builtin::FoodDrainOverTimeSideEffect>() {
                    Some(o) => Some(o.get_state()),
                    None => None
                }
                None => None
            },
            None => None
        };
        self.monitor_running_state = match &self.monitor_running {
            Some(mid) => match &monitors.get(mid) {
                Some(m) => match m.as_any().downcast_ref::<zara::health::side::builtin::RunningSideEffects>() {
                    Some(o) => Some(o.get_state()),
                    None => None
                }
                None => None
            },
            None => None
        };
        self.monitor_underwater_state = match &self.monitor_underwater {
            Some(mid) => match &monitors.get(mid) {
                Some(m) => match m.as_any().downcast_ref::<zara::health::side::builtin::UnderwaterSideEffect>() {
                    Some(o) => Some(o.get_state()),
                    None => None
                }
                None => None
            },
            None => None
        };
        self.monitor_vitals_state = match &self.monitor_vitals {
            Some(mid) => match &monitors.get(mid) {
                Some(m) => match m.as_any().downcast_ref::<zara::health::side::builtin::DynamicVitalsSideEffect>() {
                    Some(o) => Some(o.get_state()),
                    None => None
                }
                None => None
            },
            None => None
        };
        self.monitor_water_state = match &self.monitor_water {
            Some(mid) => match &monitors.get(mid) {
                Some(m) => match m.as_any().downcast_ref::<zara::health::side::builtin::WaterDrainOverTimeSideEffect>() {
                    Some(o) => Some(o.get_state()),
                    None => None
                }
                None => None
            },
            None => None
        };

        self.is_captured = true;
    }
    pub fn restore(&self, controller: &zara::ZaraController<ZaraEventsListener>) {
        if !self.is_captured { return; }

        // Restore the main Zara state
        match &self.main_state {
            Some(st) => controller.restore_state(st),
            _ => { }
        };

        // Clear diseases
        {
            let mut b = controller.health.diseases.borrow_mut();

            b.clear();
        }

        // Clear injuries
        {
            let mut b = controller.health.injuries.borrow_mut();

            b.clear();
        }

        // Restore diseases
        for (_, state) in &self.diseases {
            // For this demo, we'll do it without proper factory
            // We have only one disease here, so, we'll ignore the key
            controller.health.restore_disease(state, Box::new(Flu));
        }

        // Restore injuries
        for (_, state) in &self.injuries {
            // For this demo, we'll do it without proper factory
            // We have only one injury here, so, we'll ignore the key
            controller.health.restore_injury(state, Box::new(Cut));
        }

        // Restore states of all side effects monitors we use
        let monitors = controller.health.side_effects.borrow();

        match &self.monitor_fatigue {
            Some(mid) => match &self.monitor_fatigue_state {
                Some(st) => match monitors.get(mid) {
                    Some(m) => match m.as_any().downcast_ref::<zara::health::side::builtin::FatigueSideEffects>() {
                        Some(o) => o.restore_state(st),
                        _ => { }
                    },
                    _ => { }
                },
                _ => { }
            },
            _ => { }
        };
        match &self.monitor_water {
            Some(mid) => match &self.monitor_water_state {
                Some(st) => match monitors.get(mid) {
                    Some(m) => match m.as_any().downcast_ref::<zara::health::side::builtin::WaterDrainOverTimeSideEffect>() {
                        Some(o) => o.restore_state(st),
                        _ => { }
                    },
                    _ => { }
                },
                _ => { }
            },
            _ => { }
        };
        match &self.monitor_vitals {
            Some(mid) => match &self.monitor_vitals_state {
                Some(st) => match monitors.get(mid) {
                    Some(m) => match m.as_any().downcast_ref::<zara::health::side::builtin::DynamicVitalsSideEffect>() {
                        Some(o) => o.restore_state(st),
                        _ => { }
                    },
                    _ => { }
                },
                _ => { }
            },
            _ => { }
        };
        match &self.monitor_underwater {
            Some(mid) => match &self.monitor_underwater_state {
                Some(st) => match monitors.get(mid) {
                    Some(m) => match m.as_any().downcast_ref::<zara::health::side::builtin::UnderwaterSideEffect>() {
                        Some(o) => o.restore_state(st),
                        _ => { }
                    },
                    _ => { }
                },
                _ => { }
            },
            _ => { }
        };
        match &self.monitor_running {
            Some(mid) => match &self.monitor_running_state {
                Some(st) => match monitors.get(mid) {
                    Some(m) => match m.as_any().downcast_ref::<zara::health::side::builtin::RunningSideEffects>() {
                        Some(o) => o.restore_state(st),
                        _ => { }
                    },
                    _ => { }
                },
                _ => { }
            },
            _ => { }
        };
        match &self.monitor_food {
            Some(mid) => match &self.monitor_food_state {
                Some(st) => match monitors.get(mid) {
                    Some(m) => match m.as_any().downcast_ref::<zara::health::side::builtin::FoodDrainOverTimeSideEffect>() {
                        Some(o) => o.restore_state(st),
                        _ => { }
                    },
                    _ => { }
                },
                _ => { }
            },
            _ => { }
        };
    }
}