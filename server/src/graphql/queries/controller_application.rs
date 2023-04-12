use crate::graphql::{context::ContextExt, models::IdSelection, sql};
use async_graphql::{Context, Error, Object};

#[derive(Default, Clone)]
pub(super) struct ControllerApplicationQuery;

#[Object]
impl ControllerApplicationQuery {
    async fn controller_application_selection(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<IdSelection>, Error> {
        let pool = ctx.db_pool();

        let controller_application_selection = sql::controller_application_selection(pool)
            .await
            .map_err(Error::from);

        controller_application_selection
    }
}
