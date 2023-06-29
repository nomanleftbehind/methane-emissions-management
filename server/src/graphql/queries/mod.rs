use self::{
    // get_object::GetObjectQuery,
    dropdown_selection::DropdownSelectionQuery,
    facility::FacilityQuery,
    month_methane_emission::MonthMethaneEmissionQuery,
    nonroutine::compressor_blowdown::CompressorBlowdownQuery,
    routine::pneumatic_device::DeviceManufacturerQuery,
    user::UserQuery,
};
use async_graphql::MergedObject;

mod facility;
mod month_methane_emission;
mod nonroutine;
mod routine;
// mod get_object;
mod dropdown_selection;
mod user;

#[derive(MergedObject, Default, Clone)]
pub struct FullQuery(
    // GetObjectQuery,
    UserQuery,
    FacilityQuery,
    CompressorBlowdownQuery,
    MonthMethaneEmissionQuery,
    DeviceManufacturerQuery,
    DropdownSelectionQuery,
);

pub(crate) fn full_query() -> FullQuery {
    FullQuery(
        // GetObjectQuery,
        UserQuery,
        FacilityQuery,
        CompressorBlowdownQuery,
        MonthMethaneEmissionQuery,
        DeviceManufacturerQuery,
        DropdownSelectionQuery,
    )
}
