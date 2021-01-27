use crate::ZaraController;
use crate::utils::event::Listener;

impl<E: Listener + 'static> ZaraController<E> {

    /// State of this character
    pub fn is_alive(&self) -> bool { self.health.is_alive() }

}