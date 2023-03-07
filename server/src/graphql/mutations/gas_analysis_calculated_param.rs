use crate::graphql::{context::ContextExt, sql::insert_gas_analysis_calculated_params};
use async_graphql::{Context, Error, Object};

#[derive(Default, Clone)]
pub(super) struct GasAnalysisCalculatedParamMutation;

#[Object]
impl GasAnalysisCalculatedParamMutation {
    async fn insert_gas_analysis_calculated_params(&self, ctx: &Context<'_>) -> Result<u64, Error> {
        let pool = ctx.db_pool();
        let cookie = ctx.get_cookie()?;
        let user_id = ctx.get_session_manager()?.user_id(cookie).await?;

        let rows_inserted = insert_gas_analysis_calculated_params(pool, user_id)
            .await
            .map_err(Error::from);

        rows_inserted
    }
}
