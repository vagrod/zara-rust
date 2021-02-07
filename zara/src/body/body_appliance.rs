use crate::body::{Body, BodyPart, BodyAppliance};
use crate::utils::event::{MessageQueue, Event};

impl Body {
    pub(crate) fn on_body_appliance_put_on(&self, item_name: &String, body_part: BodyPart) {
        // All checks are done before that. This is just in case
        if self.is_applied(item_name, body_part) { return; }

        let mut b = self.appliances.borrow_mut();

        b.push(BodyAppliance {
            body_part,
            item_name: item_name.to_string()
        });

        self.queue_message(Event::BodyApplianceOn(item_name.to_string(), body_part));
    }

    pub(crate) fn remove_appliance(&self, item_name: &String, body_part: BodyPart) -> bool {
        if !self.is_applied(item_name, body_part) { return false; }

        let mut b = self.appliances.borrow_mut();

        return match b.iter().position(|x| x.body_part == body_part && &x.item_name == item_name) {
            Some(ind) => {
                b.remove(ind);

                self.queue_message(Event::BodyApplianceOff(item_name.to_string(), body_part));

                true
            },
            None => false
        }
    }

    pub(crate) fn is_applied(&self, item_name: &String, body_part: BodyPart) -> bool {
        for item in self.appliances.borrow().iter() {
            if &item.item_name == item_name && item.body_part == body_part { return true; }
        }

        return false;
    }
}