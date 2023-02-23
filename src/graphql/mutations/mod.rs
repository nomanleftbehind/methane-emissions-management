use self::{
    compressor_month_vent::CompressorMonthVentMutation,
    controller_month_vent::ControllerMonthVentMutation,
    gas_analysis_calculated_param::GasAnalysisCalculatedParamMutation, user::UserMutation,
};
use async_graphql::MergedObject;

mod compressor_month_vent;
mod controller_month_vent;
mod gas_analysis_calculated_param;
mod user;

#[derive(MergedObject, Default, Clone)]
pub struct FullMutation(
    UserMutation,
    ControllerMonthVentMutation,
    CompressorMonthVentMutation,
    GasAnalysisCalculatedParamMutation,
);

pub(crate) fn full_mutation() -> FullMutation {
    FullMutation(
        UserMutation,
        ControllerMonthVentMutation,
        CompressorMonthVentMutation,
        GasAnalysisCalculatedParamMutation,
    )
}
