use crate::inventory::crafting::fluent::BuilderStepResultItem;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

mod fluent;

/// Describes item in combination
pub struct ItemInCombination {
    /// Unique name of the item kind
    pub item_name: String,
    /// Count of items needed
    pub count: usize
}

impl ItemInCombination {
    /// Creates new `ItemInCombination`.
    ///
    /// # Parameters
    /// - `name`: unique name of the inventory item kind
    /// - `count`: how many these items we'll need
    pub fn new(name: &str, count: usize) -> Self {
        ItemInCombination {
            item_name: String::from(name),
            count
        }
    }

    /// Creates a copy of this instance
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
    /// Result item kind
    pub result_item: String,
    /// Items involved
    pub items: Rc<RefCell<HashMap<String, ItemInCombination>>>
}

/// Used to build a crafting combination (crafting reciepe)
///
/// # Example
///
/// ```
/// use zara::inventory::crafting;
///
/// let combination = crafting::Builder::start()
///   .build_for("FishingRod")
///     .is("Stick", 1)
///     .plus("Liana", 1)
///     .plus("Pin", 1)
///     .and("Worm", 2)
///   .build();
/// ```
pub struct Builder {
    pub result_item: RefCell<String>,
    pub items: Rc<RefCell<Vec<ItemInCombination>>>
}

impl Builder {
    pub fn start() -> Box<dyn BuilderStepResultItem> {
        Box::new(Builder {
            result_item: RefCell::new(String::new()),
            items: Rc::new(RefCell::new(Vec::new()))
        })
    }
}

impl CraftingCombination {
    pub fn new(result_item: String, items: Vec<ItemInCombination>) -> Self {
        let mut mapped = HashMap::new();
        let mut copy = Vec::from(items);
        let key = &mut String::from(&result_item);
        let mut b = [0; 2];
        let sep = '\u{0003}'.encode_utf8(&mut b);

        key.push_str(sep);
        copy.sort_by(|a, b| a.item_name.cmp(&b.item_name));

        for item in copy.iter() {
            mapped.insert(String::from(&item.item_name), item.copy());
            key.push_str(&item.item_name);
            key.push_str(&sep);
        }

        CraftingCombination {
            key: key.to_string(),
            result_item,
            items: Rc::new(RefCell::new(mapped))
        }
    }
}