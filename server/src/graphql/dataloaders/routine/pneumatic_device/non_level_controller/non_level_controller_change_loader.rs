use crate::graphql::models::routine::pneumatic_device::non_level_controller::NonLevelControllerChange;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct NonLevelControllerChangeLoader {
    pool: Data<PgPool>,
}

impl NonLevelControllerChangeLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for NonLevelControllerChangeLoader {
    type Value = NonLevelControllerChange;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let non_level_controller_changes = query_as!(
            NonLevelControllerChange,
            "SELECT * FROM non_level_controller_change WHERE id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|non_level_controller_change| {
            (non_level_controller_change.id, non_level_controller_change)
        })
        .collect();

        Ok(non_level_controller_changes)
    }
}

pub struct CreatedNonLevelControllerChangesLoader {
    pool: Data<PgPool>,
}

impl CreatedNonLevelControllerChangesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedNonLevelControllerChangesLoader {
    type Value = Vec<NonLevelControllerChange>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut non_level_controller_changes = query_as!(
            NonLevelControllerChange,
            "SELECT * FROM non_level_controller_change WHERE created_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        non_level_controller_changes
            .sort_by_key(|non_level_controller_change| non_level_controller_change.created_by_id);

        let non_level_controller_changes = non_level_controller_changes
            .into_iter()
            .group_by(|non_level_controller_change| non_level_controller_change.created_by_id)
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(non_level_controller_changes)
    }
}

pub struct UpdatedNonLevelControllerChangesLoader {
    pool: Data<PgPool>,
}

impl UpdatedNonLevelControllerChangesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedNonLevelControllerChangesLoader {
    type Value = Vec<NonLevelControllerChange>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut non_level_controller_changes = query_as!(
            NonLevelControllerChange,
            "SELECT * FROM non_level_controller_change WHERE updated_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        non_level_controller_changes
            .sort_by_key(|non_level_controller| non_level_controller.updated_by_id);

        let non_level_controller_changes = non_level_controller_changes
            .into_iter()
            .group_by(|non_level_controller| non_level_controller.updated_by_id)
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(non_level_controller_changes)
    }
}

pub struct NonLevelControllerChangesByNonLevelControllerLoader {
    pool: Data<PgPool>,
}

impl NonLevelControllerChangesByNonLevelControllerLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for NonLevelControllerChangesByNonLevelControllerLoader {
    type Value = Vec<NonLevelControllerChange>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut non_level_controller_change = query_as!(
            NonLevelControllerChange,
            "SELECT * FROM non_level_controller_change WHERE non_level_controller_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        non_level_controller_change.sort_by_key(|non_level_controller_change| {
            non_level_controller_change.non_level_controller_id
        });

        let non_level_controller_changes = non_level_controller_change
            .into_iter()
            .group_by(|non_level_controller_change| {
                non_level_controller_change.non_level_controller_id
            })
            .into_iter()
            .map(|(non_level_controller_id, group)| (non_level_controller_id, group.collect()))
            .collect();

        Ok(non_level_controller_changes)
    }
}
