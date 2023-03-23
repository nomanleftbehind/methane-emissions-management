use crate::graphql::models::{EmittersByInput, TankFarm};
use sqlx::{query_as, PgPool};

pub async fn query_tank_farms(
    pool: &PgPool,
    EmittersByInput { facility_id }: EmittersByInput,
) -> Result<Vec<TankFarm>, sqlx::Error> {
    query_as!(
        TankFarm,
        "SELECT * FROM tank_farms WHERE facility_id = $1",
        facility_id
    )
    .fetch_all(pool)
    .await
}
