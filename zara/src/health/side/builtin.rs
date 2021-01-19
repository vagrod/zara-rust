use std::cell::Cell;

mod side_running;
mod side_fluctuate;
mod side_fatigue;
mod side_food_drain;
mod side_water_drain;

/// Side effects monitor that checks if player is running and increases his
/// heart rate, blood pressure, affects stamina, fatigue and water level
pub struct RunningSideEffects {
    running_state: Cell<bool>,
    sleeping_state: Cell<bool>,
    running_time: Cell<f32>, // game seconds
    gained_fatigue: Cell<f32>
}

/// This side effect will make player's vitals slightly change back and forth over time
/// to make it look more interesting and alive
pub struct DynamicVitalsSideEffect {
    first_iteration: Cell<bool>,
    counter: Cell<f32>,
    half_duration: Cell<f32>,
    direction: Cell<f32>,
    body_temperature_ceiling: Cell<f32>,
    heart_rate_ceiling: Cell<f32>,
    top_pressure_ceiling: Cell<f32>,
    bottom_pressure_ceiling: Cell<f32>
}

/// Side effects monitor that will increase fatigue level according to the player's last sleep time
/// and last sleep duration. Fatigue value will be calculated as "current fatigue"+"left fatigue"
pub struct FatigueSideEffects {
    hours_until_exhausted: Cell<usize>
}

/// Will enable food drain over time
pub struct FoodDrainOverTimeSideEffect {
    /// Drain speed, 0..100 percents per game second
    drain_amount: Cell<f32>
}

/// Will enable water drain over time
pub struct WaterDrainOverTimeSideEffect {
    /// Drain speed, 0..100 percents per game second
    drain_amount: Cell<f32>
}