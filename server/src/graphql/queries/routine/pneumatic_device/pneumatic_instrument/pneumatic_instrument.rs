use crate::graphql::{
    context::ContextExt,
    models::{
        input::{GetPneumaticInstrumentSubtableInput, GetPneumaticInstrumentsInput},
        routine::pneumatic_device::pneumatic_instrument::{
            PneumaticInstrument, PneumaticInstrumentControlledCharacterization,
            PneumaticInstrumentEmissionRate, PneumaticInstrumentMonthHours,
        },
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

    async fn get_pneumatic_instrument_emission_rates(
        &self,
        ctx: &Context<'_>,
        get_pneumatic_instrument_emission_rates_input: GetPneumaticInstrumentSubtableInput,
    ) -> Result<Vec<PneumaticInstrumentEmissionRate>, Error> {
        let pool = ctx.db_pool();

        pneumatic_instrument::get_pneumatic_instrument_emission_rates(
            pool,
            get_pneumatic_instrument_emission_rates_input,
        )
        .await
        .map_err(Error::from)
    }

    async fn get_pneumatic_instrument_month_hours(
        &self,
        ctx: &Context<'_>,
        get_pneumatic_instrument_month_hours_input: GetPneumaticInstrumentSubtableInput,
    ) -> Result<Vec<PneumaticInstrumentMonthHours>, Error> {
        let pool = ctx.db_pool();

        pneumatic_instrument::get_pneumatic_instrument_month_hours(
            pool,
            get_pneumatic_instrument_month_hours_input,
        )
        .await
        .map_err(Error::from)
    }

    async fn get_pneumatic_instrument_controlled_characterizations(
        &self,
        ctx: &Context<'_>,
        get_pneumatic_instrument_controlled_characterizations_input: GetPneumaticInstrumentSubtableInput,
    ) -> Result<Vec<PneumaticInstrumentControlledCharacterization>, Error> {
        let pool = ctx.db_pool();

        pneumatic_instrument::get_pneumatic_instrument_controlled_characterizations(
            pool,
            get_pneumatic_instrument_controlled_characterizations_input,
        )
        .await
        .map_err(Error::from)
    }
}
