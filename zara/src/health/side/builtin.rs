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

/// Contains state snapshot for the fatigue side effect monitor
#[derive(Debug, Clone)]
pub struct FatigueSideEffectsStateContract {
    /// Captured state of the `hours_until_exhausted` field
    pub hours_until_exhausted: usize
}

/// Contains state snapshot for the dynamic vitals side effect monitor
#[derive(Debug, Clone)]
pub struct DynamicVitalsSideEffectStateContract {
    /// Captured state of the `first_iteration` field
    pub first_iteration: bool,
    /// Captured state of the `counter` field
    pub counter: f32,
    /// Captured state of the `half_duration` field
    pub half_duration: f32,
    /// Captured state of the `direction` field
    pub direction: f32,
    /// Captured state of the `body_temperature_ceiling` field
    pub body_temperature_ceiling: f32,
    /// Captured state of the `heart_rate_ceiling` field
    pub heart_rate_ceiling: f32,
    /// Captured state of the `top_pressure_ceiling` field
    pub top_pressure_ceiling: f32,
    /// Captured state of the `bottom_pressure_ceiling` field
    pub bottom_pressure_ceiling: f32
}

/// Contains state snapshot for the food drain side effect monitor
#[derive(Debug, Clone)]
pub struct FoodDrainOverTimeSideEffectStateContract {
    /// Captured state of the `drain_amount` field
    pub drain_amount: f32
}

/// Contains state snapshot for the running side effect monitor
#[derive(Debug, Clone)]
pub struct RunningSideEffectsStateContract {
    /// Captured state of the `stamina_drain_amount` field
    pub stamina_drain_amount: f32,
    /// Captured state of the `water_drain_amount` field
    pub water_drain_amount: f32,
    /// Captured state of the `running_state` field
    pub running_state: bool,
    /// Captured state of the `sleeping_state` field
    pub sleeping_state: bool,
    /// Captured state of the `running_time` field
    pub running_time: f32,
    /// Captured state of the `gained_fatigue` field
    pub gained_fatigue: f32
}

/// Contains state snapshot for the underwater side effect monitor
#[derive(Debug, Clone)]
pub struct UnderwaterSideEffectStateContract {
    /// Captured state of the `oxygen_drain_amount` field
    pub oxygen_drain_amount: f32,
    /// Captured state of the `stamina_drain_amount` field
    pub stamina_drain_amount: f32,
    /// Captured state of the `sleeping_state` field
    pub sleeping_state: bool,
    /// Captured state of the `gained_fatigue` field
    pub gained_fatigue: f32,
    /// Captured state of the `underwater_state` field
    pub underwater_state: bool,
    /// Captured state of the `time_under_water` field
    pub time_under_water: f32
}

/// Contains state snapshot for the water drain side effect monitor
#[derive(Debug, Clone)]
pub struct WaterDrainOverTimeSideEffectStateContract {
    /// Captured state of the `drain_amount` field
    pub drain_amount: f32
}