use crate::graphql::{
    context::ContextExt,
    models::{MonthMethaneEmission, MonthMethaneEmissionBySourceIdInput},
    // models::pneumatic_device::{ControllerMonthVent, ControllerMonthVentBy},
    sql,
};
use async_graphql::{Context, Error, Object};

#[derive(Default, Clone)]
pub(super) struct MonthMethaneEmissionQuery;

#[Object]
impl MonthMethaneEmissionQuery {
    async fn controller_month_vents(
        &self,
        ctx: &Context<'_>,
        by: MonthMethaneEmissionBySourceIdInput,
    ) -> Result<Vec<MonthMethaneEmission>, Error> {
        let pool = ctx.db_pool();

        let controller_month_vents = sql::select_month_methane_emissions(pool, by)
            .await
            .map_err(Error::from);

        controller_month_vents
    }
}
