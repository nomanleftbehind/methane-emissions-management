use serde::{Deserialize, Serialize};
use strum_macros::Display;

/// `month_methane_emission` table doesn't have a foreign key constraint for `source_table_id` column because a value from that column could be refering to a column from multiple tables. This type represents a table it is refering to.
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(async_graphql::Enum, sqlx::Type),
    sqlx(type_name = "methane_emission_source_table", rename_all = "snake_case")
)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize, Display)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum MethaneEmissionSourceTable {
    PneumaticInstrument,
    LevelController,
    PneumaticPump,
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

/// Type representing top level categorization of methane emission sources as illustrated in AER Directive 060 [`Section 8, Figure 10`](https://static.aer.ca/prd/documents/directives/Directive060.pdf#page=71).
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(async_graphql::Enum, sqlx::Type),
    sqlx(
        type_name = "methane_emission_category",
        rename_all = "SCREAMING_SNAKE_CASE"
    )
)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize, Display)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "title_case")]
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

/// Type representing lowest level categorization of methane emission sources as illustrated in AER Directive 060 [`Section 8, Figure 10`](https://static.aer.ca/prd/documents/directives/Directive060.pdf#page=71).
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(async_graphql::Enum, sqlx::Type),
    sqlx(
        type_name = "methane_emission_source",
        rename_all = "SCREAMING_SNAKE_CASE"
    )
)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize, Display)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "title_case")]
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

#[cfg_attr(not(target_arch = "wasm32"), derive(async_graphql::Enum))]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum MonthMethaneEmissionsByVariant {
    FacilityId,
    SiteId,
    SourceTableId,
}
