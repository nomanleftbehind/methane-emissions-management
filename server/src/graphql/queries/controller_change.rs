use crate::graphql::{
    context::ContextExt,
    models::{ControllerChange, ControllerChangeInput},
    sql,
};
use async_graphql::{Context, Error, Object};

#[derive(Default, Clone)]
pub(super) struct ControllerChangeQuery;

#[Object]
impl ControllerChangeQuery {
    async fn get_controller_changes(
        &self,
        ctx: &Context<'_>,
        input: ControllerChangeInput,
    ) -> Result<Vec<ControllerChange>, Error> {
        let pool = ctx.db_pool();

        let controller_month_vents = sql::get_controller_changes(pool, input)
            .await
            .map_err(Error::from);

        controller_month_vents
    }
}
