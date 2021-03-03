/// Initially by user `locka` as an answer to this stackoverflow question:
/// https://stackoverflow.com/questions/37572734/how-can-i-implement-the-observer-pattern-in-rust

use crate::inventory::items::{ConsumableC, ApplianceC};
use crate::body::BodyPart;

use std::sync::{Arc, Weak};
use std::cell::{RefCell, RefMut};
use std::collections::BTreeMap;
use std::fmt;

pub(crate) trait MessageQueue {
    fn has_messages(&self) -> bool;
    fn queue_message(&self, message: Event);
    fn get_message_queue(&self) -> RefMut<'_, BTreeMap<usize, Event>>;
}

/// All Zara public events
/// 
/// # Links
/// See [this wiki article](https://github.com/vagrod/zara-rust/wiki/Game-events) for more info.
#[derive(Clone, Debug)]
pub enum Event {
    /// When sleep is started.
    /// # Parameters
    /// - Duration, in game hours
    SleepStarted(f32),
    /// When woke up
    WokeUp,

    /// When stamina level is less than 5%
    StaminaDrained,
    /// When oxygen level is less than 5%
    OxygenDrained,
    /// When blood level is less than 5%
    BloodDrained,
    /// When food level is less than 5%
    FoodDrained,
    /// When water level is less than 5%
    WaterDrained,

    /// When fatigue level is more than 70%
    Tired,
    /// When fatigue level is more than 90%
    Exhausted,

    /// When medical agent is getting activated
    /// # Parameters
    /// - Medical agent unique name
    MedicalAgentActivated(String),
    /// When medical agent is getting deactivated
    /// # Parameters
    /// - Medical agent unique name
    MedicalAgentDeactivated(String),
    /// When medical agent receives a new dose
    /// # Parameters
    /// - Medical agent unique name
    /// - Appliance item unique name
    MedicalAgentDoseReceived(String, String),

    /// When body appliance is put on
    /// # Parameters
    /// - Appliance item unique name
    /// - Body part
    BodyApplianceOn(String, BodyPart),
    /// When body appliance is taken off
    /// # Parameters
    /// - Appliance item unique name
    /// - Body part
    BodyApplianceOff(String, BodyPart),
    /// When clothes item is put on
    /// # Parameters
    /// - Clothes item unique name
    ClothesOn(String),
    /// When clothes item is taken off
    /// # Parameters
    /// - Clothes item unique name
    ClothesOff(String),

    /// When disease stage death chance is satisfied
    /// # Parameters
    /// - Disease unique name
    DeathFromDisease(String),
    /// When injury stage death chance is satisfied
    /// # Parameters
    /// - Injury unique name
    /// - Body part
    DeathFromInjury(String, BodyPart),

    /// When disease is spawned or scheduled
    /// # Parameters
    /// - Unique disease name
    DiseaseSpawned(String),
    /// When disease is removed
    /// # Parameters
    /// - Unique disease name
    DiseaseRemoved(String),
    /// When disease starts self-healing process
    /// # Parameters
    /// - Unique disease name
    DiseaseSelfHealStarted(String),
    /// When disease chain is inverted
    /// # Parameters
    /// - Unique disease name
    DiseaseInverted(String),
    /// When disease chain is inverted back
    /// # Parameters
    /// - Unique disease name
    DiseaseResumed(String),
    /// When disease passed its lifetime
    /// # Parameters
    /// - Unique disease name
    DiseaseExpired(String),

    /// When injury is spawned or scheduled
    /// # Parameters
    /// - Unique injury name
    /// - Body part
    InjurySpawned(String, BodyPart),
    /// When injury is removed
    /// # Parameters
    /// - Unique injury name
    /// - Body part
    InjuryRemoved(String, BodyPart),
    /// When injury starts self-healing process
    /// # Parameters
    /// - Unique injury name
    /// - Body part
    InjurySelfHealStarted(String, BodyPart),
    /// When injury chain is inverted
    /// # Parameters
    /// - Unique injury name
    /// - Body part
    InjuryInverted(String, BodyPart),
    /// When injury chain is inverted back
    /// # Parameters
    /// - Unique injury name
    /// - Body part
    InjuryResumed(String, BodyPart),
    /// When injury passed its lifetime
    /// # Parameters
    /// - Unique injury name
    /// - Body part
    InjuryExpired(String, BodyPart),
    /// When injury blood loss forcibly stopped
    /// # Parameters
    /// - Unique injury name
    /// - Body part
    BloodLossStopped(String, BodyPart),
    /// When injury blood loss forcibly resumed
    /// # Parameters
    /// - Unique injury name
    /// - Body part
    BloodLossResumed(String, BodyPart),

