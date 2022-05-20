use crate::db::DbPool;
use crate::db2::user::user::{User, UserNewForm};
use crate::diesel::{
    debug_query, insert_into,
    pg::Pg,
    query_dsl::filter_dsl::{FilterDsl, FindDsl},
    ExpressionMethods, RunQueryDsl,
};
use crate::schema::user::dsl::*;
use actix_web::web::Data;
use juniper::FieldResult;
use log::debug;
use uuid::Uuid;

pub struct Repository;

impl Repository {
    pub fn all(pool: &Data<DbPool>) -> FieldResult<Vec<User>> {
        let connection = pool.get()?;

        Ok(user.load(&connection)?)
    }

    pub fn any(pool: &Data<DbPool>, keys: &[Uuid]) -> FieldResult<Vec<User>> {
        let connection = pool.get()?;
        let query = user.filter(id.eq_any(keys));

        let sql = debug_query::<Pg, _>(&query).to_string();
        debug!("{}", sql);

        Ok(query.get_results(&connection)?)
    }

    pub fn find_by_id(pool: &Data<DbPool>, key_id: &Uuid) -> FieldResult<User> {
        let connection = pool.get()?;
        let query = user.find(key_id);

        let sql = debug_query::<Pg, _>(&query).to_string();
        debug!("{}", sql);

        Ok(query.get_result(&connection)?)
    }

    pub fn find_by_name(pool: &Data<DbPool>, key_name: &str) -> FieldResult<Vec<User>> {
        let connection = pool.get()?;
        let query = user.filter(first_name.eq(key_name));

        let sql = debug_query::<Pg, _>(&query).to_string();
        debug!("{}", sql);

        Ok(query.get_results(&connection)?)
    }

    pub fn insert(pool: &Data<DbPool>, user_form: UserNewForm) -> FieldResult<User> {
        let connection = pool.get()?;
        let query = insert_into(user).values(user_form);

        let sql = debug_query::<Pg, _>(&query).to_string();
        debug!("{}", sql);

        Ok(query.get_result(&connection)?)
    }

    // pub fn delete(pool: &Data<DbPool>, key_id: &str) -> FieldResult<User> {
    //     let connection = pool.get()?;

    //     scores::Repository::delete_by_user_id(pool, key_id)?;
    //     let query = delete(user.find(key_id));

    //     let sql = debug_query::<Pg, _>(&query).to_string();
    //     debug!("{}", sql);

    //     Ok(query.get_result(&connection)?)
    // }
}
