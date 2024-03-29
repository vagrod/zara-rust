use error::*;
use utils::{GameTime, EnvironmentC};
use utils::event::{Event, Listener, Dispatcher, Dispatchable};
use player::{PlayerStatus};
use inventory::items::{ConsumableC, ApplianceC};
use body::BodyPart;

use std::sync::Arc;
use std::cell::{Cell, RefCell};
use std::time::Duration;

mod update;
mod status_methods;

pub mod state;
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
/// 
/// # Links
/// See [this wiki article](https://github.com/vagrod/zara-rust/wiki/Getting-Started) for more info.
pub struct ZaraController<E: Listener + 'static> {
    /// Environment node.
    ///
    /// Use this to control weather and game time.
    pub environment: Arc<world::EnvironmentData>,
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
    /// Use this to sleep, see clothes and body appliances and look at wetness and warmth levels.
    pub body: Arc<body::Body>,
    /// Player status runtime data
    ///
    /// Use this to tell Zara state of a player (is he running, walking, swimming etc.)
    pub player_state: Arc<PlayerStatus>,

    // Private fields
    /// How many seconds passed since last `update` call
    update_counter: Cell<f32>,
    /// How many seconds passed since last queue check
    queue_counter: Cell<f32>,
    /// Game time snapshot at the time of the last `update` call
    last_update_game_time: Cell<Duration>,
    /// Game time of the last update frame
    last_frame_game_time: Cell<Duration>,
    /// Is controller paused
    is_paused: Cell<bool>,
    /// Events dispatcher
    dispatcher: Arc<RefCell<Dispatcher<E>>>,
    // Need this reference here to keep listener in memory
    // or else notifications won't dispatch
    #[allow(dead_code)]
    listener: Arc<RefCell<E>>
}

