use super::inventory;
use super::events::ZaraEventsListener;

use zara::health::medagent::{CurveType};
use zara::health::{MedicalAgentBuilder};
use std::cell::Cell;

pub fn init_zara_instance() -> zara::ZaraController<ZaraEventsListener>{
    // Instantiate our events listener
    let events_listener = ZaraEventsListener;

    // Describe environment conditions
    let environment = zara::utils::EnvironmentC::new(24., 2., 0.);

    // Initialize Zara instance
    let person =
        zara::ZaraController::with_environment (
            events_listener, environment
        );

    person.health.register_medical_agent (
        MedicalAgentBuilder::start()
            .for_agent("Aspirin")
            .activates(CurveType::Immediately)
            .and_peaks_in_minutes(30.)
            .contains(
                vec![
                    "Big Green Leaves",
                    "Aspirin Pills",
                    "Syringe With Aspirin",
                    "This Strange Glowy Pink Goop That I Found In Thaaaaat Very Cave Yesterday When I Was Wandering Here At Night And..."
                ]
            ).build()
    );

    add_side_effects(&person);
    populate_inventory(&person);

    return person;
}

fn populate_inventory(person: &zara::ZaraController<ZaraEventsListener>) {
    let meat = inventory::Meat{ count: Cell::new(2) };
    let knife = inventory::Knife{ count: Cell::new(1) };
    let rope = inventory::Rope{ count: Cell::new(5) };
    let mre = inventory::MRE{ count: Cell::new(2) };

    person.inventory.add_item(Box::new(meat));
    person.inventory.add_item(Box::new(knife));
    person.inventory.add_item(Box::new(rope));
    person.inventory.add_item(Box::new(mre));
}

fn add_side_effects(person: &zara::ZaraController<ZaraEventsListener>) {
    let vitals_effects = zara::health::side::builtin::DynamicVitalsSideEffect::new();
    person.health.register_side_effect_monitor(Box::new(vitals_effects));

    let running_effects = zara::health::side::builtin::RunningSideEffects::new();
    person.health.register_side_effect_monitor(Box::new(running_effects));

    let fatigue_effects = zara::health::side::builtin::FatigueSideEffects::new(8);
    person.health.register_side_effect_monitor(Box::new(fatigue_effects));

    let food_drain_effect =  zara::health::side::builtin::FoodDrainOverTimeSideEffect::new(0.01);
    person.health.register_side_effect_monitor(Box::new(food_drain_effect));

    let water_drain_effect =  zara::health::side::builtin::WaterDrainOverTimeSideEffect::new(0.03);
    person.health.register_side_effect_monitor(Box::new(water_drain_effect));
}