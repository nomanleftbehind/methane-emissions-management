use crate::repository::controller::{
    provider::find_many_controllers_by_author_ids, resolver::ControllerObject,
};
use crate::repository::db::DbPool;
use async_graphql::dataloader::Loader;
use async_graphql::*;
use std::collections::HashMap;
use uuid::Uuid;

pub struct ControllerLoader {
    pub pool: DbPool,
}

#[async_trait::async_trait]
impl Loader<Uuid> for ControllerLoader {
    type Value = Vec<ControllerObject>;
    type Error = Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let conn = self.pool.get().expect("Can't get DB connection");
        let controllers =
            find_many_controllers_by_author_ids(keys, &conn).expect("Can't get user's controllers");

        let mut map: HashMap<Uuid, Vec<ControllerObject>> = HashMap::new();

        for controller in controllers {
            let list = map
                .entry(controller.created_by_id.clone())
                .or_insert_with(|| Vec::<ControllerObject>::new());
            list.push(ControllerObject::from(&controller));
        }

        Ok(map)
    }
}
