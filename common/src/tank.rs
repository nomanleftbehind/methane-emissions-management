use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// `CalculationMethod` is a database defined enum, so we have to provide matching Rust type and `Display` trait implementation.
///
/// It is defined in common library so it can be used by both server and client.
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
