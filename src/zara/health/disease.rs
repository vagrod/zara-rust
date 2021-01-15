use super::super::health::{Health};
use super::super::utils::GameTimeC;

use std::sync::Arc;

pub trait DiseaseMonitor {
    fn check(&self, game_time_delta: f32, game_time: &GameTimeC);
    fn set_health(&mut self, health: Arc<Health>);
}