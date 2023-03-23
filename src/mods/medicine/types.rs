use serde::{Serialize, Deserialize};

use crate::mods::types::Time;

pub struct Time2Concentration {
    pub time_start: f64,
    pub time_end: f64,
    pub time_step: f64,
    pub concentrations: Vec<(f64,f64)>, // 用于存储每个时间点的浓度
}

impl Time2Concentration {
    pub fn new(time_start: f64, time_end: f64, time_step: f64) -> Self {
        let mut concentrations = vec![];
        let mut time = time_start;
        while time <= time_end {
            concentrations.push((time,0.0));
            time += time_step;
        }
        // println!("{:?}", concentrations);
        Self {
            time_start,
            time_end,
            time_step,
            concentrations,
        }
    }

    pub fn get_day_concentration(&self, day: f64) -> Vec<(f64,f64)> {
        let mut concentrations = vec![];
        let time_start = self.time_start + (day - 1.0) * 24.0;
        let time_end = time_start + 24.0;
        for single_concentration in self.concentrations.iter() {
            if single_concentration.0 >= time_start && single_concentration.0 <= time_end {
                concentrations.push((single_concentration.0 - time_start, single_concentration.1));
            }
        }
        // println!("{:?}", concentrations);
        concentrations
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SingleMedicine {
    // 药物名称
    pub name: String,
    // 半衰期
    pub half_life: f64,
    // 单次服用量
    pub dose: f64,
    // 服用时间
    pub time: Vec<Time>,
    // 药物达到峰值的时间
    pub peak_time: f64,
    // 药物达到峰值的浓度
    pub peak_concentration: f64,
}

impl Default for SingleMedicine {
    fn default() -> Self {
        Self {
            name: "药物".to_string(),
            half_life: 0.0,
            dose: 0.0,
            time: vec![],
            peak_time: 0.0,
            peak_concentration: 0.0,
        }
    }
}