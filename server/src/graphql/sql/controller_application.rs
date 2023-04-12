use crate::graphql::models::{ControllerApplication, IdSelection};
use sqlx::{query_as, PgPool};

pub async fn query_all_controller_applications(
    pool: &PgPool,
) -> Result<Vec<ControllerApplication>, sqlx::Error> {
    let controller_applications = query_as!(
        ControllerApplication,
        "SELECT * FROM controller_applications"
    )
    .fetch_all(pool)
    .await;

    controller_applications
}

pub async fn controller_application_selection(
    pool: &PgPool,
) -> Result<Vec<IdSelection>, sqlx::Error> {
    let controller_application_selection = query_as!(
        IdSelection,
        "SELECT id, application as name FROM controller_applications"
    )
    .fetch_all(pool)
    .await;

    controller_application_selection
}
