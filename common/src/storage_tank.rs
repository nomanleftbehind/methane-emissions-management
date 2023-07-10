use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// Type representing equipment used to control vent gas as described in AER Manual 015 section [`1.1.2`](https://static.aer.ca/prd/documents/manuals/Manual015.pdf#page=10).
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(async_graphql::Enum, sqlx::Type),
    sqlx(type_name = "control_device", rename_all = "SCREAMING_SNAKE_CASE")
)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ControlDevice {
    Flare,
    VapourRecoveryUnit,
}

impl Display for ControlDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ControlDevice::Flare => write!(f, "Flare"),
            ControlDevice::VapourRecoveryUnit => write!(f, "Vapour Recovery Unit"),
        }
    }
}

/// Type representing reason for periods of inactivity of equipment used to control vent gas, rendering emissions from controlled emission sources nonroutine or fugitive during those periods as described in AER Manual 015 section [`1.1.2`](https://static.aer.ca/prd/documents/manuals/Manual015.pdf#page=10).
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(async_graphql::Enum, sqlx::Type),
    sqlx(
        type_name = "control_device_inactivity_reason",
        rename_all = "SCREAMING_SNAKE_CASE"
    )
)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ControlDeviceInactivityReason {
    PlannedMaintenance,
    UnplannedMaintenance,
    Malfunction,
}

impl Display for ControlDeviceInactivityReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ControlDeviceInactivityReason::PlannedMaintenance => write!(f, "Planned Maintenance"),
            ControlDeviceInactivityReason::UnplannedMaintenance => {
                write!(f, "Unplanned Maintenance")
            }
            ControlDeviceInactivityReason::Malfunction => write!(f, "Malfunction"),
        }
    }
}

#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(async_graphql::Enum, sqlx::Type),
    sqlx(type_name = "calculation_method", rename_all = "SCREAMING_SNAKE_CASE")
)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CalculationMethod {
    Equation,
    Measured,
}

impl Display for CalculationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CalculationMethod::Equation => write!(f, "Equation"),
            CalculationMethod::Measured => write!(f, "Measured"),
        }
    }
}
