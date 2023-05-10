use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// `NonLevelControllerType` is an externally defined enum inside schema, so we have to provide matching Rust type and `Display` trait implementation.
///
/// It is defined in common library so it can be used by both server and client.
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(async_graphql::Enum, sqlx::Type),
    sqlx(
        type_name = "non_level_controller_type",
        rename_all = "SCREAMING_SNAKE_CASE"
    )
)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NonLevelControllerType {
    PressureController,
    TemperatureController,
    Switch,
    Transducer,
    Positioner,
    PneumaticPump,
    GenericPneumaticInstrument,
}

impl Display for NonLevelControllerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NonLevelControllerType::PressureController => write!(f, "Pressure Controller"),
            NonLevelControllerType::TemperatureController => write!(f, "Temperature Controller"),
            NonLevelControllerType::Switch => write!(f, "Switch"),
            NonLevelControllerType::Transducer => write!(f, "Transducer"),
            NonLevelControllerType::Positioner => write!(f, "Positioner"),
            NonLevelControllerType::PneumaticPump => write!(f, "Pneumatic Pump"),
            NonLevelControllerType::GenericPneumaticInstrument => {
                write!(f, "Generic Pneumatic Instrument")
            }
        }
    }
}
