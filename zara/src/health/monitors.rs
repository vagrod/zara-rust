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
    /// 
    /// # Examples
    /// ```
    /// let mid = person.health.register_disease_monitor(boxed_monitor);
    /// ```
    /// 
    /// # Links
    /// See [this wiki article](https://github.com/vagrod/zara-rust/wiki/Disease-Monitors) for more info.
    pub fn register_disease_monitor(&self, monitor: Box<dyn DiseaseMonitor>) -> usize {
        let mut b = self.disease_monitors.borrow_mut();
        let key = b.keys().max().unwrap_or(&0) + 1;

        b.insert(key, monitor);

        key
    }

    /// Unregisters disease monitor
    ///
    /// # Parameters
    /// - `key`: unique key given as a result of a [`register_disease_monitor`] method.
    ///
    /// [`register_disease_monitor`]: #method.register_disease_monitor
    /// 
    /// # Examples
    /// ```
    /// let result = person.health.unregister_disease_monitor(mid);
    /// ```
    /// 
    /// # Links
    /// See [this wiki article](https://github.com/vagrod/zara-rust/wiki/Disease-Monitors) for more info.
    pub fn unregister_disease_monitor(&self, key: usize) -> Result<(), UnregisterMonitorErr> {
        let mut b = self.disease_monitors.borrow_mut();

        if !b.contains_key(&key)
        {
            return Err(UnregisterMonitorErr::MonitorIdNotFound);
        }

        b.remove(&key);

        Ok(())
    }

    /// Registers new side effects monitor instance
    ///
    /// # Parameters
    /// - `monitor`: an instance of an object that implements
    ///
    /// # Returns
    /// `usize`: unique key of this registered instance
    /// 
    /// # Examples
    /// ```
    /// let mid = person.health.register_side_effect_monitor(Box::new(RunningMonitor::new()));
    /// ```
    /// 
    /// # Links
    /// See [this wiki article](https://github.com/vagrod/zara-rust/wiki/Side-effects-Monitors) for more info.
    pub fn register_side_effect_monitor(&self, monitor: Box<dyn SideEffectsMonitor>) -> usize {
        let mut b = self.side_effects.borrow_mut();
        let key = b.keys().max().unwrap_or(&0) + 1;

        b.insert(key, monitor);

        key
    }

    /// Unregisters side effects monitor
    ///
    /// # Parameters
    /// - `key`: unique key given as a result of a [`register_side_effect_monitor`] method.
    ///
    /// [`register_side_effect_monitor`]: #method.register_side_effect_monitor
    /// 
    /// # Examples
    /// ```
    /// let result = person.health.unregister_side_effect_monitor(mid);
    /// ```
    /// 
    /// # Links
    /// See [this wiki article](https://github.com/vagrod/zara-rust/wiki/Side-effects-Monitors) for more info.
    pub fn unregister_side_effect_monitor(&self, key: usize) -> Result<(), UnregisterMonitorErr> {
        let mut b = self.side_effects.borrow_mut();

        if !b.contains_key(&key)
        {
            return Err(UnregisterMonitorErr::MonitorIdNotFound);
        }

        b.remove(&key);

        Ok(())
    }
}