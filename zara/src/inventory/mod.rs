use crate::inventory::items::InventoryItem;
use crate::inventory::crafting::CraftingCombination;

use std::collections::HashMap;
use std::cell::{Cell, RefCell};
use std::sync::Arc;
use std::rc::Rc;

mod crud;
mod update;

pub mod items;
pub mod crafting;

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
    crafting_combinations: Rc<RefCell<Vec<CraftingCombination>>>
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
            crafting_combinations: Rc::new(RefCell::new(Vec::new())),
            weight: Cell::new(0.)
        }
    }

    /// Shorthand function to change count of a given kind
    pub fn change_item_count(&self, name: &String, new_value: usize) {
        let b = self.items.borrow();
        let res = b.get(name);

        if res.is_some(){
            res.unwrap().set_count(new_value);

            self.recalculate_weight();
        }
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