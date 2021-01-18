use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

/// Describes item in combination
pub struct ItemInCombination {
    /// Unique name of the item kind
    pub item_name: String,
    /// Count of items needed
    pub count: usize
}

impl ItemInCombination {
    pub fn copy(&self) -> ItemInCombination {
        ItemInCombination {
            item_name: String::from(&self.item_name),
            count: self.count
        }
    }
}

/// Describes crafting recipe
pub struct CraftingCombination {
    /// Unique key of this combination
    pub key: String,
    /// Items involved
    pub items: Rc<RefCell<HashMap<String, ItemInCombination>>>
}

impl CraftingCombination {
    pub fn new(items: Vec<ItemInCombination>) -> Self {
        let mut mapped = HashMap::new();
        let mut copy = Vec::from(items);
        let key = &mut String::new();
        let mut b = [0; 2];
        let sep = '\u{0003}'.encode_utf8(&mut b);

        copy.sort_by(|a, b| a.item_name.cmp(&b.item_name));

        for item in copy.iter() {
            mapped.insert(String::from(&item.item_name), item.copy());
            key.push_str(&item.item_name);
            key.push_str(&sep);
        }

        CraftingCombination {
            key: key.to_string(),
            items: Rc::new(RefCell::new(mapped))
        }
    }
}