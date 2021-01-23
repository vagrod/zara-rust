/// Initially by user `locka` as an answer to this stackoverflow question:
/// https://stackoverflow.com/questions/37572734/how-can-i-implement-the-observer-pattern-in-rust

use std::sync::{Arc, Weak};
use std::cell::RefCell;
use super::ConsumableC;

pub enum Event {
    WokeUp,
    ItemConsumed(ConsumableC),
    StaminaDrained,
    Tired,
    Exhausted
}

pub trait Listener {
    fn notify(&mut self, event: &Event);
}

pub trait Dispatchable<T>
    where T: Listener
{
    fn register_listener(&mut self, listener: Arc<RefCell<T>>);
}

pub struct Dispatcher<T>
    where T: Listener
{
    /// A list of synchronous weak refs to listeners
    listeners: Vec<Weak<RefCell<T>>>
}

impl<T> Dispatchable<T> for Dispatcher<T>
    where T: Listener
{
    /// Registers a new listener
    fn register_listener(&mut self, listener: Arc<RefCell<T>>) {
        self.listeners.push(Arc::downgrade(&listener));
    }
}

impl<T> Dispatcher<T>
    where T: Listener
{
    pub fn new() -> Dispatcher<T> {
        Dispatcher { listeners: Vec::new() }
    }

    pub fn num_listeners(&self) -> usize {
        self.listeners.len()
    }

    pub fn dispatch(&mut self, event: Event) {
        let mut cleanup = false;
        // Call the listeners
        for l in self.listeners.iter() {
            if let Some(listener_rc) = l.upgrade() {
                let mut listener = listener_rc.borrow_mut();
                listener.notify(&event);
            } else {
                println!("Cannot get listener, cleanup necessary");
                cleanup = true;
            }
        }
        // If there were invalid weak refs, clean up the list
        if cleanup {
            println!("Dispatcher is cleaning up weak refs");
            self.listeners.retain(|ref l| {
                // Only retain valid weak refs
                let got_ref = l.clone().upgrade();
                match got_ref {
                    None => false,
                    _ => true,
                }
            });
        }
    }
}