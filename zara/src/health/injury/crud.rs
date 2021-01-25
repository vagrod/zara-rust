use crate::health::Health;
use crate::health::injury::{ActiveInjury, Injury};
use crate::utils::GameTimeC;
use crate::error::{SpawnInjuryErr, RemoveInjuryErr};

use std::rc::Rc;

/// Contains code related to injury handling

impl Health {

    /// Spawns a new injury. If injury is already scheduled or active, nothing will happen, and
    /// `Err` will be returned
    ///
    /// # Parameters
    /// - `injury`: instance of an object with the [`Injury`](crate::health::injury::Injury) trait
    /// - `activation_time`: game time when this injury will activate. Use the
    ///     current game time to activate immediately (on the next `update` pass)
    ///
    /// # Returns
    /// Ok on success
    ///
    /// # Notes
    /// This method borrows the `injuries` collection
    pub fn spawn_injury(&self, injury: Box<dyn Injury>, activation_time: GameTimeC)
                                                                    -> Result<(), SpawnInjuryErr> {
        if !self.is_alive.get() { return Err(SpawnInjuryErr::CharacterIsDead); }

        let mut b = self.injuries.borrow_mut();
        let injury_name = injury.get_name();

        if b.contains_key(&injury_name) {
            return Err(SpawnInjuryErr::InjuryAlreadyAdded);
        }

        b.insert(injury_name, Rc::new(ActiveInjury::new(
            injury,
            activation_time
        )));

        return Ok(());
    }

    /// Removes active injury if exists. Returns `Err` if not.
    ///
    /// # Parameters
    /// - `injury_name`: unique name of the injury
    ///
    /// # Returns
    /// Ok on success
    ///
    /// # Notes
    /// This method borrows the `injuries` collection
    pub fn remove_injury(&self, injury_name: &String) -> Result<(), RemoveInjuryErr> {
        if !self.is_alive.get() { return Err(RemoveInjuryErr::CharacterIsDead); }

        let mut b = self.injuries.borrow_mut();

        if !b.contains_key(injury_name) {
            return Err(RemoveInjuryErr::InjuryNotFound);
        }

        b.remove(injury_name);

        return Ok(());
    }

}