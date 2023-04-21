use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// `SiteType` is an externally defined enum inside schema, so we have to provide matching Rust type and `Display` trait implementation.
///
/// It is defined in common library so it can be used by both server and client.
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(async_graphql::Enum, sqlx::Type),
    sqlx(type_name = "facility_type", rename_all = "SCREAMING_SNAKE_CASE")
)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SiteType {
    Battery,
    Satellite,
    Well,
    GasPlant,
    Compressor,
}

impl Display for SiteType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SiteType::Battery => write!(f, "Battery"),
            SiteType::Satellite => write!(f, "Satellite"),
            SiteType::Well => write!(f, "Well"),
            SiteType::GasPlant => write!(f, "GasPlant"),
            SiteType::Compressor => write!(f, "Compressor"),
        }
    }
}
