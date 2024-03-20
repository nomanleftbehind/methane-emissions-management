use crate::graphql::models::routine::pneumatic_device::pneumatic_pump::PneumaticPump;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct PneumaticPumpLoader {
    pool: Data<PgPool>,
}

impl PneumaticPumpLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for PneumaticPumpLoader {
    type Value = PneumaticPump;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let pneumatic_pumps = query_as!(
            PneumaticPump,
            "SELECT * FROM pneumatic_pump WHERE id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|pneumatic_pump| (pneumatic_pump.id, pneumatic_pump))
        .collect();

        Ok(pneumatic_pumps)
    }
}

pub struct CreatedPneumaticPumpsLoader {
    pool: Data<PgPool>,
}

impl CreatedPneumaticPumpsLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for CreatedPneumaticPumpsLoader {
    type Value = Vec<PneumaticPump>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut pneumatic_pumps = query_as!(
            PneumaticPump,
            "SELECT * FROM pneumatic_pump WHERE created_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;
        pneumatic_pumps.sort_by_key(|pneumatic_pump| pneumatic_pump.created_by_id);

        let created_pneumatic_pumps = pneumatic_pumps
            .into_iter()
            .group_by(|pneumatic_pump| pneumatic_pump.created_by_id)
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(created_pneumatic_pumps)
    }
}

pub struct UpdatedPneumaticPumpsLoader {
    pool: Data<PgPool>,
}

impl UpdatedPneumaticPumpsLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for UpdatedPneumaticPumpsLoader {
    type Value = Vec<PneumaticPump>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut pneumatic_pumps = query_as!(
            PneumaticPump,
            "SELECT * FROM pneumatic_pump WHERE updated_by_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;
        pneumatic_pumps.sort_by_key(|pneumatic_pump| pneumatic_pump.updated_by_id);

        let updated_pneumatic_pumps = pneumatic_pumps
            .into_iter()
            .group_by(|pneumatic_pump| pneumatic_pump.updated_by_id)
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(updated_pneumatic_pumps)
    }
}

pub struct SitePneumaticPumpsLoader {
    pool: Data<PgPool>,
}

impl SitePneumaticPumpsLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for SitePneumaticPumpsLoader {
    type Value = Vec<PneumaticPump>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut pneumatic_pumps = query_as!(
            PneumaticPump,
            "SELECT * FROM pneumatic_pump WHERE site_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;
        pneumatic_pumps.sort_by_key(|pneumatic_pump| pneumatic_pump.site_id);

        let pneumatic_pumps = pneumatic_pumps
            .into_iter()
            .group_by(|pneumatic_pump| pneumatic_pump.site_id)
            .into_iter()
            .map(|(facility_id, group)| (facility_id, group.collect()))
            .collect();

        Ok(pneumatic_pumps)
    }
}

pub struct PneumaticPumpsByManufacturerLoader {
    pool: Data<PgPool>,
}

impl PneumaticPumpsByManufacturerLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for PneumaticPumpsByManufacturerLoader {
    type Value = Vec<PneumaticPump>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut pneumatic_pumps = query_as!(
            PneumaticPump,
            "SELECT * FROM pneumatic_pump WHERE manufacturer_id = ANY($1)",
            keys
        )
        .fetch_all(&**self.pool)
        .await?;
        pneumatic_pumps.sort_by_key(|pneumatic_pump| pneumatic_pump.manufacturer_id);

        let pneumatic_pumps = pneumatic_pumps
            .into_iter()
            .group_by(|pneumatic_pump| pneumatic_pump.manufacturer_id)
            .into_iter()
            .map(|(manufacturer_id, group)| (manufacturer_id, group.collect()))
            .collect();

        Ok(pneumatic_pumps)
    }
}
