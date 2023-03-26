use crate::graphql::models::MonthBeginningValidator;
use async_graphql::InputObject;
use chrono::{NaiveDate, NaiveDateTime};
use common::UpdateFieldVariant;
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
pub struct UpdateFieldInput {
    pub id: Uuid,
    pub update_field_variant: UpdateFieldVariant,
    pub value: UpdateFieldValue,
}
