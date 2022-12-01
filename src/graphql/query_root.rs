use super::{
    domain::{
        ControllerApplication, ControllersBy, Facility, FacilityBy, LimitOffsetInput, User, UserBy,
    },
    sql::{query_all_controller_applications, query_all_facilities, query_facilities, query_user},
};
use crate::graphql::{
    context::ContextExt,
    domain::Controller,
    sql::{query_all_users, query_controllers, query_user_by_id, query_user_posts},
};
use async_graphql::*;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn me(&self, ctx: &Context<'_>) -> Result<Option<User>> {
        let cookie = ctx.get_cookie();

        match cookie {
            Err(_) => Ok(None),
            Ok(cookie) => {
                let user_id = ctx.get_session_manager()?.user_id(cookie).await?;
                let user = query_user_by_id(ctx.db_pool(), user_id)
                    .await
                    .map_err(Error::from);

                user
            }
        }
    }

    async fn user(&self, ctx: &Context<'_>, by: UserBy) -> Result<Option<User>> {
        let pool = ctx.db_pool();
        query_user(pool, by).await.map_err(Error::from)
    }

    async fn user_controllers(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = 10)] limit: i64,
        #[graphql(default = 0)] offset: i64,
    ) -> Result<Vec<Controller>> {
        let pool = ctx.db_pool();

        let cookie = ctx.get_cookie()?;
        let user_id = ctx.get_session_manager()?.user_id(cookie).await?;

        let posts = query_user_posts(pool, user_id, limit, offset)
            .await
            .map_err(Error::from);

        posts
    }

    async fn controllers_by(
        &self,
        ctx: &Context<'_>,
        by: ControllersBy,
    ) -> Result<Vec<Controller>> {
        let pool = ctx.db_pool();
        let controllers = query_controllers(pool, by).await.map_err(Error::from);

        controllers
    }

    async fn facilities_by(
        &self,
        ctx: &Context<'_>,
        by: FacilityBy,
        #[graphql(default = 10)] limit: i64,
        #[graphql(default = 0)] offset: i64,
    ) -> Result<Vec<Facility>> {
        let pool = ctx.db_pool();

        let facilities = query_facilities(pool, by, limit, offset)
            .await
            .map_err(Error::from);

        facilities
    }

    async fn all_facilities(
        &self,
        ctx: &Context<'_>,
        limit_offset_input: Option<LimitOffsetInput>,
    ) -> Result<Vec<Facility>> {
        let pool = ctx.db_pool();
        let facilities = query_all_facilities(pool, limit_offset_input)
            .await
            .map_err(Error::from);
        facilities
    }

    async fn all_users(&self, ctx: &Context<'_>, test_str: String) -> Result<Vec<User>> {
        println!("test string: {}", test_str);

        let pool = ctx.db_pool();

        let users = query_all_users(pool).await.map_err(Error::from);

        users
    }

    async fn all_controller_applications(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<ControllerApplication>> {
        let pool = ctx.db_pool();
        let controller_applications = query_all_controller_applications(pool)
            .await
            .map_err(Error::from);

        controller_applications
    }
}
