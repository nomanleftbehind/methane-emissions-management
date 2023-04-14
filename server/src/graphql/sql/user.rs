use crate::graphql::models::{pneumatic_device::NonLevelController, User, UserBy};
use sqlx::PgExecutor;
use uuid::Uuid;

pub async fn query_user<'e, E: PgExecutor<'e>>(
    executor: E,
    by: UserBy,
) -> Result<Option<User>, sqlx::Error> {
    let user =
        match by {
            UserBy::Id(id) => {
                sqlx::query_as!(
                    User,
                    r#"SELECT id, email, password, first_name, last_name, role as "role: _" FROM "users" WHERE id = $1"#,
                    id
                )
                .fetch_optional(executor)
                .await
            }
            UserBy::Email(email) => sqlx::query_as!(
                User,
                r#"SELECT id, email, password, first_name, last_name, role as "role: _" FROM "users" WHERE email = $1"#,
                email
            )
            .fetch_optional(executor)
            .await,
        };
    user
}

pub async fn query_user_controllers<'e, E: PgExecutor<'e>>(
    executor: E,
    user_id: Uuid,
    limit: i64,
    offset: i64,
) -> Result<Vec<NonLevelController>, sqlx::Error> {
    let controllers = sqlx::query_as!(
        NonLevelController,
        r#"SELECT p.* FROM "controllers" p WHERE p."created_by_id" = $1 LIMIT $2 OFFSET $3"#,
        user_id,
        limit,
        offset
    )
    .fetch_all(executor)
    .await;

    controllers
}

pub async fn query_user_by_id<'e, E: PgExecutor<'e>>(
    executor: E,
    user_id: Uuid,
) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        r#"SELECT id, email, password, first_name, last_name, role as "role: _" FROM users WHERE id = $1"#,
        user_id
    )
    .fetch_optional(executor)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(user)
}

pub async fn query_all_users<'e, E: PgExecutor<'e>>(executor: E) -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query_as!(
        User,
        r#"SELECT id, email, password, first_name, last_name, role as "role: _" FROM users"#,
    )
    .fetch_all(executor)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    });

    users
}

// pub async fn query_user_controller_ids<'e, E: PgExecutor<'e>>(
//     executor: E,
//     // cred: &UserCredential,
//     user_id: Uuid,
//     limit: i64,
//     offset: i64,
// ) -> Result<Vec<i64>, sqlx::Error> {
//     query(include_str!("user_controllers.sql"))
//         // .bind(cred.user_id())
//         .bind(user_id)
//         .bind(limit)
//         .bind(offset)
//         .map(|row: PgRow| row.get("id"))
//         .fetch_all(executor)
//         .await
// }

// pub async fn query_controllers_by_user_id(
//     pool: &PgPool,
//     user_id: Uuid,
// ) -> Result<Vec<Post>, sqlx::Error> {
//     let controllers = sqlx::query_as!(
//         Post,
//         r#"
//         SELECT * FROM controller
//         WHERE
//         user_id = $1
//         "#,
//         user_id
//     )
//     .fetch_all(pool)
//     .await?;
//     Ok(controllers)
// }

// pub async fn query_role<'e, E: PgExecutor<'e>>(
//     _pool: E,
//     user_id: i64,
// ) -> Result<Option<Role>, sqlx::Error> {
//     if user_id == 1 {
//         return Ok(Some(Role::Administrator));
//     }
//     Ok(Some(Role::Regular))
// }
