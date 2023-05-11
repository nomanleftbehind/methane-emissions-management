use self::{
    compressor_blowdown::CompressorBlowdownMutation,
    // compressor_month_vent::CompressorMonthVentMutation,
    // controller::ControllerMutation,
    gas_analysis::GasAnalysisCalculatedParamMutation,
    // manual_mutation::ManualMutation,
    month_methane_emission::MonthMethaneEmissionMutation,
    // tank_farm_month_vent::TankFarmMonthVentMutation,
    routine::defined_vent_gas::storage_tank::StorageTankGasInSolutionFactorCalculatedMutation,
    user::UserMutation,
};
use async_graphql::MergedObject;

mod compressor_blowdown;
// mod compressor_month_vent;
// mod controller;
// mod manual_mutation;
pub mod gas_analysis;
mod month_methane_emission;
// mod tank_farm_month_vent;
pub mod routine;
mod user;

#[derive(MergedObject, Default, Clone)]
pub struct FullMutation(
    // ManualMutation,
    UserMutation,
    // ControllerMutation,
    MonthMethaneEmissionMutation,
    // CompressorMonthVentMutation,
    CompressorBlowdownMutation,
    StorageTankGasInSolutionFactorCalculatedMutation,
    // TankFarmMonthVentMutation,
    GasAnalysisCalculatedParamMutation,
);

pub(crate) fn full_mutation() -> FullMutation {
    FullMutation(
        // ManualMutation,
        UserMutation,
        // ControllerMutation,
        MonthMethaneEmissionMutation,
        // CompressorMonthVentMutation,
        CompressorBlowdownMutation,
        StorageTankGasInSolutionFactorCalculatedMutation,
        // TankFarmMonthVentMutation,
        GasAnalysisCalculatedParamMutation,
    )
}
