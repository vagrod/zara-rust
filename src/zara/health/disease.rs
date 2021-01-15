use super::super::health::{Health};
use super::super::utils::{SummaryC};

pub trait DiseaseMonitor {
    fn check(&self, health: &Health, frame_data: &SummaryC);
}