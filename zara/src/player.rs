use std::cell::Cell;
use std::fmt;
use std::hash::{Hash, Hasher};

/// Runtime player game state. You can change any of its values at any time
/// to give Zara up-to-date information on player's status
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub struct PlayerStatus {
    /// Is player walking now
    pub is_walking: Cell<bool>,
    /// Is player running now
    pub is_running: Cell<bool>,
    /// Is player swimming now
    pub is_swimming: Cell<bool>,
    /// Is player under the water now
    pub is_underwater: Cell<bool>
}
impl fmt::Display for PlayerStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Player status")
    }
}
impl Hash for PlayerStatus {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.is_walking.get().hash(state);
        self.is_running.get().hash(state);
        self.is_swimming.get().hash(state);
        self.is_underwater.get().hash(state);
    }
}
impl PlayerStatus {
    /// Creates an empty default player state
    pub fn empty() -> Self {
        PlayerStatus {
            is_walking: Cell::new(false),
            is_running: Cell::new(false),
            is_swimming: Cell::new(false),
            is_underwater: Cell::new(false)
        }
    }
}