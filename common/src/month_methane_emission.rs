use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// `MethaneEmissionSourceTable` is an externally defined enum inside schema, so we have to provide matching Rust type and `Display` trait implementation.
///
/// It is defined in common library so it can be used by both server and client.
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(async_graphql::Enum, sqlx::Type),
    sqlx(type_name = "methane_emission_source_table", rename_all = "snake_case")
)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MethaneEmissionSourceTable {
    PneumaticDevice,
    CompressorSeal,
    CompressorBlowdown,
    Tank,
}

#[cfg(not(target_arch = "wasm32"))]
impl sqlx::postgres::PgHasArrayType for MethaneEmissionSourceTable {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_methane_emission_source_table")
    }
}

impl Display for MethaneEmissionSourceTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MethaneEmissionSourceTable::PneumaticDevice => write!(f, "Pneumatic Device"),
            MethaneEmissionSourceTable::CompressorSeal => write!(f, "Compressor Seal"),
            MethaneEmissionSourceTable::CompressorBlowdown => write!(f, "Compressor Blowdown"),
            MethaneEmissionSourceTable::Tank => write!(f, "Tank"),
        }
    }
}

/// `MethaneEmissionSource` is an externally defined enum inside schema, so we have to provide matching Rust type and `Display` trait implementation.
///
/// It is defined in common library so it can be used by both server and client.
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(async_graphql::Enum, sqlx::Type),
    sqlx(
        type_name = "methane_emission_source",
        rename_all = "SCREAMING_SNAKE_CASE"
    )
)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MethaneEmissionSource {
    PneumaticDevice,
    CompressorSeal,
    GlycolDehydrator,
    DefinedVentGas,
    Planned,
    Unplanned,
    Fugitive,
}

#[cfg(not(target_arch = "wasm32"))]
impl sqlx::postgres::PgHasArrayType for MethaneEmissionSource {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_methane_emission_source")
    }
}

impl Display for MethaneEmissionSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MethaneEmissionSource::PneumaticDevice => write!(f, "Pneumatic Device"),
            MethaneEmissionSource::CompressorSeal => write!(f, "Compressor Seal"),
            MethaneEmissionSource::GlycolDehydrator => write!(f, "Glycol Dehydrator"),
            MethaneEmissionSource::DefinedVentGas => write!(f, "Defined Vent Gas"),
            MethaneEmissionSource::Planned => write!(f, "Planned"),
            MethaneEmissionSource::Unplanned => write!(f, "Unplanned"),
            MethaneEmissionSource::Fugitive => write!(f, "Fugitive"),
        }
    }
}

/// `MethaneEmissionCategory` is an externally defined enum inside schema, so we have to provide matching Rust type and `Display` trait implementation.
///
/// It is defined in common library so it can be used by both server and client.
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(async_graphql::Enum, sqlx::Type),
    sqlx(
        type_name = "methane_emission_category",
        rename_all = "SCREAMING_SNAKE_CASE"
    )
)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MethaneEmissionCategory {
    Routine,
    Nonroutine,
    Fugitive,
}

#[cfg(not(target_arch = "wasm32"))]
impl sqlx::postgres::PgHasArrayType for MethaneEmissionCategory {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_methane_emission_category")
    }
}

impl Display for MethaneEmissionCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MethaneEmissionCategory::Routine => write!(f, "Routine"),
            MethaneEmissionCategory::Nonroutine => write!(f, "Nonroutine"),
            MethaneEmissionCategory::Fugitive => write!(f, "Fugitive"),
        }
    }
}
