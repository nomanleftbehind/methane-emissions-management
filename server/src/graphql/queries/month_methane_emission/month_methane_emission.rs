use crate::graphql::{
    context::ContextExt,
    models::{input::GetMonthMethaneEmissionsInput, month_methane_emission::MonthMethaneEmission},
    sql,
};
use async_graphql::{Context, Error, Object};

#[derive(Default, Clone)]
pub struct MonthMethaneEmissionQuery;

#[Object]
impl MonthMethaneEmissionQuery {
    async fn get_month_methane_emissions(
        &self,
        ctx: &Context<'_>,
        get_month_methane_emissions_input: GetMonthMethaneEmissionsInput,
    ) -> Result<Vec<MonthMethaneEmission>, Error> {
        let pool = ctx.db_pool();

        sql::get_month_methane_emissions(pool, get_month_methane_emissions_input)
            .await
            .map_err(Error::from)
    }
}
