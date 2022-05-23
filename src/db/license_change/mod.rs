mod repository;
pub use repository::Repository;

use crate::db::user::{self, User};
use crate::schema::license_change;
use crate::schemas::root::Context;
use chrono::NaiveDateTime;
use juniper::{graphql_object, FieldResult, GraphQLInputObject};
use uuid::Uuid;

#[derive(Queryable)]
#[diesel(belongs_to(parent = "User", foreign_key = "created_by_id"))]
pub struct LicenseChange {
    id: Uuid,
    status: String,
    substance: String,
    date: NaiveDateTime,
    comment: Option<String>,
    link_to_documentation: Option<String>,
    created_by_id: Uuid,
    created_at: NaiveDateTime,
    updated_by_id: Uuid,
    updated_at: NaiveDateTime,
}

#[derive(GraphQLInputObject, Insertable)]
#[table_name = "license_change"]
pub struct NewLicenseChangeForm {
    status: String,
    substance: String,
    date: NaiveDateTime,
    comment: Option<String>,
    link_to_documentation: Option<String>,
    // This is a temporary solution. The goal is to have this generated from user session
    created_by_id: Uuid,
    updated_by_id: Uuid,
}

#[graphql_object(context = Context)]
impl LicenseChange {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn status(&self) -> &str {
        &self.status
    }

    fn substance(&self) -> &str {
        &self.substance
    }

    fn date(&self) -> &NaiveDateTime {
        &self.date
    }

    fn comment(&self) -> &Option<String> {
        &self.comment
    }

    fn link_to_documentation(&self) -> &Option<String> {
        &self.link_to_documentation
    }

    fn created_by_id(&self) -> &Uuid {
        &self.created_by_id
    }

    fn created_at(&self) -> &NaiveDateTime {
        &self.created_at
    }

    fn updated_by_id(&self) -> &Uuid {
        &self.updated_by_id
    }

    fn updated_at(&self) -> &NaiveDateTime {
        &self.updated_at
    }

    fn created_by(&self, context: &Context) -> FieldResult<User> {
        user::Repository::find_by_id(&context.db_pool, &self.created_by_id)
    }
}
