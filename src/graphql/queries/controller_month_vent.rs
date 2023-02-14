use crate::graphql::{
    context::ContextExt, domain::ControllerMonthVent, sql::query_all_controller_month_vents,
};
use async_graphql::{Context, Error, Object};

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

        let calculated_controller_month_vents = query_all_controller_month_vents(pool)
            .await
            .map_err(Error::from)?;

        let stc = calculated_controller_month_vents
            .into_iter()
            .map(|calculated_controller_month_vent| {
                (calculated_controller_month_vent, user_id).into()
            })
            .collect::<Vec<_>>();

        let y: String = stc.into_iter().map(|c: ControllerMonthVent| {
            
            format!("({}, {}, {}, {}),", c.id, c.controller_id, c.created_by_id, c.month)
        }).collect();

        // Ok(stc)
        Ok(y)
    }
}
