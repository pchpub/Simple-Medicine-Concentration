use super::types::{SingleMedicine, Time2Concentration};

pub fn get_time2concentration(medicine: &SingleMedicine, time_step: f64) -> Time2Concentration {
    const DAYS: usize = 11;
    const TIME_START: f64 = 0 as f64;
    let time_end: f64 = 24.0 * (10.0 + 1.0);
    let mut time2concentration = Time2Concentration::new(TIME_START, time_end, time_step);

    for time in medicine.time.iter() {
        for current_day in 0..DAYS {
            let current_time =
                time.hour as f64 + time.minute as f64 / 60.0 + current_day as f64 * 24.0; // 服用时间
            for single_concentration in time2concentration.concentrations.iter_mut() {
                if single_concentration.0 >= current_time {
                    let difference_time = single_concentration.0 - current_time;
                    let concentration = time_related_concentration(
                        medicine.half_life,
                        medicine.peak_time,
                        medicine.peak_concentration,
                        difference_time,
                    );
                    single_concentration.1 += concentration;
                }
            }
        }
    }

    time2concentration
}

// 用于计算血液中药物随时间变化的关系
fn time_related_concentration(
    half_life: f64,
    peak_time: f64,
    peak_concentration: f64,
    current_time: f64,
) -> f64 {
    let return_data = if current_time < peak_time * (43.0 / 76.88) {
        // peak_concentration * current_time / peak_time
        (116.0 / 1849.0) * (current_time * (76.88 / peak_time)).powi(2) / 187.0 * peak_concentration
    } else if current_time < peak_time {
        // peak_concentration * (0.5_f64).powf((current_time - peak_time) / half_life)
        ((-71.0 / 1225.0) * ((current_time * (76.88 / peak_time)) - 76.88).powi(2) + 187.0) / 187.0
            * peak_concentration
    } else {
        peak_concentration * (0.5_f64).powf((current_time - peak_time) / half_life)
    };
    return_data
}
