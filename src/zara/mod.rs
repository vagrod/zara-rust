use utils::{GameTime, GameTimeC, EnvironmentC, FrameC, ConsumableC, PlayerStatusC,
            HealthC, ActiveDiseaseC, FrameSummaryC};
use utils::event::{Event, Listener, Dispatcher, Dispatchable};
use player::{PlayerStatus};

use std::sync::Arc;
use std::cell::{Cell, RefCell};
use std::time::Duration;

pub mod world;
pub mod utils;
pub mod health;
pub mod inventory;
pub mod body;
pub mod player;

/// How frequently should Zara update all its controllers,
/// recalculate values and check monitors (real seconds)
/// when player is awake
const UPDATE_INTERVAL: f32 = 1.;
/// How frequently should Zara update all its controllers,
/// recalculate values and check monitors (real seconds)
/// when player is sleeping
const SLEEPING_UPDATE_INTERVAL: f32 = UPDATE_INTERVAL / 5.;

/// Zara survival framework controller.
///
/// To set up a new `ZaraController` instance, use [`new`] or [`with_environment`] methods.
///
/// [`new`]: #method.new
/// [`with_environment`]: #method.with_environment
pub struct ZaraController<E: Listener + 'static> {
    /// Environment node.
    ///
    /// Use this to control weather and game time.
    pub environment : Arc<world::EnvironmentData>,
    /// Health node.
    ///
    /// Use this to check and control health.
    pub health: Arc<health::Health>,
    /// Inventory node.
    ///
    /// Use this to control inventory.
    pub inventory: Arc<inventory::Inventory>,
    /// Body node.
    ///
    /// Use this to sleep, control clothes and see wetness and warmth levels.
    pub body: Arc<body::Body>,
    /// Player status runtime data
    ///
    /// Use this to tell Zara state of a player (is he running, walking, swimming etc.)
    pub player_state: Arc<PlayerStatus>,

    // Private fields
    /// How many seconds passed since last `update` call
    update_counter: Cell<f32>,
    /// Game time snapshot at the time of the last `update` call
    last_update_game_time: Cell<Duration>,
    /// Game time of the last update frame
    last_frame_game_time: Cell<Duration>,
    /// Events dispatcher
    dispatcher: Arc<RefCell<Dispatcher<E>>>,
    // Need this reference here to keep listener in memory
    // or else notifications won't dispatch
    #[allow(dead_code)]
    listener: Arc<RefCell<E>>
}

