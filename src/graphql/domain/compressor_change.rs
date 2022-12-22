use async_graphql::{Enum, SimpleObject};
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Enum, Copy, Clone, Deserialize, Serialize, Eq, PartialEq, Debug, sqlx::Type)]
#[sqlx(type_name = "calculation_method", rename_all = "UPPERCASE")]
pub enum CalculationMethod {
    Equation,
    Measured,
}

#[derive(SimpleObject, Clone, FromRow, Debug)]
pub struct CompressorChange {
    pub id: Uuid,
    pub compressor_id: Uuid,
    pub date: NaiveDate,
    pub calculation_method: CalculationMethod,
    pub number_of_throws: i32,
    pub vent_rate_per_hour: f64,
    pub created_by_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_by_id: Uuid,
    pub updated_at: NaiveDateTime,
}
