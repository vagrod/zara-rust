use super::utils::{FrameC, GameTimeC};
use super::utils::event::{Dispatcher, Listener, Event};

use std::cell::{Cell, RefCell};
use std::time::Duration;

pub struct Body {
    /// Game time when player slept last time
    pub last_sleep_time: RefCell<Option<GameTimeC>>,
    /// Is player sleeping now
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
    pub fn update<E: Listener + 'static>(&self, _frame: &mut FrameC<E>){

    }

    /// Is called every frame by Zara controller.
    /// Cannot be called in `update` because we need time precision
    pub fn sleep_check<E: Listener + 'static>
            (&self, events: &mut Dispatcher<E>, game_time: &Duration, game_time_delta: f32) {
        if self.is_sleeping.get(){
            let left = self.sleeping_counter.get() - game_time_delta as f64;

            if left <= 0.
            {
                self.is_sleeping.set(false);
                self.sleeping_counter.set(0.);
                self.last_sleep_time.replace(Option::Some(GameTimeC::from_duration(*game_time)));

                events.dispatch(Event::WokeUp);
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