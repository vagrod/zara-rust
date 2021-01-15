use utils::{SummaryC, GameTimeC, EnvironmentC, FrameC, ConsumableC};
use utils::event::{Listener, Dispatcher, Dispatchable};

use std::sync::Arc;
use std::cell::{Cell, RefCell};
use std::time::Duration;

pub mod env;
pub mod utils;
pub mod health;
pub mod inv;
pub mod body;

/// How frequently should Zara update all its controllers,
/// recalculate values and check monitors (real seconds)
const UPDATE_INTERVAL: f32 = 1.;

/// Zara survival framework controller.
///
/// To set up a new `ZaraController` instance, use [`new`] or [`with_environment`] methods.
///
/// [`new`]: #method.new
/// [`with_environment`]: #method.with_environment
pub struct ZaraController {
    /// Environment node.
    ///
    /// Use this to control weather and game time.
    pub environment : Arc<env::EnvironmentData>,
    /// Health node.
    ///
    /// Use this to check and control health.
    pub health: Arc<health::Health>,
    /// Inventory node.
    ///
    /// Use this to control inventory.
    pub inventory: Arc<inv::Inventory>,
    /// Body node.
    ///
    /// Use this to sleep, control clothes and see wetness and warmth levels.
    pub body: Arc<body::Body>,

    /// How many seconds passed since last `update` call
    update_counter: Cell<f32>,
    /// Game time snapshot at the time of the last `update` call
    last_update_game_time: Cell<Duration>,
}

impl ZaraController {
    /// Creates new `ZaraController` without pre-defined environment.
    /// To set up environment right away, use [`with_environment`] method.
    ///
    /// [`with_environment`]: #method.with_environment
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use zara;
    ///
    /// let zara = zara::ZaraController::new();
    /// ```
    pub fn new() -> Self {
        ZaraController::with_environment(EnvironmentC::empty())
    }

    /// Creates a new `ZaraController` with pre-defined environment.
    /// To create `ZaraController` with empty environment, use [`new`] method.
    ///
    /// [`new`]: #method.new
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use zara;
    ///
    /// let zara = zara::ZaraController::with_environment(env_desc);
    /// ```
    pub fn with_environment(env: EnvironmentC) -> Self {
        ZaraController {
            environment: Arc::new(env::EnvironmentData::from_description(env)),
            health: Arc::new(health::Health::new()),
            inventory: Arc::new(inv::Inventory::new()),
            body: Arc::new(body::Body::new()),

            update_counter: Cell::new(0.),
            last_update_game_time: Cell::new(Duration::new(0,0)),
        }
    }

    /// Progresses Zara controller state.
    ///
    /// This method should be called every frame.
    ///
    /// # Parameters
    ///
    /// - `E`: trait type that implements [`Listener`](crate::zara:evt::Listener) trait
    /// - `frame_time`: time, `in seconds`, since last `update` call.
    /// - `listener`: [`Listener`](crate::zara:evt::Listener) instance whose methods will be called
    ///     as events
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// zara_controller.update::<MyEventListener>(time_delta, listener);
    /// ```
    pub fn update<E: Listener + 'static>(&self, frame_time: f32, listener : E)
    {
        let elapsed = self.update_counter.get() + frame_time;

        if elapsed >= UPDATE_INTERVAL {
            // Retrieve the summary for sub-controllers
            let summary = &self.get_summary();

            // Register external events listener
            let dispatcher: &mut Dispatcher<E> = &mut Dispatcher::<E>::new();
            let listener_rc = Arc::new(RefCell::new(listener));

            dispatcher.register_listener(listener_rc.clone());

            // Form the frame data structure
            let mut frame_data = &mut FrameC {
                events: dispatcher,
                data: summary
            };

            // Update all sub-controllers
            self.health.update(&mut frame_data);
            self.inventory.update(&mut frame_data);
            self.body.update(&mut frame_data);

            // Reset the counter and last update time
            self.last_update_game_time.set(self.environment.game_time.duration.get());
            self.update_counter.set(0.);
        } else {
            self.update_counter.set(elapsed);
        }
    }

    /// Consumes the item. Item which name is passed must implement the
    /// [`ConsumableBehavior`](crate::zara:inv::ConsumableBehavior) trait, or `false` will be
    /// returned
    ///
    /// # Parameters
    /// - `item_name`: unique name of the item that is being consumed
    ///
    /// # Returns
    /// `bool`: `true` on success
    ///
    /// # Notes
    /// This method borrows the `inventory.items` collection
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// zara_controller.consume(item_name);
    /// ```
    pub fn consume(&self, item_name: &String) -> bool {
        let items_count: usize;
        let mut consumable= ConsumableC::new();

        {
            // Cant borrow `items` for long
            let b = self.inventory.items.borrow();

            if !b.contains_key(item_name) {
                return false;
            }

            let item = b.get(item_name).unwrap();

            items_count = item.get_count();

            if !item.consumable().is_some() {
                return false;
            }

            let c = item.consumable().unwrap();

            consumable.name = item.get_name();
            consumable.is_water = c.is_water();
            consumable.is_food = c.is_food();
            consumable.consumed_count = 1; // so far
        }

        let new_count = items_count - 1;

        if new_count <= 0 {
            return false
        }

        // Notify health controller about the event
        self.health.on_item_consumed(&consumable);

        // Change items count
        self.inventory.change_item_count(item_name, new_count);

        return true;
    }

    /// Gets all the info needed for all the controllers to process one frame
    fn get_summary(&self) -> utils::SummaryC {
        let time_delta = self.environment.game_time.duration.get() - self.last_update_game_time.get();

        SummaryC {
            game_time : GameTimeC{
                day: self.environment.game_time.day.get(),
                hour: self.environment.game_time.hour.get(),
                minute: self.environment.game_time.minute.get(),
                second: self.environment.game_time.second.get()
            },
            game_time_delta: time_delta.as_secs_f32(),
            wind_speed: self.environment.wind_speed.get()
        }
    }
}
