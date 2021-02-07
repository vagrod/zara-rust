use crate::inventory::Inventory;

pub struct InventoryStateContract {
    pub weight: f32,
    pub clothes_cache: Vec<String>,
}

impl Inventory {
    pub(crate) fn get_state(&self) -> InventoryStateContract {
        InventoryStateContract {
            weight: self.weight.get(),
            clothes_cache: self.clothes_cache.borrow().clone()
        }
    }
    pub(crate) fn restore_state(&self, state: &InventoryStateContract) {
        self.weight.set(state.weight);
        self.clothes_cache.replace(state.clothes_cache.clone());
    }
}