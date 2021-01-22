extern crate termion;

use std::thread;
use std::time::{Duration, Instant};
use std::thread::sleep;
use std::cell::Cell;

use zara::utils::event::{Listener, Event};
use zara::utils::{FrameSummaryC, ConsumableC, GameTimeC};
use zara::health::{Health};
use zara::health::disease::{DiseaseMonitor, Disease, StageBuilder, StageLevel};
use zara::health::side::builtin::{RunningSideEffects, DynamicVitalsSideEffect, FatigueSideEffects};
use zara::inventory::items::{InventoryItem, ConsumableBehavior, SpoilingBehavior};
use zara::inventory::crafting;

use termion::{color, style};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

mod diseases;
mod inventory;

// This will spawn a new thread for the "game loop"
fn main() {
    let game_loop = thread::spawn(|| {
        let mut is_disease_inverted = false;
        let two_millis= Duration::new(0, 2_000_000); // 2ms
        let mut frame_time= 0_f32;
        let mut now = Instant::now();
        let mut console_update_counter = 0.;

        let mut stdout = stdout().into_raw_mode().unwrap();

        // Instantiate our events listener
        let events_listener = ZaraEventsListener;

        // Describe environment conditions
        let environment = zara::utils::EnvironmentC::new(24., 2., 0.);

        // Initialize Zara instance
        let person =
            zara::ZaraController::with_environment (
                events_listener, environment
            );

        add_side_effects(&person);
        populate_inventory(&person);
        spawn_diseases(&person);

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

            // Disease "invert" test
            if person.environment.game_time.minute.get() == 20 {
                if !is_disease_inverted {
                    person.health.diseases.borrow().get("Flu").unwrap().invert(&person.environment.game_time.to_contract());
                    person.health.diseases.borrow().get("Flu").unwrap().invert_back(&person.environment.game_time.to_contract());
                    is_disease_inverted = true;
                }
            }

            // Update Zara state
            person.update(frame_time);

            // Update console data
            console_update_counter += frame_time;

            if console_update_counter >= 1. {
                console_update_counter = 0.;

                flush_data(&mut stdout, &person);
            }
        }
    });

    game_loop.join().unwrap();
}


