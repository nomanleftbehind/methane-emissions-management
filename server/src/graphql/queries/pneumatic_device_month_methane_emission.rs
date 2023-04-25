use crate::graphql::{
    context::ContextExt,
    models::pneumatic_device::{ControllerMonthVent, ControllerMonthVentBy},
    sql::select_month_methane_emissions,
};
use async_graphql::{Context, Error, Object};

#[derive(Default, Clone)]
pub(super) struct ControllerMonthVentQuery;

#[Object]
impl ControllerMonthVentQuery {
    async fn controller_month_vents(
        &self,
        ctx: &Context<'_>,
        by: ControllerMonthVentBy,
    ) -> Result<Vec<ControllerMonthVent>, Error> {
        let pool = ctx.db_pool();

        let controller_month_vents = select_month_methane_emissions(pool, by)
            .await
            .map_err(Error::from);

        controller_month_vents
    }
}
