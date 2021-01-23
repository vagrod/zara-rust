/// Is used by `ActiveDisease.invert()` method
pub enum ChainInvertErr {
    /// When calling `invert()` on already inverted chain
    AlreadyInverted,
    /// When calling `invert()` with time that is outside of disease
    /// active time
    DiseaseNotActiveAtGivenTime,
    /// When calling `invert()` with time that cannot be mapped to any
    /// active stage
    NoActiveStageAtGivenTime
}

/// Is used by `ActiveDisease.invert_back()` method
pub enum ChainInvertBackErr {
    /// When calling `invert_back()` on already inverted back chain
    AlreadyInvertedBack,
    /// When calling `invert_back()` with time that is outside of disease
    /// active time
    DiseaseNotActiveAtGivenTime,
    /// When calling `invert_back()` with time that cannot be mapped to any
    /// active stage
    NoActiveStageAtGivenTime,
    /// When calling `invert_back()` with time that points to the `HealthyStage` as active stage
    CannotInvertBackWhenOnHealthyStage
}

/// Is used by `Health.spawn_disease` method
pub enum SpawnDiseaseErr {
    /// When disease you trying to spawn was already spawned
    DiseaseAlreadyAdded
}

/// Is used by `Health.remove_disease` method
pub enum RemoveDiseaseErr {
    /// When disease you trying to delete was not found
    DiseaseNotFound
}

/// Is used by `Health.unregister_disease_monitor`, `unregister_side_effect_monitor`,
/// `Inventory.unregister_monitor` methods
pub enum UnregisterMonitorErr {
    /// When trying to unregister the monitor which id is nt registered
    MonitorIdNotFound
}

/// Is used by `Inventory.change_item_count`, `remove_item` methods
pub enum InventoryItemAccessErr {
    /// When given item key was not found in the inventory
    ItemNotFound
}

/// Is used by `ZaraController.consume` method
pub enum ItemConsumeErr {
    /// When given item key was not found in the inventory
    ItemNotFound,
    /// When item `count` is not enough
    NotEnoughResources,
    /// When item has no `consumable` option
    ItemIsNotConsumable
}