use crate::graphql::models::{
    compressor::Compressor, pneumatic_device::NonLevelController, Facility, User,
};
use async_graphql::Interface;
use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Interface)]
#[graphql(
    field(name = "id", type = "&Uuid"),
    field(name = "fdc_rec_id", type = "String"),
    field(name = "facility", type = "Result<Option<Facility>, Error>"),
    field(name = "created_by", type = "Result<Option<User>, Error>"),
    field(name = "created_at", type = "&NaiveDateTime"),
    field(name = "updated_by", type = "Result<Option<User>, Error>"),
    field(name = "updated_at", type = "&NaiveDateTime")
)]
pub enum EmitterInterface {
    Controller(NonLevelController),
    Compressor(Compressor),
}
