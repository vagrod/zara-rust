use crate::utils::event::{Event, MessageQueue};
use crate::inventory::items::InventoryItem;
use crate::inventory::crafting::CraftingCombination;
use crate::inventory::monitors::InventoryMonitor;
use crate::error::InventoryUseErr;

use std::collections::{HashMap, BTreeMap};
use std::cell::{Cell, RefCell, RefMut};
use std::sync::Arc;
use std::rc::Rc;

mod crud;
mod update;

pub mod items;
pub mod crafting;
pub mod monitors;

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
    pub items: Arc<RefCell<HashMap<String, Box<dyn InventoryItem>>>>,

    /// Weight of all inventory items (in grams)
    weight: Cell<f32>,
    /// Registered crafting combinations (recipes)
    crafting_combinations: Rc<RefCell<HashMap<String, CraftingCombination>>>,
    /// Registered inventory monitors
    inventory_monitors: Rc<RefCell<HashMap<usize, Box<dyn InventoryMonitor>>>>,
    /// Messages queued for sending on the next frame
    message_queue: RefCell<BTreeMap<usize, Event>>
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
            items: Arc::new(RefCell::new(HashMap::new())),
            crafting_combinations: Rc::new(RefCell::new(HashMap::new())),
            inventory_monitors: Rc::new(RefCell::new(HashMap::new())),
            weight: Cell::new(0.),
            message_queue: RefCell::new(BTreeMap::new())
        }
    }

    /// Decreases item count for a given item kind. If count becomes zero, removes item from
    /// the inventory. If item is an infinite resource, nothing will happen.
    ///
    /// Will recalculate weight automatically on success
    ///
    /// # Parameters
    /// - `amount`: this number will be subtracted from the count
    ///
    /// ## Note
    /// Borrows `items` collection
    pub fn use_item(&self, name: &String, amount: usize) -> Result<(), InventoryUseErr> {
        {
            let mut b = self.items.borrow_mut();
            self.use_item_internal(name, amount, &mut b)?
        }

        self.recalculate_weight();

        Ok(())
    }

    fn use_item_internal(&self, name: &String, amount: usize, items_mut: &mut HashMap<String, Box<dyn InventoryItem>>) -> Result<(), InventoryUseErr> {
        match items_mut.get_mut(name) {
            Some(o) => {
                if o.get_is_infinite() { return Ok(()) }

                let c = o.get_count();
                if amount > c { return Err(InventoryUseErr::InsufficientResources) }

                if c - amount == 0 {
                    // Need to clean up
                    items_mut.remove(name);
                } else {
                    o.set_count(c - amount);
                }
            },
            None => return Err(InventoryUseErr::ItemNotFound)
        };

        Ok(())
    }

    /// Returns total cached inventory weight (in grams)
    pub fn get_weight(&self) -> f32 { self.weight.get() }

    /// Recalculates the inventory weight
    fn recalculate_weight(&self) {
        let mut total_weight: f32;

        total_weight = 0.;

        for (_key, item) in self.items.borrow().iter() {
            total_weight += item.get_total_weight();
        }

        self.weight.set(total_weight);
    }
}

impl MessageQueue for Inventory {
    fn has_messages(&self) -> bool {
        self.message_queue.borrow().len() > 0
    }

    fn queue_message(&self, message: Event) {
        let mut q = self.message_queue.borrow_mut();
        let id = q.len();

        q.insert(id, message);
    }

    fn get_message_queue(&self) -> RefMut<'_, BTreeMap<usize, Event>> {
        self.message_queue.borrow_mut()
    }
}