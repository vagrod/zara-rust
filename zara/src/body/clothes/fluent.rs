use crate::body::clothes::{ClothesGroup, ClothesItem};
use crate::body::ClothesGroupBuilder;
use crate::inventory::items::{ClothesDescription};

use std::collections::HashMap;

impl ClothesGroupBuilder {
    fn as_group_cold(&self) -> &dyn ClothesGroupCold { self }
    fn as_group_water(&self) -> &dyn ClothesGroupWater { self }
    fn as_group_items(&self) -> &dyn ClothesGroupItems { self }
    fn as_group_end(&self) -> &dyn ClothesGroupEnd { self }
}

/// Group fluent step trait
pub trait ClothesGroupStart {
    /// Unique name of a clothes group. Will become its key
    fn with_name(&self, name: &str) -> &dyn ClothesGroupCold;
}

/// Group fluent step trait
pub trait ClothesGroupCold {
    /// Bonus cold resistance level that gets applied when player is wearing the whole
    /// clothes group set on top of all other resistances. 0..100 percents.
    fn bonus_cold_resistance(&self, value: usize) -> &dyn ClothesGroupWater;
}

/// Group fluent step trait
pub trait ClothesGroupWater {
    /// Bonus water resistance level that gets applied when player is wearing the whole
    /// clothes group set on top of all other resistances. 0..100 percents.
    fn bonus_water_resistance(&self, value: usize) -> &dyn ClothesGroupItems;
}

/// Group fluent step trait
pub trait ClothesGroupItems {
    /// Description of all clothes that form this group ("suit")
    ///
    /// First argument is a unique inventory item name, second is a description of the item's
    /// `ClothesDescription` option.
    ///
    /// # Examples:
    /// ```
    /// includes(
    ///     vec![
    ///         ("Jacket", Box::new(JacketClothes)),
    ///         ("Pants", Box::new(PantsClothes))
    ///     ]
    /// )
    /// ```
    fn includes(&self, items: Vec<(&str, Box<dyn ClothesDescription>)>) -> &dyn ClothesGroupEnd;
}

/// Group fluent step trait
pub trait ClothesGroupEnd {
    /// Builds resulted clothes group according with the information provided
    fn build(&self) -> ClothesGroup;
}

impl ClothesGroupStart for ClothesGroupBuilder {
    fn with_name(&self, name: &str) -> &dyn ClothesGroupCold {
        self.name.replace(name.to_string());

        self.as_group_cold()
    }
}

impl ClothesGroupCold for ClothesGroupBuilder {
    fn bonus_cold_resistance(&self, value: usize) -> &dyn ClothesGroupWater {
        self.bonus_cold_resistance.set(value);

        self.as_group_water()
    }
}

impl ClothesGroupWater for ClothesGroupBuilder {
    fn bonus_water_resistance(&self, value: usize) -> &dyn ClothesGroupItems {
        self.bonus_water_resistance.set(value);

        self.as_group_items()
    }
}

impl ClothesGroupItems for ClothesGroupBuilder {
    fn includes(&self, items: Vec<(&str, Box<dyn ClothesDescription>)>) -> &dyn ClothesGroupEnd {
        let mut list = HashMap::new();
        for (name, item) in items {
            list.insert(name.to_string(),
                        ClothesItem::new(name.to_string(),
                                         item.water_resistance(),
                                         item.cold_resistance()));
        }
        self.items.replace(list);

        self.as_group_end()
    }
}

impl ClothesGroupEnd for ClothesGroupBuilder {
    fn build(&self) -> ClothesGroup {
        let mut items = HashMap::new();

        for (name, group) in self.items.borrow().iter() {
            items.insert(name.to_string(), group.clone());
        }

        ClothesGroup {
            name: self.name.borrow().to_string(),
            bonus_cold_resistance: self.bonus_cold_resistance.get(),
            bonus_water_resistance: self.bonus_water_resistance.get(),
            items
        }
    }
}