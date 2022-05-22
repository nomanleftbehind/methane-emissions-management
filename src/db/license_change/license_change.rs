use crate::schema::license_change::{self, dsl::*};
use chrono::NaiveDateTime;
use juniper::{FieldResult, GraphQLInputObject, GraphQLObject};
use uuid::Uuid;

use crate::db::DbPool;
use actix_web::web::Data;

use diesel::prelude::*;
use diesel::{debug_query, delete, insert_into, pg::Pg};
use log::debug;

#[derive(GraphQLObject, Queryable, Clone)]
#[graphql(description = "License Change")]
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
#[graphql(description = "New license change input")]
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

impl LicenseChange {
    pub fn get_latest(connection: &PgConnection) -> QueryResult<LicenseChange> {
        use crate::schema::license_change::dsl::*;

        license_change
            .order(date.desc())
            .limit(1)
            .get_result(connection)
    }

    pub fn get_history(connection: &PgConnection) -> QueryResult<Vec<LicenseChange>> {
        use crate::schema::license_change::dsl::*;

        license_change
            .order(date.desc())
            .limit(20)
            .load::<LicenseChange>(connection)
    }

    pub fn insert(
        pool: &Data<DbPool>,
        license_change_form: NewLicenseChangeForm,
    ) -> FieldResult<LicenseChange> {
        let connection = pool.get()?;
        let query = insert_into(license_change).values(license_change_form);

        let sql = debug_query::<Pg, _>(&query).to_string();
        debug!("{}", sql);

        Ok(query.get_result(&connection)?)
    }

    pub fn insert_pg_connection(
        connection: &PgConnection,
        license_change_form: NewLicenseChangeForm,
    ) -> QueryResult<usize> {
        diesel::insert_into(license_change::table)
            .values(&license_change_form)
            .execute(connection)
    }

    pub fn delete(pool: &Data<DbPool>, key_id: Uuid) -> FieldResult<LicenseChange> {
        let connection = pool.get()?;
        let query = delete(license_change.find(key_id));

        let sql = debug_query::<Pg, _>(&query).to_string();
        debug!("{}", sql);

        Ok(query.get_result(&connection)?)
    }

    fn id(&self) -> Uuid {
        self.id.clone()
    }

    fn status(&self) -> String {
        self.status.clone()
    }

    fn substance(&self) -> String {
        self.substance.clone()
    }

    fn date(&self) -> NaiveDateTime {
        self.date.clone()
    }

    fn comment(&self) -> Option<String> {
        self.comment.clone()
    }

    fn link_to_documentation(&self) -> Option<String> {
        self.link_to_documentation.clone()
    }

    fn created_by_id(&self) -> Uuid {
        self.created_by_id.clone()
    }

    fn created_at(&self) -> NaiveDateTime {
        self.created_at.clone()
    }

    fn updated_by_id(&self) -> Uuid {
        self.updated_by_id.clone()
    }

    fn updated_at(&self) -> NaiveDateTime {
        self.updated_at.clone()
    }
}
