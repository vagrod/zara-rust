/// Is used by `ActiveDisease/ActiveInjury.invert()` method
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

/// Is used by `ActiveDisease/ActiveInjury.invert_back()` method
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

/// Is used by `Inventory.remove_item` methods
pub enum InventoryItemAccessErr {
    /// When given item key was not found in the inventory
    ItemNotFound
}

/// Is used by `Inventory.use_item` methods
pub enum InventoryUseErr {
    /// When given item key was not found in the inventory
    ItemNotFound,
    /// When requested amount is greater that the actual items count
    InsufficientResources
}

/// Is used by `ZaraController.consume` method
pub enum ItemConsumeErr {
    /// When `consume` called on a dead character
    CharacterIsDead,
    /// When given item key was not found in the inventory
    ItemNotFound,
    /// When item `count` is not enough
    InsufficientResources,
    /// When item has no `consumable` option
    ItemIsNotConsumable,
    /// When could not update item count
    CouldNotUseItem(InventoryUseErr)
}

/// Is used by `ZaraController.take_appliance` method
pub enum ApplianceTakeErr {
    /// When `consume` called on a dead character
    CharacterIsDead,
    /// When given item key was not found in the inventory
    ItemNotFound,
    /// When item `count` is not enough
    InsufficientResources,
    /// When item has no `appliance` option
    ItemIsNotAppliance,
    /// When passed body part is unknown
    UnknownBodyPart,
    /// When could not update item count
    CouldNotUseItem(InventoryUseErr),
    /// Is this kind of body appliance already applied to a given body part
    AlreadyApplied
}

/// Is used by `ZaraController.remove_appliance` method
pub enum ApplianceRemoveErr {
    /// When `consume` called on a dead character
    CharacterIsDead,
    /// When given appliance kind is not found on a body part
    ApplianceNotFound
}

/// Is used by `ZaraController.update` method
pub enum ZaraUpdateErr {
    /// When `update` called on a dead character
    CharacterIsDead
}

/// Is used by `MedicalAgentsMonitor.is_active` method
pub enum MedicalAgentErr {
    /// When given medical agent key was not found
    AgentNotFound
}

/// Is used by `ZaraController.put_on_clothes` method
pub enum ClothesOnActionErr {
    /// When given item key was not found
    ItemNotFound,
    /// When item count is zero
    InsufficientResources,
    /// When given clothes is already on
    AlreadyHaveThisItemOn,
    /// When given item has no `clothes` option
    IsNotClothesType
}

/// Is used by `ZaraController.take_off_clothes` method
pub enum ClothesOffActionErr {
    /// When given item key was not found
    ItemNotFound,
    /// When item count is zero
    InsufficientResources,
    /// When given clothes is not on
    ItemIsNotOn,
    /// When given item has no `clothes` option
    IsNotClothesType
}

pub(crate) enum RequestClothesOnErr {
    AlreadyHaveThisItemOn
}

pub(crate) enum RequestClothesOffErr {
    ItemIsNotOn
}

/// Is used by `Inventory.check_for_resources` method
pub enum CheckForResourcesErr {
    /// When given combination key was not found
    CombinationNotFound,
    /// When a particular item in a combination recipe is not found in the inventory
    ItemNotFound(String),
    /// When a particular item in a combination recipe count is less that the count needed
    /// for this combination to be executed
    InsufficientResources(String)
}

/// Is used by `Inventory.execute_combination` method
pub enum CombinationExecuteErr {
    /// When resources check failed
    ResourceError(CheckForResourcesErr),
    /// When failed to properly use an item (count mismatch for example)
    UseItemError(InventoryUseErr),
    /// When given combination key was not found
    CombinationNotFound
}