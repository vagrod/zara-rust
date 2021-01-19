use crate::inventory::crafting::fluent::BuilderStepResultItem;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use crate::inventory::Inventory;

mod fluent;

impl Inventory {

    /// Registers crafting combinations (recipes) for this Zara instance
    ///
    /// # Examples
    ///
    /// ```
    /// use zara::inventory::crafting;
    ///
    /// person.inventory.register_crafting_combinations(
    ///    vec! [
    ///         crafting::Builder::start()
    ///            .build_for("StoneAxe")
    ///                .is("SharpenStone", 1)
    ///                .plus("Stick", 3)
    ///                .and("Rope", 2)
    ///            .build(),
    ///
    ///        crafting::Builder::start()
    ///            .build_for("LeafHat")
    ///                .is("Leaf", 30)
    ///                .and("NeedleAndThread", 1)
    ///            .build(),
    ///
    ///        crafting::Builder::start()
    ///            .build_for("FishingRod")
    ///                .is("Stick", 1)
    ///                .plus("Liana", 1)
    ///                .plus("Pin", 1)
    ///                .and("Worm", 2)
    ///            .build(),
    ///    ]
    /// );
    /// ```
    pub fn register_crafting_combinations(&self, combinations: Vec<CraftingCombination>) {
        for combination in combinations {
            self.crafting_combinations.borrow_mut().push(combination);
        }
    }

    /// Returns a list of `combination unique keys` for the combinations that can be done
    /// using a set of passed items (**without checking for resources availability**)
    pub fn get_suitable_combinations_for(&self, items: Vec<String>) -> Vec<String> {
        let key_to_check_against = get_match_key(items);
        let mut result = Vec::new();

        for cmb in self.crafting_combinations.borrow().iter() {
            if cmb.match_key == key_to_check_against {
                result.push(String::from(&cmb.unique_key));
            }
        }

        return result;
    }

}

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
    pub unique_key: String,
    /// Key for matching resources
    pub match_key: String,
    /// Result item kind
    pub result_item: String,
    /// Items involved
    pub items: Rc<RefCell<HashMap<String, ItemInCombination>>>
}

impl CraftingCombination {
    pub fn new(result_item: String, items: Vec<ItemInCombination>) -> Self {
        let mut mapped = HashMap::new();
        let mut copy = Vec::from(items);
        let key = &mut String::from(&result_item);
        let mut item_names: Vec<String> = Vec::new();
        let mut b = [0; 2];
        let sep = '\u{0003}'.encode_utf8(&mut b);

        key.push_str(sep);
        copy.sort_by(|a, b| a.item_name.cmp(&b.item_name));

        for item in copy.iter() {
            item_names.push(String::from(&item.item_name));

            mapped.insert(String::from(&item.item_name), item.copy());
            key.push_str(&item.item_name);
            key.push_str(&sep);
            key.push_str(&item.count.to_string());
            key.push_str(&sep);
        }

        CraftingCombination {
            unique_key: key.to_string(),
            match_key: get_match_key(item_names).to_string(),
            result_item,
            items: Rc::new(RefCell::new(mapped))
        }
    }
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
    result_item: RefCell<String>,
    items: Rc<RefCell<Vec<ItemInCombination>>>
}

impl Builder {
    pub fn start() -> Box<dyn BuilderStepResultItem> {
        Box::new(Builder {
            result_item: RefCell::new(String::new()),
            items: Rc::new(RefCell::new(Vec::new()))
        })
    }
}

fn get_match_key(items: Vec<String>) -> String {
    let mut match_key: String = String::new();
    let mut copy = Vec::from(items);
    let mut b = [0; 2];
    let sep = '\u{0003}'.encode_utf8(&mut b);

    copy.sort_by(|a, b| a.cmp(b));

    for item in copy.iter() {
        match_key.push_str(item);
        match_key.push_str(&sep);
    }

    return match_key.to_string();
}