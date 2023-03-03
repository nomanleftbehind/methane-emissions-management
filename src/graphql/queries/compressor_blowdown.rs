use crate::graphql::{
    context::ContextExt, models::CompressorBlowdownInterim, queries::FromToDateInput,
    sql::select_compressor_blowdowns_interim,
};
use async_graphql::{Context, Error, Object};

#[derive(Default, Clone)]
pub(super) struct CompressorBlowdownQuery;

#[Object]
impl CompressorBlowdownQuery {
    async fn compressor_blowdowns_interim(
        &self,
        ctx: &Context<'_>,
        date_range: FromToDateInput,
    ) -> Result<Vec<CompressorBlowdownInterim>, Error> {
        let fdc_client = ctx.fdc_client()?;

        let compressor_blowdowns_interim =
            select_compressor_blowdowns_interim(fdc_client, date_range)
                .await
                .map_err(Error::from);

        compressor_blowdowns_interim
    }
}
