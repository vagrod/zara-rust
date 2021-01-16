use std::cell::Cell;

/// Runtime player game state
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