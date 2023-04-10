use crate::graphql::models::{InsertCompressorInput, InsertControllerInput, InsertEntryInput};
use sqlx::{query, Error, PgPool};
use uuid::Uuid;

pub async fn insert_entry(
    pool: &PgPool,
    input: InsertEntryInput,
    user_id: Uuid,
) -> Result<u64, Error> {
    let now = chrono::Utc::now().naive_utc();

    let query = match input {
        InsertEntryInput {controller: Some(InsertControllerInput {fdc_rec_id, facility_id, application_id, manufacturer_id, model, serial_number}), ..} => query!(
            "INSERT INTO controllers (id, fdc_rec_id, manufacturer_id, model, serial_number, application_id, facility_id, created_by_id, created_at, updated_by_id, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11);",
            Uuid::new_v4(),
            fdc_rec_id,
            manufacturer_id,
            model,
            serial_number,
            application_id,
            facility_id,
            user_id,
            now,
            user_id,
            now),
        InsertEntryInput {compressor: Some(InsertCompressorInput {facility_id, fdc_rec_id, name, serial_number, install_date, remove_date}), ..} => query!(
            "INSERT INTO compressors (id, fdc_rec_id, facility_id, name, serial_number, install_date, remove_date, created_by_id, created_at, updated_by_id, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11);",
            Uuid::new_v4(),
            fdc_rec_id,
            facility_id,
            name,
            serial_number,
            install_date,
            remove_date,
            user_id,
            now,
            user_id,
            now),

            _ => return Err(Error::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("No input provided"))))

    };

    let res = query.execute(pool).await?.rows_affected();

    println!("insert rows affected: {}", res);

    Ok(res)

    // Ok(query.execute(pool).await?.rows_affected())
}
