use crate::graphql::{
    context::ContextExt,
    models::{
        input::MonthMethaneEmissionBySourceIdInput, month_methane_emission::MonthMethaneEmission,
    },
    sql,
};
use async_graphql::{Context, Error, Object};

#[derive(Default, Clone)]
pub struct MonthMethaneEmissionQuery;

#[Object]
impl MonthMethaneEmissionQuery {
    async fn controller_month_vents(
        &self,
        ctx: &Context<'_>,
        by: MonthMethaneEmissionBySourceIdInput,
    ) -> Result<Vec<MonthMethaneEmission>, Error> {
        let pool = ctx.db_pool();

        let month_methane_emissions = sql::get_month_methane_emissions(pool, by)
            .await
            .map_err(Error::from);

        month_methane_emissions
    }
}
