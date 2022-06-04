use super::user::UserLoader;
use crate::db::DbPool;
use anymap::{any::Any, Map};
use async_graphql::dataloader::DataLoader;

pub type LoaderMap = Map<AnyLoader>;
pub type AnyLoader = dyn Any + Send + Sync;

#[derive(Debug)]
pub struct LoaderRegistry {
    pub loaders: LoaderMap,
}

impl LoaderRegistry {
    pub fn get<T: anymap::any::Any + Send + Sync>(&self) -> &T {
        match self.loaders.get::<T>() {
            Some(loader) => loader,
            None => unreachable!("{} not found", std::any::type_name::<T>()),
        }
    }
}

pub async fn get_loaders(pool: &DbPool) -> LoaderMap {
    let mut loaders: LoaderMap = LoaderMap::new();

    let user_loader =
        DataLoader::new(UserLoader { pool: pool.clone() }, async_std::task::spawn);

    loaders.insert(user_loader);

    loaders
}

