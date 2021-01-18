use crate::inventory::Inventory;
use crate::inventory::items::InventoryItem;

/// Contains code that adds, remove  or upate the inventory

impl Inventory {

    /// Returns `true` if item of this kind exists in the inventory
    ///
    ///# Parameters
    ///- `item_name`: unique name of the item (item kind)
    pub fn has_item(&self, item_name: &String) -> bool {
        let b = self.items.borrow();

        b.contains_key(item_name)
    }

    /// Adds new item to the inventory and recalculates inventory weight
    ///
    /// # Parameters
    /// - `item`: any boxed object that supports `InventoryItem` trait
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
    ///
    /// `false` if a given kind was not found.
    pub fn remove_item(&self, item_kind: &String) -> bool {
        let mut b = self.items.borrow_mut();

        if b.contains_key(item_kind) {
            b.remove(item_kind);

            self.recalculate_weight();

            return true;
        }

        return false;
    }

}