// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings. 

// Constraints: assembly line's speed can range from `0` (off) to `10` (maximum).
// At lowests speed '1', 221 cars are produced per hour
// - `1` to `4`: 100% success rate.
// - `5` to `8`: 90% success rate.
// - `9` and `10`: 77% success rate.

// Task:
// 1. Calculate production rate per hour and return f64
// 2. Calculate the number of working items produced per minute and return u32

#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let cars_per_hour; 
    if speed >= 1 && speed <= 4 {
        cars_per_hour = speed as f64 * 221.0
    }
    else if speed >= 5 && speed <= 8 {
        cars_per_hour = speed as f64 * 221.0 *0.9
    }
    else if speed >= 9 && speed <= 10 {
        cars_per_hour = speed as f64 * 221.0 * 0.77
    }
    else {
        cars_per_hour = speed as f64 * 0.0
    }
    println!("calculate hourly production rate at speed: {}", speed);
    cars_per_hour
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let working_items; 
    if speed >= 1 && speed <= 4 {
        working_items = production_rate_per_hour(speed) 
    }
    else if speed >= 5 && speed <= 8 {
        working_items = production_rate_per_hour(speed)
    }
    else if speed >= 9 && speed <= 10 {
        working_items = production_rate_per_hour(speed) 
    }
    else {
        working_items = 0.0
    }
    println!("calculate the amount of working items at speed: {}", speed);
    working_items as u32 / 60
}

pub fn main() {
    println!("production is {}", production_rate_per_hour(7));
}