use crate::graphql::{
    context::ContextExt,
    models::{Controller, ControllerApplication, EmittersByInput},
    sql::{query_all_controller_applications, query_controllers, query_user_controllers},
};
use async_graphql::{Context, Error, Object};

#[derive(Default, Clone)]
pub(super) struct ControllerQuery;

#[Object]
impl ControllerQuery {
    async fn user_controllers(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = 10)] limit: i64,
        #[graphql(default = 0)] offset: i64,
    ) -> Result<Vec<Controller>, Error> {
        let pool = ctx.db_pool();

        let cookie = ctx.get_cookie()?;
        let user_id = ctx.get_session_manager()?.user_id(cookie).await?;

        let controllers = query_user_controllers(pool, user_id, limit, offset)
            .await
            .map_err(Error::from);

        controllers
    }

    async fn controllers_by(
        &self,
        ctx: &Context<'_>,
        by: EmittersByInput,
    ) -> Result<Vec<Controller>, Error> {
        let pool = ctx.db_pool();
        let controllers = query_controllers(pool, by).await.map_err(Error::from);

        controllers
    }

    async fn all_controller_applications(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<ControllerApplication>, Error> {
        let pool = ctx.db_pool();
        let controller_applications = query_all_controller_applications(pool)
            .await
            .map_err(Error::from);

        controller_applications
    }
}
