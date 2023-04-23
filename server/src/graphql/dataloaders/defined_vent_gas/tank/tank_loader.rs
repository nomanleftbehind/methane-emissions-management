use crate::graphql::models::defined_vent_gas::tank::Tank;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct TankLoader {
    pool: Data<PgPool>,
}

impl TankLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for TankLoader {
    type Value = Tank;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let tanks = query_as!(Tank, "SELECT * FROM tank WHERE id = ANY($1)", keys)
            .fetch_all(&**self.pool)
            .await?
            .into_iter()
            .map(|tank| (tank.id, tank))
            .collect();

        Ok(tanks)
    }
}

pub struct CreatedTanksLoader {
    pool: Data<PgPool>,
}

impl CreatedTanksLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedTanksLoader {
    type Value = Vec<Tank>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut tanks = query_as!(
            Tank,
            "SELECT * FROM tank WHERE created_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;
        tanks.sort_by_key(|tank| tank.created_by_id);

        let tanks = tanks
            .into_iter()
            .group_by(|tank| tank.created_by_id)
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(tanks)
    }
}

pub struct UpdatedTanksLoader {
    pool: Data<PgPool>,
}

impl UpdatedTanksLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedTanksLoader {
    type Value = Vec<Tank>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut tanks = query_as!(
            Tank,
            "SELECT * FROM tank WHERE updated_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;
        tanks.sort_by_key(|tank| tank.updated_by_id);

        let tanks = tanks
            .into_iter()
            .group_by(|tank| tank.updated_by_id)
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(tanks)
    }
}

pub struct SiteTanksLoader {
    pool: Data<PgPool>,
}

impl SiteTanksLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for SiteTanksLoader {
    type Value = Vec<Tank>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut tanks = query_as!(Tank, "SELECT * FROM tank WHERE site_id = ANY($1)", keys)
            .fetch_all(&**self.pool)
            .await?;
        tanks.sort_by_key(|tank| tank.site_id);

        let tanks = tanks
            .into_iter()
            .group_by(|tank| tank.site_id)
            .into_iter()
            .map(|(site_id, group)| (site_id, group.collect()))
            .collect();

        Ok(tanks)
    }
}
