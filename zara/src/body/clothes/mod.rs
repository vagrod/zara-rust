use crate::body::Body;

use std::collections::HashMap;

pub mod fluent;

impl Body {
    /// Registers new clothes group.
    ///
    /// ## Parameters
    /// - `group`: clothes group to register. Use [`ClothesGroupBuilder`](crate::body::ClothesGroupBuilder)
    ///     to create one.
    ///
    /// ## Examples
    ///
    ///```
    /// use crate::zara::body::ClothesGroupBuilder;
    ///
    /// person.body.register_clothes_group(
    ///     ClothesGroupBuilder::start()
    ///         .with_name("Group Name")
    ///             .cold_resistance(5)
    ///             .water_resistance(12)
    ///               //.. and so on
    ///     // .build()
    ///  );
    ///```
    ///
    pub fn register_clothes_group(&self, group: ClothesGroup) {
        self.clothes_groups.borrow_mut().insert(group.name.to_string(), group);
    }
}

#[derive(Clone)]
pub struct ClothesItem {
    pub name: String,
    pub water_resistance: usize,
    pub cold_resistance: usize
}
impl ClothesItem {
    pub fn new(name: String, water_resistance: usize, cold_resistance: usize) -> Self {
        ClothesItem {
            name,
            water_resistance,
            cold_resistance
        }
    }
}

pub struct ClothesGroup {
    pub name: String,
    pub items: HashMap<String, ClothesItem>,
    pub bonus_cold_resistance: usize,
    pub bonus_water_resistance: usize
}
impl ClothesGroup {
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
    pub fn contains(&self, item_name: &String) -> bool { self.items.contains_key(item_name) }
}
