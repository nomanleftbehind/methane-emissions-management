use crate::{
    configuration::DefaultGasParams,
    graphql::{context::ContextExt, models::input::MonthRangeInput, sql},
};
use async_graphql::{Context, Error, Object};

#[derive(Default, Clone)]
pub struct MonthMethaneEmissionMutation;

#[Object]
impl MonthMethaneEmissionMutation {
    async fn insert_month_methane_emissions(
        &self,
        ctx: &Context<'_>,
        month_range: MonthRangeInput,
    ) -> Result<u64, Error> {
        let pool = ctx.db_pool();
        let cookie = ctx.get_cookie()?;
        let user_id = &ctx.get_session_manager()?.user_id(cookie).await?;
        let DefaultGasParams {
            c1,
            co2,
            gas_gravity,
        } = ctx.get_default_gas_params();

        let rows_inserted =
            sql::insert_month_methane_emissions(pool, user_id, &month_range, c1, co2, gas_gravity)
                .await
                .map_err(Error::from);

        rows_inserted
    }
}
