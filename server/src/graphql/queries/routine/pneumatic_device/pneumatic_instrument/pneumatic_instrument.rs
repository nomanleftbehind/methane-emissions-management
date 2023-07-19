use crate::graphql::{
    context::ContextExt,
    models::{
        input::GetPneumaticInstrumentsInput,
        routine::pneumatic_device::pneumatic_instrument::PneumaticInstrument,
    },
    sql::routine::pneumatic_device::pneumatic_instrument,
};
use async_graphql::{Context, Error, Object};

#[derive(Default, Clone)]
pub struct PneumaticInstrumentQuery;

#[Object]
impl PneumaticInstrumentQuery {
    async fn get_pneumatic_instruments(
        &self,
        ctx: &Context<'_>,
        get_pneumatic_instruments_input: GetPneumaticInstrumentsInput,
    ) -> Result<Vec<PneumaticInstrument>, Error> {
        let pool = ctx.db_pool();

        pneumatic_instrument::get_pneumatic_instruments(pool, get_pneumatic_instruments_input)
            .await
            .map_err(Error::from)
    }
}