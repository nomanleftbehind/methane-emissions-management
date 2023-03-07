use crate::graphql::models::{
    Controller,
    ControllersBy::{self, CreatedById, FacilityId, UpdatedById},
};
use sqlx::{query_as, Error, PgExecutor};

pub async fn query_controllers<'e, E: PgExecutor<'e>>(
    executor: E,
    by: ControllersBy,
) -> Result<Vec<Controller>, Error> {
    match by {
        FacilityId(facility_id) => {
            query_as!(
                Controller,
                "SELECT * FROM controllers WHERE facility_id = $1",
                facility_id
            )
            .fetch_all(executor)
            .await
        }
        CreatedById(created_by_id) => {
            query_as!(
                Controller,
                "SELECT * FROM controllers WHERE created_by_id = $1",
                created_by_id
            )
            .fetch_all(executor)
            .await
        }
        UpdatedById(updated_by_id) => {
            query_as!(
                Controller,
                "SELECT * FROM controllers WHERE updated_by_id = $1",
                updated_by_id
            )
            .fetch_all(executor)
            .await
        }
    }
}
