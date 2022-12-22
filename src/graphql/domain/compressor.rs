use async_graphql::SimpleObject;
use chrono::{NaiveDate, NaiveDateTime};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(SimpleObject, Clone, FromRow, Debug)]
pub struct Compressor {
    pub id: Uuid,
    pub fdc_rec_id: String,
    pub facility_id: Uuid,
    pub name: String,
    pub serial_number: String,
    pub install_date: NaiveDate,
    pub remove_date: Option<NaiveDate>,
    pub created_by_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_by_id: Uuid,
    pub updated_at: NaiveDateTime,
}
