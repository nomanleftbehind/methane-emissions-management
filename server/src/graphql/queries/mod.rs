use self::{
    compressor::CompressorQuery, compressor_blowdown::CompressorBlowdownQuery,
    compressor_month_vent::CompressorMonthVentQuery, controller::ControllerQuery,
    controller_month_vent::ControllerMonthVentQuery, facility::FacilityQuery,
    tank_farm_month_vent::TankFarmMonthVentQuery, user::UserQuery,
};
use crate::graphql::mutations::validators::MonthBeginningValidator;
use async_graphql::{InputObject, MergedObject};
use chrono::NaiveDate;

mod compressor;
mod compressor_blowdown;
mod compressor_month_vent;
mod controller;
mod controller_month_vent;
mod facility;
mod tank_farm_month_vent;
mod user;

#[derive(MergedObject, Default, Clone)]
pub struct FullQuery(
    UserQuery,
    FacilityQuery,
    ControllerQuery,
    CompressorQuery,
    CompressorBlowdownQuery,
    ControllerMonthVentQuery,
    CompressorMonthVentQuery,
    TankFarmMonthVentQuery,
);

pub(crate) fn full_query() -> FullQuery {
    FullQuery(
        UserQuery,
        FacilityQuery,
        ControllerQuery,
        CompressorQuery,
        CompressorBlowdownQuery,
        ControllerMonthVentQuery,
        CompressorMonthVentQuery,
        TankFarmMonthVentQuery,
    )
}

#[derive(InputObject, Debug)]
pub struct FromToMonthInput {
    #[graphql(validator(custom = "MonthBeginningValidator"))]
    pub from_month: NaiveDate,
    #[graphql(validator(custom = "MonthBeginningValidator"))]
    pub to_month: NaiveDate,
}
