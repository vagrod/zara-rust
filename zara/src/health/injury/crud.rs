use crate::utils::event::{Event, MessageQueue};
use crate::health::{Health, InjuryKey};
use crate::health::injury::{ActiveInjury, Injury};
use crate::utils::GameTimeC;
use crate::error::{SpawnInjuryErr, RemoveInjuryErr};
use crate::body::BodyParts;

use std::rc::Rc;

impl Health {

    /// Spawns a new injury. If injury is already scheduled or active, nothing will happen, and
    /// `Err` will be returned
    ///
    /// # Parameters
    /// - `injury`: instance of an object with the [`Injury`](crate::health::injury::Injury) trait
    /// - `body_part`: body part associated with this injury
    /// - `activation_time`: game time when this injury will activate. Use the
    ///     current game time to activate immediately (on the next `update` pass)
    ///
    /// # Returns
    /// Ok on success
    ///
    /// # Notes
    /// This method borrows the `injuries` collection
    pub fn spawn_injury(&self, injury: Box<dyn Injury>, body_part: BodyParts, activation_time: GameTimeC)
                                                                    -> Result<(), SpawnInjuryErr> {
        if !self.is_alive.get() { return Err(SpawnInjuryErr::CharacterIsDead); }

        let mut b = self.injuries.borrow_mut();
        let injury_name = injury.get_name();
        let name_for_message= injury.get_name().to_string();
        let key = InjuryKey::new(injury_name, body_part);

        if b.contains_key(&key) {
            return Err(SpawnInjuryErr::InjuryAlreadyAdded);
        }

        b.insert(key, Rc::new(ActiveInjury::new(
            injury,
            body_part,
            activation_time
        )));

        self.queue_message(Event::InjurySpawned(name_for_message, body_part));

        return Ok(());
    }

    /// Removes active injury if exists. Returns `Err` if not.
    ///
    /// # Parameters
    /// - `injury_name`: unique name of the injury
    /// - `body_part`: body part to remove injury from
    ///
    /// # Returns
    /// Ok on success
    ///
    /// # Notes
    /// This method borrows the `injuries` collection
    pub fn remove_injury(&self, injury_name: String, body_part: BodyParts) -> Result<(), RemoveInjuryErr> {
        if !self.is_alive.get() { return Err(RemoveInjuryErr::CharacterIsDead); }

        let mut b = self.injuries.borrow_mut();
        let key = InjuryKey::new(injury_name, body_part);

        if !b.contains_key(&key) {
            return Err(RemoveInjuryErr::InjuryNotFound);
        }

        b.remove(&key);

        self.queue_message(Event::InjuryRemoved(key.injury.to_string(), key.body_part));

        return Ok(());
    }

}