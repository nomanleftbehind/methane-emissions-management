use crate::graphql::{context::ContextExt, models::IdSelection, sql::dropdown_selection};
use async_graphql::{Context, Error, Object};
use common::IdSelectionVariant;

#[derive(Default, Clone)]
pub struct DropdownSelectionQuery;

#[Object]
impl DropdownSelectionQuery {
    async fn id_selection(
        &self,
        ctx: &Context<'_>,
        variant: IdSelectionVariant,
    ) -> Result<Vec<IdSelection>, Error> {
        let pool = ctx.db_pool();
        dropdown_selection::id_selection(pool, variant)
            .await
            .map_err(Error::from)
    }
}
