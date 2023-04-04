use crate::graphql::models::{ControllerChange, ControllerChangeInput};
use sqlx::{query_as, Error, PgPool};

pub async fn get_controller_changes(
    pool: &PgPool,
    ControllerChangeInput { controller_id }: ControllerChangeInput,
) -> Result<Vec<ControllerChange>, Error> {
    query_as!(
        ControllerChange,
        "SELECT * FROM controller_changes WHERE controller_id = $1 ORDER BY id",
        controller_id
    )
    .fetch_all(pool)
    .await
}
