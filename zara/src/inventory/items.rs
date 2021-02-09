use crate::utils::GameTimeC;

use std::any::Any;

/// Macro for declaring a simple inventory item with particular weight
///
/// # Examples
///
/// ```
/// zara::inv_item!(
///     Meat,
///     "Meat",
///     /* weight per unit */ 351.
/// );
/// ```
#[macro_export]
macro_rules! inv_item(
    ($t:ty, $nm:expr, $wt:expr) => (
        impl zara::inventory::items::InventoryItem for $t {
            fn get_count(&self) -> usize { self.count }
            fn set_count(&mut self, new_count: usize) { self.count = new_count; }
            fn get_name(&self) -> String { String::from($nm) }
            fn get_is_infinite(&self) -> bool { false }
            fn get_total_weight(&self) -> f32 { self.count as f32 * $wt }
            fn consumable(&self) -> Option<&dyn zara::inventory::items::ConsumableDescription> { None }
            fn appliance(&self) ->  Option<&dyn zara::inventory::items::ApplianceDescription> { None }
            fn clothes(&self) -> Option<&dyn zara::inventory::items::ClothesDescription> { None }
            fn as_any(&self) -> &dyn std::any::Any { self }
        }
    );
);

/// Macro for declaring an infinite inventory item with particular weight
///
/// # Examples
///
/// ```
/// zara::inv_infinite!(
///     NeedleAndThread,
///     "NeedleAndThread",
///     /* weight per unit */ 351.
/// );
/// ```
#[macro_export]
macro_rules! inv_infinite(
    ($t:ty, $nm:expr, $wt:expr) => (
        impl zara::inventory::items::InventoryItem for $t {
            fn get_count(&self) -> usize { self.count }
            fn set_count(&mut self, new_count: usize) { self.count = new_count; }
            fn get_name(&self) -> String { String::from($nm) }
            fn get_is_infinite(&self) -> bool { true }
            fn get_total_weight(&self) -> f32 { self.count as f32 * $wt }
            fn consumable(&self) -> Option<&dyn zara::inventory::items::ConsumableDescription> { None }
            fn appliance(&self) ->  Option<&dyn zara::inventory::items::ApplianceDescription> { None }
            fn clothes(&self) -> Option<&dyn zara::inventory::items::ClothesDescription> { None }
            fn as_any(&self) -> &dyn std::any::Any { self }
        }
    );
);

/// Macro for declaring consumable inventory item
///
/// # Examples
///
/// ```
/// zara::inv_item_cons!(
///     Meat,
///     "Meat",
///     /* weight per unit */ 351.,
///     /* consumable option */ Some(&MeatConsumableOption)
/// );
/// ```
#[macro_export]
macro_rules! inv_item_cons(
    ($t:ty, $nm:expr, $wt:expr, $cons:expr) => (
        impl zara::inventory::items::InventoryItem for $t {
            fn get_count(&self) -> usize { self.count }
            fn set_count(&mut self, new_count: usize) { self.count = new_count; }
            fn get_name(&self) -> String { String::from($nm) }
            fn get_is_infinite(&self) -> bool { false }
            fn get_total_weight(&self) -> f32 { self.count as f32 * $wt }
            fn consumable(&self) -> Option<&dyn zara::inventory::items::ConsumableDescription> { $cons }
            fn appliance(&self) ->  Option<&dyn zara::inventory::items::ApplianceDescription> { None }
            fn clothes(&self) -> Option<&dyn zara::inventory::items::ClothesDescription> { None }
            fn as_any(&self) -> &dyn std::any::Any { self }
        }
    );
);

/// Macro for declaring an appliance behavior
///
/// # Examples
///
/// ```
/// zara::inv_item_appl!(
///     MorphineInjection,
///     "MorphineInjection",
///     /* weight per unit */ 87.,
///     /* appliance option */ Some(&MorphineInjectionAppliance)
/// )
/// ```
#[macro_export]
macro_rules! inv_item_appl (
    ($t:ty, $nm:expr, $wt:expr, $appl:expr) => (
        impl zara::inventory::items::InventoryItem for $t {
            fn get_count(&self) -> usize { self.count }
            fn set_count(&mut self, new_count: usize) { self.count = new_count; }
            fn get_name(&self) -> String { String::from($nm) }
            fn get_is_infinite(&self) -> bool { false }
            fn get_total_weight(&self) -> f32 { self.count as f32 * $wt }
            fn consumable(&self) -> Option<&dyn zara::inventory::items::ConsumableDescription> { None }
            fn appliance(&self) ->  Option<&dyn zara::inventory::items::ApplianceDescription> { $appl }
            fn clothes(&self) -> Option<&dyn zara::inventory::items::ClothesDescription> { None }
            fn as_any(&self) -> &dyn std::any::Any { self }
        }
    );
);

