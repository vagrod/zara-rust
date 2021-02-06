use crate::body::Body;

pub struct BodyStateContract {

}

impl Body {
    pub(crate) fn get_state(&self) -> BodyStateContract {
        BodyStateContract {

        }
    }

    pub(crate) fn restore_state(&self, state: BodyStateContract) {

    }
}