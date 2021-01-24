use utils::{GameTime, EnvironmentC};
use utils::event::{Event, Listener, Dispatcher, Dispatchable};
use player::{PlayerStatus};
use error::{ItemConsumeErr};
use inventory::items::ConsumableC;

use std::sync::Arc;
use std::cell::{Cell, RefCell};
use std::time::Duration;

mod update;

pub mod world;
pub mod utils;
pub mod error;
pub mod health;
pub mod inventory;
pub mod body;
pub mod player;

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
    /// Is this character alive
    pub is_alive: Cell<bool>,

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
    pub fn new(listener : E) -> Self { ZaraController::init(listener, EnvironmentC::default()) }

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
    pub fn with_environment(listener : E, env: EnvironmentC) -> Self { ZaraController::init(listener, env) }

    /// Private initialization function
    fn init(listener : E, env: EnvironmentC) -> Self {
        // Register external events listener
        let mut dispatcher: Dispatcher<E> = Dispatcher::<E>::new();
        let listener_rc = Arc::new(RefCell::new(listener));

        dispatcher.register_listener(listener_rc.clone());

        ZaraController {
            is_alive: Cell::new(true),
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

    /// Consumes the item. Item which name is passed must have the
    /// [`ConsumableBehavior`](crate::inventory::ConsumableBehavior) option present, or
    /// `false` will be returned
    ///
    /// # Parameters
    /// - `item_name`: unique name of the item that is being consumed
    ///
    /// # Returns
    /// Ok on success
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
    pub fn consume(&self, item_name: &String) -> Result<(), ItemConsumeErr> {
        if !self.is_alive.get() { return Err(ItemConsumeErr::CharacterIsDead); }

        let items_count: usize;
        let mut consumable = ConsumableC::new();
        let b = self.inventory.items.borrow();

        let item = match b.get(item_name) {
            Some(o) => o,
            None => return Err(ItemConsumeErr::ItemNotFound)
        };

        items_count = item.get_count();

        if items_count - 1 <= 0 { // 1 so far
            return Err(ItemConsumeErr::NotEnoughResources);
        }

        let c = match item.consumable() {
            Some(c) => c,
            None => return Err(ItemConsumeErr::ItemIsNotConsumable)
        };

        consumable.name = item.get_name();
        consumable.is_water = c.is_water();
        consumable.is_food = c.is_food();
        consumable.consumed_count = 1; // so far

        let new_count = items_count - 1;
        let game_time = GameTime::from_duration(self.last_update_game_time.get()).to_contract();
        let items = self.inventory.items.borrow();

        // Notify health controller about the event
        self.health.on_consumed(&game_time, &consumable, &*items);

        // Change items count
        self.inventory.change_item_count(item_name, new_count)
            .or_else(|err| Err(ItemConsumeErr::CouldNotUpdateItemCount(err)))?;

        // Send the event
        self.dispatcher.borrow_mut().dispatch(Event::ItemConsumed(consumable));

        return Ok(());
    }

}
