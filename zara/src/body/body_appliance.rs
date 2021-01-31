use crate::body::{Body, BodyParts, BodyAppliance};

impl Body {
    pub(crate) fn on_body_appliance_put_on(&self, item_name: &String, body_part: BodyParts) {
        // All checks are done before that. This is just in case
        if self.is_applied(item_name, body_part) { return; }

        let mut b = self.appliances.borrow_mut();

        b.push(BodyAppliance {
            body_part,
            item_name: item_name.to_string()
        });
    }

    pub(crate) fn remove_appliance(&self, item_name: &String, body_part: BodyParts) -> bool {
        if !self.is_applied(item_name, body_part) { return false; }

        let mut b = self.appliances.borrow_mut();

        return match b.iter().position(|x| x.body_part == body_part && &x.item_name == item_name) {
            Some(ind) => {
                b.remove(ind); true
            },
            None => false
        }
    }

    pub(crate) fn is_applied(&self, item_name: &String, body_part: BodyParts) -> bool {
        for item in self.appliances.borrow().iter() {
            if &item.item_name == item_name && item.body_part == body_part { return true; }
        }

        return false;
    }
}