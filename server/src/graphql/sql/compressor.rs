use crate::graphql::models::{Compressor, CompressorsByFacilityId};
use sqlx::PgPool;

pub async fn query_compressors(
    pool: &PgPool,
    CompressorsByFacilityId { facility_id }: CompressorsByFacilityId,
) -> Result<Vec<Compressor>, sqlx::Error> {
    sqlx::query_as!(
        Compressor,
        "SELECT * FROM compressors WHERE facility_id = $1",
        facility_id
    )
    .fetch_all(pool)
    .await
}
