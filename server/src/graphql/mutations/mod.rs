use self::{
    compressor_blowdown::CompressorBlowdownMutation,
    // compressor_month_vent::CompressorMonthVentMutation,
    // controller::ControllerMutation,
    gas_analysis_calculated_param::GasAnalysisCalculatedParamMutation,
    // manual_mutation::ManualMutation,
    month_methane_emission::MonthMethaneEmissionMutation,
    // tank_farm_month_vent::TankFarmMonthVentMutation,
    // tank_farm_vent_factor_calculated::TankFarmVentFactorCalculatedMutation,
    user::UserMutation,
};
use async_graphql::MergedObject;

mod compressor_blowdown;
// mod compressor_month_vent;
// mod controller;
mod gas_analysis_calculated_param;
// mod manual_mutation;
mod month_methane_emission;
// mod tank_farm_month_vent;
// mod tank_farm_vent_factor_calculated;
mod user;

#[derive(MergedObject, Default, Clone)]
pub struct FullMutation(
    // ManualMutation,
    UserMutation,
    // ControllerMutation,
    MonthMethaneEmissionMutation,
    // CompressorMonthVentMutation,
    CompressorBlowdownMutation,
    // TankFarmVentFactorCalculatedMutation,
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
        // TankFarmVentFactorCalculatedMutation,
        // TankFarmMonthVentMutation,
        GasAnalysisCalculatedParamMutation,
    )
}
