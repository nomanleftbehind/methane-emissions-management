use super::resolver::{ControllerInput, ControllerObject, ControllerUpdateInput};
use crate::repository::schema::controllers;
use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const NEW_POST_USER_CACHE: &str = "newBlogPostofUser";

#[derive(Queryable, Debug, Serialize, Deserialize, PartialEq, Clone, Identifiable)]
#[table_name = "controllers"]
pub struct Controller {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub created_by_id: Uuid,
    pub updated_by_id: Uuid,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub function: Option<String>,
}

impl From<&Controller> for ControllerObject {
    fn from(oop: &Controller) -> Self {
        Self {
            id: oop.id.into(),
            created_by_id: oop.created_by_id.into(),
            updated_by_id: oop.updated_by_id.into(),
            created_at: oop.created_at.clone(),
            updated_at: oop.updated_at.clone(),
            manufacturer: oop.manufacturer.clone(),
            model: oop.model.clone(),
            serial_number: oop.serial_number.clone(),
            function: oop.function.clone(),
        }
    }
}

#[derive(Insertable, Serialize, AsChangeset, Deserialize, Debug, Clone, PartialEq)]
#[table_name = "controllers"]
pub struct ControllerForm {
    pub created_by_id: Uuid,
    pub updated_by_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub function: Option<String>,
}

impl From<&ControllerInput> for ControllerForm {
    fn from(f: &ControllerInput) -> Self {
        Self {
            created_by_id: f.created_by_id.parse::<Uuid>().expect(""),
            updated_by_id: f.updated_by_id.parse::<Uuid>().expect(""),
            created_at: f.created_at,
            updated_at: f.updated_at,
            manufacturer: f.manufacturer.clone(),
            model: f.model.clone(),
            serial_number: f.serial_number.clone(),
            function: f.function.clone(),
        }
    }
}

#[derive(Insertable, Serialize, AsChangeset, Deserialize, Debug, Clone, PartialEq)]
#[table_name = "controllers"]
pub struct ControllerUpdateForm {
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub function: Option<String>,
}

impl From<&ControllerUpdateInput> for ControllerUpdateForm {
    fn from(f: &ControllerUpdateInput) -> Self {
        Self {
            manufacturer: f.manufacturer.clone(),
            model: f.model.clone(),
            serial_number: f.serial_number.clone(),
            function: f.function.clone(),
        }
    }
}
