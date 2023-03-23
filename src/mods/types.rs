use serde::{Deserialize, Serialize};

use super::medicine::types::SingleMedicine;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub time_step: f64,
    #[serde(default)]
    pub medicines: Vec<SingleMedicine>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            title: "药物动态模拟".to_string(),
            medicines: vec![SingleMedicine::default()],
            time_step: 0.1,
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.medicines.len()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Time {
    pub hour: u8,
    pub minute: u8,
}

impl Default for Time {
    fn default() -> Self {
        Self {
            hour: 0,
            minute: 0,
        }
    }
}