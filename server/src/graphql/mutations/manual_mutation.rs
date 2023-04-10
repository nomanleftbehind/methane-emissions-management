use crate::graphql::{
    context::ContextExt,
    models::{DeleteEntryInput, InsertEntryInput, UpdateFieldInput},
    sql,
};
use async_graphql::{Context, Error, Object};

#[derive(Default, Clone)]
pub(super) struct ManualMutation;

#[Object]
impl ManualMutation {
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

    async fn insert_entry(
        &self,
        ctx: &Context<'_>,
        insert_entry_input: InsertEntryInput,
    ) -> Result<u64, Error> {
        let pool = ctx.db_pool();
        let cookie = ctx.get_cookie()?;
        let user_id = ctx.get_session_manager()?.user_id(cookie).await?;

        let rows_inserted = sql::insert_entry(pool, insert_entry_input, user_id)
            .await
            .map_err(Error::from);

        rows_inserted
    }

    async fn delete_entry(
        &self,
        ctx: &Context<'_>,
        delete_entry_input: DeleteEntryInput,
    ) -> Result<u64, Error> {
        let pool = ctx.db_pool();
        let cookie = ctx.get_cookie()?;
        let _updated_by_id = ctx.get_session_manager()?.user_id(cookie).await?;

        let rows_inserted = sql::delete_entry(pool, delete_entry_input)
            .await
            .map_err(Error::from);

        rows_inserted
    }
}
