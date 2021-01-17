use crate::utils::GameTimeC;

/// Trait that must be implemented by all inventory items
pub trait InventoryItem {
    /// Returns count of items of this kind in the inventory
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let n = item.get_count();
    /// ```
    fn get_count(&self) -> usize;

    /// Sets new count for items of this kind
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// item.set_count(new_value);
    /// ```
    fn set_count(&self, new_count: usize);

    /// Gets unique name for all items of this kind
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let s = item.get_name();
    /// ```
    fn get_name(&self) -> String;

    /// Gets calculated weight of all items of this kind, in grams.
    ///
    /// Most of the time, it is `count` * `weight per item`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let n = item.get_total_weight();
    /// ```
    fn get_total_weight(&self) -> f32;

    /// Node that describes behavior of this item as a consumable
    fn consumable(&self) -> Option<&dyn ConsumableBehavior>;
}

/// Trait to describe consumable behavior of the inventory item
pub trait ConsumableBehavior {
    /// True if this item should be treated as food
    fn is_food(&self) -> bool;
    /// True if this item should be treated as water
    fn is_water(&self) -> bool;
    /// How much water points consuming of this item gives (0..100 scale)
    fn water_gain_per_dose(&self) -> f32;
    /// How much food points consuming of this item gives (0..100 scale)
    fn food_gain_per_dose(&self) -> f32;
    /// Node that describes the spoiling options of this consumable
    fn spoiling(&self) -> Option<&dyn SpoilingBehavior>;
}

/// Trait to describe the spoiling options of the consumable
pub trait SpoilingBehavior {
    /// Chance of getting a food poisoning after eating one fresh item (0..100 scale)
    fn fresh_poisoning_chance(&self) -> usize;
    /// Chance of getting a food poisoning after eating one fresh item (0..100 scale)
    fn spoil_poisoning_chance(&self) -> usize;
    /// Time that is needed for fresh item to become spoiled
    /// ([GameTimeC](crate::utils::GameTimeC) structure)
    fn spoil_time(&self) -> GameTimeC;
}