use super::utils::{FrameC, GameTimeC};
use super::utils::event::{Listener, Event};

use std::cell::{Cell, RefCell};

pub struct Body {
    pub last_sleep_time: RefCell<Option<GameTimeC>>,
    pub is_sleeping: Cell<bool>,

    // Private fields
    sleeping_counter: Cell<f64>
}

impl Body {
    /// Creates new ready-to-use `Body`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use zara::body;
    ///
    /// let b = body::Body::new();
    /// ```
    pub fn new() -> Self {
        Body {
            last_sleep_time: RefCell::new(Option::None),
            is_sleeping: Cell::new(false),
            sleeping_counter: Cell::new(0.)
        }
    }

    /// This method is called every `UPDATE_INTERVAL` real seconds
    ///
    /// # Parameters
    /// - `frame`: summary information for this frame
    pub fn update<E: Listener + 'static>(&self, frame: &mut FrameC<E>){
        println!("From body update: game secs passed - {}", frame.data.game_time_delta);

        if self.is_sleeping.get(){
            let left = self.sleeping_counter.get() - frame.data.game_time_delta as f64;

            if left <= 0.
            {
                self.is_sleeping.set(false);
                self.sleeping_counter.set(0.);
                self.last_sleep_time.replace(Option::Some(frame.data.game_time.copy()));

                frame.events.dispatch(Event::WokeUp);
            } else {
                self.sleeping_counter.set(left);
            }
        }
    }

    /// Starts sleeping. `is_sleeping` will be set to `true`, and on wake up `WokeUp` event will
    /// be triggered
    ///
    /// # Parameters
    /// - `game_hours`: for how many game hours should player sleep
    pub fn start_sleeping(&self, game_hours: f64) -> bool {
        self.is_sleeping.set(true);
        self.sleeping_counter.set(game_hours * 60. * 60.);

        return true;
    }

}