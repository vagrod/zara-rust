use std::thread;
use std::io::{self, Write};
use std::time::{Duration, Instant};
use std::thread::sleep;
use std::cell::Cell;

use crate::events::ZaraEventsListener;
use crate::ui::ui_frame;

use zara::body::{BodyParts};
use zara::utils::event::{Listener, Event};
use zara::utils::{FrameSummaryC, GameTimeC};
use zara::health::{Health, StageLevel, MedicalAgentBuilder};
use zara::health::medagent::{CurveType};
use zara::health::disease::{DiseaseMonitor, Disease};
use zara::health::side::builtin::{RunningSideEffects, DynamicVitalsSideEffect, FatigueSideEffects};
use zara::inventory::items::{InventoryItem, ConsumableC, ConsumableBehavior, SpoilingBehavior};
use zara::inventory::crafting;

use crossterm::terminal;
use crossterm::execute;

mod diseases;
mod inventory;
mod injuries;
mod events;
mod ui;

// This will spawn a new thread for the "game loop"
fn main() {
    let game_loop = thread::spawn(|| {
        let mut stderr = io::stdout();

        execute!(stderr, terminal::EnterAlternateScreen).ok();

        terminal::enable_raw_mode().ok();

        let mut is_disease_inverted = false;
        let mut is_item_consumed = false;

        let two_millis= Duration::new(0, 2_000_000); // 2ms
        let mut frame_time= 0_f32;
        let mut now = Instant::now();
        let mut console_update_counter = 0.;

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
        spawn_diseases(&person);
        spawn_injuries(&person);

        loop {
            now = Instant::now();

            // Cap the "framerate"
            sleep(two_millis);

            frame_time = now.elapsed().as_secs_f32();

            if person.body.is_sleeping() {
                // Progress time faster during the sleep
                person.environment.game_time.add_seconds(frame_time * 1800.); // 30 game minutes per real second
            } else {
                // Game time is 10x the real one
                person.environment.game_time.add_seconds(frame_time * 10.);
            }

            if person.environment.game_time.minute.get() == 4 && !is_item_consumed {
                person.consume(&format!("Meat"));
                is_item_consumed = true;
            }

            // Disease "invert" test
            if person.environment.game_time.minute.get() == 20 || person.environment.game_time.minute.get() == 42 {
                if !is_disease_inverted {
                    person.health.diseases.borrow().get("Flu").unwrap().invert(&person.environment.game_time.to_contract());
                    is_disease_inverted = true;
                }
            }
            // Disease "invert back" test
            if person.environment.game_time.minute.get() == 33 {
                if is_disease_inverted {
                    person.health.diseases.borrow().get("Flu").unwrap().invert_back(&person.environment.game_time.to_contract());
                    is_disease_inverted = false;
                }
            }

            // Update Zara state
            person.update(frame_time);

            // Update console data
            console_update_counter += frame_time;

            if console_update_counter >= 1. {
                console_update_counter = 0.;

                ui_frame(&mut stderr, &person);
               // flush_data(&mut stdout, &person);
            }
        }
    });

    game_loop.join().unwrap();
}

fn spawn_diseases(person: &zara::ZaraController<ZaraEventsListener>) {
    person.health.spawn_disease(Box::new(diseases::Flu), zara::utils::GameTimeC::new(0,0,3,30.));
    //person.health.spawn_disease(Box::new(diseases::Angina), zara::utils::GameTimeC::new(0,0,2,42.));
}

fn spawn_injuries(person: &zara::ZaraController<ZaraEventsListener>) {
    person.health.spawn_injury(Box::new(injuries::Cut), BodyParts::LeftShoulder, zara::utils::GameTimeC::new(0,0,2,25.));
    person.health.spawn_injury(Box::new(injuries::Cut), BodyParts::Forehead, zara::utils::GameTimeC::new(0,0,7,25.));
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