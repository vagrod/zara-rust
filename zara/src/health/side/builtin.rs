use std::cell::Cell;

mod side_running;
mod side_fluctuate;
mod side_fatigue;
mod side_food_drain;
mod side_water_drain;
mod side_underwater;

/// Side effects monitor that checks if player is running and increases his
/// heart rate, blood pressure, affects stamina, fatigue and water level
#[derive(Debug, Clone)]
pub struct RunningSideEffects {
    /// Stamina drain speed, 0..100 percents per game second
    stamina_drain_amount: Cell<f32>,
    /// Water level drain speed, 0..100 percents per game second
    water_drain_amount: Cell<f32>,

    running_state: Cell<bool>,
    sleeping_state: Cell<bool>,
    running_time: Cell<f32>, // game seconds
    gained_fatigue: Cell<f32>
}

/// This side effect will make player's vitals slightly change back and forth over time
/// to make it look more interesting and alive
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct FatigueSideEffects {
    hours_until_exhausted: Cell<usize>
}

/// Will enable food drain over time
#[derive(Debug, Clone)]
pub struct FoodDrainOverTimeSideEffect {
    /// Drain speed, 0..100 percents per game second
    drain_amount: Cell<f32>
}

/// Will enable water drain over time
#[derive(Debug, Clone)]
pub struct WaterDrainOverTimeSideEffect {
    /// Drain speed, 0..100 percents per game second
    drain_amount: Cell<f32>
}

/// Will enable oxygen drain over time when under water
#[derive(Debug, Clone)]
pub struct UnderwaterSideEffect {
    /// Oxygen drain speed, 0..100 percents per game second
    oxygen_drain_amount: Cell<f32>,
    /// Stamina drain speed, 0..100 percents per game second
    stamina_drain_amount: Cell<f32>,

    sleeping_state: Cell<bool>,
    gained_fatigue: Cell<f32>,
    underwater_state: Cell<bool>,
    time_under_water: Cell<f32> // game seconds
}

#[derive(Debug, Clone)]
pub struct FatigueSideEffectsStateContract {
    pub hours_until_exhausted: usize
}

#[derive(Debug, Clone)]
pub struct DynamicVitalsSideEffectStateContract {
    pub first_iteration: bool,
    pub counter: f32,
    pub half_duration: f32,
    pub direction: f32,
    pub body_temperature_ceiling: f32,
    pub heart_rate_ceiling: f32,
    pub top_pressure_ceiling: f32,
    pub bottom_pressure_ceiling: f32
}

#[derive(Debug, Clone)]
pub struct FoodDrainOverTimeSideEffectStateContract {
    pub drain_amount: f32
}

#[derive(Debug, Clone)]
pub struct RunningSideEffectsStateContract {
    pub stamina_drain_amount: f32,
    pub water_drain_amount: f32,
    pub running_state: bool,
    pub sleeping_state: bool,
    pub running_time: f32,
    pub gained_fatigue: f32
}

#[derive(Debug, Clone)]
pub struct UnderwaterSideEffectStateContract {
    pub oxygen_drain_amount: f32,
    pub stamina_drain_amount: f32,
    pub sleeping_state: bool,
    pub gained_fatigue: f32,
    pub underwater_state: bool,
    pub time_under_water: f32
}

#[derive(Debug, Clone)]
pub struct WaterDrainOverTimeSideEffectStateContract {
    pub drain_amount: f32
}