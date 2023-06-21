use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// `PneumaticInstrumentType` is an externally defined enum inside schema, so we have to provide matching Rust type and `Display` trait implementation.
///
/// It is defined in common library so it can be used by both server and client.
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(async_graphql::Enum, sqlx::Type),
    sqlx(
        type_name = "pneumatic_instrument_type",
        rename_all = "SCREAMING_SNAKE_CASE"
    )
)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PneumaticInstrumentType {
    PressureController,
    TemperatureController,
    Switch,
    Transducer,
    Positioner,
    GenericPneumaticInstrument,
}

impl Display for PneumaticInstrumentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PneumaticInstrumentType::PressureController => write!(f, "Pressure Controller"),
            PneumaticInstrumentType::TemperatureController => write!(f, "Temperature Controller"),
            PneumaticInstrumentType::Switch => write!(f, "Switch"),
            PneumaticInstrumentType::Transducer => write!(f, "Transducer"),
            PneumaticInstrumentType::Positioner => write!(f, "Positioner"),
            PneumaticInstrumentType::GenericPneumaticInstrument => {
                write!(f, "Generic Pneumatic Instrument")
            }
        }
    }
}
