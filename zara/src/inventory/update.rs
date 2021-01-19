use crate::inventory::Inventory;
use crate::utils::event::Listener;
use crate::utils::FrameC;

/// Contains `update` function

impl Inventory {

    /// This method is called every `UPDATE_INTERVAL` real seconds
    ///
    /// # Parameters
    /// - `frame`: summary information for this frame
    pub fn update<E: Listener + 'static>(&self, _frame: &mut FrameC<E>) {

    }

}