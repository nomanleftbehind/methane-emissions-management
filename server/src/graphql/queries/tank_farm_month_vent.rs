use crate::graphql::{
    context::ContextExt,
    models::{TankFarmMonthVent, TankFarmMonthVentBy},
    sql::select_tank_farm_month_vents,
};
use async_graphql::{Context, Error, Object};

#[derive(Default, Clone)]
pub(super) struct TankFarmMonthVentQuery;

#[Object]
impl TankFarmMonthVentQuery {
    async fn tank_farm_month_vents(
        &self,
        ctx: &Context<'_>,
        by: TankFarmMonthVentBy,
    ) -> Result<Vec<TankFarmMonthVent>, Error> {
        let pool = ctx.db_pool();

        let tank_farm_month_vents = select_tank_farm_month_vents(pool, by)
            .await
            .map_err(Error::from);

        tank_farm_month_vents
    }
}
