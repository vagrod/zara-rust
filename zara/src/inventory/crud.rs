use crate::inventory::Inventory;
use crate::inventory::items::InventoryItem;
use crate::error::InventoryItemAccessErr;
use crate::utils::event::{MessageQueue, Event};

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
    ///
    /// # Notes
    /// Borrows the `items` collection
    pub fn add_item(&self, item: Box<dyn InventoryItem>) {
        let key = item.get_name();
        let key_for_message = item.get_name().to_string();

        self.items.borrow_mut().insert(key, item);
        self.recalculate_weight();

        self.queue_message(Event::InventoryItemAdded(key_for_message));
    }

    /// Removes item kind from the inventory and recalculates inventory weight
    ///
    /// # Parameters
    /// - `item_kind`: unique name of the item ("InventoryItem.name")
    ///
    /// # Returns
    /// Ok on success.
    ///
    /// # Notes
    /// Borrows the `items` collection
    pub fn remove_item(&self, item_kind: &String) -> Result<(), InventoryItemAccessErr> {
        let mut b = self.items.borrow_mut();

        if b.contains_key(item_kind) {
            b.remove(item_kind);

            self.recalculate_weight();

            self.queue_message(Event::InventoryItemRemoved(item_kind.to_string()));

            return Ok(());
        }

        Err(InventoryItemAccessErr::ItemNotFound)
    }
}