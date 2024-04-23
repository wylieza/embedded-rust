extern crate alloc;

use alloc::vec::Vec;
use nrf52833_hal::{pac::ficr::temp, Temp};

pub struct Temperature {
    pub temperature_c: f32,
    pub timestamp_ms: i32,
}

impl Temperature {
    pub fn new(temperature_c: f32, timestamp_ms: i32) -> Self {
        Temperature{temperature_c: temperature_c, timestamp_ms: timestamp_ms}
    }

}

pub struct TemperatureStore {
    readings: Vec<Temperature>,
}

impl TemperatureStore {
    pub fn new() -> Self {
        TemperatureStore {readings: Vec::with_capacity(128)}
    }

    pub fn add(&mut self, temperature: Temperature) {
        self.readings.push(temperature)
    }

    pub fn get_last_added(&mut self) -> Option<&Temperature> {
        return self.readings.last();
    }
}