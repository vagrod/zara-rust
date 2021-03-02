use crate::body::{Body, ClothesItemC};
use crate::error::{RequestClothesOffErr, RequestClothesOnErr};
use crate::inventory::items::ClothesDescription;
use crate::utils::ClothesGroupC;
use crate::utils::event::{MessageQueue, Event};

use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};

mod warmth;
mod wetness;

pub mod fluent;

impl Body {
    /// Registers a list of clothes groups.
    ///
    /// # Parameters
    /// - `groups`: a list of clothes groups to register. Use [`ClothesGroupBuilder`](crate::body::ClothesGroupBuilder)
    ///     to create one.
    ///
    /// # Examples
    ///
    ///```
    /// use crate::zara::body::ClothesGroupBuilder;
    ///
    /// person.body.register_clothes_groups(
    ///     vec![
    ///         ClothesGroupBuilder::start()
    ///             .with_name("Group Name")
    ///                 .bonus_cold_resistance(5)
    ///                 .bonus_water_resistance(12)
    ///                 .includes(
    ///                     vec![
    ///                         ("Jacket", JacketClothes),
    ///                         ("Pants", PantsClothes),
    ///                         //.. and so on
    ///                     ]
    ///                 )
    ///             .build()
    ///     ]
    ///  );
    ///```
    pub fn register_clothes_groups(&self, groups: Vec<ClothesGroup>) {
        let mut b = self.clothes_groups.borrow_mut();
        for group in groups {
            b.insert(group.name.to_string(), group);
        }
    }

    pub(crate) fn request_clothes_on(&self, item_name: &String, data: &dyn ClothesDescription) -> Result<(), RequestClothesOnErr> {
        {
            let mut clothes = self.clothes.borrow_mut();
            if clothes.contains(item_name) {
                return Err(RequestClothesOnErr::AlreadyHaveThisItemOn);
            }

            clothes.push(item_name.to_string());

            let mut cdata = self.clothes_data.borrow_mut();
            cdata.insert(item_name.to_string(), ClothesItemC {
                cold_resistance: data.cold_resistance(),
                water_resistance: data.water_resistance()
            });
        }

        self.refresh_clothes_group();
        self.recalculate_warmth_level();

        self.queue_message(Event::ClothesOn(item_name.to_string()));

        Ok(())
    }

    pub(crate) fn request_clothes_off(&self, item_name: &String) -> Result<(), RequestClothesOffErr> {
        {
            let mut clothes = self.clothes.borrow_mut();
            match clothes.iter().position(|x| x == item_name) {
                Some(ind) => {
                    clothes.remove(ind);
                },
                None => {
                    return Err(RequestClothesOffErr::ItemIsNotOn);
                }
            }

            let mut cdata = self.clothes_data.borrow_mut();
            if cdata.contains_key(item_name) {
                cdata.remove(item_name);
            }
        }

        self.refresh_clothes_group();
        self.recalculate_warmth_level();

        self.queue_message(Event::ClothesOff(item_name.to_string()));

        Ok(())
    }

    fn refresh_clothes_group(&self) {
        match self.clothes_groups.borrow_mut().iter().find(|(_, group)|
            (*group).has_complete(self.clothes.borrow().clone())) {
            Some((key, g)) => {
                self.clothes_group.replace(Some(ClothesGroupC {
                    name: key.to_string(),
                    bonus_cold_resistance: g.bonus_cold_resistance,
                    bonus_water_resistance: g.bonus_water_resistance
                }))
            },
            None => self.clothes_group.replace(None)
        };
    }
}

/// Holds the information about clothes item
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Default)]
pub struct ClothesItem {
    /// Name of the inventory item
    pub name: String,
    /// Water resistance value, 0..100
    pub water_resistance: usize,
    /// Cold resistance value, 0..100
    pub cold_resistance: usize
}
impl fmt::Display for ClothesItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl ClothesItem {
    /// Creates a new clothes item
    ///
    /// # Parameters
    /// - `name`: unique name of the inventory item
    /// - `water_resistance`: percent 0..100 of the water resistance value
    /// - `cold_resistance`: percent 0..100 of the cold resistance value
    pub fn new(name: String, water_resistance: usize, cold_resistance: usize) -> Self {
        ClothesItem {
            name,
            water_resistance,
            cold_resistance
        }
    }
}

/// Holds the information about clothes group set
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct ClothesGroup {
    /// Name of the group
    pub name: String,
    /// Items that are included in this group
    pub items: HashMap<String, ClothesItem>,
    /// Group cold resistance bonus, 0..100
    pub bonus_cold_resistance: usize,
    /// Group water resistance bonus, 0..100
    pub bonus_water_resistance: usize
}
impl fmt::Display for ClothesGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {} items", self.name, self.items.len())
    }
}
impl Hash for ClothesGroup {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.items.iter().for_each(|(key, _)| key.hash(state));
        self.bonus_cold_resistance.hash(state);
        self.bonus_water_resistance.hash(state);
    }
}

impl ClothesGroup {
    /// Creates new clothes group set. You can use [`ClothesGroupBuilder`](crate::body::ClothesGroupBuilder) to construct new group.
    pub fn new(name: String, items: Vec<ClothesItem>, bonus_cold_resistance: usize, bonus_water_resistance: usize) -> Self {
        let mut items_map = HashMap::new();

        for item in items {
            items_map.insert(item.name.to_string(), item.clone());
        }

        ClothesGroup {
            name,
            items: items_map,
            bonus_cold_resistance,
            bonus_water_resistance
        }
    }
    /// Returns `true` is this group contains particular clothes item
    pub fn contains(&self, item_name: &String) -> bool { self.items.contains_key(item_name) }
    /// Returns `true` if given set of clothes has all the items needed for this group
    pub fn has_complete(&self, items: Vec<String>) -> bool {
        if items.len() == 0 { return false; }

        for (key, _) in self.items.iter() {
            if items.iter().all(|x| x != key) {
                return false;
            }
        }

        true
    }
}