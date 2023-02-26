use self::{
    compressor_month_vent::CompressorMonthVentMutation,
    controller_month_vent::ControllerMonthVentMutation,
    gas_analysis_calculated_param::GasAnalysisCalculatedParamMutation,
    tank_farm_month_vent::TankFarmMonthVentMutation,
    tank_farm_vent_factor_calculated::TankFarmVentFactorCalculatedMutation, user::UserMutation,
};
use async_graphql::MergedObject;

mod compressor_month_vent;
mod controller_month_vent;
mod gas_analysis_calculated_param;
mod tank_farm_month_vent;
mod tank_farm_vent_factor_calculated;
mod user;
mod validators;

#[derive(MergedObject, Default, Clone)]
pub struct FullMutation(
    UserMutation,
    ControllerMonthVentMutation,
    CompressorMonthVentMutation,
    TankFarmVentFactorCalculatedMutation,
    TankFarmMonthVentMutation,
    GasAnalysisCalculatedParamMutation,
);

pub(crate) fn full_mutation() -> FullMutation {
    FullMutation(
        UserMutation,
        ControllerMonthVentMutation,
        CompressorMonthVentMutation,
        TankFarmVentFactorCalculatedMutation,
        TankFarmMonthVentMutation,
        GasAnalysisCalculatedParamMutation,
    )
}
