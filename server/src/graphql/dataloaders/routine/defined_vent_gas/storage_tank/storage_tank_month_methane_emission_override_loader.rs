use crate::graphql::models::routine::defined_vent_gas::storage_tank::StorageTankMonthMethaneEmissionOverride;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct StorageTankMonthMethaneEmissionOverrideLoader {
    pool: Data<PgPool>,
}

impl StorageTankMonthMethaneEmissionOverrideLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for StorageTankMonthMethaneEmissionOverrideLoader {
    type Value = StorageTankMonthMethaneEmissionOverride;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let storage_tank_month_methane_emission_overrides = query_as!(
            StorageTankMonthMethaneEmissionOverride,
            r#"SELECT * FROM storage_tank_month_methane_emission_override WHERE id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|storage_tank_month_methane_emission_override| {
            (
                storage_tank_month_methane_emission_override.id,
                storage_tank_month_methane_emission_override,
            )
        })
        .collect();

        Ok(storage_tank_month_methane_emission_overrides)
    }
}

pub struct CreatedStorageTankMonthMethaneEmissionOverridesLoader {
    pool: Data<PgPool>,
}

impl CreatedStorageTankMonthMethaneEmissionOverridesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for CreatedStorageTankMonthMethaneEmissionOverridesLoader {
    type Value = Vec<StorageTankMonthMethaneEmissionOverride>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut storage_tank_month_methane_emission_overrides = query_as!(
            StorageTankMonthMethaneEmissionOverride,
            r#"SELECT * FROM storage_tank_month_methane_emission_override WHERE created_by_id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        storage_tank_month_methane_emission_overrides.sort_by_key(
            |storage_tank_month_methane_emission_override| {
                storage_tank_month_methane_emission_override.created_by_id
            },
        );

        let storage_tank_month_methane_emission_overrides =
            storage_tank_month_methane_emission_overrides
                .into_iter()
                .group_by(|storage_tank_month_methane_emission_override| {
                    storage_tank_month_methane_emission_override.created_by_id
                })
                .into_iter()
                .map(|(created_by_id, group)| (created_by_id, group.collect()))
                .collect();

        Ok(storage_tank_month_methane_emission_overrides)
    }
}

pub struct UpdatedStorageTankMonthMethaneEmissionOverridesLoader {
    pool: Data<PgPool>,
}

impl UpdatedStorageTankMonthMethaneEmissionOverridesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for UpdatedStorageTankMonthMethaneEmissionOverridesLoader {
    type Value = Vec<StorageTankMonthMethaneEmissionOverride>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut storage_tank_month_methane_emission_overrides = query_as!(
            StorageTankMonthMethaneEmissionOverride,
            r#"SELECT * FROM storage_tank_month_methane_emission_override WHERE updated_by_id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        storage_tank_month_methane_emission_overrides.sort_by_key(
            |storage_tank_month_methane_emission_override| {
                storage_tank_month_methane_emission_override.updated_by_id
            },
        );

        let storage_tank_month_methane_emission_overrides =
            storage_tank_month_methane_emission_overrides
                .into_iter()
                .group_by(|storage_tank_month_methane_emission_override| {
                    storage_tank_month_methane_emission_override.updated_by_id
                })
                .into_iter()
                .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
                .collect();

        Ok(storage_tank_month_methane_emission_overrides)
    }
}

pub struct StorageTankMonthMethaneEmissionOverridesByStorageTankLoader {
    pool: Data<PgPool>,
}

impl StorageTankMonthMethaneEmissionOverridesByStorageTankLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for StorageTankMonthMethaneEmissionOverridesByStorageTankLoader {
    type Value = Vec<StorageTankMonthMethaneEmissionOverride>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut storage_tank_month_methane_emission_overrides = query_as!(
            StorageTankMonthMethaneEmissionOverride,
            r#"SELECT * FROM storage_tank_month_methane_emission_override WHERE storage_tank_id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        storage_tank_month_methane_emission_overrides.sort_by_key(
            |storage_tank_month_methane_emission_override| {
                storage_tank_month_methane_emission_override.storage_tank_id
            },
        );

        let storage_tank_month_methane_emission_overrides =
            storage_tank_month_methane_emission_overrides
                .into_iter()
                .group_by(|storage_tank_month_methane_emission_override| {
                    storage_tank_month_methane_emission_override.storage_tank_id
                })
                .into_iter()
                .map(|(storage_tank_id, group)| (storage_tank_id, group.collect()))
                .collect();

        Ok(storage_tank_month_methane_emission_overrides)
    }
}
