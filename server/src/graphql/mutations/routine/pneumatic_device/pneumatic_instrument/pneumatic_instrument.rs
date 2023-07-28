use crate::graphql::{
    context::ContextExt,
    models::input::{InsertPneumaticInstrumentEmissionRateInput, InsertPneumaticInstrumentInput},
    sql::routine::pneumatic_device::pneumatic_instrument,
};
use async_graphql::{Context, Error, Object};

#[derive(Default, Clone)]
pub struct PneumaticInstrumentMutation;

#[Object]
impl PneumaticInstrumentMutation {
    async fn insert_pneumatic_instrument(
        &self,
        ctx: &Context<'_>,
        insert_pneumatic_instrument_input: InsertPneumaticInstrumentInput,
    ) -> Result<u64, Error> {
        let pool = ctx.db_pool();
        let cookie = ctx.get_cookie()?;
        let user_id = &ctx.get_session_manager()?.user_id(cookie).await?;

        pneumatic_instrument::insert_pneumatic_instrument(
            pool,
            user_id,
            insert_pneumatic_instrument_input,
        )
        .await
        .map_err(Error::from)
    }

    async fn insert_pneumatic_instrument_emission_rate(
        &self,
        ctx: &Context<'_>,
        insert_pneumatic_instrument_emission_rate_input: InsertPneumaticInstrumentEmissionRateInput,
    ) -> Result<u64, Error> {
        let pool = ctx.db_pool();
        let cookie = ctx.get_cookie()?;
        let user_id = &ctx.get_session_manager()?.user_id(cookie).await?;

        pneumatic_instrument::insert_pneumatic_instrument_emission_rate(
            pool,
            user_id,
            insert_pneumatic_instrument_emission_rate_input,
        )
        .await
        .map_err(Error::from)
    }
}
