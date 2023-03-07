use crate::graphql::models::{
    Compressor, CompressorMap,
    CompressorsBy::{self, CreatedById, FacilityId, UpdatedById},
};
use sqlx::{PgExecutor, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub async fn query_compressors<'e, E: PgExecutor<'e>>(
    executor: E,
    by: CompressorsBy,
) -> Result<Vec<Compressor>, sqlx::Error> {
    match by {
        FacilityId(facility_id) => {
            sqlx::query_as!(
                Compressor,
                "SELECT * FROM compressors WHERE facility_id = $1",
                facility_id
            )
            .fetch_all(executor)
            .await
        }
        CreatedById(created_by_id) => {
            sqlx::query_as!(
                Compressor,
                "SELECT * FROM compressors WHERE created_by_id = $1",
                created_by_id
            )
            .fetch_all(executor)
            .await
        }
        UpdatedById(updated_by_id) => {
            sqlx::query_as!(
                Compressor,
                "SELECT * FROM compressors WHERE updated_by_id = $1",
                updated_by_id
            )
            .fetch_all(executor)
            .await
        }
    }
}

/// Get compressor fdc_rec_id and id as key-value pairs collected into `HashMap`.
///
/// Both keys and values are guaranteed to be unique as the database enforces uniquness of both fields individually in `compressors` table.
pub async fn get_compressor_db_crossref(pool: &PgPool) -> Result<HashMap<String, Uuid>, sqlx::Error> {
    let v = sqlx::query_as!(CompressorMap, "SELECT fdc_rec_id, id FROM compressors")
        .fetch_all(pool)
        .await?;

    let hm = v.into_iter().map(|cm| (cm.fdc_rec_id, cm.id)).collect();

    Ok(hm)
}
