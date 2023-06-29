use self::{
    gas_analysis::GasAnalysisCalculatedParamMutation,
    // manual_mutation::ManualMutation,
    month_methane_emission::MonthMethaneEmissionMutation,
    nonroutine::compressor_blowdown::CompressorBlowdownMutation,
    routine::defined_vent_gas::storage_tank::StorageTankGasInSolutionFactorCalculatedMutation,
    user::UserMutation,
};
use async_graphql::MergedObject;

// mod manual_mutation;
mod gas_analysis;
mod month_methane_emission;
mod nonroutine;
mod routine;
mod user;

#[derive(MergedObject, Default, Clone)]
pub struct FullMutation(
    // ManualMutation,
    UserMutation,
    MonthMethaneEmissionMutation,
    CompressorBlowdownMutation,
    StorageTankGasInSolutionFactorCalculatedMutation,
    GasAnalysisCalculatedParamMutation,
);

pub(crate) fn full_mutation() -> FullMutation {
    FullMutation(
        // ManualMutation,
        UserMutation,
        MonthMethaneEmissionMutation,
        CompressorBlowdownMutation,
        StorageTankGasInSolutionFactorCalculatedMutation,
        GasAnalysisCalculatedParamMutation,
    )
}
