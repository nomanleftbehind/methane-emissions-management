use self::{
    gas_analysis::GasAnalysisCalculatedParamMutation,
    manual_mutation::ManualMutation,
    month_methane_emission::MonthMethaneEmissionMutation,
    nonroutine::compressor_blowdown::CompressorBlowdownMutation,
    routine::{
        defined_vent_gas::storage_tank::StorageTankGasInSolutionFactorCalculatedMutation,
        pneumatic_device::pneumatic_instrument::PneumaticInstrumentMutation,
    },
    user::UserMutation,
};
use async_graphql::MergedObject;

mod gas_analysis;
mod manual_mutation;
mod month_methane_emission;
mod nonroutine;
mod routine;
mod user;

#[derive(MergedObject, Default, Clone)]
pub struct FullMutation(
    ManualMutation,
    UserMutation,
    PneumaticInstrumentMutation,
    MonthMethaneEmissionMutation,
    CompressorBlowdownMutation,
    StorageTankGasInSolutionFactorCalculatedMutation,
    GasAnalysisCalculatedParamMutation,
);

pub(crate) fn full_mutation() -> FullMutation {
    FullMutation(
        ManualMutation,
        UserMutation,
        PneumaticInstrumentMutation,
        MonthMethaneEmissionMutation,
        CompressorBlowdownMutation,
        StorageTankGasInSolutionFactorCalculatedMutation,
        GasAnalysisCalculatedParamMutation,
    )
}
