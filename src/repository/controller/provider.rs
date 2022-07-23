use super::models::{Controller, ControllerForm, ControllerUpdateForm};
use crate::repository::schema::controllers;
use diesel::prelude::*;
use uuid::Uuid;

pub fn get_all(conn: &PgConnection) -> QueryResult<Vec<Controller>> {
    controllers::table.load(conn)
}
pub fn get_controller_by_id(id: Uuid, conn: &PgConnection) -> QueryResult<Controller> {
    controllers::table
        .filter(controllers::id.eq(id))
        .first::<Controller>(conn)
}
pub fn get_controllers_by_author(
    author_id: Uuid,
    conn: &PgConnection,
) -> QueryResult<Vec<Controller>> {
    controllers::table
        .filter(controllers::created_by_id.eq(author_id))
        .load(conn)
}
// pub fn get_for_user(conn: &PgConnection, created_by_id: Uuid) -> QueryResult<>
pub fn create_controller(form: ControllerForm, conn: &PgConnection) -> QueryResult<Controller> {
    diesel::insert_into(controllers::table)
        .values(form)
        .get_result::<Controller>(conn)?;
    controllers::table
        .order(controllers::id.desc())
        .select(controllers::all_columns)
        .first(conn)
        .map_err(Into::into)
}
pub fn delete_controller(
    controller_author: Uuid,
    controller_id: Uuid,
    conn: &PgConnection,
) -> QueryResult<bool> {
    diesel::delete(
        controllers::table
            .filter(controllers::created_by_id.eq(controller_author))
            .find(controller_id),
    )
    .execute(conn)?;

    Ok(true)
}

pub fn update_controller(
    controller_id: Uuid,
    form: ControllerUpdateForm,
    conn: &PgConnection,
) -> QueryResult<Controller> {
    diesel::update(controllers::table.find(controller_id))
        .set(form)
        .get_result::<Controller>(conn)
}