impl<E: Listener + 'static> ZaraController<E> {
    /// Creates new `ZaraController` without pre-defined environment. You can change environment
    /// parameters later at any time by accessing the `environment` field.
    ///
    /// To create `ZaraController` with pre-defined environment, use [`with_environment`] method.
    ///
    /// [`with_environment`]: #method.with_environment
    ///
    /// # Parameters
    /// - `listener`: [`Listener`](crate::utils::event::Listener) instance whose `notify` will be
    ///     called when Zara event occurs
    ///
    /// # Examples
    /// ```
    /// use zara;
    ///
    /// let person = zara::ZaraController::new(listener);
    /// ```
    pub fn new(listener : E) -> Self { ZaraController::init(listener, EnvironmentC::default()) }

    /// Creates a new `ZaraController` with pre-defined environment.
    ///
    /// To create `ZaraController` without pre-defined environment, use [`new`] method.
    ///
    /// [`new`]: #method.new
    ///
    /// # Parameters
    /// - `listener`: [`Listener`](crate::utils::event::Listener) instance whose `notify` will be
    ///     called when Zara event occurs
    /// - `env`: [`EnvironmentC`](crate::utils::EnvironmentC) object that describes initial state of the environment
    ///
    /// # Examples
    /// ```
    /// use zara;
    ///
    /// let person = zara::ZaraController::with_environment(listener, env);
    /// ```
    pub fn with_environment(listener : E, env: EnvironmentC) -> Self { ZaraController::init(listener, env) }

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
            queue_counter: Cell::new(0.),
            last_update_game_time: Cell::new(Duration::new(0,0)),
            last_frame_game_time: Cell::new(Duration::new(0,0)),
            player_state: Arc::new(PlayerStatus::empty()),
            is_paused: Cell::new(false),

            dispatcher: Arc::new(RefCell::new(dispatcher)),
            listener: listener_rc
        }
    }

    /// Consumes the item. Item which name is passed must have the
    /// [`ConsumableDescription`](crate::inventory::items::ConsumableDescription) option present, or
    /// `Err` will be returned
    ///
    /// # Parameters
    /// - `item_name`: unique name of the item that is being consumed
    ///
    /// # Returns
    /// Ok on success
    ///
    /// # Examples
    /// ```
    /// person.consume(item_name);
    /// ```
    ///
    /// # Links
    /// See [this wiki article](https://github.com/vagrod/zara-rust/wiki/How-to-eat-or-drink) for more info.
    ///
    /// ## Notes
    /// Borrows `inventory.items` collection
    pub fn consume(&self, item_name: &String) -> Result<(), ItemConsumeErr> {
        if !self.health.is_alive() { return Err(ItemConsumeErr::CharacterIsDead); }
        if self.is_paused() { return Err(ItemConsumeErr::InstancePaused); }

        let mut consumable = ConsumableC::new();
        {
            let consumed_count = 1_usize;
            let items_count: usize;
            let inv_items = self.inventory.items.borrow();

            let item = match inv_items.get(item_name) {
                Some(o) => o,
                None => return Err(ItemConsumeErr::ItemNotFound)
            };

            items_count = item.get_count();

            if !item.get_is_infinite() && (items_count as i32) - (consumed_count as i32) < 0 {
                return Err(ItemConsumeErr::InsufficientResources);
            }

            let c = match item.consumable() {
                Some(c) => c,
                None => return Err(ItemConsumeErr::ItemIsNotConsumable)
            };

            consumable.name = item.get_name();
            consumable.is_water = c.is_water();
            consumable.is_food = c.is_food();
            consumable.food_gain = c.food_gain_per_dose();
            consumable.water_gain = c.water_gain_per_dose();
            consumable.consumed_count = consumed_count;

            if let Some(s) = c.spoiling() {
                consumable.fresh_poisoning_chance = s.fresh_poisoning_chance();
                consumable.spoiled_poisoning_chance = s.spoil_poisoning_chance();
                consumable.spoil_time = Some(s.spoil_time());
            }

            let game_time = GameTime::from_duration(self.last_update_game_time.get()).to_contract();

            // Notify health controller about the event
            self.health.on_consumed(&game_time, &consumable, &*inv_items);
        }

        // Change items count
        self.inventory.use_item(item_name, consumable.consumed_count)
            .or_else(|e| Err(ItemConsumeErr::CouldNotUseItem(e)))?;

        // Send the event
        self.dispatcher.borrow_mut().dispatch(Event::ItemConsumed(consumable));

        Ok(())
    }

    /// Takes an appliance (like bandage or injection). Item which name is passed must have the
    /// [`ApplianceDescription`](crate::inventory::items::ApplianceDescription) option present, or
    /// `Err` will be returned
    ///
    /// # Parameters
    /// - `item_name`: unique name of the item that is being applied
    /// - `body_part`: part of the body where this item needs to be applied to
    ///
    /// # Returns
    /// Ok on success
    ///
    /// # Examples
    /// ```
    /// person.take_appliance(item_name, body_part);
    /// ```
    ///
    /// # Links
    /// See [this wiki article](https://github.com/vagrod/zara-rust/wiki/Appliances) for more info.
    ///
    /// ## Notes
    /// Borrows `inventory.items` collection, can borrow `body.appliances` collection
    pub fn take_appliance(&self, item_name: &String, body_part: BodyPart) -> Result<(), ApplianceTakeErr> {
        if !self.health.is_alive() { return Err(ApplianceTakeErr::CharacterIsDead); }
        if self.is_paused() { return Err(ApplianceTakeErr::InstancePaused); }
        if body_part == BodyPart::Unknown { return Err(ApplianceTakeErr::UnknownBodyPart); }

        let mut appliance = ApplianceC::new();
        {
            let taken_count = 1_usize;
            let items_count: usize;
            let inv_items = self.inventory.items.borrow();

            let item = match inv_items.get(item_name) {
                Some(o) => o,
                None => return Err(ApplianceTakeErr::ItemNotFound)
            };

            items_count = item.get_count();

            if !item.get_is_infinite() && (items_count as i32) - (taken_count as i32) < 0 {
                return Err(ApplianceTakeErr::InsufficientResources);
            }

            let a = match item.appliance() {
                Some(a) => a,
                None => return Err(ApplianceTakeErr::ItemIsNotAppliance)
            };

            appliance.name = item.get_name();
            appliance.is_body_appliance = a.is_body_appliance();
            appliance.is_injection = a.is_injection();
            appliance.taken_count = taken_count;

            if appliance.is_body_appliance && self.body.is_applied(item_name, body_part) {
                return Err(ApplianceTakeErr::AlreadyApplied);
            }

            let game_time = GameTime::from_duration(self.last_update_game_time.get()).to_contract();

            // Notify health controller about the event
            self.health.on_appliance_taken(&game_time, &appliance, body_part, &*inv_items);
        }

        // Change items count
        self.inventory.use_item(item_name, appliance.taken_count)
            .or_else(|e| Err(ApplianceTakeErr::CouldNotUseItem(e)))?;

        if appliance.is_body_appliance {
            // Notify body controller
            self.body.on_body_appliance_put_on(item_name, body_part);
        }

        // Send the event
        self.dispatcher.borrow_mut().dispatch(Event::ApplianceTaken(appliance, body_part));

        Ok(())
    }

    /// Removes body appliance. Item is **not** added back to the inventory.
    ///
    /// # Parameters
    /// - `item_name`: inventory kind of appliance to remove
    /// - `body_part`: from which body part
    ///
    /// # Examples
    /// ```
    /// person.remove_appliance(item_name, body_part);
    /// ```
    ///
    /// # Links
    /// See [this wiki article](https://github.com/vagrod/zara-rust/wiki/Appliances) for more info.
    ///
    /// ## Notes
    /// Borrows `body.appliances` collection
    pub fn remove_appliance(&self, item_name: &String, body_part: BodyPart) -> Result<(), ApplianceRemoveErr> {
        if !self.health.is_alive() { return Err(ApplianceRemoveErr::CharacterIsDead); }
        if self.is_paused() { return Err(ApplianceRemoveErr::InstancePaused); }

        if !self.body.remove_appliance(item_name, body_part) {
            return Err(ApplianceRemoveErr::ApplianceNotFound);
        }

        Ok(())
    }

    /// Sets controller alive state to `false`
    ///
    /// # Examples
    /// ```
    /// person.declare_dead();
    /// ```
    ///
    /// # Links
    /// See [this wiki article](https://github.com/vagrod/zara-rust/wiki/Declare-dead) for more info.
    pub fn declare_dead(&self) -> Result<(), DeclareDeadErr> {
        if self.is_paused() { return Err(DeclareDeadErr::InstancePaused); }
        self.health.declare_dead();

        // Send the event
        self.dispatcher.borrow_mut().dispatch(Event::DeclaredDead);

        Ok(())
    }

    /// Pause this instance (all `update` calls will be ignored)
    ///
    /// # Examples
    /// ```
    /// person.pause();
    /// ```
    ///
    /// # Links
    /// See [this wiki article](https://github.com/vagrod/zara-rust/wiki/Pausing-Zara) for more info.
    pub fn pause(&self) { self.is_paused.set(true); }

    /// Resume this instance (all `update` calls will be working again)
    ///
    /// # Examples
    /// ```
    /// person.resume();
    /// ```
    ///
    /// # Links
    /// See [this wiki article](https://github.com/vagrod/zara-rust/wiki/Pausing-Zara) for more info.
    pub fn resume(&self) { self.is_paused.set(false); }

    /// Adds given item to the `body.clothes` collection and recalculates inventory weight.
    ///
    /// # Parameters
    /// - `item_name`: unique inventory item name. Item must have `clothes` option present.
    ///
    /// # Returns
    /// Ok on success
    ///
    /// # Examples
    /// ```
    /// person.put_on_clothes(jacket_name);
    /// ```
    ///
    /// # Links
    /// See [this wiki article](https://github.com/vagrod/zara-rust/wiki/Clothes) for more info.
    ///
    /// ## Notes
    /// This method borrows `body.clothes` collection.
    pub fn put_on_clothes(&self, item_name: &String) -> Result<(), ClothesOnActionErr> {
        if !self.health.is_alive() { return Err(ClothesOnActionErr::CharacterIsDead); }
        if self.is_paused() { return Err(ClothesOnActionErr::InstancePaused); }

        match self.inventory.items.borrow().get(item_name) {
            Some(item) => {
                if item.get_count() <= 0 {
                    return Err(ClothesOnActionErr::InsufficientResources)
                }
                match item.clothes() {
                    Some(c) => {
                        match self.body.request_clothes_on(item_name, c) {
                            Err(RequestClothesOnErr::AlreadyHaveThisItemOn) => {
                                Err(ClothesOnActionErr::AlreadyHaveThisItemOn)
                            },
                            _ => {
                                self.inventory.update_clothes_cache(self.body.clothes.borrow().clone());
                                Ok(())
                            }
                        }
                    },
                    None => Err(ClothesOnActionErr::IsNotClothesType)
                }
            },
            None => Err(ClothesOnActionErr::ItemNotFound)
        }
    }

    /// Removes given item from the `body.clothes` collection and recalculates inventory weight.
    ///
    /// # Parameters
    /// - `item_name`: unique name of the inventory item that was put on earlier.
    ///
    /// # Returns
    /// Ok on success
    ///
    /// # Examples
    /// ```
    /// person.take_off_clothes(jacket_name);
    /// ```
    ///
    /// # Links
    /// See [this wiki article](https://github.com/vagrod/zara-rust/wiki/Clothes) for more info.
    ///
    /// ## Notes
    /// This method borrows `body.clothes` collection.
    pub fn take_off_clothes(&self, item_name: &String) -> Result<(), ClothesOffActionErr> {
        if !self.health.is_alive() { return Err(ClothesOffActionErr::CharacterIsDead); }
        if self.is_paused() { return Err(ClothesOffActionErr::InstancePaused); }

        match self.inventory.items.borrow().get(item_name) {
            Some(item) => {
                if item.get_count() <= 0 {
                    return Err(ClothesOffActionErr::InsufficientResources)
                }
                if item.clothes().is_none() {
                    return Err(ClothesOffActionErr::IsNotClothesType)
                }
            },
            None => return Err(ClothesOffActionErr::ItemNotFound)
        };

        match self.body.request_clothes_off(item_name) {
            Err(RequestClothesOffErr::ItemIsNotOn) => {
                Err(ClothesOffActionErr::ItemIsNotOn)
            },
            _ => {
                self.inventory.update_clothes_cache(self.body.clothes.borrow().clone());
                Ok(())
            }
        }
    }
}