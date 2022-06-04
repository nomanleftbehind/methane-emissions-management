use crate::db::DbPool;
use crate::graphql_module::models::user_model::{provider::get_users_by_ids, resolver::User};
use async_graphql::dataloader::Loader;
use async_graphql::*;
use std::collections::HashMap;
use uuid::Uuid;

pub struct UserLoader {
    pub pool: DbPool,
}

#[async_trait::async_trait]
impl Loader<Uuid> for UserLoader {
    type Value = User;
    type Error = Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let conn = self.pool.get().expect("Can't get DB connection");
        let users = get_users_by_ids(keys, &conn).expect("Can't get controller's creator");

        Ok(users
            .iter()
            .map(|user_object| (user_object.id, User::from(user_object)))
            .collect::<HashMap<_, _>>())
    }
}
