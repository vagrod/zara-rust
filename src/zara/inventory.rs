use super::utils::{FrameC, GameTimeC};
use super::utils::event::{Event, Listener};

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::collections::HashMap;

/// Controls player's inventory
pub struct Inventory {
    /// All inventory items
    ///
    /// # Important
    /// Do not add or remove elements by hand. Use the
    /// [`add_item`] and [`remove_item`] methods. Otherwise
    /// inventory weight will not be correctly recalculated
    ///
    /// [`add_item`]: #method.add_item
    /// [`remove_item`]: #method.remove_item
    pub items: Rc<RefCell<HashMap<String, Box<dyn InventoryItem>>>>,

    /// Weight of all inventory items (in grams)
    pub weight: Cell<f32>
}

impl Inventory {
    /// Creates new ready-to-use `Inventory`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use zara::inv;
    ///
    /// let inv = inv::Inventory::new();
    /// ```
    pub fn new() -> Self {
        Inventory{
            items: Rc::new(RefCell::new(HashMap::new())),
            weight: Cell::new(0.)
        }
    }

    /// Adds new item to the inventory and recalculates inventory weight
    ///
    /// # Parameters
    /// - `item`: any boxed object that supports `InventoryItem` trait
    ///
    /// # Notes
    /// This method borrows the `items` collection
    pub fn add_item(&self, item: Box<dyn InventoryItem>) {
        let key = item.get_name();

        self.items.borrow_mut().insert(key, item);
        self.recalculate_weight();
    }

    /// Removes item kind from the inventory and recalculates inventory weight
    ///
    /// # Parameters
    /// - `item_kind`: unique name of the item ("InventoryItem.name")
    ///
    /// # Returns
    /// `true` on success.
    /// `false` if a given kind was not found.
    ///
    /// # Notes
    /// This method borrows the `items` collection
    pub fn remove_item(&self, item_kind: &String) -> bool {
        let mut b = self.items.borrow_mut();

        if b.contains_key(item_kind) {
            b.remove(item_kind);

            self.recalculate_weight();

            return true;
        }

        return false;
    }

    /// This method is called every `UPDATE_INTERVAL` real seconds
    ///
    /// # Parameters
    /// - `frame`: summary information for this frame
    pub fn update<E: Listener + 'static>(&self, frame: &mut FrameC<E>){
        frame.events.dispatch(Event::Dehydration {

        });

        println!("From inventory update: game secs passed - {}", frame.data.game_time_delta);
    }

    /// Shorthand function to change count of a given kind
    ///
    /// # Notes
    /// This method borrows the `items` collection
    pub fn change_item_count(&self, name: &String, new_value: usize){
        let b = self.items.borrow();
        let res = b.get(name);

        if res.is_some(){
            res.unwrap().set_count(new_value);

            self.recalculate_weight();
        }
    }

    /// Recalculates the inventory weight
    ///
    /// # Notes
    /// This method borrows the `items` collection
    fn recalculate_weight(&self){
        let mut total_weight: f32;

        total_weight = 0.;

        for  (_key, item) in self.items.borrow().iter() {
            total_weight += item.get_total_weight();
        }

        self.weight.set(total_weight);
    }
}

/// Trait that must be implemented by all inventory items
pub trait InventoryItem {
    /// Returns count of items of this kind in the inventory
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let n = item.get_count();
    /// ```
    fn get_count(&self) -> usize;

    /// Sets new count for items of this kind
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// item.set_count(new_value);
    /// ```
    fn set_count(&self, new_count: usize);

    /// Gets unique name for all items of this kind
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let s = item.get_name();
    /// ```
    fn get_name(&self) -> String;

    /// Gets calculated weight of all items of this kind, in grams.
    ///
    /// Most of the time, it is `count` * `weight per item`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let n = item.get_total_weight();
    /// ```
    fn get_total_weight(&self) -> f32;

    /// Node that describes behavior of this item as a consumable
    fn consumable(&self) -> Option<&dyn ConsumableBehavior>;
}

/// Trait to describe consumable behavior of the inventory item
pub trait ConsumableBehavior {
    /// True if this item should be treated as food
    fn is_food(&self) -> bool;
    /// True if this item should be treated as water
    fn is_water(&self) -> bool;
    /// How much water points consuming of this item gives (0..100 scale)
    fn water_gain_per_dose(&self) -> f32;
    /// How much food points consuming of this item gives (0..100 scale)
    fn food_gain_per_dose(&self) -> f32;
    /// Node that describes the spoiling options of this consumable
    fn spoiling(&self) -> Option<&dyn SpoilingBehavior>;
}

/// Trait to describe the spoiling options of the consumable
pub trait SpoilingBehavior {
    /// Chance of getting a food poisoning after eating one fresh item (0..100 scale)
    fn fresh_poisoning_chance(&self) -> usize;
    /// Chance of getting a food poisoning after eating one fresh item (0..100 scale)
    fn spoil_poisoning_chance(&self) -> usize;
    /// Time that is needed for fresh item to become spoiled
    /// ([GameTimeC](crate::utils::GameTimeC) structure)
    fn spoil_time(&self) -> GameTimeC;
}