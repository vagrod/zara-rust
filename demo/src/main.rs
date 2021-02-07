use std::thread;
use std::io::{self, Write};
use std::time::{Duration, Instant};
use std::thread::sleep;

use crate::events::ZaraEventsListener;
use crate::ui::ui_frame;
use crate::zara_init::init_zara_instance;
use crate::diseases::Flu;
use crate::injuries::Cut;

use zara::body::BodyPart;
use zara::health::InjuryKey;
use crossterm::terminal;
use crossterm::execute;

mod zara_init;
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

        let mut is_disease_inverted = false;
        let mut is_item_consumed = false;
        let mut is_jacket_off = false;

        let mut state = None;
        let mut disease_state = None;
        let mut injury1_state = None;
        let mut injury2_state = None;

        let two_millis= Duration::new(0, 2_000_000); // 2ms
        let mut frame_time= 0_f32;
        let mut now = Instant::now();
        let mut console_update_counter = 0.;

        let person = init_zara_instance();

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
                person.consume(&format!("Aspirin Pills"));
                is_item_consumed = true;
            }

            if person.environment.game_time.minute.get() == 5 {
                match &state {
                    None => {
                        let b = person.health.injuries.borrow();
                        let i1 = b.get(&InjuryKey { injury: format!("Cut"), body_part: BodyPart::LeftShoulder }).unwrap();
                        let i2 = b.get(&InjuryKey { injury: format!("Cut"), body_part: BodyPart::Forehead }).unwrap();

                        state = Some(person.get_state());
                        disease_state = Some(person.health.diseases.borrow().get("Flu").unwrap().get_state());
                        injury1_state = Some(i1.get_state());
                        injury2_state = Some(i2.get_state());
                    },
                    Some(_) => {}
                }
            }

            if person.environment.game_time.minute.get() == 6 && !is_jacket_off {
                person.take_off_clothes(&format!("Jacket"));
                person.player_state.is_running.set(true);
                is_jacket_off = true;
            }

            /* State restore test -- roll back to 5 min mark
            if person.environment.game_time.minute.get() == 10 {
                match &state {
                    Some(st) => {
                        person.restore_state(st);

                        person.health.diseases.borrow_mut().clear();
                        person.health.injuries.borrow_mut().clear();

                        match &disease_state {
                            Some(st) => {
                                person.health.restore_disease(st, Box::new(Flu));
                            },
                            None => { }
                        }
                        match &injury1_state {
                            Some(st) => {
                                person.health.restore_injury(st, Box::new(Cut));
                            },
                            None => { }
                        }
                        match &injury2_state {
                            Some(st) => {
                                person.health.restore_injury(st, Box::new(Cut));
                            },
                            None => { }
                        }

                        state = None;
                        disease_state = None;
                        injury1_state = None;
                        injury2_state = None;
                    },
                    None => { }
                }
            }*/

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
            person.update(frame_time);

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

fn spawn_diseases(person: &zara::ZaraController<ZaraEventsListener>) {
    person.health.spawn_disease(Box::new(diseases::Flu), zara::utils::GameTimeC::new(0,0,3,30.));
    //person.health.spawn_disease(Box::new(diseases::Angina), zara::utils::GameTimeC::new(0,0,2,42.));
}

fn spawn_injuries(person: &zara::ZaraController<ZaraEventsListener>) {
    let key_result = person.health.spawn_injury(Box::new(injuries::Cut), BodyPart::LeftShoulder, zara::utils::GameTimeC::new(0,0,2,25.));

    person.health.spawn_injury(Box::new(injuries::Cut), BodyPart::Forehead, zara::utils::GameTimeC::new(0,0,7,25.));

    // Body appliances test
    person.take_appliance(&format!("Bandage"), BodyPart::LeftShoulder);

    match key_result {
        Ok(key) => person.health.injuries.borrow().get(&key).unwrap().stop_blood_loss(),
        _ => { }
    }
    //person.remove_appliance(&format!("Bandage"), BodyParts::LeftShoulder);
}