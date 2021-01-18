use crate::inventory::crafting::{CraftingCombination, ItemInCombination, Builder};

pub trait BuilderStepResultItem {
    fn build_for(&self, key: &str) -> &dyn BuilderStepFirstItem;
}

pub trait BuilderStepFirstItem {
    fn is(&self, key: &str, count: usize) -> &dyn BuilderStepItemNode;
}

pub trait BuilderStepItemNode {
    fn plus(&self, key: &str, count: usize) -> &dyn BuilderStepItemNode;
    fn and(&self, key: &str, count: usize) -> &dyn BuilderStepDone;
}

pub trait BuilderStepDone {
    fn build(&self) -> CraftingCombination;
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
        self.items.borrow_mut().push(ItemInCombination {
            count,
            item_name: String::from(key)
        });

        self.as_builder_step_item_node()
    }
}

impl BuilderStepItemNode for Builder {
    fn plus(&self, key: &str, count: usize) -> &dyn BuilderStepItemNode {
        self.items.borrow_mut().push(ItemInCombination {
            count,
            item_name: String::from(key)
        });


        self.as_builder_step_item_node()
    }

    fn and(&self, key: &str, count: usize) -> &dyn BuilderStepDone {
        self.items.borrow_mut().push(ItemInCombination {
            count,
            item_name: String::from(key)
        });


        self.as_builder_step_done()
    }
}

impl BuilderStepDone for Builder {
    fn build(&self) -> CraftingCombination {
        let mut items = Vec::new();

        for item in self.items.borrow().iter() {
            items.push(item.copy());
        }

        CraftingCombination::new(self.result_item.borrow().to_string(), items)
    }
}