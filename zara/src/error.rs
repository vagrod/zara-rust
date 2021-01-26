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
    NoActiveStageAtGivenTime
}

/// Is used by `Health.spawn_disease` method
pub enum SpawnDiseaseErr {
    /// When `spawn_disease` called on a dead character
    CharacterIsDead,
    /// When disease you trying to spawn was already spawned
    DiseaseAlreadyAdded
}

/// Is used by `Health.spawn_injury` method
pub enum SpawnInjuryErr {
    /// When `spawn_injury` called on a dead character
    CharacterIsDead,
    /// When injury you trying to spawn was already spawned on this body part
    InjuryAlreadyAdded
}

/// Is used by `Health.remove_disease` method
pub enum RemoveDiseaseErr {
    /// When `spawn_disease` called on a dead character
    CharacterIsDead,
    /// When disease you trying to delete was not found
    DiseaseNotFound
}

/// Is used by `Health.remove_injury` method
pub enum RemoveInjuryErr {
    /// When `remove_injury` called on a dead character
    CharacterIsDead,
    /// When injury you trying to delete was not found
    InjuryNotFound
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
    /// When `consume` called on a dead character
    CharacterIsDead,
    /// When given item key was not found in the inventory
    ItemNotFound,
    /// When item `count` is not enough
    NotEnoughResources,
    /// When item has no `consumable` option
    ItemIsNotConsumable,
    /// When could not update item count
    CouldNotUpdateItemCount(InventoryItemAccessErr)
}

/// Is used by `ZaraController.take_appliance` method
pub enum ApplianceTakeErr {
    /// When `consume` called on a dead character
    CharacterIsDead,
    /// When given item key was not found in the inventory
    ItemNotFound,
    /// When item `count` is not enough
    NotEnoughResources,
    /// When item has no `appliance` option
    ItemIsNotAppliance,
    /// When passed body part is unknown
    UnknownBodyPart,
    /// When could not update item count
    CouldNotUpdateItemCount(InventoryItemAccessErr)
}

/// Is used by `ZaraController.update` method
pub enum ZaraUpdateErr {
    /// When `update` called on a dead character
    CharacterIsDead
}