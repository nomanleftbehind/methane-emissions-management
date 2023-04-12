use crate::graphql::models::IdSelection;
use sqlx::{query_as, PgPool};

pub async fn controller_manufacturer_selection(
    pool: &PgPool,
) -> Result<Vec<IdSelection>, sqlx::Error> {
    let controller_manufacturer_selection = query_as!(
        IdSelection,
        "SELECT id, manufacturer as name FROM controller_manufacturers"
    )
    .fetch_all(pool)
    .await;

    controller_manufacturer_selection
}
