use crate::health::Health;
use crate::health::side::SideEffectsMonitor;
use crate::health::disease::DiseaseMonitor;
use crate::error::UnregisterMonitorErr;

impl Health {
    /// Registers new disease monitor instance
    ///
    /// # Parameters
    /// - `monitor`: an instance of an object that implements
    /// [`DiseaseMonitor`](crate::health::disease::DiseaseMonitor) trait
    ///
    /// # Returns
    /// `usize`: unique key of this registered instance
    pub fn register_disease_monitor(&self, monitor: Box<dyn DiseaseMonitor>) -> usize {
        let mut b = self.disease_monitors.borrow_mut();
        let key = b.keys().max().unwrap_or(&0) + 1;

        b.insert(key, monitor);

        return key;
    }

    /// Unregisters disease monitor
    ///
    /// # Parameters
    /// - `key`: unique key given as a result of a [`register_disease_monitor`] method.
    ///
    /// [`register_disease_monitor`]: #method.register_disease_monitor
    pub fn unregister_disease_monitor(&self, key: usize) -> Result<(), UnregisterMonitorErr> {
        let mut b = self.disease_monitors.borrow_mut();

        if !b.contains_key(&key)
        {
            return Err(UnregisterMonitorErr::MonitorIdNotFound);
        }

        b.remove(&key);

        return Ok(());
    }

    /// Registers new side effects monitor instance
    ///
    /// # Parameters
    /// - `monitor`: an instance of an object that implements
    ///
    /// # Returns
    /// `usize`: unique key of this registered instance
    pub fn register_side_effect_monitor(&self, monitor: Box<dyn SideEffectsMonitor>) -> usize {
        let mut b = self.side_effects.borrow_mut();
        let key = b.keys().max().unwrap_or(&0) + 1;

        b.insert(key, monitor);

        return key;
    }

    /// Unregisters side effects monitor
    ///
    /// # Parameters
    /// - `key`: unique key given as a result of a [`register_side_effect_monitor`] method.
    ///
    /// [`register_side_effect_monitor`]: #method.register_side_effect_monitor
    pub fn unregister_side_effect_monitor(&self, key: usize) -> Result<(), UnregisterMonitorErr> {
        let mut b = self.side_effects.borrow_mut();

        if !b.contains_key(&key)
        {
            return Err(UnregisterMonitorErr::MonitorIdNotFound);
        }

        b.remove(&key);

        return Ok(());
    }
}