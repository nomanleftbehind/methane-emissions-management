use crate::graphql::{context::ContextExt, sql::gas_analysis};
use async_graphql::{Context, Error, Object};

#[derive(Default, Clone)]
pub struct GasAnalysisCalculatedParamMutation;

#[Object]
impl GasAnalysisCalculatedParamMutation {
    async fn insert_gas_analysis_calculated_param(&self, ctx: &Context<'_>) -> Result<u64, Error> {
        let pool = ctx.db_pool();
        let cookie = ctx.get_cookie()?;
        let user_id = ctx.get_session_manager()?.user_id(cookie).await?;

        let rows_inserted = gas_analysis::insert_gas_analysis_calculated_param(pool, user_id)
            .await
            .map_err(Error::from);

        rows_inserted
    }
}
