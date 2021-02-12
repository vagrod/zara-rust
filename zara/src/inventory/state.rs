use crate::inventory::Inventory;

use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Default)]
pub struct InventoryStateContract {
    pub weight: f32,
    pub clothes_cache: Vec<String>,
}
impl fmt::Display for InventoryStateContract {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Inventory weight {:.0}g [DOES NOT CONTAIN ITEMS]", self.weight)
    }
}
impl Eq for InventoryStateContract { }
impl PartialEq for InventoryStateContract {
    fn eq(&self, other: &Self) -> bool {
        const EPS: f32 = 0.0001;

        self.clothes_cache == other.clothes_cache &&
        f32::abs(self.weight - other.weight) < EPS
    }
}
impl Hash for InventoryStateContract {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.clothes_cache.hash(state);

        state.write_u32(self.weight as u32);
    }
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