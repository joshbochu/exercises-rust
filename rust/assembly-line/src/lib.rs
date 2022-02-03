// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let speed = speed as f64;
    let rate = speed * 221.0;
    if speed == 0.0 {
        0.0
    } else if speed >= 1.0 && speed < 5.0 {
        rate
    } else if speed >= 5.0 && speed < 9.0 {
        rate * 0.90
    } else  {
        rate * 0.77
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}
