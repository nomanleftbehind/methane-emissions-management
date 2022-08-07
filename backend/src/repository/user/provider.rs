use super::model::{NewUser, UserObject};
use crate::repository::schema::users;
use diesel::prelude::*;
use diesel::dsl::any;
use uuid::Uuid;

pub fn get_all_users(conn: &PgConnection) -> QueryResult<Vec<UserObject>> {
    use crate::repository::schema::users::dsl::*;
    users.load(conn)
}
pub fn get_users_by_ids(user_ids: &[Uuid], conn: &PgConnection) -> QueryResult<Vec<UserObject>> {
    users::table
        .filter(users::id.eq(any(user_ids)))
        .load::<UserObject>(conn)
}
pub fn get_user_by_id(user_id: &Uuid, conn: &PgConnection) -> QueryResult<UserObject> {
    users::table.filter(users::id.eq(user_id)).first(conn)
}
pub fn get_user_by_email(user_email: String, conn: &PgConnection) -> QueryResult<UserObject> {
    users::table.filter(users::email.eq(user_email)).first(conn)
}
pub fn get_user_by_username(user_username: String, conn: &PgConnection) -> QueryResult<UserObject> {
    users::table
        .filter(users::username.eq(user_username))
        .first(conn)
}
pub fn create_user(form: NewUser, conn: &PgConnection) -> QueryResult<UserObject> {
    diesel::insert_into(users::table)
        .values(form)
        .get_result(conn)
}
pub fn delete_user(user_id: Uuid, conn: &PgConnection) -> QueryResult<bool> {
    use crate::repository::schema::users::dsl::*;
    diesel::delete(users.filter(id.eq(user_id))).execute(conn)?;
    Ok(true)
}
pub fn update_user_details(
    user_id: Uuid,
    form: NewUser,
    conn: &PgConnection,
) -> QueryResult<UserObject> {
    diesel::update(users::table)
        .filter(users::id.eq(user_id))
        .set(form)
        .get_result(conn)
}
pub fn update_password(new_hash: String, conn: &PgConnection) -> QueryResult<usize> {
    diesel::update(users::table)
        .set(users::hash.eq(new_hash))
        .execute(conn)
}
