use crate::graphql::models::{
    input::{GetPneumaticInstrumentChangesInput, InsertPneumaticInstrumentChangeInput},
    routine::pneumatic_device::pneumatic_instrument::PneumaticInstrumentChange,
};
use common::PneumaticInstrumentChangesByVariant::{
    CreatedById, PneumaticInstrumentId, UpdatedById,
};
use sqlx::{query, query_as, Error, PgPool};
use uuid::Uuid;

pub async fn get_pneumatic_instrument_changes(
    pool: &PgPool,
    GetPneumaticInstrumentChangesInput { by, id }: GetPneumaticInstrumentChangesInput,
) -> Result<Vec<PneumaticInstrumentChange>, Error> {
    match by {
        PneumaticInstrumentId => query_as!(
            PneumaticInstrumentChange,
            "SELECT * FROM pneumatic_instrument_change WHERE pneumatic_instrument_id = $1 ORDER BY pneumatic_instrument_id, id",
            id
            ).fetch_all(pool)
            .await,
            CreatedById => query_as!(
            PneumaticInstrumentChange,
            "SELECT * FROM pneumatic_instrument_change WHERE created_by_id = $1 ORDER BY pneumatic_instrument_id, id",
            id
            ).fetch_all(pool)
            .await,
            UpdatedById => query_as!(
            PneumaticInstrumentChange,
            "SELECT * FROM pneumatic_instrument_change WHERE updated_by_id = $1 ORDER BY pneumatic_instrument_id, id",
            id
            ).fetch_all(pool)
            .await,
    }
}

pub async fn insert_pneumatic_instrument_change(
    pool: &PgPool,
    user_id: &Uuid,
    InsertPneumaticInstrumentChangeInput {
        pneumatic_instrument_id,
        date,
        rate,
    }: InsertPneumaticInstrumentChangeInput,
) -> Result<u64, Error> {
    let rows_inserted = query!(
        "INSERT INTO pneumatic_instrument_change (id, pneumatic_instrument_id, date, rate, created_by_id, created_at, updated_by_id, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $5, $6);",
        Uuid::new_v4(),
        pneumatic_instrument_id,
        date,
        rate,
        user_id,
        chrono::Utc::now().naive_utc())
    .execute(pool)
    .await?
    .rows_affected();

    Ok(rows_inserted)
}
