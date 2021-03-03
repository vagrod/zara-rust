use crate::inventory::Inventory;
use crate::utils::FrameSummaryC;
use crate::error::UnregisterMonitorErr;

use std::any::Any;

/// Trait for implementing the inventory monitor functionality
/// 
/// # Links
/// See [this wiki article](https://github.com/vagrod/zara-rust/wiki/Inventory-Monitors) for more info.
pub trait InventoryMonitor {
    /// Method is called once a `UPDATE_INTERVAL` real seconds.
    ///
    /// # Parameters
    /// - `inventory`: inventory controller object. It can be used to alter the inventory
    /// - `frame_data`: summary containing all environmental data, game time, health snapshot and etc.
    fn check(&self, inventory: &Inventory, frame_data: &FrameSummaryC);

    /// For downcasting
    fn as_any(&self) -> &dyn Any;
}

impl Inventory {
    /// Registers new inventory monitor instance
    ///
    /// # Parameters
    /// - `monitor`: an instance of an object that implements
    /// [`InventoryMonitor`](crate::inventory::monitors::InventoryMonitor) trait
    ///
    /// # Returns
    /// `usize`: unique key of this registered instance
    /// 
    /// # Examples
    /// ```
    /// let mid = person.inventory.register_monitor(boxed_monitor);
    /// ```
    /// 
    /// # Links
    /// See [this wiki article](https://github.com/vagrod/zara-rust/wiki/Inventory-Monitors) for more info.
    pub fn register_monitor(&self, monitor: Box<dyn InventoryMonitor>) -> usize {
        let mut b = self.inventory_monitors.borrow_mut();
        let key = b.keys().max().unwrap_or(&0) + 1;

        b.insert(key, monitor);

        key
    }

    /// Unregisters inventory monitor
    ///
    /// # Parameters
    /// - `key`: unique key given as a result of a [`register_monitor`] method.
    ///
    /// [`register_monitor`]: #method.register_monitor
    /// 
    /// # Examples
    /// ```
    /// person.inventory.unregister_monitor(mid);
    /// ```
    /// 
    /// # Links
    /// See [this wiki article](https://github.com/vagrod/zara-rust/wiki/Inventory-Monitors) for more info.
    pub fn unregister_monitor(&self, key: usize) -> Result<(), UnregisterMonitorErr> {
        let mut b = self.inventory_monitors.borrow_mut();

        if !b.contains_key(&key)
        {
            return Err(UnregisterMonitorErr::MonitorIdNotFound);
        }

        b.remove(&key);

        Ok(())
    }
}