use crate::graphql::{
    context::ContextExt,
    models::{User, UserBy},
    sql::{query_all_users, query_user, query_user_by_id},
};
use async_graphql::{Context, Error, Object};

#[derive(Default, Clone)]
pub(super) struct UserQuery;

#[Object]
impl UserQuery {
    async fn me(&self, ctx: &Context<'_>) -> Result<Option<User>, Error> {
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

    async fn user(&self, ctx: &Context<'_>, by: UserBy) -> Result<Option<User>, Error> {
        let pool = ctx.db_pool();
        query_user(pool, by).await.map_err(Error::from)
    }

    async fn all_users(&self, ctx: &Context<'_>) -> Result<Vec<User>, Error> {
        let pool = ctx.db_pool();

        let users = query_all_users(pool).await.map_err(Error::from);

        users
    }
}
