use crate::graphql::models::{
    input::{GetPneumaticInstrumentsInput, InsertPneumaticInstrumentInput},
    routine::pneumatic_device::pneumatic_instrument::PneumaticInstrument,
};
use common::{
    PneumaticInstrumentType,
    PneumaticInstrumentsByVariant::{FacilityId, ManufacturerId, SiteId},
};
use sqlx::{query, query_as, Error, PgPool};
use uuid::Uuid;

pub async fn get_pneumatic_instruments(
    pool: &PgPool,
    GetPneumaticInstrumentsInput { by, id }: GetPneumaticInstrumentsInput,
) -> Result<Vec<PneumaticInstrument>, Error> {
    match by {
        FacilityId => query_as!(
            PneumaticInstrument,
            r#"
            SELECT
            id, site_id, type as "type: _", manufacturer_id, model, serial_number, start_date, end_date, created_by_id, created_at, updated_by_id, updated_at
            FROM pneumatic_instrument
            WHERE site_id IN (SELECT id FROM site WHERE facility_id = $1)
            ORDER BY site_id, id
            "#,
            id
            ).fetch_all(pool)
            .await,
        SiteId => query_as!(
            PneumaticInstrument,
            r#"
            SELECT
            id, site_id, type as "type: _", manufacturer_id, model, serial_number, start_date, end_date, created_by_id, created_at, updated_by_id, updated_at
            FROM pneumatic_instrument
            WHERE site_id = $1
            ORDER BY site_id, id
            "#,
            id
            ).fetch_all(pool)
            .await,
        ManufacturerId => query_as!(
            PneumaticInstrument,
            r#"
            SELECT
            id, site_id, type as "type: _", manufacturer_id, model, serial_number, start_date, end_date, created_by_id, created_at, updated_by_id, updated_at
            FROM pneumatic_instrument
            WHERE manufacturer_id = $1
            ORDER BY site_id, id
            "#,
            id
            ).fetch_all(pool)
            .await,
    }
}

pub async fn insert_pneumatic_instrument(
    pool: &PgPool,
    user_id: &Uuid,
    InsertPneumaticInstrumentInput {
        site_id,
        manufacturer_id,
        model,
        serial_number,
        r#type,
        start_date,
        end_date,
    }: InsertPneumaticInstrumentInput,
) -> Result<u64, Error> {
    let rows_inserted = query!(
        "INSERT INTO pneumatic_instrument (id, site_id, type, manufacturer_id, model, serial_number, start_date, end_date, created_by_id, created_at, updated_by_id, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12);",
        Uuid::new_v4(),
        site_id,
        r#type as PneumaticInstrumentType,
        manufacturer_id,
        model,
        serial_number,
        start_date,
        end_date,
        user_id,
        chrono::Utc::now().naive_utc(),
        user_id,
        chrono::Utc::now().naive_utc())
    .execute(pool)
    .await?
    .rows_affected();

    Ok(rows_inserted)
}
