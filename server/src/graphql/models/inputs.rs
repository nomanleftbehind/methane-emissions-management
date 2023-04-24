use crate::graphql::models::MonthBeginningValidator;
use async_graphql::InputObject;
use chrono::{NaiveDate, NaiveDateTime};
use common::{DeleteEntryVariant, GetObjectVariant, UpdateFieldVariant};
use uuid::Uuid;

#[derive(Debug, InputObject)]
pub struct EmittersByInput {
    pub facility_id: Uuid,
}

#[derive(InputObject, Debug)]
pub struct FromToMonthInput {
    #[graphql(validator(custom = "MonthBeginningValidator"))]
    pub from_month: NaiveDate,
    #[graphql(validator(custom = "MonthBeginningValidator"))]
    pub to_month: NaiveDate,
}

#[derive(InputObject, Debug)]
pub struct UpdateFieldValue {
    pub string_value: Option<String>,
    pub integer_value: Option<i64>,
    pub float_value: Option<f64>,
    pub uuid_value: Option<Uuid>,
    pub naive_date_value: Option<NaiveDate>,
    pub naive_date_time_value: Option<NaiveDateTime>,
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
pub struct InsertControllerInput {
    pub fdc_rec_id: String,
    pub manufacturer_id: Uuid,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub application_id: Option<Uuid>,
    pub facility_id: Uuid,
}

#[derive(InputObject, Debug)]
pub struct InsertCompressorInput {
    pub fdc_rec_id: String,
    pub facility_id: Uuid,
    pub name: String,
    pub serial_number: String,
    pub install_date: NaiveDate,
    pub remove_date: Option<NaiveDate>,
}

#[derive(InputObject, Debug)]
pub struct InsertEntryInput {
    pub controller: Option<InsertControllerInput>,
    pub compressor: Option<InsertCompressorInput>,
}

#[derive(InputObject, Debug)]
pub struct MonthMethaneEmissionBySourceIdInput {
    pub source_id: Uuid,
}
