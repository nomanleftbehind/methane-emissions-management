use crate::graphql::models::{UpdateFieldInput, UpdateFieldValue};
use common::UpdateFieldVariant::{
    CompressorFacilityId, CompressorFdcRecId, CompressorInstallDate, CompressorName,
    CompressorRemoveDate, CompressorSerialNumber, ControllerApplicationId, ControllerChangeDate,
    ControllerChangeId, ControllerChangeRate, ControllerFacilityId, ControllerFdcRecId,
    ControllerManufacturerId, ControllerModel, ControllerSerialNumber,
};
use sqlx::{query, Error, PgPool};
use uuid::Uuid;

pub async fn update_field(
    pool: &PgPool,
    input: UpdateFieldInput,
    updated_by_id: Uuid,
) -> Result<u64, Error> {
    let updated_at = chrono::Utc::now().naive_utc();

    println!("input: {:#?}", &input);
    let UpdateFieldInput {
        id,
        update_field_variant,
        value:
            UpdateFieldValue {
                string_value,
                uuid_value,
                integer_value: _,
                float_value,
                naive_date_value,
                naive_date_time_value: _,
            },
    } = input;

    let query = match update_field_variant {
        ControllerSerialNumber => query!(
            "UPDATE controllers
            SET serial_number = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            string_value,
            updated_by_id,
            updated_at,
        ),
        ControllerFdcRecId => query!(
            "UPDATE controllers
            SET fdc_rec_id = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            string_value,
            updated_by_id,
            updated_at,
        ),
        ControllerManufacturerId => query!(
            "UPDATE controllers
            SET manufacturer_id = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            uuid_value,
            updated_by_id,
            updated_at,
        ),
        ControllerApplicationId => query!(
            "UPDATE controllers
            SET application_id = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            uuid_value,
            updated_by_id,
            updated_at,
        ),
        ControllerModel => query!(
            "UPDATE controllers
            SET model = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            string_value,
            updated_by_id,
            updated_at,
        ),
        ControllerFacilityId => query!(
            "UPDATE controllers
            SET facility_id = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            uuid_value,
            updated_by_id,
            updated_at,
        ),
        CompressorName => query!(
            "UPDATE compressors
            SET name = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            string_value,
            updated_by_id,
            updated_at,
        ),
        CompressorSerialNumber => query!(
            "UPDATE compressors
            SET serial_number = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            string_value,
            updated_by_id,
            updated_at,
        ),
        CompressorFacilityId => query!(
            "UPDATE compressors
            SET facility_id = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            uuid_value,
            updated_by_id,
            updated_at,
        ),
        CompressorFdcRecId => query!(
            "UPDATE compressors
            SET fdc_rec_id = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            string_value,
            updated_by_id,
            updated_at,
        ),
        CompressorInstallDate => query!(
            "UPDATE compressors
            SET install_date = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            naive_date_value,
            updated_by_id,
            updated_at,
        ),
        CompressorRemoveDate => query!(
            "UPDATE compressors
            SET remove_date = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            naive_date_value,
            updated_by_id,
            updated_at,
        ),
        ControllerChangeId => query!(
            "UPDATE controller_changes
            SET controller_id = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            uuid_value,
            updated_by_id,
            updated_at,
        ),
        ControllerChangeDate => query!(
            "UPDATE controller_changes
            SET date = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            naive_date_value,
            updated_by_id,
            updated_at,
        ),
        ControllerChangeRate => query!(
            "UPDATE controller_changes
            SET rate = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            float_value,
            updated_by_id,
            updated_at,
        ),
    };

    let res = query.execute(pool).await?.rows_affected();

    println!("rows affected: {}", res);

    Ok(res)

    // Ok(query.execute(pool).await?.rows_affected())
}
