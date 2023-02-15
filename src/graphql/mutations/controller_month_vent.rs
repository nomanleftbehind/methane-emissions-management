use crate::graphql::{context::ContextExt, sql::insert_controller_month_vents};
use async_graphql::{Context, Error, Object};

#[derive(Default, Clone)]
pub struct ControllerMonthVentMutations;

#[Object]
impl ControllerMonthVentMutations {
    async fn insert_controller_month_vents(&self, ctx: &Context<'_>) -> Result<u64, Error> {
        let pool = ctx.db_pool();
        let cookie = ctx.get_cookie()?;
        let user_id = ctx.get_session_manager()?.user_id(cookie).await?;

        let rows_inserted = insert_controller_month_vents(pool, user_id)
            .await
            .map_err(Error::from);

        rows_inserted
    }
}
