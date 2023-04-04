use self::{
    compressor::CompressorQuery, compressor_blowdown::CompressorBlowdownQuery,
    compressor_month_vent::CompressorMonthVentQuery, controller::ControllerQuery,
    controller_change::ControllerChangeQuery, controller_month_vent::ControllerMonthVentQuery,
    facility::FacilityQuery, get_object::GetObjectQuery, tank_farm::TankFarmQuery,
    tank_farm_month_vent::TankFarmMonthVentQuery, user::UserQuery,
};
use async_graphql::MergedObject;

mod compressor;
mod compressor_blowdown;
mod compressor_month_vent;
mod controller;
mod controller_change;
mod controller_month_vent;
mod facility;
mod get_object;
mod tank_farm;
mod tank_farm_month_vent;
mod user;

#[derive(MergedObject, Default, Clone)]
pub struct FullQuery(
    GetObjectQuery,
    UserQuery,
    FacilityQuery,
    ControllerQuery,
    ControllerChangeQuery,
    CompressorQuery,
    TankFarmQuery,
    CompressorBlowdownQuery,
    ControllerMonthVentQuery,
    CompressorMonthVentQuery,
    TankFarmMonthVentQuery,
);

pub(crate) fn full_query() -> FullQuery {
    FullQuery(
        GetObjectQuery,
        UserQuery,
        FacilityQuery,
        ControllerQuery,
        ControllerChangeQuery,
        CompressorQuery,
        TankFarmQuery,
        CompressorBlowdownQuery,
        ControllerMonthVentQuery,
        CompressorMonthVentQuery,
        TankFarmMonthVentQuery,
    )
}
