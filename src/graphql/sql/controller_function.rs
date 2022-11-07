use crate::graphql::domain::ControllerFunction;
use sqlx::PgPool;

pub async fn query_all_controller_functions(
    pool: &PgPool,
) -> Result<Vec<ControllerFunction>, sqlx::Error> {
    let controller_functions =
        sqlx::query_as!(ControllerFunction, "SELECT * FROM controller_functions")
            .fetch_all(pool)
            .await;

    controller_functions
}
