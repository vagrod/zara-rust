use crate::body::Body;
use crate::utils::GameTimeC;

impl Body {
    /// Is player sleeping now
    pub fn is_sleeping(&self) -> bool { self.is_sleeping.get() }
    /// Last time slept (if any)
    pub fn last_sleep_time(&self) -> Option<GameTimeC> {
        match self.last_sleep_time.borrow().as_ref()
        {
            Some(t) => Some(t.copy()),
            _ => None
        }
    }
    /// Duration of the last sleep (zero if none)
    pub fn last_sleep_duration(&self) -> f64 { self.last_sleep_duration.get() }
}