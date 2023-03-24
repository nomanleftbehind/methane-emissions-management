use crate::graphql::models::{Compressor, EmittersByInput};
use sqlx::{query_as, Error, PgPool};

pub async fn query_compressors(
    pool: &PgPool,
    EmittersByInput { facility_id }: EmittersByInput,
) -> Result<Vec<Compressor>, Error> {
    query_as!(
        Compressor,
        "SELECT * FROM compressors WHERE facility_id = $1",
        facility_id
    )
    .fetch_all(pool)
    .await
}
