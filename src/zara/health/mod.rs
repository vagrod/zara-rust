use super::utils::{FrameC};
use crate::utils::event::{Listener};
use crate::utils::ConsumableC;
use crate::health::disease::DiseaseMonitor;

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

pub mod disease;

/// Describes and controls player's health
pub struct Health {
    pub monitors: Rc<RefCell<Vec<Box<dyn DiseaseMonitor>>>>
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
            monitors: Rc::new(RefCell::new(Vec::new()))
        }
    }

    /// This method is called every `UPDATE_INTERVAL` real seconds
    ///
    /// # Parameters
    /// - `frame`: summary information for this frame
    ///
    /// # Note
    /// Borrows `monitors` collection
    pub fn update<E: Listener + 'static>(&self, frame: &mut FrameC<E>){
        println!("From health update: wind speed is {}", frame.data.wind_speed);

        for monitor in self.monitors.borrow().iter() {
            monitor.check(self, frame.data.game_time_delta, &frame.data.game_time);
        }
    }

    /// Called by zara controller when item is consumed
    /// as food or water
    pub fn on_item_consumed(&self, item: &ConsumableC){
        println!("consumed {0} (from health): is food {1}", item.name, item.is_food);
    }

    pub fn spawn_disease(&self){
        println!("Spawn disease call");
    }
}