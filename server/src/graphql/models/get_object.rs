use super::{
    month_methane_emission::MonthMethaneEmission,
    routine::{
        compressor_seal::Compressor,
        defined_vent_gas::storage_tank::StorageTank,
        pneumatic_device::pneumatic_instrument::{
            PneumaticInstrument, PneumaticInstrumentChange, PneumaticInstrumentMonthHours,
            PneumaticInstrumentMonthMethaneEmissionOverride,
        },
    },
};
use async_graphql::SimpleObject;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(SimpleObject, Debug)]
pub struct GetObject {
    pub pneumatic_instruments: Option<Vec<PneumaticInstrument>>,
    pub compressors: Option<Vec<Compressor>>,
    pub storage_tanks: Option<Vec<StorageTank>>,
    pub pneumatic_instrument_changes: Option<Vec<PneumaticInstrumentChange>>,
    pub pneumatic_instrument_month_hours: Option<Vec<PneumaticInstrumentMonthHours>>,
    pub pneumatic_instrument_month_methane_emission_overrides:
        Option<Vec<PneumaticInstrumentMonthMethaneEmissionOverride>>,
    pub month_methane_emissions: Option<Vec<MonthMethaneEmission>>,
}

#[derive(SimpleObject, Debug, Clone, FromRow, PartialEq)]
pub struct IdSelection {
    pub id: Uuid,
    pub name: String,
}
