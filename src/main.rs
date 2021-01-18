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

// This will spawn a new thread for the "game loop"
fn main() {
    let game_loop = thread::spawn(|| {
        let two_millis= Duration::new(0, 2000000); // 2ms
        let mut frame_time= 0_f32;
        let mut now = Instant::now();

        // Instantiate our events listener
        let events_listener = ZaraEventsListener;

        // Describe environment conditions
        let environment = zara::utils::EnvironmentC::new(5.4);

        // Initialize Zara instance
        let person =
            zara::ZaraController::with_environment (
            events_listener, environment
        );

        let o = crafting::Builder::start()
            .build_for("result item")
                .is("k item", 1)
                .plus("b item", 3)
                .and("a item", 2)
            .build();

        println!("{}", o.key);

        // Testing environment change
        person.environment.wind_speed.set(22.);

        { // Testing basic inventory
            person.inventory.add_item(Box::new(TestItem::new()));

            let b = person.inventory.items.borrow();
            let item = b.get("Meat").unwrap();

            println!("Has consumable part? {}", item.consumable().is_some());
            println!("Food gain {}", item.consumable().unwrap().food_gain_per_dose());
            println!("Has spoil part? {}", item.consumable().unwrap().spoiling().is_some());
        }

        println!("Total weight {}", person.inventory.get_weight());

        // Testing disease monitors
        let flu_monitor = FluMonitor;
        person.health.register_disease_monitor(Box::new(flu_monitor));

        // Testing side effects monitors
        let running_effects = RunningSideEffects::new();
        person.health.register_side_effect_monitor(Box::new(running_effects));

        let vitals_effects = DynamicVitalsSideEffect::new();
        person.health.register_side_effect_monitor(Box::new(vitals_effects));

        let fatigue_effects = FatigueSideEffects::new();
        person.health.register_side_effect_monitor(Box::new(fatigue_effects));

        // Testing unregister
        //person.health.unregister_side_effect_monitor(test_key);

        let mut is_consumed= false;
        let mut already_stopped_running = false;

        println!("Game Loop started!");

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

            // Just for test to fire this only once at non-zero game time
            if !is_consumed && person.environment.game_time.second.get() >= 58. && person.environment.game_time.second.get() >= 59. {
                // Testing items consuming
                person.consume(&String::from("Meat"));

                // Sleeping test
                person.body.start_sleeping(6.);

                // Testing player status update
                person.player_state.is_running.set(false);
                already_stopped_running = true;

                // Total weight must change after consuming
                println!("Total weight {}", person.inventory.get_weight());

                is_consumed = true;
            } else {
                // Testing player status update
                if !already_stopped_running {
                    person.player_state.is_running.set(true);
                }
            }

            // Update Zara state
            person.update(frame_time);
        }
    });

    game_loop.join().unwrap();
}

struct TestItem {
    count: Cell<usize>
}

impl TestItem {
    pub fn new() -> Self {
        TestItem{
            count: Cell::new(11)
        }
    }
}

impl InventoryItem for TestItem {
    fn get_count(&self) -> usize { self.count.get() }
    fn set_count(&self, new_count: usize) { self.count.set(new_count); }
    fn get_name(&self) -> String { String::from("Meat") }
    fn get_total_weight(&self) -> f32 {
        const WEIGHT_PER_UNIT: f32 = 0.4;

        self.count.get() as f32 * WEIGHT_PER_UNIT
    }
    fn consumable(&self) -> Option<&dyn ConsumableBehavior> { Some(&MyFood) }
}

struct MyFood;
impl ConsumableBehavior for MyFood {
    fn is_food(&self) -> bool { true }
    fn is_water(&self) -> bool { false }
    fn water_gain_per_dose(&self) -> f32 { 6.1 }
    fn food_gain_per_dose(&self) -> f32 { 14.2 }
    fn spoiling(&self) -> Option<&dyn SpoilingBehavior> { None }
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

struct FluMonitor;
impl DiseaseMonitor for FluMonitor {
    fn check(&self, _health: &Health, frame_data: &FrameSummaryC) {
        println!("body t {}", frame_data.health.body_temperature);
        println!("fatigue {}", frame_data.health.fatigue_level);
        println!("stamina {}", frame_data.health.stamina_level);
    }

    fn on_consumed(&self, health: &Health, game_time: &GameTimeC, item: &ConsumableC) {
        println!("Flu monitor on consumed: {}", item.name);

        // 5% chance test here
        if zara::utils::roll_dice(5) {
            health.spawn_disease(Box::new(
                FluDisease::new()),
                 game_time.add_minutes(0)
            );
        }
    }
}

struct FluDisease;
impl FluDisease {
    fn new() -> Self {
        FluDisease
    }
}
impl Disease for FluDisease {
    fn get_name(&self) -> String {
        String::from("Flu")
    }
}