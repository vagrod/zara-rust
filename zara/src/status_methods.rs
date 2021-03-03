use crate::ZaraController;
use crate::utils::event::Listener;

impl<E: Listener + 'static> ZaraController<E> {
    /// State of this character
    /// 
    /// # Examples
    /// ```
    /// let value = person.is_alive();
    /// ```
    /// 
    /// # Links
    /// See [this wiki article](https://github.com/vagrod/zara-rust/wiki/Declare-dead) for more info.
    pub fn is_alive(&self) -> bool { self.health.is_alive() }

    /// Is this instance paused (all `update` calls are ignored)
    /// 
    /// # Examples
    /// ```
    /// let value = person.is_paused();
    /// ```
    /// 
    /// # Links
    /// See [this wiki article](https://github.com/vagrod/zara-rust/wiki/Pausing-Zara) for more info.
    pub fn is_paused(&self) -> bool{ self.is_paused.get() }
}