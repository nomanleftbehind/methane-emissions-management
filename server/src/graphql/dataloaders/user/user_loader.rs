use crate::graphql::models::user::User;
use actix_web::web::Data;
use async_graphql::{dataloader::*, *};
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct UserLoader {
    pool: Data<PgPool>,
}

impl UserLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UserLoader {
    type Value = User;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let users = query_as!(
            User,
            r#"SELECT id, email, password, first_name, last_name, role as "role: _" FROM "user" WHERE id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|user| (user.id, user))
        .collect();

        Ok(users)
    }
}
