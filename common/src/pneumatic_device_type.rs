use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// `PneumaticDeviceType` is an externally defined enum inside schema, so we have to provide matching Rust type and `Display` trait implementation.
///
/// It is defined in common library so it can be used by both server and client.
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(async_graphql::Enum, sqlx::Type),
    sqlx(
        type_name = "pneumatic_device_type",
        rename_all = "SCREAMING_SNAKE_CASE"
    )
)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PneumaticDeviceType {
    LevelController,
    PressureController,
    TemperatureController,
    Switch,
    Transducer,
    Positioner,
    PneumaticPump,
    GenericPneumaticInstrument,
}

impl Display for PneumaticDeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PneumaticDeviceType::LevelController => write!(f, "Level Controller"),
            PneumaticDeviceType::PressureController => write!(f, "Pressure Controller"),
            PneumaticDeviceType::TemperatureController => write!(f, "Temperature Controller"),
            PneumaticDeviceType::Switch => write!(f, "Switch"),
            PneumaticDeviceType::Transducer => write!(f, "Transducer"),
            PneumaticDeviceType::Positioner => write!(f, "Positioner"),
            PneumaticDeviceType::PneumaticPump => write!(f, "Pneumatic Pump"),
            PneumaticDeviceType::GenericPneumaticInstrument => {
                write!(f, "Generic Pneumatic Instrument")
            }
        }
    }
}
