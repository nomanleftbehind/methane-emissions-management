use self::{
    compressor_month_vent::CompressorMonthVentMutations,
    controller_month_vent::ControllerMonthVentMutations, user::UserMutations,
};
use async_graphql::MergedObject;

pub mod compressor_month_vent;
pub mod controller_month_vent;
pub mod user;

#[derive(MergedObject, Default, Clone)]
pub struct FullMutation(
    pub UserMutations,
    pub ControllerMonthVentMutations,
    pub CompressorMonthVentMutations,
);

pub fn full_mutation() -> FullMutation {
    FullMutation(
        UserMutations,
        ControllerMonthVentMutations,
        CompressorMonthVentMutations,
    )
}
