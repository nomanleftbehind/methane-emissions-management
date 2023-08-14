use crate::graphql::models::{
    input::{
        GetPneumaticInstrumentSubtableInput,
        InsertPneumaticInstrumentControlledCharacterizationInput,
    },
    routine::pneumatic_device::pneumatic_instrument::PneumaticInstrumentControlledCharacterization,
};
use common::{
    ControlDevice,
    PneumaticInstrumentSubtableByVariant::{CreatedById, PneumaticInstrumentId, UpdatedById},
};
use sqlx::{query, query_as, Error, PgPool};
use uuid::Uuid;

pub async fn get_pneumatic_instrument_controlled_characterizations(
    pool: &PgPool,
    GetPneumaticInstrumentSubtableInput { by, id }: GetPneumaticInstrumentSubtableInput,
) -> Result<Vec<PneumaticInstrumentControlledCharacterization>, Error> {
    match by {
        PneumaticInstrumentId => query_as!(
            PneumaticInstrumentControlledCharacterization,
            r#"
            SELECT
            id, pneumatic_instrument_id, start_date, end_date, control_device as "control_device: _", comment, created_by_id, created_at, updated_by_id, updated_at
            FROM pneumatic_instrument_controlled_characterization
            WHERE pneumatic_instrument_id = $1
            ORDER BY pneumatic_instrument_id, id
            "#,
            id
            ).fetch_all(pool)
            .await,
        CreatedById => query_as!(
            PneumaticInstrumentControlledCharacterization,
            r#"
            SELECT
            id, pneumatic_instrument_id, start_date, end_date, control_device as "control_device: _", comment, created_by_id, created_at, updated_by_id, updated_at
            FROM pneumatic_instrument_controlled_characterization
            WHERE created_by_id = $1
            ORDER BY pneumatic_instrument_id, id
            "#,
            id
            ).fetch_all(pool)
            .await,
        UpdatedById => query_as!(
            PneumaticInstrumentControlledCharacterization,
            r#"
            SELECT
            id, pneumatic_instrument_id, start_date, end_date, control_device as "control_device: _", comment, created_by_id, created_at, updated_by_id, updated_at
            FROM pneumatic_instrument_controlled_characterization
            WHERE updated_by_id = $1
            ORDER BY pneumatic_instrument_id, id
            "#,
            id
            ).fetch_all(pool)
            .await,
    }
}

pub async fn insert_pneumatic_instrument_controlled_characterization(
    pool: &PgPool,
    user_id: &Uuid,
    InsertPneumaticInstrumentControlledCharacterizationInput {
        pneumatic_instrument_id,
        start_date,
        end_date,
        control_device,
        comment,
    }: InsertPneumaticInstrumentControlledCharacterizationInput,
) -> Result<u64, Error> {
    let rows_inserted = query!(
        "INSERT INTO pneumatic_instrument_controlled_characterization (id, pneumatic_instrument_id, start_date, end_date, control_device, comment, created_by_id, created_at, updated_by_id, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $7, $8);",
        Uuid::new_v4(),
        pneumatic_instrument_id,
        start_date,
        end_date,
        control_device as ControlDevice,
        comment,
        user_id,
        chrono::Utc::now().naive_utc())
    .execute(pool)
    .await?
    .rows_affected();

    Ok(rows_inserted)
}
