use crate::graphql::models::routine::pneumatic_device::pneumatic_instrument::PneumaticInstrument;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct PneumaticInstrumentLoader {
    pool: Data<PgPool>,
}

impl PneumaticInstrumentLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for PneumaticInstrumentLoader {
    type Value = PneumaticInstrument;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let pneumatic_instruments = query_as!(
            PneumaticInstrument,
            r#"
            SELECT
            id, site_id, type as "type: _", manufacturer_id, model, serial_number, start_date, end_date, created_by_id, created_at, updated_by_id, updated_at
            FROM pneumatic_instrument
            WHERE id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|pneumatic_instrument| (pneumatic_instrument.id, pneumatic_instrument))
        .collect();

        Ok(pneumatic_instruments)
    }
}

pub struct CreatedPneumaticInstrumentsLoader {
    pool: Data<PgPool>,
}

impl CreatedPneumaticInstrumentsLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for CreatedPneumaticInstrumentsLoader {
    type Value = Vec<PneumaticInstrument>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut pneumatic_instruments = query_as!(
            PneumaticInstrument,
            r#"
            SELECT
            id, site_id, type as "type: _", manufacturer_id, model, serial_number, start_date, end_date, created_by_id, created_at, updated_by_id, updated_at
            FROM pneumatic_instrument
            WHERE created_by_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;
        pneumatic_instruments
            .sort_by_key(|pneumatic_instrument| pneumatic_instrument.created_by_id);

        let created_pneumatic_instruments = pneumatic_instruments
            .into_iter()
            .group_by(|pneumatic_instrument| pneumatic_instrument.created_by_id)
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(created_pneumatic_instruments)
    }
}

pub struct UpdatedPneumaticInstrumentsLoader {
    pool: Data<PgPool>,
}

impl UpdatedPneumaticInstrumentsLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for UpdatedPneumaticInstrumentsLoader {
    type Value = Vec<PneumaticInstrument>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut pneumatic_instruments = query_as!(
            PneumaticInstrument,
            r#"
            SELECT
            id, site_id, type as "type: _", manufacturer_id, model, serial_number, start_date, end_date, created_by_id, created_at, updated_by_id, updated_at
            FROM pneumatic_instrument
            WHERE updated_by_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;
        pneumatic_instruments
            .sort_by_key(|pneumatic_instrument| pneumatic_instrument.updated_by_id);

        let updated_pneumatic_instruments = pneumatic_instruments
            .into_iter()
            .group_by(|pneumatic_instrument| pneumatic_instrument.updated_by_id)
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(updated_pneumatic_instruments)
    }
}

pub struct SitePneumaticInstrumentsLoader {
    pool: Data<PgPool>,
}

impl SitePneumaticInstrumentsLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for SitePneumaticInstrumentsLoader {
    type Value = Vec<PneumaticInstrument>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut pneumatic_instruments = query_as!(
            PneumaticInstrument,
            r#"
            SELECT
            id, site_id, type as "type: _", manufacturer_id, model, serial_number, start_date, end_date, created_by_id, created_at, updated_by_id, updated_at
            FROM pneumatic_instrument
            WHERE site_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;
        pneumatic_instruments.sort_by_key(|pneumatic_instrument| pneumatic_instrument.site_id);

        let pneumatic_instruments = pneumatic_instruments
            .into_iter()
            .group_by(|pneumatic_instrument| pneumatic_instrument.site_id)
            .into_iter()
            .map(|(facility_id, group)| (facility_id, group.collect()))
            .collect();

        Ok(pneumatic_instruments)
    }
}

pub struct PneumaticInstrumentsByManufacturerLoader {
    pool: Data<PgPool>,
}

impl PneumaticInstrumentsByManufacturerLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for PneumaticInstrumentsByManufacturerLoader {
    type Value = Vec<PneumaticInstrument>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut pneumatic_instruments = query_as!(
            PneumaticInstrument,
            r#"
            SELECT
            id, site_id, type as "type: _", manufacturer_id, model, serial_number, start_date, end_date, created_by_id, created_at, updated_by_id, updated_at
            FROM pneumatic_instrument
            WHERE manufacturer_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;
        pneumatic_instruments
            .sort_by_key(|pneumatic_instrument| pneumatic_instrument.manufacturer_id);

        let pneumatic_instruments = pneumatic_instruments
            .into_iter()
            .group_by(|pneumatic_instrument| pneumatic_instrument.manufacturer_id)
            .into_iter()
            .map(|(manufacturer_id, group)| (manufacturer_id, group.collect()))
            .collect();

        Ok(pneumatic_instruments)
    }
}
