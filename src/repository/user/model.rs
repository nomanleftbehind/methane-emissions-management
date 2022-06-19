use super::resolver::User;
use crate::repository::schema::users;
use crate::utils::user_utils::Role;
use async_graphql::SimpleObject;
use chrono::NaiveDateTime;
use diesel::AsChangeset;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

// Database Models
#[derive(Identifiable, Debug, Clone, PartialEq, Serialize, Deserialize, Queryable)]
#[table_name = "users"]
pub struct UserObject {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub location: Option<String>,
    pub email: String,
    pub hash: String,
    pub role: String,
}
///  User Query Related Classes
#[derive(
    Insertable, Deserialize, SimpleObject, Serialize, Debug, AsChangeset, Clone, PartialEq,
)]
#[table_name = "users"]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub location: Option<String>,
    pub email: String,
    pub hash: String,
    pub role: String,
}

impl From<&UserObject> for User {
    fn from(oop: &UserObject) -> Self {
        User {
            id: oop.id.into(),
            created_at: oop.created_at.clone(),
            location: oop.location.clone(),
            username: oop.username.clone(),
            first_name: oop.first_name.clone(),
            last_name: oop.last_name.clone(),
            email: oop.email.clone(),
            hash: oop.hash.clone(),
            role: Role::from_str(oop.role.as_str())
                .expect("Str to Role Conversion Error")
                .to_string(),
        }
    }
}