impl<E: Listener + 'static> ZaraController<E> {

    /// Creates new `ZaraController` without pre-defined environment.
    /// To set up environment right away, use [`with_environment`] method.
    ///
    /// [`with_environment`]: #method.with_environment
    ///
    /// # Parameters
    /// - `listener`: [`Listener`](crate::utils::event::Listener) instance whose `notify` will be
    ///     called when Zara event occurs
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use zara;
    ///
    /// let zara = zara::ZaraController::new(listener);
    /// ```
    pub fn new(listener : E) -> Self {
        ZaraController::init(listener, EnvironmentC::default())
    }

    /// Creates a new `ZaraController` with pre-defined environment.
    /// To create `ZaraController` with empty environment, use [`new`] method.
    ///
    /// [`new`]: #method.new
    ///
    /// # Parameters
    /// - `listener`: [`Listener`](crate::utils::event::Listener) instance whose `notify` will be
    ///     called when Zara event occurs
    /// - `env_desc`: [`EnvironmentC`](crate::utils::EnvironmentC) object that describes initial state of the environment
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use zara;
    ///
    /// let zara = zara::ZaraController::with_environment(listener, env_desc);
    /// ```
    pub fn with_environment(listener : E, env: EnvironmentC) -> Self {
        ZaraController::init(listener, env)
    }

    /// Private initialization function
    fn init(listener : E, env: EnvironmentC) -> Self {
        // Register external events listener
        let mut dispatcher: Dispatcher<E> = Dispatcher::<E>::new();
        let listener_rc = Arc::new(RefCell::new(listener));

        dispatcher.register_listener(listener_rc.clone());

        ZaraController {
            environment: Arc::new(world::EnvironmentData::from_description(env)),
            health: Arc::new(health::Health::new()),
            inventory: Arc::new(inventory::Inventory::new()),
            body: Arc::new(body::Body::new()),

            update_counter: Cell::new(0.),
            last_update_game_time: Cell::new(Duration::new(0,0)),
            last_frame_game_time: Cell::new(Duration::new(0,0)),
            player_state: Arc::new(PlayerStatus::empty()),

            dispatcher: Arc::new(RefCell::new(dispatcher)),
            listener: listener_rc
        }
    }

    /// Progresses Zara controller state.
    ///
    /// This method should be called every frame.
    ///
    /// # Parameters
    /// - `frame_time`: time, `in seconds`, since last `update` call.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// zara_controller.update(time_delta);
    /// ```
    pub fn update(&self, frame_time: f32) {
        let elapsed = self.update_counter.get() + frame_time;
        let mut ceiling = UPDATE_INTERVAL;
        let game_time_duration = self.environment.game_time.duration.get();

        // When sleeping, our checks are more frequent
        if self.body.is_sleeping.get() {
            ceiling = SLEEPING_UPDATE_INTERVAL;

            // When sleeping, we need to check sleeping state every frame, because
            // otherwise wake up game time will be way off
            self.body.sleep_check(
                &mut self.dispatcher.borrow_mut(),
                &game_time_duration,
                (game_time_duration - self.last_frame_game_time.get()).as_secs_f32()
            );
        }

        if elapsed >= ceiling {
            // Retrieve the summary for sub-controllers
            let summary = &self.get_summary();

            // Form the frame data structure
            let mut frame_data = &mut FrameC {
                events: &mut self.dispatcher.borrow_mut(),
                data: summary
            };

            // Update all sub-controllers
            self.health.update(&mut frame_data);
            self.inventory.update(&mut frame_data);
            self.body.update(&mut frame_data);

            // Reset the counter and set last update game time
            self.last_update_game_time.set(game_time_duration);
            self.update_counter.set(0.);
        } else {
            self.update_counter.set(elapsed);
        }

        // Set last frame game time
        self.last_frame_game_time.set(Duration::from(game_time_duration));
    }

    /// Consumes the item. Item which name is passed must have the
    /// [`ConsumableBehavior`](crate::inventory::ConsumableBehavior) option present, or
    /// `false` will be returned
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
        let mut consumable = ConsumableC::new();
        let b = self.inventory.items.borrow();

        if !b.contains_key(item_name) {
            return false;
        }

        let item = b.get(item_name).unwrap();

        items_count = item.get_count();

        if items_count - 1 <= 0 { // 1 so far
            return false
        }

        if !item.consumable().is_some() {
            return false;
        }

        let c = item.consumable().unwrap();

        consumable.name = item.get_name();
        consumable.is_water = c.is_water();
        consumable.is_food = c.is_food();
        consumable.consumed_count = 1; // so far

        let new_count = items_count - 1;
        let game_time = GameTime::from_duration(self.last_update_game_time.get()).to_contract();

        // Notify health controller about the event
        self.health.on_item_consumed(&game_time, &consumable);

        // Change items count
        self.inventory.change_item_count(item_name, new_count);

        // Send the event
        self.dispatcher.borrow_mut().dispatch(Event::ItemConsumed { item: consumable });

        return true;
    }

    /// Gets all the info needed for all the controllers to process one frame
    ///
    /// # Notes
    /// This method borrows the `diseases` collection, `body.last_sleep_time` field
    fn get_summary(&self) -> utils::FrameSummaryC {
        let game_time_duration = self.environment.game_time.duration.get();
        let time_delta = game_time_duration - self.last_update_game_time.get();
        let mut active_diseases: Vec<ActiveDiseaseC> = Vec::new();
        let current_secs = game_time_duration.as_secs_f64();

        // Collect active diseases data
        for (_key, active) in self.health.diseases.borrow().iter() {
            active_diseases.push(ActiveDiseaseC {
                name: active.disease.get_name(),
                is_active: current_secs >= active.activation_time.to_duration().as_secs_f64(),
                scheduled_time: active.activation_time.copy()
            });
        };

        // Determine last sleep time
        let mut last_slept: GameTimeC = GameTimeC::empty();
        {
            let borrowed_time= self.body.last_sleep_time.borrow();

            if borrowed_time.is_some() {
                last_slept = borrowed_time.as_ref().unwrap().copy();
            }
        }

        FrameSummaryC {
            game_time : GameTimeC {
                day: self.environment.game_time.day.get(),
                hour: self.environment.game_time.hour.get(),
                minute: self.environment.game_time.minute.get(),
                second: self.environment.game_time.second.get()
            },
            player: PlayerStatusC {
                is_walking: self.player_state.is_walking.get(),
                is_running: self.player_state.is_running.get(),
                is_swimming: self.player_state.is_swimming.get(),
                is_underwater: self.player_state.is_underwater.get(),
                is_sleeping: self.body.is_sleeping.get(),
                last_slept_duration: self.body.last_sleep_duration.get(),
                last_slept
            },
            environment: EnvironmentC {
                wind_speed: self.environment.wind_speed.get()
            },
            health: HealthC {
                body_temperature: self.health.body_temperature.get(),
                blood_level: self.health.blood_level.get(),
                heart_rate: self.health.heart_rate.get(),
                water_level: self.health.water_level.get(),
                food_level: self.health.food_level.get(),
                top_pressure: self.health.top_pressure.get(),
                bottom_pressure: self.health.bottom_pressure.get(),
                stamina_level: self.health.stamina_level.get(),
                fatigue_level: self.health.fatigue_level.get(),

                diseases: active_diseases
            },
            game_time_delta: time_delta.as_secs_f32()
        }
    }

}
