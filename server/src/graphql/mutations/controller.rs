use crate::graphql::{context::ContextExt, models::UpdateFieldInput, sql};
use async_graphql::{Context, Error, Object};

#[derive(Default, Clone)]
pub(super) struct ControllerMutation;

#[Object]
impl ControllerMutation {
    async fn update_field(
        &self,
        ctx: &Context<'_>,
        update_field_input: UpdateFieldInput,
    ) -> Result<u64, Error> {
        let pool = ctx.db_pool();
        let cookie = ctx.get_cookie()?;
        let updated_by_id = ctx.get_session_manager()?.user_id(cookie).await?;

        let rows_inserted = sql::update_field(pool, update_field_input, updated_by_id)
            .await
            .map_err(Error::from);

        rows_inserted
    }
}
