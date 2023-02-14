use self::{
    compressor::CompressorQueries, controller::ControllerQueries,
    controller_month_vent::ControllerMonthVentQueries, facility::FacilityQueries,
    user::UserQueries,
};
use async_graphql::MergedObject;

pub mod compressor;
pub mod controller;
pub mod controller_month_vent;
pub mod facility;
pub mod user;

#[derive(MergedObject, Default, Clone)]
pub struct FullQuery(
    pub UserQueries,
    pub ControllerQueries,
    pub ControllerMonthVentQueries,
    pub CompressorQueries,
    pub FacilityQueries,
);

pub fn full_query() -> FullQuery {
    FullQuery(
        UserQueries,
        ControllerQueries,
        ControllerMonthVentQueries,
        CompressorQueries,
        FacilityQueries,
    )
}
