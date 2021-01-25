use crate::utils::{FrameC, GameTimeC};
use crate::utils::event::{Dispatcher, Listener, Event, MessageQueue};

use std::cell::{Cell, RefCell, RefMut};
use std::time::Duration;
use std::collections::BTreeMap;

pub struct Body {
    /// Game time when player slept last time
    pub last_sleep_time: RefCell<Option<GameTimeC>>,
    /// For how long player slept last time (game hours)
    pub last_sleep_duration: Cell<f64>,
    /// Is player sleeping now
    pub is_sleeping: Cell<bool>,

    // Private fields
    sleeping_counter: Cell<f64>,
    /// Messages queued for sending on the next frame
    message_queue: RefCell<BTreeMap<usize, Event>>
}

/// All body parts enum
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum BodyParts {
    Unknown = -1,
    Forehead = 0,
    Nape = 1,
    Eye = 2,
    Ear = 3,
    Nose = 4,
    Throat = 5,
    LeftShoulder = 6,
    RightShoulder = 7,
    LeftForearm = 8,
    RightForearm = 9,
    LeftSpokebone = 10,
    RightSpokebone = 11,
    LeftBrush = 12,
    RightBrush = 13,
    LeftChest = 14,
    RightChest = 15,
    Belly = 16,
    LeftHip = 17,
    RightHip = 18,
    LeftKnee = 19,
    RightKnee = 20,
    LeftShin = 21,
    RightShin = 22,
    LeftFoot = 23,
    RightFoot = 24,
    Back = 25
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
            sleeping_counter: Cell::new(0.),
            last_sleep_duration: Cell::new(0.),
            message_queue: RefCell::new(BTreeMap::new())
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
        self.last_sleep_duration.set(game_hours);

        return true;
    }
}

impl MessageQueue for Body {
    fn queue_message(&self, message: Event) {
        let mut q = self.message_queue.borrow_mut();
        let id = q.len();

        q.insert(id, message);
    }

    fn get_message_queue(&self) -> RefMut<'_, BTreeMap<usize, Event>> {
        self.message_queue.borrow_mut()
    }
}