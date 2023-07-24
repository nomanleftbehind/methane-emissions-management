use super::super::validator::MonthBeginningValidator;
use crate::graphql::models::validator::MonthRangeValidator;
use async_graphql::InputObject;
use chrono::{NaiveDate, NaiveDateTime};
use common::{
    CalculationMethod, CompressorSealTestingPoint, CompressorType, ControlDevice,
    ControlDeviceInactivityReason, DeleteEntryVariant, DropdownSelectionVariant, FacilityType,
    GetObjectVariant, MonthMethaneEmissionsByVariant, PneumaticInstrumentChangesByVariant,
    PneumaticInstrumentType, PneumaticInstrumentsByVariant, SealType, SiteType, UpdateFieldVariant,
};
use uuid::Uuid;

#[derive(Debug, InputObject)]
pub struct EmittersByInput {
    pub facility_id: Uuid,
}

#[derive(InputObject, Debug)]
pub struct MonthRangeInput {
    #[graphql(validator(custom = "MonthBeginningValidator"))]
    pub from_month: NaiveDate,
    #[graphql(validator(custom = "MonthBeginningValidator"))]
    pub to_month: NaiveDate,
}

#[derive(InputObject, Debug)]
pub struct UpdateFieldValue {
    pub string_value: Option<String>,
    pub integer_value: Option<i32>,
    pub float_value: Option<f64>,
    pub uuid_value: Option<Uuid>,
    pub bool_value: Option<bool>,
    pub naive_date_value: Option<NaiveDate>,
    pub naive_date_time_value: Option<NaiveDateTime>,
    pub facility_type_value: Option<FacilityType>,
    pub site_type_value: Option<SiteType>,
    pub pneumatic_instrument_type_value: Option<PneumaticInstrumentType>,
    pub compressor_type_value: Option<CompressorType>,
    pub control_device_value: Option<ControlDevice>,
    pub control_device_inactivity_reason_value: Option<ControlDeviceInactivityReason>,
    pub seal_type_value: Option<SealType>,
    pub compressor_seal_testing_point_value: Option<CompressorSealTestingPoint>,
    pub calculation_method_value: Option<CalculationMethod>,
}

#[derive(InputObject, Debug)]
pub struct GetObjectInput {
    pub id: Uuid,
    pub get_object_variant: GetObjectVariant,
}

#[derive(InputObject, Debug)]
pub struct UpdateFieldInput {
    pub id: Uuid,
    pub update_field_variant: UpdateFieldVariant,
    pub value: UpdateFieldValue,
}

#[derive(InputObject, Debug)]
pub struct DeleteEntryInput {
    pub id: Uuid,
    pub delete_entry_variant: DeleteEntryVariant,
}

#[derive(Debug, InputObject)]
pub struct ControllerChangeInput {
    pub controller_id: Uuid,
}

#[derive(InputObject, Debug)]
pub struct InsertPneumaticInstrumentInput {
    pub site_id: Uuid,
    pub r#type: PneumaticInstrumentType,
    pub manufacturer_id: Uuid,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
}

#[derive(InputObject, Debug)]
pub struct InsertPneumaticInstrumentChangeInput {
    pub pneumatic_instrument_id: Uuid,
    pub date: NaiveDate,
    pub rate: f64,
}

#[derive(InputObject, Debug)]
pub struct InsertCompressorInput {
    pub site_id: Uuid,
    pub fdc_rec_id: String,
    pub r#type: CompressorType,
    pub name: String,
    pub serial_number: String,
    pub power: f64,
    pub throw_count: Option<i32>,
    pub install_date: NaiveDate,
    pub remove_date: Option<NaiveDate>,
}

#[derive(InputObject, Debug)]
pub struct InsertEntryInput {
    pub pneumatic_instrument: Option<InsertPneumaticInstrumentInput>,
    pub compressor: Option<InsertCompressorInput>,
}

#[derive(InputObject, Debug)]
pub struct MonthMethaneEmissionBySourceIdInput {
    pub source_id: Uuid,
}

#[derive(InputObject, Debug)]
pub struct GetMonthMethaneEmissionsInput {
    pub by: MonthMethaneEmissionsByVariant,
    pub id: Uuid,
    #[graphql(validator(custom = "MonthRangeValidator::new(12)"))]
    pub month_range: MonthRangeInput,
}

#[derive(InputObject, Debug)]
pub struct GetPneumaticInstrumentsInput {
    pub by: PneumaticInstrumentsByVariant,
    pub id: Uuid,
}

#[derive(InputObject, Debug)]
pub struct GetPneumaticInstrumentChangesInput {
    pub by: PneumaticInstrumentChangesByVariant,
    pub id: Uuid,
}

#[derive(InputObject, Debug)]
pub struct GetDropdownSelectionInput {
    pub variant: DropdownSelectionVariant,
    pub id: Option<Uuid>,
}
