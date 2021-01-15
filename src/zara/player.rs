use std::cell::Cell;

/// Runtime player game state
pub struct PlayerStatus {
    /// Is player walking now
    pub is_walking: Cell<bool>
}

impl PlayerStatus {
    /// Creates an empty default player state
    pub fn empty() -> Self {
        PlayerStatus {
            is_walking: Cell::new(false)
        }
    }
}