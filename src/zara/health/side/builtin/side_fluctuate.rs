use crate::health::side::{SideEffectsMonitor, SideEffectDeltasC};
use crate::utils::{FrameSummaryC};
use crate::health::side::builtin::DynamicVitalsSideEffect;

/// DynamicVitalsSideEffect implementation

impl DynamicVitalsSideEffect {
    pub fn new() -> Self {
        DynamicVitalsSideEffect {

        }
    }
}
impl SideEffectsMonitor for DynamicVitalsSideEffect {
    fn check(&self, _frame_data: &FrameSummaryC) -> SideEffectDeltasC {
        SideEffectDeltasC {
            body_temp_bonus: 0.,
            ..Default::default()
        }
    }
}