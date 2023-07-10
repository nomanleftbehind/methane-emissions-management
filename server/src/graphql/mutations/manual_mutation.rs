use crate::graphql::{
    context::ContextExt,
    models::input::{DeleteEntryInput, InsertEntryInput, UpdateFieldInput},
    sql::manual_mutation,
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
        let user_id = &ctx.get_session_manager()?.user_id(cookie).await?;

        let rows_inserted = manual_mutation::update_field(pool, update_field_input, user_id)
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
        let user_id = &ctx.get_session_manager()?.user_id(cookie).await?;

        let rows_inserted = manual_mutation::insert_entry(pool, insert_entry_input, user_id)
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
        let _user_id = ctx.get_session_manager()?.user_id(cookie).await?;

        let rows_inserted = manual_mutation::delete_entry(pool, delete_entry_input)
            .await
            .map_err(Error::from);

        rows_inserted
    }
}
