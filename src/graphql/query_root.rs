use super::{
    domain::{ControllerFunction, User, UserBy},
    sql::{query_all_controller_functions, query_user},
};
use crate::graphql::{
    context::ContextExt,
    domain::Controller,
    sql::{query_all_users, query_user_by_id, query_user_posts},
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

    async fn all_users(&self, ctx: &Context<'_>, test_str: String) -> Result<Vec<User>> {
        println!("test string: {}", test_str);

        let pool = ctx.db_pool();

        let users = query_all_users(pool).await.map_err(Error::from);

        users
    }

    async fn all_controller_functions(&self, ctx: &Context<'_>) -> Result<Vec<ControllerFunction>> {
        let pool = ctx.db_pool();
        let controller_functions = query_all_controller_functions(pool)
            .await
            .map_err(Error::from);

        controller_functions
    }
}
