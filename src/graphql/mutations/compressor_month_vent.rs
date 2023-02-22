use crate::graphql::{context::ContextExt, sql::insert_compressor_month_vents};
use async_graphql::{Context, Error, Object};
use chrono::NaiveDate;

#[derive(Default, Clone)]
pub(super) struct CompressorMonthVentMutation;

#[Object]
impl CompressorMonthVentMutation {
    async fn insert_compressor_month_vents(
        &self,
        ctx: &Context<'_>,
        month: NaiveDate,
    ) -> Result<u64, Error> {
        let pool = ctx.db_pool();
        let cookie = ctx.get_cookie()?;
        let user_id = ctx.get_session_manager()?.user_id(cookie).await?;
        let default_mole_fractions = ctx.get_default_mole_fractions();

        let rows_inserted =
            insert_compressor_month_vents(pool, user_id, month, default_mole_fractions)
                .await
                .map_err(Error::from);

        rows_inserted
    }
}
