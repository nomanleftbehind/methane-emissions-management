use self::{
    compressor_blowdown::CompressorBlowdownQuery,
    // compressor_month_vent::CompressorMonthVentQuery,
    // controller_application::ControllerApplicationQuery,
    // controller_manufacturer::ControllerManufacturerQuery,
    facility::FacilityQuery,
    // get_object::GetObjectQuery,
    // id_selection::IdSelectionQuery,
    month_methane_emission::MonthMethaneEmissionQuery,
    // tank_farm_month_vent::TankFarmMonthVentQuery,
    user::UserQuery,
};
use async_graphql::MergedObject;

mod compressor_blowdown;
// mod compressor_month_vent;
// mod controller_application;
// mod controller_manufacturer;
mod facility;
// mod get_object;
// mod id_selection;
mod month_methane_emission;
// mod tank_farm_month_vent;
mod user;

#[derive(MergedObject, Default, Clone)]
pub struct FullQuery(
    // GetObjectQuery,
    UserQuery,
    FacilityQuery,
    CompressorBlowdownQuery,
    MonthMethaneEmissionQuery,
    // CompressorMonthVentQuery,
    // TankFarmMonthVentQuery,
    // IdSelectionQuery,
    // ControllerManufacturerQuery,
    // ControllerApplicationQuery,
);

pub(crate) fn full_query() -> FullQuery {
    FullQuery(
        // GetObjectQuery,
        UserQuery,
        FacilityQuery,
        CompressorBlowdownQuery,
        MonthMethaneEmissionQuery,
        // CompressorMonthVentQuery,
        // TankFarmMonthVentQuery,
        // IdSelectionQuery,
        // ControllerManufacturerQuery,
        // ControllerApplicationQuery,
    )
}
