use crate::graphql::{
    context::ContextExt,
    models::{GetObject, GetObjectInput},
    sql,
};
use async_graphql::{Context, Error, Object};

#[derive(Default, Clone)]
pub(super) struct GetObjectQuery;

#[Object]
impl GetObjectQuery {
    async fn get_object(
        &self,
        ctx: &Context<'_>,
        input: GetObjectInput,
    ) -> Result<GetObject, Error> {
        let pool = ctx.db_pool();
        let object = sql::get_object(pool, input).await.map_err(Error::from);

        object
    }
}
