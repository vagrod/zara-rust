use std::thread;
use std::io::{self, Write};
use std::time::{Duration, Instant};
use std::thread::sleep;

use crate::events::ZaraEventsListener;
use crate::ui::ui_frame;
use crate::zara_init::init_zara_instance;
use crate::state::StateObject;

use zara::body::BodyPart;
use crossterm::terminal;
use crossterm::execute;

mod zara_init;
mod state;
mod diseases;
mod injuries;
mod inventory;
mod events;
mod ui;

// This will spawn a new thread for the "game loop"
fn main() {
    let game_loop = thread::spawn(|| {
        let mut stderr = io::stdout();

        execute!(stderr, terminal::EnterAlternateScreen).ok();

        terminal::enable_raw_mode().ok();

        let two_millis= Duration::new(0, 2_000_000); // 2ms

        let mut state = StateObject::new();
        let mut is_disease_inverted = false;
        let mut is_item_consumed = false;
        let mut is_jacket_off = false;
        let mut frame_time;
        let mut now;
        let mut console_update_counter = 0.;

        let person = init_zara_instance();

        register_side_effects(&person, &mut state);
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
                person.consume(&format!("Aspirin Pills")).ok();
                is_item_consumed = true;
            }

            if person.environment.game_time.minute.get() == 5 {
                state.capture(&person);
            }

            if person.environment.game_time.minute.get() == 6 && !is_jacket_off {
                person.take_off_clothes(&format!("Jacket")).ok();
                person.player_state.is_running.set(true);
                is_jacket_off = true;
            }

            // State restore test -- roll back to 5 min mark
            if person.environment.game_time.minute.get() == 10 {
                state.restore(&person);
            }

            // Disease "invert" test
            if person.environment.game_time.minute.get() == 20 || person.environment.game_time.minute.get() == 42 {
                if !is_disease_inverted {
                    person.health.diseases.borrow().get("Flu").unwrap().invert(&person.environment.game_time.to_contract()).ok();
                    is_disease_inverted = true;
                }
            }
            // Disease "invert back" test
            if person.environment.game_time.minute.get() == 33 {
                if is_disease_inverted {
                    person.health.diseases.borrow().get("Flu").unwrap().invert_back(&person.environment.game_time.to_contract()).ok();
                    is_disease_inverted = false;
                }
            }

            // Wetness/drying test
            /*if person.environment.game_time.minute.get() == 6 && person.environment.game_time.second.get() < 30. {
                if !person.player_state.is_underwater.get() {
                    person.player_state.is_underwater.set(true);
                }
            }
            if person.environment.game_time.minute.get() == 8 && person.environment.game_time.second.get() < 20. {
                //person.player_state.is_running.set(false);
                if person.player_state.is_underwater.get() {
                    person.player_state.is_underwater.set(false);
                }
            }*/

            // Update Zara state
            person.update(frame_time).ok();

            // Update console data
            console_update_counter += frame_time;

            if console_update_counter >= 1. {
                console_update_counter = 0.;

                ui_frame(&mut stderr, &person);
            }
        }
    });

    game_loop.join().unwrap();
}

fn register_side_effects(person: &zara::ZaraController<ZaraEventsListener>, state: &mut StateObject) {
    let vitals_effects = Box::new(zara::health::side::builtin::DynamicVitalsSideEffect::new());
    let running_effects = Box::new(zara::health::side::builtin::RunningSideEffects::new(0.22, 0.009));
    let fatigue_effects = Box::new(zara::health::side::builtin::FatigueSideEffects::new(8));
    let food_drain_effect = Box::new(zara::health::side::builtin::FoodDrainOverTimeSideEffect::new(0.01));
    let water_drain_effect = Box::new(zara::health::side::builtin::WaterDrainOverTimeSideEffect::new(0.03));
    let underwater_effect = Box::new(zara::health::side::builtin::UnderwaterSideEffect::new(0.15, 0.28));

    state.monitor_vitals = Some(person.health.register_side_effect_monitor(vitals_effects));
    state.monitor_running = Some(person.health.register_side_effect_monitor(running_effects));
    state.monitor_fatigue = Some(person.health.register_side_effect_monitor(fatigue_effects));
    state.monitor_food = Some(person.health.register_side_effect_monitor(food_drain_effect));
    state.monitor_water = Some(person.health.register_side_effect_monitor(water_drain_effect));
    state.monitor_underwater = Some(person.health.register_side_effect_monitor(underwater_effect));
}

fn spawn_diseases(person: &zara::ZaraController<ZaraEventsListener>) {
    person.health.spawn_disease(Box::new(diseases::Flu), zara::utils::GameTimeC::new(0,0,3,30.)).ok();
    //person.health.spawn_disease(Box::new(diseases::Angina), zara::utils::GameTimeC::new(0,0,2,42.));
}

fn spawn_injuries(person: &zara::ZaraController<ZaraEventsListener>) {
    let key_result = person.health.spawn_injury(Box::new(injuries::Cut), BodyPart::LeftShoulder, zara::utils::GameTimeC::new(0,0,2,25.));

    person.health.spawn_injury(Box::new(injuries::Cut), BodyPart::Forehead, zara::utils::GameTimeC::new(0,0,7,25.)).ok();

    // Body appliances test
    person.take_appliance(&format!("Bandage"), BodyPart::LeftShoulder).ok();

    if let Ok(key) = key_result {
        person.health.injuries.borrow().get(&key).unwrap().stop_blood_loss();
    }
    //person.remove_appliance(&format!("Bandage"), BodyParts::LeftShoulder);
}