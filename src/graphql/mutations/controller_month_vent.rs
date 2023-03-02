use crate::{
    configuration::DefaultGasParams,
    graphql::{
        context::ContextExt, mutations::validators::MonthBeginningValidator,
        sql::insert_controller_month_vents,
    },
};
use async_graphql::{Context, Error, Object};
use chrono::NaiveDate;

#[derive(Default, Clone)]
pub(super) struct ControllerMonthVentMutation;

#[Object]
impl ControllerMonthVentMutation {
    async fn insert_controller_month_vents(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(list, custom = "MonthBeginningValidator"))] months: Vec<NaiveDate>,
    ) -> Result<u64, Error> {
        let pool = ctx.db_pool();
        let cookie = ctx.get_cookie()?;
        let user_id = ctx.get_session_manager()?.user_id(cookie).await?;
        let DefaultGasParams { c1, co2, .. } = ctx.get_default_gas_params();

        let rows_inserted = insert_controller_month_vents(pool, user_id, &months, c1, co2)
            .await
            .map_err(Error::from);

        rows_inserted
    }
}
