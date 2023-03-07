use crate::{
    configuration::DefaultGasParams,
    graphql::{context::ContextExt, sql::insert_tank_farm_vent_factors_calculated},
};
use async_graphql::{Context, Error, Object};

#[derive(Default, Clone)]
pub(super) struct TankFarmVentFactorCalculatedMutation;

#[Object]
impl TankFarmVentFactorCalculatedMutation {
    async fn insert_tank_farm_vent_factors(&self, ctx: &Context<'_>) -> Result<u64, Error> {
        let pool = ctx.db_pool();
        let cookie = ctx.get_cookie()?;
        let user_id = ctx.get_session_manager()?.user_id(cookie).await?;
        let DefaultGasParams { gas_gravity, .. } = ctx.get_default_gas_params();

        let rows_inserted = insert_tank_farm_vent_factors_calculated(pool, user_id, gas_gravity)
            .await
            .map_err(Error::from);

        rows_inserted
    }
}