/// Macro for declaring clothes inventory item
///
/// # Examples
///
/// ```
/// zara::inv_item_clothes!(
///     Jacket,
///     "Jacket",
///     /* weight per unit */ 1280.,
///     /* clothes item description */ Some(&JacketClothes).
/// );
/// ```
#[macro_export]
macro_rules! inv_item_clothes (
    ($t:ty, $nm:expr, $wt:expr, $cl:expr) => (
        impl zara::inventory::items::InventoryItem for $t {
            fn get_count(&self) -> usize { self.count }
            fn set_count(&mut self, new_count: usize) { self.count = new_count; }
            fn get_name(&self) -> String { String::from($nm) }
            fn get_is_infinite(&self) -> bool { false }
            fn get_total_weight(&self) -> f32 { self.count as f32 * $wt }
            fn consumable(&self) -> Option<&dyn zara::inventory::items::ConsumableDescription> { None }
            fn appliance(&self) ->  Option<&dyn zara::inventory::items::ApplianceDescription> { None }
            fn clothes(&self) -> Option<&dyn zara::inventory::items::ClothesDescription> { $cl }
            fn as_any(&self) -> &dyn std::any::Any { self }
        }
    );
);

/// Macro for declaring body appliance option
///
/// # Examples
///
/// ```
/// zara::inv_body_appliance!(BandageOption);
/// ```
#[macro_export]
macro_rules! inv_body_appliance (
    ($t:ty) => (
        impl zara::inventory::items::ApplianceDescription for $t {
            fn is_body_appliance(&self) -> bool { true }
            fn is_injection(&self) -> bool { false }
        }
    );
);

/// Macro for declaring injection appliance option
///
/// # Examples
///
/// ```
/// zara::inv_injection_appliance!(InjectionOption);
/// ```
#[macro_export]
macro_rules! inv_injection_appliance (
    ($t:ty) => (
        impl zara::inventory::items::ApplianceDescription for $t {
            fn is_body_appliance(&self) -> bool { false }
            fn is_injection(&self) -> bool { true }
        }
    );
);

/// Macro for declaring food consumable option
///
/// # Examples
///
/// ```
/// zara::inv_food!(
///     MeatConsumableOption,
///     /* water gain, 0..100% */ 10.,
///     /* food gain, 0..100% */ 68.,
///     /* spoil option */ Some(&MeatSpoiling)
/// );
/// ```
#[macro_export]
macro_rules! inv_food(
    ($t:ty, $wg:expr, $fg:expr, $sp:expr) => (
        impl zara::inventory::items::ConsumableDescription for $t {
            fn is_food(&self) -> bool { true }
            fn is_water(&self) -> bool { false}
            fn water_gain_per_dose(&self) -> f32 { $wg as f32}
            fn food_gain_per_dose(&self) -> f32 { $fg as f32 }
            fn spoiling(&self) -> Option<&dyn zara::inventory::items::SpoilingBehavior> { $sp }
        }
    );
);

/// Macro for declaring water consumable option
///
/// # Examples
///
/// ```
/// zara::inv_water!(
///     DrinkableWaterOption,
///     /* water gain, 0..100% */ 27.,
///     /* food gain, 0..100% */ 0.,
///     /* spoil option */ Some(&DrinkableWaterSpoiling)
/// );
/// ```
#[macro_export]
macro_rules! inv_water(
    ($t:ty, $wg:expr, $fg:expr, $sp:ty) => (
        impl zara::inventory::items::ConsumableDescription for $t {
            fn is_food(&self) -> bool { false }
            fn is_water(&self) -> bool { true }
            fn water_gain_per_dose(&self) -> f32 { $wg as f32}
            fn food_gain_per_dose(&self) -> f32 { $fg as f32 }
            fn spoiling(&self) -> Option<&dyn zara::inventory::items::SpoilingBehavior> { $sp }
        }
    );
);

