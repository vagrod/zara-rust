use zara::utils::event::{Event, Listener};

pub struct ZaraEventsListener;
impl Listener for ZaraEventsListener {
    fn notify(&mut self, event: &Event) {
        match event {
            _ => {
                //println!("{:?}", event)
            }
        }
    }
}