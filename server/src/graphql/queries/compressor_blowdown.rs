use crate::graphql::{
    context::ContextExt, models::nonroutine::compressor_blowdown::CompressorBlowdown,
    sql::select_compressor_blowdowns,
};
use async_graphql::{Context, Error, Object};
use uuid::Uuid;

#[derive(Default, Clone)]
pub(super) struct CompressorBlowdownQuery;

#[Object]
impl CompressorBlowdownQuery {
    async fn compressor_blowdowns(
        &self,
        ctx: &Context<'_>,
        compressor_id: Uuid,
    ) -> Result<Vec<CompressorBlowdown>, Error> {
        let pool = ctx.db_pool();

        let compressor_blowdowns = select_compressor_blowdowns(pool, compressor_id)
            .await
            .map_err(Error::from);

        compressor_blowdowns
    }
}