/// Macro for declaring a spoiling option
///
/// # Examples:
///
/// ```
/// zara::inv_spoil!(
///     MeatSpoilOption,
///     /* fresh poisoning chance, 0..100% probability */ 2,
///     /* spoiled poisoning chance, 0..100% probability */ 15,
///     /* spoil time */ GameTimeC::new(0,4,30,0.)
/// );
/// ```
#[macro_export]
macro_rules! inv_spoil(
    ($t:ty, $c1:expr, $c2:expr, $st:expr) => (
        impl zara::inventory::items::SpoilingBehavior for $t {
            fn fresh_poisoning_chance(&self) -> usize { $c1 as usize }
            fn spoil_poisoning_chance(&self) -> usize { $c2 as usize }
            fn spoil_time(&self) -> zara::utils::GameTimeC { $st }
        }
    );
);

/// Macro for declaring clothes description option
///
/// # Examples
///
/// ```
/// zara::inv_clothes!(
///     PantsClothes,
///     /* cold resistance, 0..100% */ 1.,
///     /* water resistance, 0..100% */ 14.
/// );
/// ```
#[macro_export]
macro_rules! inv_clothes(
    ($t:ty, $c1:expr, $c2:expr) => (
        impl zara::inventory::items::ClothesDescription for $t {
            fn cold_resistance(&self) -> usize { $c1 as usize }
            fn water_resistance(&self) -> usize { $c2 as usize }
        }
    );
);

/// Describes consumable contract
#[derive(Clone, Debug)]
pub struct ConsumableC {
    /// Unique name of the item
    pub name: String,
    /// Is this consumable a food
    pub is_food: bool,
    /// Is this consumable a water
    pub is_water: bool,
    /// How many items of this type has been consumed
    pub consumed_count: usize
}
impl ConsumableC {
    pub fn new() -> Self {
        ConsumableC {
            name: String::new(),
            is_food: false,
            is_water: false,
            consumed_count: 0
        }
    }
}

/// Describes appliance  contract
#[derive(Clone, Debug)]
pub struct ApplianceC {
    /// Unique name of the item
    pub name: String,
    /// Is this item is a body appliance (like bandage)
    pub is_body_appliance: bool,
    /// Is this item is an injection (like syringe with something)
    pub is_injection: bool,
    /// How many of these items has been applied
    pub taken_count: usize
}
impl ApplianceC {
    pub fn new() -> Self {
        ApplianceC {
            name: String::new(),
            is_body_appliance: false,
            is_injection: false,
            taken_count: 0
        }
    }
}

/// Trait that must be implemented by all inventory items
pub trait InventoryItem {
    /// Returns count of items of this kind in the inventory
    ///
    /// # Examples
    /// ```
    /// let n = item.get_count();
    /// ```
    fn get_count(&self) -> usize;

    /// Sets new count for items of this kind
    ///
    /// # Examples
    /// ```
    /// item.set_count(new_value);
    /// ```
    fn set_count(&mut self, new_count: usize);

    /// Gets unique name for all items of this kind
    ///
    /// # Examples
    /// ```
    /// let s = item.get_name();
    /// ```
    fn get_name(&self) -> String;

    /// Returns `true` is this item is an infinite resource
    ///
    /// # Examples
    /// ```
    /// let f = item.get_is_infinite();
    /// ```
    fn get_is_infinite(&self) -> bool;

    /// Gets calculated weight of all items of this kind, in grams.
    ///
    /// Most of the time, it is `count` * `weight per item`.
    ///
    /// # Examples
    /// ```
    /// let n = item.get_total_weight();
    /// ```
    fn get_total_weight(&self) -> f32;

    /// Node that describes behavior of this item as a consumable
    fn consumable(&self) -> Option<&dyn ConsumableDescription>;
    /// Node that describes behavior of this item as an appliance
    fn appliance(&self) -> Option<&dyn ApplianceDescription>;
    /// Node that describes clothes options for this item
    fn clothes(&self) -> Option<&dyn ClothesDescription>;
    /// For downcasting
    fn as_any(&self) -> &dyn Any;
}

/// Trait to describe appliance behavior of the inventory item
pub trait ApplianceDescription {
    /// True if this appliance is a body appliance (like bandage)
    fn is_body_appliance(&self) -> bool;
    /// True if this appliance is an injection type (like syringe with something)
    fn is_injection(&self) -> bool;
}

/// Trait to describe consumable behavior of the inventory item
pub trait ConsumableDescription {
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
    /// ([GameTimeC](crate::zara::utils::GameTimeC) structure)
    fn spoil_time(&self) -> GameTimeC;
}

/// Trait to describe clothes-related options of the item
pub trait ClothesDescription {
    /// Cold resistance value (0..100 scale)
    fn cold_resistance(&self) -> usize;
    /// Water resistance value (0..100 scale)
    fn water_resistance(&self) -> usize;
}