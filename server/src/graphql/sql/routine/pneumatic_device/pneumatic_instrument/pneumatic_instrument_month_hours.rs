use crate::graphql::models::{
    input::{GetPneumaticInstrumentSubtableInput, InsertPneumaticInstrumentMonthHoursInput},
    routine::pneumatic_device::pneumatic_instrument::PneumaticInstrumentMonthHours,
};
use common::PneumaticInstrumentSubtableByVariant::{
    CreatedById, PneumaticInstrumentId, UpdatedById,
};
use sqlx::{query, query_as, Error, PgPool};
use uuid::Uuid;

pub async fn get_pneumatic_instrument_month_hours(
    pool: &PgPool,
    GetPneumaticInstrumentSubtableInput { by, id }: GetPneumaticInstrumentSubtableInput,
) -> Result<Vec<PneumaticInstrumentMonthHours>, Error> {
    match by {
        PneumaticInstrumentId => query_as!(
            PneumaticInstrumentMonthHours,
            "SELECT * FROM pneumatic_instrument_month_hours WHERE pneumatic_instrument_id = $1 ORDER BY pneumatic_instrument_id, id",
            id
            ).fetch_all(pool)
            .await,
            CreatedById => query_as!(
            PneumaticInstrumentMonthHours,
            "SELECT * FROM pneumatic_instrument_month_hours WHERE created_by_id = $1 ORDER BY pneumatic_instrument_id, id",
            id
            ).fetch_all(pool)
            .await,
            UpdatedById => query_as!(
            PneumaticInstrumentMonthHours,
            "SELECT * FROM pneumatic_instrument_month_hours WHERE updated_by_id = $1 ORDER BY pneumatic_instrument_id, id",
            id
            ).fetch_all(pool)
            .await,
    }
}

pub async fn insert_pneumatic_instrument_month_hours(
    pool: &PgPool,
    user_id: &Uuid,
    InsertPneumaticInstrumentMonthHoursInput {
        pneumatic_instrument_id,
        month,
        hours_on,
    }: InsertPneumaticInstrumentMonthHoursInput,
) -> Result<u64, Error> {
    let rows_inserted = query!(
        "INSERT INTO pneumatic_instrument_month_hours (id, pneumatic_instrument_id, month, hours_on, created_by_id, created_at, updated_by_id, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $5, $6);",
        Uuid::new_v4(),
        pneumatic_instrument_id,
        month,
        hours_on,
        user_id,
        chrono::Utc::now().naive_utc())
    .execute(pool)
    .await?
    .rows_affected();

    Ok(rows_inserted)
}
