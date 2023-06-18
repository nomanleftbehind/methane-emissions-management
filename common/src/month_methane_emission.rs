use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// `month_methane_emission` table doesn't have a foreign key constraint for `source_table_id` column because a value from that column could be refering to a column from multiple tables. This type represents a table it is refering to.
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(async_graphql::Enum, sqlx::Type),
    sqlx(type_name = "methane_emission_source_table", rename_all = "snake_case")
)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MethaneEmissionSourceTable {
    NonLevelController,
    LevelController,
    CompressorSeal,
    CompressorBlowdown,
    StorageTank,
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
            MethaneEmissionSourceTable::NonLevelController => write!(f, "Non-Level Controller"),
            MethaneEmissionSourceTable::LevelController => write!(f, "Level Controller"),
            MethaneEmissionSourceTable::CompressorSeal => write!(f, "Compressor Seal"),
            MethaneEmissionSourceTable::CompressorBlowdown => write!(f, "Compressor Blowdown"),
            MethaneEmissionSourceTable::StorageTank => write!(f, "Storage Tank"),
        }
    }
}

/// Type representing top level categorization of methane emission sources as illustrated in AER Directive 060 [`Section 8, Figure 10`](https://static.aer.ca/prd/documents/directives/Directive060.pdf#page=71).
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

/// Type representing lowest level categorization of methane emission sources as illustrated in AER Directive 060 [`Section 8, Figure 10`](https://static.aer.ca/prd/documents/directives/Directive060.pdf#page=71).
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
