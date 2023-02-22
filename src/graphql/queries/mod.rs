use self::{
    compressor::CompressorQuery, compressor_month_vent::CompressorMonthVentQuery,
    controller::ControllerQuery, controller_month_vent::ControllerMonthVentQuery,
    facility::FacilityQuery, user::UserQuery,
};
use async_graphql::MergedObject;

mod compressor;
mod compressor_month_vent;
mod controller;
mod controller_month_vent;
mod facility;
mod user;

#[derive(MergedObject, Default, Clone)]
pub struct FullQuery(
    UserQuery,
    ControllerQuery,
    CompressorQuery,
    FacilityQuery,
    ControllerMonthVentQuery,
    CompressorMonthVentQuery,
);

pub(crate) fn full_query() -> FullQuery {
    FullQuery(
        UserQuery,
        ControllerQuery,
        CompressorQuery,
        FacilityQuery,
        ControllerMonthVentQuery,
        CompressorMonthVentQuery,
    )
}
