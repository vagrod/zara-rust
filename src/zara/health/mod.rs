use super::utils::{FrameC};
use crate::utils::event::{Listener};
use crate::utils::ConsumableC;

pub mod disease;

/// Describes and controls player's health
pub struct Health {

}

impl Health {
    /// Creates new ready-to-use `Health`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use zara::health;
    ///
    /// let h = health::Health::new();
    /// ```
    pub fn new() -> Self {
        Health {

        }
    }

    /// This method is called every `UPDATE_INTERVAL` real seconds
    ///
    /// # Parameters
    /// - `frame`: summary information for this frame
    pub fn update<E: Listener + 'static>(&self, frame: &mut FrameC<E>){
        println!("From health update: wind speed is {}", frame.data.wind_speed);
    }

    /// Called by zara controller when item is consumed
    /// as food or water
    pub fn on_item_consumed(&self, item: &ConsumableC){
        println!("consumed {0} (from health): is food {1}", item.name, item.is_food);
    }
}