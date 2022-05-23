use crate::db::license_change::{LicenseChange, NewLicenseChangeForm};
use crate::db::DbPool;

use diesel::{
    debug_query,
    dsl::{delete, insert_into},
    pg::Pg,
    prelude::*,
};

use crate::schema::license_change::dsl::*;
use actix_web::web::Data;
use juniper::FieldResult;
use log::debug;
use uuid::Uuid;

pub struct Repository;

impl Repository {
    pub fn all(pool: &Data<DbPool>) -> FieldResult<Vec<LicenseChange>> {
        let connection = pool.get()?;
        let query = license_change.order(date.asc());

        let sql = debug_query::<Pg, _>(&query).to_string();
        debug!("{}", sql);

        Ok(query.get_results(&connection)?)
    }

    pub fn any(pool: &Data<DbPool>, keys: &[Uuid]) -> FieldResult<Vec<LicenseChange>> {
        let connection = pool.get()?;
        let query = license_change.filter(id.eq_any(keys));

        let sql = debug_query::<Pg, _>(&query).to_string();
        debug!("{}", sql);

        Ok(query.get_results(&connection)?)
    }

    pub fn any_user_license_changes(
        pool: &Data<DbPool>,
        keys: &[Uuid],
    ) -> FieldResult<Vec<LicenseChange>> {
        let connection = pool.get()?;
        let query = license_change.filter(created_by_id.eq_any(keys));

        let sql = debug_query::<Pg, _>(&query).to_string();
        debug!("{}", sql);

        Ok(query.get_results(&connection)?)
    }

    pub fn find_by_id(pool: &Data<DbPool>, key_id: Uuid) -> FieldResult<LicenseChange> {
        let connection = pool.get()?;
        let query = license_change.find(key_id);

        let sql = debug_query::<Pg, _>(&query).to_string();
        debug!("{}", sql);

        Ok(query.get_result(&connection)?)
    }

    pub fn find_by_user_id(
        pool: &Data<DbPool>,
        key_user_id: &Uuid,
    ) -> FieldResult<Vec<LicenseChange>> {
        let connection = pool.get()?;
        let query = license_change.filter(created_by_id.eq(key_user_id));

        let sql = debug_query::<Pg, _>(&query).to_string();
        debug!("{}", sql);

        Ok(query.get_results(&connection)?)
    }

    pub fn find_user_license_changes(
        pool: &Data<DbPool>,
        key_user_id: &Uuid,
    ) -> FieldResult<Vec<LicenseChange>> {
        let connection = pool.get()?;
        let query = license_change
            .filter(created_by_id.eq(key_user_id))
            .order(date.asc());

        let sql = debug_query::<Pg, _>(&query).to_string();
        debug!("{}", sql);

        Ok(query.get_results(&connection)?)
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

    pub fn delete(pool: &Data<DbPool>, key_id: Uuid) -> FieldResult<LicenseChange> {
        let connection = pool.get()?;
        let query = delete(license_change.find(key_id));

        let sql = debug_query::<Pg, _>(&query).to_string();
        debug!("{}", sql);

        Ok(query.get_result(&connection)?)
    }

    pub fn delete_by_user_id(
        pool: &Data<DbPool>,
        key_user_id: &Uuid,
    ) -> FieldResult<Vec<LicenseChange>> {
        let connection = pool.get()?;
        let query = delete(license_change.filter(created_by_id.eq(key_user_id)));

        let sql = debug_query::<Pg, _>(&query).to_string();
        debug!("{}", sql);

        Ok(query.get_results(&connection)?)
    }
}
