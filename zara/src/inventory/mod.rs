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

pub(crate) mod state;

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
    /// Registered inventory monitors.
    ///
    /// # Important
    /// Do not alter this collection manually. Use
    /// [`register_monitor`] and [`unregister_monitor`] methods instead.
    ///
    /// [`register_monitor`]: #method.register_monitor
    /// [`unregister_monitor`]: #method.unregister_monitor
    pub inventory_monitors: Rc<RefCell<HashMap<usize, Box<dyn InventoryMonitor>>>>,

    /// Weight of all inventory items (in grams)
    weight: Cell<f32>,
    /// Registered crafting combinations (recipes)
    crafting_combinations: Rc<RefCell<HashMap<String, CraftingCombination>>>,
    /// Clothes cache
    clothes_cache: RefCell<Vec<String>>,
    /// Messages queued for sending on the next frame
    message_queue: RefCell<BTreeMap<usize, Event>>
}

impl Inventory {
    pub(crate) fn new() -> Self {
        Inventory {
            items: Arc::new(RefCell::new(HashMap::new())),
            crafting_combinations: Rc::new(RefCell::new(HashMap::new())),
            inventory_monitors: Rc::new(RefCell::new(HashMap::new())),
            weight: Cell::new(0.),
            message_queue: RefCell::new(BTreeMap::new()),
            clothes_cache: RefCell::new(Vec::new())
        }
    }

    /// Returns count of a certain item kind. None if not found.
    ///
    /// # Parameters
    /// - `name`: unique item kind name
    pub fn get_count_of(&self, name: &String) -> Option<usize> {
        match self.items.borrow().get(name) {
            Some(item) => Some(item.get_count()),
            None => None
        }
    }

    /// Returns total weight of a certain item. None if not found.
    ///
    /// # Parameters
    /// - `name`: unique item kind name
    pub fn get_weight_of(&self, name: &String) -> Option<f32> {
        match self.items.borrow().get(name) {
            Some(item) => Some(item.get_total_weight()),
            None => None
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
                if o.get_is_infinite() {
                    self.queue_message(Event::InventoryItemUsedPartially(name.to_string(), amount));
                    return Ok(())
                }

                let c = o.get_count();
                if amount > c { return Err(InventoryUseErr::InsufficientResources) }

                if c - amount == 0 {
                    // Need to clean up
                    items_mut.remove(name);
                    self.queue_message(Event::InventoryItemUsedAll(name.to_string(), amount));
                } else {
                    o.set_count(c - amount);

                    self.queue_message(Event::InventoryItemUsedPartially(name.to_string(), amount));
                }
            },
            None => return Err(InventoryUseErr::ItemNotFound)
        };

        Ok(())
    }

    /// Returns total cached inventory weight (in grams)
    pub fn get_weight(&self) -> f32 { self.weight.get() }

    /// Recalculates the inventory weight. Is called automatically every time inventory
    /// or clothes changes
    pub fn recalculate_weight(&self) {
        let old_weight = self.weight.get();
        let mut new_weight: f32;

        new_weight = 0.;

        let cc = self.clothes_cache.borrow();
        for (name, item) in self.items.borrow().iter() {
            // Do not count clothes we're wearing
            if !cc.contains(name) {
                new_weight += item.get_total_weight();
            }
        }

        self.weight.set(new_weight);

        if old_weight != new_weight {
            self.queue_message(Event::InventoryWeightChanged(old_weight, new_weight));
        }
    }

    pub(crate) fn update_clothes_cache(&self, new_clothes: Vec<String>) {
        self.clothes_cache.replace(new_clothes);
        self.recalculate_weight();
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