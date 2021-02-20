use crate::error::{CheckForResourcesErr, CombinationExecuteErr};
use crate::inventory::crafting::fluent::BuilderStepResultItem;
use crate::inventory::Inventory;
use crate::inventory::items::InventoryItem;
use crate::utils::event::{MessageQueue, Event};

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Debug;

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
    ///            .build(zara::inv_result!(StoneAxe { count: 1 })),
    ///
    ///        crafting::Builder::start()
    ///            .build_for("LeafHat")
    ///                .is("Leaf", 30)
    ///                .and("NeedleAndThread", 1)
    ///            .build(zara::inv_result!(LeafHat { count: 1 })),
    ///
    ///        crafting::Builder::start()
    ///            .build_for("FishingRod")
    ///                .is("Stick", 1)
    ///                .plus("Liana", 1)
    ///                .plus("Pin", 1)
    ///                .and("Worm", 2)
    ///            .build(zara::inv_result!(FishingRod { count: 1 })),
    ///    ]
    /// );
    /// ```
    pub fn register_crafting_combinations(&self, combinations: Vec<CraftingCombination>) {
        let mut b = self.crafting_combinations.borrow_mut();
        for combination in combinations {
            b.insert(combination.unique_key.to_string(), combination);
        }
    }

    /// Returns a list of `combination unique keys` for the combinations that can be done
    /// using a set of passed items (**without checking for resources availability**)
    ///
    /// # Examples
    /// ```
    /// let ids = person.inventory.get_suitable_combinations_for(
    ///     vec![
    ///         &format!("Stick"),
    ///         &format!("Rope"),
    ///         &format!("SharpStone")
    ///     ]
    /// );
    /// ```
    pub fn get_suitable_combinations_for(&self, items: Vec<&String>) -> Vec<String> {
        let key_to_check_against = get_match_key(items);
        let mut result = Vec::new();

        for (key, cmb) in self.crafting_combinations.borrow().iter() {
            if cmb.match_key == key_to_check_against {
                result.push(key.to_string());
            }
        }

        result
    }

    /// Checks if inventory has enough resources to execute a given combination
    ///
    /// # Parameters
    /// - `combination_id`: unique id of a combination to check
    pub fn check_for_resources(&self, combination_id: &String) -> Result<(), CheckForResourcesErr> {
        match self.crafting_combinations.borrow().get(combination_id) {
            Some(cmb) => {
                for (name, item_data) in cmb.items.borrow().iter() {
                    match self.items.borrow().get(name) {
                        Some(item) => {
                            if !item.get_is_infinite() && item.get_count() < item_data.count {
                                return Err(CheckForResourcesErr::InsufficientResources(name.to_string()));
                            }
                        },
                        None => return Err(CheckForResourcesErr::ItemNotFound(name.to_string()))
                    }
                }

                Ok(())
            },
            None => Err(CheckForResourcesErr::CombinationNotFound)
        }
    }

    /// Executes given crafting combination. This method will check for resources availability
    /// before trying.
    ///
    /// # Parameters
    /// - `combination_id`: unique key of a combination to execute
    ///
    /// ## Note
    /// Borrows `items` collection
    pub fn execute_combination(&self, combination_id: &String) -> Result<(), CombinationExecuteErr> {
        let cc = self.crafting_combinations.borrow();
        let cmb = match cc.get(combination_id) {
            Some(c) => c,
            None => return Err(CombinationExecuteErr::CombinationNotFound)
        };

        self.check_for_resources(combination_id).or_else(|e| Err(CombinationExecuteErr::ResourceError(e)))?;
        {
            let mut b = self.items.borrow_mut();
            for (key, item_data) in cmb.items.borrow().iter() {
                // Properly use the item. It should return ok because we just checked resources
                self.use_item_internal(key, item_data.count, &mut b)
                    .or_else(|e| Err(CombinationExecuteErr::UseItemError(e)))?;
            }

            let resulted = (cmb.create)();
            match b.get_mut(&cmb.result_item) {
                Some(item) => {
                    // Increase count if we have item already
                    item.set_count(item.get_count() + resulted.get_count())
                },
                None => {
                    // Add a new instance otherwise
                    b.insert(cmb.result_item.to_string(), resulted);
                }
            }
        }

        self.recalculate_weight();
        self.queue_message(Event::CraftingCombinationExecuted(combination_id.to_string()));

        Ok(())
    }
}

/// Describes item in combination
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Default)]
pub struct ItemInCombination {
    /// Unique name of the item kind
    pub item_name: String,
    /// Count of items needed
    pub count: usize
}
impl fmt::Display for ItemInCombination {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} of {}", self.count, self.item_name)
    }
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
    pub items: Rc<RefCell<HashMap<String, ItemInCombination>>>,
    /// Function to instantiate the resulted item (hello reflection :)
    create: Box<dyn Fn() -> Box<dyn InventoryItem> + 'static>
}
impl Debug for CraftingCombination {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CraftingCombination")
            .field("unique_key", &self.unique_key)
            .field("match_key", &self.match_key)
            .field("result_item", &self.result_item)
            .field("items", &self.items)
        .finish()
    }
}
impl fmt::Display for CraftingCombination {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Combination for {} ({} items). id={}", self.result_item, self.items.borrow().len(), self.unique_key)
    }
}
impl CraftingCombination {
    pub fn new(result_item: String, items: Vec<ItemInCombination>,
               create: Box<dyn Fn() -> Box<dyn InventoryItem> + 'static>) -> Self {
        let mut mapped = HashMap::new();
        let mut copy = Vec::from(items);
        let key = &mut String::from(&result_item);
        let mut item_names: Vec<&String> = Vec::new();
        let mut b = [0; 2];
        let sep = '\u{0003}'.encode_utf8(&mut b);

        key.push_str(sep);
        copy.sort_by(|a, b| a.item_name.cmp(&b.item_name));

        for item in copy.iter() {
            item_names.push(&item.item_name);

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
            items: Rc::new(RefCell::new(mapped)),
            create
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
///   .build(zara::inv_result!(FishingRod { count: 1 }));
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

fn get_match_key(items: Vec<&String>) -> String {
    let mut match_key: String = String::new();
    let mut copy = Vec::from(items);
    let mut b = [0; 2];
    let sep = '\u{0003}'.encode_utf8(&mut b);

    copy.sort_by(|a, b| a.cmp(b));

    for item in copy.iter() {
        match_key.push_str(item);
        match_key.push_str(&sep);
    }

    match_key.to_string()
}