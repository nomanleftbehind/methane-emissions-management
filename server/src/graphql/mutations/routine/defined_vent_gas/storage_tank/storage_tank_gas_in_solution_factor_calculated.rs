use crate::{
    configuration::DefaultGasParams,
    graphql::{
        context::ContextExt,
        models::{input::MonthRangeInput, validator::MonthRangeValidator},
        sql::routine::defined_vent_gas::storage_tank,
    },
};
use async_graphql::{Context, Error, Object};

#[derive(Default, Clone)]
pub struct StorageTankGasInSolutionFactorCalculatedMutation;

#[Object]
impl StorageTankGasInSolutionFactorCalculatedMutation {
    async fn insert_storage_tank_gas_in_solution_factor_calculated(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(custom = "MonthRangeValidator::new(12)"))] month_range: MonthRangeInput,
    ) -> Result<u64, Error> {
        let pool = ctx.db_pool();
        let cookie = ctx.get_cookie()?;
        let user_id = &ctx.get_session_manager()?.user_id(cookie).await?;
        let DefaultGasParams { gas_gravity, .. } = ctx.get_default_gas_params();

        let rows_inserted = storage_tank::insert_storage_tank_gas_in_solution_factor_calculated(
            pool,
            user_id,
            &month_range,
            gas_gravity,
        )
        .await
        .map_err(Error::from);

        rows_inserted
    }
}
