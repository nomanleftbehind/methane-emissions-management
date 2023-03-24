use crate::graphql::{context::ContextExt, sql};
use async_graphql::{Context, Error, Object};
use uuid::Uuid;

#[derive(Default, Clone)]
pub(super) struct ControllerMutation;

#[Object]
impl ControllerMutation {
    async fn update_controller_serial_number(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        value: Option<String>,
    ) -> Result<u64, Error> {
        let pool = ctx.db_pool();
        let cookie = ctx.get_cookie()?;
        let updated_by_id = ctx.get_session_manager()?.user_id(cookie).await?;

        let rows_inserted = sql::update_controller_serial_number(pool, id, value, updated_by_id)
            .await
            .map_err(Error::from);

        rows_inserted
    }
}
