use std::time::Duration;

use buttplug::core::message::ActuatorType;
use cxx::{CxxString, CxxVector};
use tracing::error;

pub fn read_input_string(list: &CxxVector<CxxString>) -> Vec<String> {
    // automatically discards any empty strings to account for papyrus
    // inability to do dynamic array sizes
    list.iter()
        .filter(|d| !d.is_empty())
        .map(|d| d.to_string_lossy().into_owned())
        .collect()
}

pub fn read_input_duration(secs: f32) -> Duration {
    if secs > 0.0 {
        Duration::from_millis((secs * 1000.0) as u64)
    } else {
        Duration::MAX
    }
}

fn _read_input_actuators(actuator: &str) -> ActuatorType {
    let lower = actuator.to_ascii_lowercase();
    match lower.as_str() {
        "constrict" => ActuatorType::Constrict,
        "inflate" => ActuatorType::Inflate,
        "oscillate" => ActuatorType::Oscillate,
        "vibrate" => ActuatorType::Vibrate,
        _ => {
            error!("unknown actuator {:?}", lower);
            ActuatorType::Vibrate
        }
    }
}