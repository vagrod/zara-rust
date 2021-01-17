use std::cell::Cell;

mod side_running;
mod side_fluctuate;

/// Side effects monitor that checks if player is running and increases his
/// heart rate, blood pressure, affects stamina, fatigue and water level
pub struct RunningSideEffects {
    running_state: Cell<bool>,
    running_time: Cell<f32> // game seconds
}

/// This side effect will make player's vitals slightly change back and forth over time
/// to make it look more interesting and alive
pub struct DynamicVitalsSideEffect {

}