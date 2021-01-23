use crate::health::Health;
use crate::health::disease::{ActiveDisease, Disease};
use crate::utils::GameTimeC;
use crate::error::{SpawnDiseaseErr, RemoveDiseaseErr};

use std::rc::Rc;

/// Contains code related to disease handling

impl Health {

    /// Spawns a new disease. If disease is already scheduled or active, nothing will happen, and
    /// `false` will be returned
    ///
    /// # Parameters
    /// - `disease`: instance of an object with the [`Disease`](crate::health::disease::Disease) trait
    /// - `activation_time`: game time when this disease will activate. Use the
    ///     current game time to activate immediately (on the next `update` pass)
    ///
    /// # Returns
    /// Ok on success.
    ///
    /// # Notes
    /// This method borrows the `diseases` collection
    pub fn spawn_disease(&self, disease: Box<dyn Disease>, activation_time: GameTimeC)
                                                                    -> Result<(), SpawnDiseaseErr> {
        let mut b = self.diseases.borrow_mut();
        let disease_name = disease.get_name();

        if b.contains_key(&disease_name) {
            return Err(SpawnDiseaseErr::DiseaseAlreadyAdded);
        }

        b.insert(disease_name, Rc::new(ActiveDisease::new(
            disease,
            activation_time
        )));

        return Ok(());
    }

    /// Removes active disease if exists. Returns `false` if not.
    ///
    /// # Parameters
    /// - `disease_name`: unique name of the disease
    ///
    /// # Returns
    /// Ok on success
    ///
    /// # Notes
    /// This method borrows the `diseases` collection
    pub fn remove_disease(&self, disease_name: &String) -> Result<(), RemoveDiseaseErr> {
        let mut b = self.diseases.borrow_mut();

        if !b.contains_key(disease_name) {
            return Err(RemoveDiseaseErr::DiseaseNotFound);
        }

        b.remove(disease_name);

        return Ok(());
    }

}