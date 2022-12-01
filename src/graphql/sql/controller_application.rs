use crate::graphql::domain::ControllerApplication;
use sqlx::PgPool;

pub async fn query_all_controller_applications(
    pool: &PgPool,
) -> Result<Vec<ControllerApplication>, sqlx::Error> {
    let controller_applications = sqlx::query_as!(
        ControllerApplication,
        "SELECT * FROM controller_applications"
    )
    .fetch_all(pool)
    .await;

    controller_applications
}
