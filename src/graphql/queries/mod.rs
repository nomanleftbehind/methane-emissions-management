use self::{
    compressor::CompressorQueries, controller::ControllerQueries, facility::FacilityQueries,
    user::UserQueries,
};
use async_graphql::MergedObject;

pub mod compressor;
pub mod controller;
pub mod facility;
pub mod user;

#[derive(MergedObject, Default, Clone)]
pub struct FullQuery(
    pub UserQueries,
    pub ControllerQueries,
    pub CompressorQueries,
    pub FacilityQueries,
);

pub fn full_query() -> FullQuery {
    FullQuery(
        UserQueries,
        ControllerQueries,
        CompressorQueries,
        FacilityQueries,
    )
}
