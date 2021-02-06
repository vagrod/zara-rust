use crate::health::Health;

pub struct HealthStateContract {

}

impl Health {
    pub(crate) fn get_state(&self) -> HealthStateContract {
        HealthStateContract {

        }
    }

    pub(crate) fn restore_state(&self, state: HealthStateContract) {

    }

}