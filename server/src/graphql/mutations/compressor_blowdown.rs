use crate::graphql::{
    context::ContextExt, models::input::FromToMonthInput,
    sql::mutatation_insert_compressor_blowdowns_from_fdc,
};
use async_graphql::{Context, Error, Object};

#[derive(Default, Clone)]
pub(super) struct CompressorBlowdownMutation;

#[Object]
impl CompressorBlowdownMutation {
    async fn insert_compressor_blowdowns_from_fdc(
        &self,
        ctx: &Context<'_>,
        month_range: FromToMonthInput,
    ) -> Result<u64, Error> {
        let pool = ctx.db_pool();
        let cookie = ctx.get_cookie()?;
        let user_id = ctx.get_session_manager()?.user_id(cookie).await?;

        let atomic_mssql_fdc_client = ctx.mssql_fdc_client()?;
        let atomic_mssql_fdc_client_ptr = atomic_mssql_fdc_client.clone();
        let mut mssql_fdc_client_guard = atomic_mssql_fdc_client_ptr.lock().await;
        let mssql_fdc_client = &mut *mssql_fdc_client_guard;

        let rows_inserted = mutatation_insert_compressor_blowdowns_from_fdc(
            pool,
            mssql_fdc_client,
            user_id,
            month_range,
        )
        .await
        .map_err(Error::from);

        rows_inserted
    }
}
