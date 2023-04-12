use crate::graphql::{context::ContextExt, models::IdSelection, sql};
use async_graphql::{Context, Error, Object};
use common::IdSelectionVariant;

#[derive(Default, Clone)]
pub(super) struct IdSelectionQuery;

#[Object]
impl IdSelectionQuery {
    async fn id_selection(
        &self,
        ctx: &Context<'_>,
        variant: IdSelectionVariant,
    ) -> Result<Vec<IdSelection>, Error> {
        let pool = ctx.db_pool();
        sql::id_selection(pool, variant).await.map_err(Error::from)
    }
}