    /// When item is consumed
    /// # Parameters
    /// - Consumable option description
    ItemConsumed(ConsumableC),
    /// When appliance is taken (like injection or bandage)
    /// # Parameters
    /// - Appliance option description
    /// - Body part
    ApplianceTaken(ApplianceC, BodyPart),

    /// When inventory item is added
    /// # Parameters
    /// - Item unique name
    InventoryItemAdded(String),
    /// When inventory item is removed
    /// # Parameters
    /// - Item unique name
    InventoryItemRemoved(String),
    /// When inventory crafting combination successfully executed
    /// # Parameters
    /// - Combination unique key
    CraftingCombinationExecuted(String),
    /// When inventory weight has changed
    /// # Parameters
    /// - Old weight value (grams)
    /// - New weight value (grams)
    InventoryWeightChanged(f32, f32),
    /// When inventory item is used (wasted) completely and removed from the inventory
    /// # Parameters
    /// - Unique item name
    /// - Amount of items of this kind used
    InventoryItemUsedAll(String, usize),
    /// When inventory item is used (wasted) partially
    /// # Parameters
    /// - Unique item name
    /// - Amount of items of this kind used
    InventoryItemUsedPartially(String, usize),

    /// When blood pressure is too high
    HighBloodPressureDanger,
    /// When blood pressure is too low
    LowBloodPressureDanger,
    /// When heart rate is too high
    HighHeartRateDanger,
    /// When heart rate is too low
    LowHeartRateDanger,
    /// When body temperature is too high
    HighBodyTemperatureDanger,
    /// When body temperature is too low
    LowBodyTemperatureDanger,
    /// When character forcibly declared dead
    DeclaredDead
}
impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Zara game events listener trait
pub trait Listener {
    fn notify(&mut self, event: &Event);
}

/// Zara game events dispatcher trait
pub trait Dispatchable<T>
    where T: Listener
{
    fn register_listener(&mut self, listener: Arc<RefCell<T>>);
}

/// Zara events dispatcher object
pub struct Dispatcher<T>
    where T: Listener
{
    /// A list of synchronous weak refs to listeners
    listeners: Vec<Weak<RefCell<T>>>
}

impl<T> Dispatchable<T> for Dispatcher<T>
    where T: Listener
{
    /// Registers a new listener
    fn register_listener(&mut self, listener: Arc<RefCell<T>>) {
        self.listeners.push(Arc::downgrade(&listener));
    }
}

impl<T> Dispatcher<T>
    where T: Listener
{
    /// Creates new instance of the `Dispatcher`
    /// 
    /// # Examples
    /// ```
    /// use zara::utils;
    /// 
    /// let o = utils::Dispatcher::new();
    /// ```
    pub fn new() -> Dispatcher<T> {
        Dispatcher { listeners: Vec::new() }
    }

    /// Returns count of active listeners
    /// 
    /// # Examples
    /// ```
    /// let value = dispatcher.num_listeners();
    /// ```
    pub fn num_listeners(&self) -> usize {
        self.listeners.len()
    }

    /// Dispatches a message to all active listeners
    /// 
    /// # Examples
    /// ```
    /// dispatcher.dispatch(event);
    /// ```
    pub fn dispatch(&mut self, event: Event) {
        let mut cleanup = false;
        // Call the listeners
        for l in self.listeners.iter() {
            if let Some(listener_rc) = l.upgrade() {
                let mut listener = listener_rc.borrow_mut();
                listener.notify(&event);
            } else {
                println!("Cannot get listener, cleanup necessary");
                cleanup = true;
            }
        }
        // If there were invalid weak refs, clean up the list
        if cleanup {
            println!("Dispatcher is cleaning up weak refs");
            self.listeners.retain(|ref l| {
                // Only retain valid weak refs
                let got_ref = l.clone().upgrade();
                match got_ref {
                    None => false,
                    _ => true,
                }
            });
        }
    }
}