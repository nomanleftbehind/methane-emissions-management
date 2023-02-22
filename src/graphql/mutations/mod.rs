use self::{
    compressor_month_vent::CompressorMonthVentMutation,
    controller_month_vent::ControllerMonthVentMutation, user::UserMutation,
};
use async_graphql::MergedObject;

mod compressor_month_vent;
mod controller_month_vent;
mod user;

#[derive(MergedObject, Default, Clone)]
pub struct FullMutation(
    UserMutation,
    ControllerMonthVentMutation,
    CompressorMonthVentMutation,
);

pub(crate) fn full_mutation() -> FullMutation {
    FullMutation(
        UserMutation,
        ControllerMonthVentMutation,
        CompressorMonthVentMutation,
    )
}
