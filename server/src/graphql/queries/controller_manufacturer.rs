use crate::graphql::{context::ContextExt, models::IdSelection, sql};
use async_graphql::{Context, Error, Object};

#[derive(Default, Clone)]
pub(super) struct ControllerManufacturerQuery;

#[Object]
impl ControllerManufacturerQuery {
    async fn controller_manufacturer_selection(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<IdSelection>, Error> {
        let pool = ctx.db_pool();

        let controller_manufacturer_selection = sql::controller_manufacturer_selection(pool)
            .await
            .map_err(Error::from);

        controller_manufacturer_selection
    }
}
