use self::{
    compressor::CompressorQuery, controller::ControllerQuery,
    controller_month_vent::ControllerMonthVentQuery, facility::FacilityQuery, user::UserQuery,
};
use async_graphql::MergedObject;

mod compressor;
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
);

pub(crate) fn full_query() -> FullQuery {
    FullQuery(
        UserQuery,
        ControllerQuery,
        CompressorQuery,
        FacilityQuery,
        ControllerMonthVentQuery,
    )
}
