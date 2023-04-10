use crate::graphql::{context::ContextExt, models::InsertControllerInput, sql};
use async_graphql::{Context, Error, Object};

#[derive(Default, Clone)]
pub(super) struct ControllerMutation;

#[Object]
impl ControllerMutation {
    async fn insert_controller(
        &self,
        ctx: &Context<'_>,
        insert_controller_input: InsertControllerInput,
    ) -> Result<u64, Error> {
        let pool = ctx.db_pool();
        let cookie = ctx.get_cookie()?;
        let user_id = ctx.get_session_manager()?.user_id(cookie).await?;

        let rows_inserted = sql::insert_controller(pool, user_id, insert_controller_input)
            .await
            .map_err(Error::from);

        rows_inserted
    }
}
