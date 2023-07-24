use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

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
#[derive(
    Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize, EnumString, Display, EnumIter,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "title_case")]
pub enum PneumaticInstrumentType {
    PressureController,
    TemperatureController,
    Switch,
    Transducer,
    Positioner,
    GenericPneumaticInstrument,
}

#[cfg_attr(not(target_arch = "wasm32"), derive(async_graphql::Enum))]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize)]
pub enum PneumaticInstrumentsByVariant {
    FacilityId,
    SiteId,
    ManufacturerId,
}

#[cfg_attr(not(target_arch = "wasm32"), derive(async_graphql::Enum))]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize)]
pub enum PneumaticInstrumentChangesByVariant {
    PneumaticInstrumentId,
    CreatedById,
    UpdatedById,
}