fn spawn_diseases(person: &zara::ZaraController<ZaraEventsListener>) {
    person.health.spawn_disease(Box::new(diseases::Flu), zara::utils::GameTimeC::new(0,0,1,15.));
    //person.health.spawn_disease(Box::new(diseases::Angina), zara::utils::GameTimeC::new(0,0,2,42.));
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

fn flush_data<W: Write>(stdout: &mut W, person: &zara::ZaraController<ZaraEventsListener>) {
    // Cls
    write!(stdout, "{}{}", termion::cursor::Goto(1, 1), termion::clear::All).unwrap();

    // Show game time
    writeln!(stdout, "{}{}Game time: {}d {}h {}m {:.0}s", termion::cursor::Goto(1, 1), color::Fg(color::Cyan),
             person.environment.game_time.day.get(),
             person.environment.game_time.hour.get(),
             person.environment.game_time.minute.get(),
             person.environment.game_time.second.get()).unwrap();

    // Show vitals
    writeln!(stdout, "{}{}Vitals", termion::cursor::Goto(1, 2), color::Fg(color::Green)).unwrap();
    writeln!(stdout, "{}  Body temp: {:.2}°C", termion::cursor::Goto(1, 3), person.health.body_temperature.get()).unwrap();
    writeln!(stdout, "{}  Heart rate: {:.0} bpm", termion::cursor::Goto(1, 4), person.health.heart_rate.get()).unwrap();
    writeln!(stdout, "{}  Blood pressure: {:.0}/{:.0} mmHg", termion::cursor::Goto(1, 5), person.health.top_pressure.get(), person.health.bottom_pressure.get()).unwrap();
    writeln!(stdout, "{}  Water: {:.0}%", termion::cursor::Goto(1, 6), person.health.water_level.get()).unwrap();
    writeln!(stdout, "{}  Food: {:.0}%", termion::cursor::Goto(1, 7), person.health.food_level.get()).unwrap();
    writeln!(stdout, "{}  Stamina: {:.0}%", termion::cursor::Goto(1, 8), person.health.stamina_level.get()).unwrap();
    writeln!(stdout, "{}  Fatigue: {:.2}%", termion::cursor::Goto(1, 9), person.health.fatigue_level.get()).unwrap();

    let vitals_h = 9;

    // Show inventory
    writeln!(stdout, "{}{}Inventory", color::Fg(color::Blue), termion::cursor::Goto(50, 1));
    let mut invent_h = 2;
    for (name, item) in person.inventory.items.borrow().iter() {
        write!(stdout, "{}  {} - {}, weight {:.0}g", termion::cursor::Goto(50, invent_h), name, item.get_count(), item.get_total_weight());
        let consumable = item.consumable();
        if consumable.is_some() {
            let cons = item.consumable().unwrap();
            write!(stdout, " (consumable +{:.0}% food and +{:.0}% water", cons.food_gain_per_dose(), cons.water_gain_per_dose());
            if cons.spoiling().is_some(){
                write!(stdout, ", can spoil in ");
                let spoil = cons.spoiling().unwrap();
                let time = spoil.spoil_time();
                write!(stdout, "{}d {}h {}m {:.0}s",
                         time.day,
                         time.hour,
                         time.minute,
                         time.second).unwrap();
                write!(stdout, ")");
            } else {
                write!(stdout, ", cannot spoil)");
            }
        }
        invent_h +=1;
    }
    writeln!(stdout, "{}  _______________________", termion::cursor::Goto(50, invent_h));
    writeln!(stdout, "{}  Total weight: {}g", termion::cursor::Goto(50, invent_h +1), person.inventory.get_weight());

    invent_h+=1;

    // Show weather
    writeln!(stdout, "{}{}Weather", color::Fg(color::LightMagenta), termion::cursor::Goto(1, vitals_h+2));
    writeln!(stdout, "{}  Temp: {}°C", termion::cursor::Goto(1, vitals_h+3), person.environment.temperature.get());
    writeln!(stdout, "{}  Wind: {:.1} m/s", termion::cursor::Goto(1, vitals_h+4), person.environment.wind_speed.get());
    writeln!(stdout, "{}  Rain (0..1): {:.1}", termion::cursor::Goto(1, vitals_h+5), person.environment.rain_intensity.get());

    // Show other player stats
    writeln!(stdout, "{}{}Stats", color::Fg(color::LightYellow), termion::cursor::Goto(50, invent_h+2));
    writeln!(stdout, "{}  Is sleeping: {}", termion::cursor::Goto(50, invent_h+3), person.body.is_sleeping.get());
    let sleep_time = person.body.last_sleep_time.borrow();
    if sleep_time.is_some() {
        let time = sleep_time.as_ref().unwrap();
        writeln!(stdout, "{}  Last time slept: {}d {}h {}m {:.0}s (for {}h)", termion::cursor::Goto(50, invent_h+4),
                 time.day,
                 time.hour,
                 time.minute,
                 time.second,
                 person.body.last_sleep_duration.get()).unwrap();
    } else {
        writeln!(stdout, "{}  Last time slept: none", termion::cursor::Goto(50, invent_h+4));
    }

    let mut diseases_height = 2;

    // Show diseases
    writeln!(stdout, "{}{}Diseases", color::Fg(color::LightGreen), termion::cursor::Goto(150, 1));
    for (name, disease) in person.health.diseases.borrow().iter() {
        let is_active = disease.get_is_active(&person.environment.game_time.to_contract());
        if is_active {
            writeln!(stdout, "{}  {}: active", termion::cursor::Goto(150, diseases_height), name);
            diseases_height+=1;
            if disease.needs_treatment {
                if disease.will_end {
                    let time = disease.end_time.as_ref().unwrap();
                    writeln!(stdout, "{}    will end @{}d {}h {}m {:.0}s", termion::cursor::Goto(150, diseases_height),
                             time.day,
                             time.hour,
                             time.minute,
                             time.second
                    );
                } else {
                    writeln!(stdout, "{}    will not end", termion::cursor::Goto(150, diseases_height));
                }
            } else {
                writeln!(stdout, "{}    don't need treatment, will self-heal", termion::cursor::Goto(150, diseases_height));
            }
        } else {
            let time = &disease.activation_time;
            writeln!(stdout, "{}  {}: scheduled to activate @{}d {}h {}m {:.0}s", termion::cursor::Goto(150, diseases_height), name,
                     time.day,
                     time.hour,
                     time.minute,
                     time.second
            );
        }

        diseases_height += 1;
    }
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