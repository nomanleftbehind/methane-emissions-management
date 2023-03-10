use crate::graphql::models::{Controller, ControllersByFacilityId};
use sqlx::{query_as, Error, PgExecutor};

pub async fn query_controllers<'e, E: PgExecutor<'e>>(
    executor: E,
    ControllersByFacilityId { facility_id }: ControllersByFacilityId,
) -> Result<Vec<Controller>, Error> {
    query_as!(
        Controller,
        "SELECT * FROM controllers WHERE facility_id = $1",
        facility_id
    )
    .fetch_all(executor)
    .await
}
