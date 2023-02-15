use crate::graphql::models::{
    Compressor,
    CompressorsBy::{self, CreatedById, FacilityId, UpdatedById},
};
use sqlx::PgExecutor;

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
