use super::super::health::{Health};
use super::super::utils::GameTimeC;

use std::sync::Arc;

pub trait DiseaseMonitor {
    fn check(&self, health: &Health, game_time_delta: f32, game_time: &GameTimeC);
}