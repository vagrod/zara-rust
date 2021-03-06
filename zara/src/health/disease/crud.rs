use crate::utils::event::{Event, MessageQueue};
use crate::health::Health;
use crate::health::disease::{ActiveDisease, Disease};
use crate::utils::GameTimeC;
use crate::error::{SpawnDiseaseErr, RemoveDiseaseErr};

use std::rc::Rc;

impl Health {
    /// Spawns a new disease. If disease is already scheduled or active, nothing will happen, and
    /// `Err` will be returned
    ///
    /// # Parameters
    /// - `disease`: instance of an object with the [`Disease`](crate::health::disease::Disease) trait
    /// - `activation_time`: game time when this disease will activate. Use the
    ///     current game time to activate immediately (on the next `update` pass)
    ///
    /// # Returns
    /// Disease key on success
    /// 
    /// # Examples
    /// ```
    /// person.health.spawn_disease(disease, game_time);
    /// ```
    /// 
    /// # Links
    /// See [this wiki article](https://github.com/vagrod/zara-rust/wiki/Spawning-a-Disease) for more info.
    ///
    /// ## Notes
    /// This method borrows the `diseases` collection
    pub fn spawn_disease(&self, disease: Box<dyn Disease>, activation_time: GameTimeC)
                                                                    -> Result<String, SpawnDiseaseErr> {
        if !self.is_alive.get() { return Err(SpawnDiseaseErr::CharacterIsDead); }

        let mut b = self.diseases.borrow_mut();
        let disease_name = disease.get_name();

        if b.contains_key(&disease_name) {
            return Err(SpawnDiseaseErr::DiseaseAlreadyAdded);
        }

        self.queue_message(Event::DiseaseSpawned(disease_name.to_string()));

        b.insert(disease_name.to_string(), Rc::new(ActiveDisease::new(
            disease,
            activation_time
        )));

        Ok(disease_name)
    }

    /// Removes active disease if exists. Returns `Err` if not.
    ///
    /// # Parameters
    /// - `disease_name`: unique name of the disease
    ///
    /// # Returns
    /// Ok on success
    /// 
    /// # Examples
    /// ```
    /// person.health.remove_disease(disease_name);
    /// ```
    ///
    /// ## Notes
    /// This method borrows the `diseases` collection
    pub fn remove_disease(&self, disease_name: &String) -> Result<(), RemoveDiseaseErr> {
        if !self.is_alive.get() { return Err(RemoveDiseaseErr::CharacterIsDead); }

        let mut b = self.diseases.borrow_mut();

        if !b.contains_key(disease_name) {
            return Err(RemoveDiseaseErr::DiseaseNotFound);
        }

        b.remove(disease_name);

        self.queue_message(Event::DiseaseRemoved(disease_name.to_string()));

        Ok(())
    }
}