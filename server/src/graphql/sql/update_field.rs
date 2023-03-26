use crate::graphql::models::{UpdateFieldInput, UpdateFieldValue};
use common::UpdateFieldVariant::{
    ControllerApplicationId, ControllerFacilityId, ControllerFdcRecId, ControllerManufacturerId,
    ControllerModel, ControllerSerialNumber,
};
use sqlx::{query, Error, PgPool};
use uuid::Uuid;

pub async fn update_field(
    pool: &PgPool,
    UpdateFieldInput {
        id,
        update_field_variant,
        value:
            UpdateFieldValue {
                string_value,
                uuid_value,
                integer_value: _,
                float_value: _,
                naive_date_value: _,
                naive_date_time_value: _,
            },
    }: UpdateFieldInput,
    updated_by_id: Uuid,
) -> Result<u64, Error> {
    let updated_at = chrono::Utc::now().naive_utc();
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
        // _ => {
        //     return Err(Error::ColumnNotFound(
        //         "Resolver for field not implemented".to_string(),
        //     ))
        // }
    };

    Ok(query.execute(pool).await?.rows_affected())
}
