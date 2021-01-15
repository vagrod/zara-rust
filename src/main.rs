use std::thread;
use std::time::{Duration, Instant};
use std::thread::sleep;
use std::cell::Cell;

use zara::inv::{InventoryItem, ConsumableBehavior, SpoilingBehavior};
use zara::utils::event::{Listener, Event};
use zara::health::{Health};
use zara::health::disease::{DiseaseMonitor};
use zara::utils::{SummaryC, ConsumableC};

// This will spawn a new thread for the "game loop"
fn main() {
    let game_loop = thread::spawn(|| {
        let two_millis= Duration::new(0, 2000000); // 2ms

        let mut frame_time= 0_f32;
        let mut now = Instant::now();
        let person = zara::ZaraController::with_environment(zara::utils::EnvironmentC::new(5.4));

        { // Testing basic inventory
            person.inventory.add_item(Box::new(TestItem::new()));

            let b = person.inventory.items.borrow();
            let item = b.get("Meat").unwrap();

            println!("Has consumable part? {}", item.consumable().is_some());
            println!("Food gain {}", item.consumable().unwrap().food_gain_per_dose());
            println!("Has spoil part? {}", item.consumable().unwrap().spoiling().is_some());

            println!("Total weight {}", person.inventory.weight.get());
        }

        println!("Game Loop started!");

        // Testing disease monitors
        let mon = FluMonitor;
        person.register_disease_monitor(Box::new(mon));

        // Testing items consuming
        person.consume(&String::from("Meat"));

        // Testing player status update
        person.player_state.is_walking.set(true);

        // Total weight must change after consuming
        println!("Total weight {}", person.inventory.weight.get());

        loop {
            now = Instant::now();

            // Cap the "framerate"
            sleep(two_millis);

            frame_time = now.elapsed().as_secs_f32();

            // Game time is 10x the real one
            person.environment.game_time.add_seconds(frame_time * 10.);

            let events_listener = ZaraEventsListener;

            // Update Zara state
            person.update::<ZaraEventsListener>(frame_time, events_listener);
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
        println!("Notify called with {:?}", event);
        if let Event::Dehydration = event {
            println!("Dehydration");
        }
    }
}

struct FluMonitor;
impl DiseaseMonitor for FluMonitor {
    fn check(&self, health: &Health, frame_data: &SummaryC) {
        println!("Flu monitor check: {}", frame_data.game_time_delta);

        health.spawn_disease();
    }

    fn on_consumed(&self, health: &Health, item: &ConsumableC) {
        println!("Flu monitor on consumed: {}", item.name);
    }
}