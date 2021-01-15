use super::utils::{FrameC, ConsumableC};
use super::utils::event::{Listener};
use super::health::disease::{DiseaseMonitor};

use std::cell::RefCell;
use std::rc::Rc;

pub mod disease;

/// Describes and controls player's health
pub struct Health {
    /// Stores all registered disease monitors
    monitors: Rc<RefCell<Vec<Box<dyn DiseaseMonitor>>>>
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
    pub fn update<E: Listener + 'static>(&self, frame: &mut FrameC<E>){
        println!("From health update: wind speed is {}", frame.data.environment.wind_speed);

        // Update disease monitors
        for monitor in self.monitors.borrow().iter() {
            monitor.check(self, &frame.data);
        }
    }

    /// Registers new disease monitor instance
    ///
    /// # Parameters
    /// - `monitor`: an instance of an object that implements [`DiseaseMonitor`](crate::zara::health::disease::DiseaseMonitor) trait
    pub fn register_disease_monitor(&self, monitor: Box<dyn DiseaseMonitor>){
        self.monitors.borrow_mut().insert(0, monitor);
    }

    /// Called by zara controller when item is consumed
    /// as food or water
    pub fn on_item_consumed(&self, item: &ConsumableC){
        println!("consumed {0} (from health): is food {1}", item.name, item.is_food);

        // Notify disease monitors
        for monitor in self.monitors.borrow().iter() {
            monitor.on_consumed(self, item);
        }
    }

    /// Spawns a new disease. If disease is already scheduled or active, nothing will happen
    pub fn spawn_disease(&self){
        println!("Spawn disease call");
    }
}