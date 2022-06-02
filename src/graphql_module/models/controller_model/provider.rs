use super::models::{Controller, ControllerForm};
use crate::schema::controllers;
use diesel::prelude::*;
use uuid::Uuid;

pub fn get_all(conn: &PgConnection) -> QueryResult<Vec<Controller>> {
    controllers::table.load(conn)
}
pub fn get_post_by_id(id: Uuid, conn: &PgConnection) -> QueryResult<Controller> {
    controllers::table
        .filter(controllers::id.eq(id))
        .first::<Controller>(conn)
}
pub fn get_by_posts_by_author(
    author_id: Uuid,
    conn: &PgConnection,
) -> QueryResult<Vec<Controller>> {
    controllers::table
        .filter(controllers::created_by_id.eq(author_id))
        .load(conn)
}
// pub fn get_for_user(conn: &PgConnection, created_by_id: Uuid) -> QueryResult<>
pub fn create_post(form: ControllerForm, conn: &PgConnection) -> QueryResult<Controller> {
    diesel::insert_into(controllers::table)
        .values(form)
        .get_result::<Controller>(conn)?;
    controllers::table
        .order(controllers::id.desc())
        .select(controllers::all_columns)
        .first(conn)
        .map_err(Into::into)
}
pub fn delete_post(post_author: Uuid, post_id: Uuid, conn: &PgConnection) -> QueryResult<bool> {
    diesel::delete(
        controllers::table
            .filter(controllers::created_by_id.eq(post_author))
            .find(post_id),
    )
    .execute(conn)?;

    Ok(true)
}

pub fn update_post(
    post_id: Uuid,
    created_by_id: Uuid,
    form: ControllerForm,
    conn: &PgConnection,
) -> QueryResult<Controller> {
    diesel::update(
        controllers::table
            .filter(controllers::created_by_id.eq(created_by_id))
            .find(post_id),
    )
    .set(form)
    .get_result::<Controller>(conn)
}
