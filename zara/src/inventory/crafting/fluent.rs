use crate::inventory::crafting::{CraftingCombination, ItemInCombination, Builder};
use crate::inventory::items::InventoryItem;

/// Macro to describe crafting combination resulting instance creation.
///
/// Receives an instance of an object that implements
/// [`InventoryItem`](crate::zara::inventory::InventoryItem) trait.
#[macro_export]
macro_rules! inv_result(
    ($r:expr) => (
        Box::new(|| Box::new($r))
    );
);

pub trait BuilderStepResultItem {
    /// Defines the resulted item kind.
    ///
    /// # Parameters
    /// - `key`: unique name of a resulted item
    fn build_for(&self, key: &str) -> &dyn BuilderStepFirstItem;
}

pub trait BuilderStepFirstItem {
    /// Adds first item to the combination. Items order does not matter.
    ///
    /// # Parameters
    /// - `key`: unique name of an item
    /// - `count`: how many items of this kind this combination demands
    fn is(&self, key: &str, count: usize) -> &dyn BuilderStepItemNode;
}

pub trait BuilderStepItemNode {
    /// Adds new item to the combination. Items order does not matter.
    ///
    /// # Parameters
    /// - `key`: unique name of an item
    /// - `count`: how many items of this kind this combination demands
    fn plus(&self, key: &str, count: usize) -> &dyn BuilderStepItemNode;
    /// Adds last item to the combination. Items order does not matter.
    ///
    /// # Parameters
    /// - `key`: unique name of an item
    /// - `count`: how many items of this kind this combination demands
    fn and(&self, key: &str, count: usize) -> &dyn BuilderStepDone;
}

pub trait BuilderStepDone {
    /// Builds the crafting combination based on the info provided.
    ///
    /// # Parameters
    /// - `create`: function that returns an instance of the resulted object. You can use `inv_result!`
    ///     macro here
    /// ```
    /// build(zara::inv_result!(FishingRod { count: 1 }))
    /// ```
    fn build(&self, create: Box<dyn Fn() -> Box<dyn InventoryItem> + 'static>) -> CraftingCombination;
}

impl Builder {
    fn as_builder_step_first_item(&self) -> &dyn BuilderStepFirstItem { self }
    fn as_builder_step_item_node(&self) -> &dyn BuilderStepItemNode { self }
    fn as_builder_step_done(&self) -> &dyn BuilderStepDone { self }
}

impl BuilderStepResultItem for Builder {
    fn build_for(&self, key: &str) -> &dyn BuilderStepFirstItem {
        self.result_item.replace(String::from(key));

        self.as_builder_step_first_item()
    }
}

impl BuilderStepFirstItem for Builder {
    fn is(&self, key: &str, count: usize) -> &dyn BuilderStepItemNode {
        self.items.borrow_mut().push(ItemInCombination::new(key, count));

        self.as_builder_step_item_node()
    }
}

impl BuilderStepItemNode for Builder {
    fn plus(&self, key: &str, count: usize) -> &dyn BuilderStepItemNode {
        self.items.borrow_mut().push(ItemInCombination::new(key, count));

        self.as_builder_step_item_node()
    }

    fn and(&self, key: &str, count: usize) -> &dyn BuilderStepDone {
        self.items.borrow_mut().push(ItemInCombination::new(key, count));

        self.as_builder_step_done()
    }
}

impl BuilderStepDone for Builder {
    fn build(&self, create: Box<dyn Fn() -> Box<dyn InventoryItem> + 'static>) -> CraftingCombination {
        let mut items = Vec::new();

        for item in self.items.borrow().iter() {
            items.push(item.copy());
        }

        CraftingCombination::new(self.result_item.borrow().to_string(), items, create)
    }
}