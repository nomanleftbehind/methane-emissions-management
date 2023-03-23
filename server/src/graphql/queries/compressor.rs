use async_graphql::{Context, Error, Object};

use crate::graphql::{
    context::ContextExt,
    models::{Compressor, EmittersByInput},
    sql::query_compressors,
};

#[derive(Default, Clone)]
pub(super) struct CompressorQuery;

#[Object]
impl CompressorQuery {
    async fn compressors_by(
        &self,
        ctx: &Context<'_>,
        by: EmittersByInput,
    ) -> Result<Vec<Compressor>, Error> {
        let pool = ctx.db_pool();
        let compressors = query_compressors(pool, by).await.map_err(Error::from);

        compressors
    }
}
