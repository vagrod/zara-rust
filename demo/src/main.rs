use std::thread;
use std::io::{self, Write};
use std::time::{Duration, Instant};
use std::thread::sleep;

use crate::events::ZaraEventsListener;
use crate::ui::ui_frame;
use crate::zara_init::init_zara_instance;

use zara::body::BodyParts;
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