use crate::graphql::models::input::{
    InsertCompressorInput, InsertEntryInput, InsertPneumaticInstrumentInput,
};
use common::{CompressorType, PneumaticInstrumentType};
use sqlx::{query, Error, PgPool};
use uuid::Uuid;

pub async fn insert_entry(
    pool: &PgPool,
    input: InsertEntryInput,
    user_id: Uuid,
) -> Result<u64, Error> {
    let now = chrono::Utc::now().naive_utc();

    let query = match input {
        InsertEntryInput {pneumatic_instrument: Some(
            InsertPneumaticInstrumentInput {site_id, r#type, manufacturer_id, model, serial_number, start_date, end_date}
        ), ..} => {
                query!(
            "INSERT INTO pneumatic_instrument (id, site_id, type, manufacturer_id, model, serial_number, start_date, end_date, created_by_id, created_at, updated_by_id, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $9, $10);",
            Uuid::new_v4(),
            site_id,
            r#type as PneumaticInstrumentType,
            manufacturer_id,
            model,
            serial_number,
            start_date,
            end_date,
            user_id,
            now,
            )
        },
        InsertEntryInput {compressor: Some(InsertCompressorInput { site_id, fdc_rec_id, r#type, name, serial_number, power, throw_count, install_date, remove_date}), ..} => query!(
            "INSERT INTO compressor (id, site_id, fdc_rec_id, type, name, serial_number, power, throw_count, install_date, remove_date, created_by_id, created_at, updated_by_id, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $11, $12);",
            Uuid::new_v4(),
            site_id,
            fdc_rec_id,
            r#type as CompressorType,
            name,
            serial_number,
            power,
            throw_count,
            install_date,
            remove_date,
            user_id,
            now,
            ),

            _ => return Err(Error::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("No input provided"))))

    };

    let res = query.execute(pool).await?.rows_affected();

    println!("insert rows affected: {}", res);

    Ok(res)

    // Ok(query.execute(pool).await?.rows_affected())
}
