use crate::graphql::{
    context::ContextExt, domain::ControllerMonthVentInsertValuesRow,
    sql::query_controller_month_vents_calculated,
};
use async_graphql::{Context, Error, Object};
use itertools::Itertools;

#[derive(Default, Clone)]
pub struct ControllerMonthVentQueries;

#[Object]
impl ControllerMonthVentQueries {
    async fn all_controller_month_vents(
        &self,
        ctx: &Context<'_>,
    ) -> Result</*Vec<ControllerMonthVent>*/ String, Error> {
        let pool = ctx.db_pool();
        let cookie = ctx.get_cookie()?;
        let user_id = ctx.get_session_manager()?.user_id(cookie).await?;

        let controller_month_vents_calculated = query_controller_month_vents_calculated(pool)
            .await
            .map_err(Error::from)?;

        let insert_rows_string = controller_month_vents_calculated
            .into_iter()
            .map(|controller_month_vent_calculated| {
                let insert_row = ControllerMonthVentInsertValuesRow::new(
                    user_id,
                    controller_month_vent_calculated,
                );
                let insert_row_string: String = insert_row.into();
                insert_row_string
            })
            .join(",");

        let insert_statement = format!("INSERT INTO controller_month_vent (id, month, volume, controller_id, created_by_id, created_at, updated_by_id, updated_at) VALUES {};", insert_rows_string);
        Ok(insert_statement)
    }
}
