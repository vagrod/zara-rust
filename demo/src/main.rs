extern crate termion;

use std::thread;
use std::time::{Duration, Instant};
use std::thread::sleep;
use std::cell::Cell;

use zara::utils::event::{Listener, Event};
use zara::utils::{FrameSummaryC, ConsumableC, GameTimeC};
use zara::health::{Health};
use zara::health::disease::{DiseaseMonitor, Disease};
use zara::health::side::builtin::{RunningSideEffects, DynamicVitalsSideEffect, FatigueSideEffects};
use zara::inventory::items::{InventoryItem, ConsumableBehavior, SpoilingBehavior};
use zara::inventory::crafting;

use termion::{color, style};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

mod inventory;

// This will spawn a new thread for the "game loop"
fn main() {
    let game_loop = thread::spawn(|| {
        let two_millis= Duration::new(0, 2_000_000); // 2ms
        let mut frame_time= 0_f32;
        let mut now = Instant::now();
        let mut console_update_counter = 0.;

        let mut stdout = stdout().into_raw_mode().unwrap();

        // Instantiate our events listener
        let events_listener = ZaraEventsListener;

        // Describe environment conditions
        let environment = zara::utils::EnvironmentC::new(5.4);

        // Initialize Zara instance
        let person =
            zara::ZaraController::with_environment (
                events_listener, environment
            );

        add_side_effects(&person);
        populate_inventory(&person);

        write!(stdout, "{}", termion::cursor::Hide).unwrap();

        loop {
            now = Instant::now();

            // Cap the "framerate"
            sleep(two_millis);

            frame_time = now.elapsed().as_secs_f32();

            if person.body.is_sleeping.get() {
                // Progress time faster during the sleep
                person.environment.game_time.add_seconds(frame_time * 1800.); // 30 game minutes per real second
            } else {
                // Game time is 10x the real one
                person.environment.game_time.add_seconds(frame_time * 10.);
            }

            // Update Zara state
            person.update(frame_time);

            console_update_counter += frame_time;

            if console_update_counter >= 1. {
                console_update_counter = 0.;

                flush_data(&mut stdout, &person);
            }
        }
    });

    game_loop.join().unwrap();
}

fn populate_inventory(person: &zara::ZaraController<ZaraEventsListener>) {
    let meat = inventory::Meat{ count: Cell::new(2) };
    let knife = inventory::Knife{ count: Cell::new(1) };
    let rope = inventory::Rope{ count: Cell::new(5) };

    person.inventory.add_item(Box::new(meat));
    person.inventory.add_item(Box::new(knife));
    person.inventory.add_item(Box::new(rope));
}

fn add_side_effects(person: &zara::ZaraController<ZaraEventsListener>) {
    let vitals_effects = zara::health::side::builtin::DynamicVitalsSideEffect::new();
    person.health.register_side_effect_monitor(Box::new(vitals_effects));

    let running_effects = zara::health::side::builtin::RunningSideEffects::new();
    person.health.register_side_effect_monitor(Box::new(running_effects));

    let vitals_effects = zara::health::side::builtin::DynamicVitalsSideEffect::new();
    person.health.register_side_effect_monitor(Box::new(vitals_effects));

    let fatigue_effects = zara::health::side::builtin::FatigueSideEffects::new();
    person.health.register_side_effect_monitor(Box::new(fatigue_effects));

    let food_drain_effect =  zara::health::side::builtin::FoodDrainOverTimeSideEffect::new(0.01);
    person.health.register_side_effect_monitor(Box::new(food_drain_effect));

    let water_drain_effect =  zara::health::side::builtin::WaterDrainOverTimeSideEffect::new(0.03);
    person.health.register_side_effect_monitor(Box::new(water_drain_effect));
}

fn flush_data<W: Write>(stdout: &mut W, person: &zara::ZaraController<ZaraEventsListener>) {
    write!(stdout,
           "{}{}",
           termion::cursor::Goto(1, 1),
           termion::clear::All)
        .unwrap();

    writeln!(stdout, "{}{}Game time: {}d {}h {}m {:.0}s", termion::cursor::Goto(1, 1), color::Fg(color::Cyan),
             person.environment.game_time.day.get(),
             person.environment.game_time.hour.get(),
             person.environment.game_time.minute.get(),
             person.environment.game_time.second.get()).unwrap();

    writeln!(stdout, "{}{}Body temp: {:.2} °C", termion::cursor::Goto(1, 2), color::Fg(color::Green), person.health.body_temperature.get()).unwrap();
    writeln!(stdout, "{}Heart rate: {:.0} bpm", termion::cursor::Goto(1, 3), person.health.heart_rate.get()).unwrap();
    writeln!(stdout, "{}Blood pressure: {:.0}/{:.0} mmHg", termion::cursor::Goto(1, 4), person.health.top_pressure.get(), person.health.bottom_pressure.get()).unwrap();
    writeln!(stdout, "{}Water: {:.0}%", termion::cursor::Goto(1, 5), person.health.water_level.get()).unwrap();
    writeln!(stdout, "{}Food: {:.0}%", termion::cursor::Goto(1, 6), person.health.food_level.get()).unwrap();
    writeln!(stdout, "{}Stamina: {:.0}%", termion::cursor::Goto(1, 7), person.health.stamina_level.get()).unwrap();
    writeln!(stdout, "{}Fatigue: {:.2}%", termion::cursor::Goto(1, 8), person.health.fatigue_level.get()).unwrap();

    writeln!(stdout, "{}{}Inventory:", color::Fg(color::Blue), termion::cursor::Goto(50, 1));

    let mut y = 2;
    for (name, item) in person.inventory.items.borrow().iter() {
        writeln!(stdout, "{}   {} - {}, weight {:.0}g", termion::cursor::Goto(50, y), name, item.get_count(), item.get_total_weight());
        y+=1;
    }

    writeln!(stdout, "{}   ____________________________", termion::cursor::Goto(50, y));
    writeln!(stdout, "{}   Total weight: {}g", termion::cursor::Goto(50, y+1), person.inventory.get_weight());
}

struct ZaraEventsListener;
impl Listener for ZaraEventsListener {
    fn notify(&mut self, event: &Event) {
        match event {
            Event::ItemConsumed {item} => {
                println!("Item {} consumed", item.name);
            },
            Event::WokeUp => {
                println!("Woke up!");
            },
            _ => println!("Other event")
        }
    }
}