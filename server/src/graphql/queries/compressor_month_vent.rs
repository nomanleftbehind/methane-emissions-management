use crate::graphql::{
    context::ContextExt,
    models::{CompressorMonthVent, CompressorMonthVentBy},
    sql::select_compressor_month_vents,
};
use async_graphql::{Context, Error, Object};

#[derive(Default, Clone)]
pub(super) struct CompressorMonthVentQuery;

#[Object]
impl CompressorMonthVentQuery {
    async fn compressor_month_vents(
        &self,
        ctx: &Context<'_>,
        by: CompressorMonthVentBy,
    ) -> Result<Vec<CompressorMonthVent>, Error> {
        let pool = ctx.db_pool();

        let compressor_month_vents = select_compressor_month_vents(pool, by)
            .await
            .map_err(Error::from);

        compressor_month_vents
    }
}
