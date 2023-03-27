use crate::graphql::models::{Controller, EmittersByInput};
use sqlx::{query_as, Error, PgPool};

pub async fn query_controllers(
    pool: &PgPool,
    EmittersByInput { facility_id }: EmittersByInput,
) -> Result<Vec<Controller>, Error> {
    query_as!(
        Controller,
        "SELECT * FROM controllers WHERE facility_id = $1 ORDER BY id",
        facility_id
    )
    .fetch_all(pool)
    .await
}
