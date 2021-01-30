use crate::inventory::Inventory;
use crate::utils::event::Listener;
use crate::utils::FrameC;

/// Contains `update` function

impl Inventory {

    /// This method is called every `UPDATE_INTERVAL` real seconds
    ///
    /// # Parameters
    /// - `frame`: summary information for this frame
    pub(crate) fn update<E: Listener + 'static>(&self, frame: &mut FrameC<E>) {
        // Check all inventory monitors
        for (_, monitor) in self.inventory_monitors.borrow().iter() {
            monitor.check(&self, &frame.data);
        }
    }

}