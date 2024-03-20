use crate::graphql::models::routine::defined_vent_gas::storage_tank::StorageTankMonthLiquidHydrocarbonEntering;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct StorageTankMonthLiquidHydrocarbonEnteringLoader {
    pool: Data<PgPool>,
}

impl StorageTankMonthLiquidHydrocarbonEnteringLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for StorageTankMonthLiquidHydrocarbonEnteringLoader {
    type Value = StorageTankMonthLiquidHydrocarbonEntering;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let storage_tank_month_liquid_hydrocarbon_entering = query_as!(
            StorageTankMonthLiquidHydrocarbonEntering,
            r#"SELECT * FROM storage_tank_month_liquid_hydrocarbon_entering WHERE id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|storage_tank_month_liquid_hydrocarbon_entering| {
            (
                storage_tank_month_liquid_hydrocarbon_entering.id,
                storage_tank_month_liquid_hydrocarbon_entering,
            )
        })
        .collect();

        Ok(storage_tank_month_liquid_hydrocarbon_entering)
    }
}

pub struct CreatedStorageTankMonthLiquidHydrocarbonEnteringLoader {
    pool: Data<PgPool>,
}

impl CreatedStorageTankMonthLiquidHydrocarbonEnteringLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for CreatedStorageTankMonthLiquidHydrocarbonEnteringLoader {
    type Value = Vec<StorageTankMonthLiquidHydrocarbonEntering>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut storage_tank_month_liquid_hydrocarbon_entering = query_as!(
            StorageTankMonthLiquidHydrocarbonEntering,
            r#"SELECT * FROM storage_tank_month_liquid_hydrocarbon_entering WHERE created_by_id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        storage_tank_month_liquid_hydrocarbon_entering.sort_by_key(
            |storage_tank_month_liquid_hydrocarbon_entering| {
                storage_tank_month_liquid_hydrocarbon_entering.created_by_id
            },
        );

        let storage_tank_month_liquid_hydrocarbon_entering =
            storage_tank_month_liquid_hydrocarbon_entering
                .into_iter()
                .group_by(|storage_tank_month_liquid_hydrocarbon_entering| {
                    storage_tank_month_liquid_hydrocarbon_entering.created_by_id
                })
                .into_iter()
                .map(|(created_by_id, group)| (created_by_id, group.collect()))
                .collect();

        Ok(storage_tank_month_liquid_hydrocarbon_entering)
    }
}

pub struct UpdatedStorageTankMonthLiquidHydrocarbonEnteringLoader {
    pool: Data<PgPool>,
}

impl UpdatedStorageTankMonthLiquidHydrocarbonEnteringLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for UpdatedStorageTankMonthLiquidHydrocarbonEnteringLoader {
    type Value = Vec<StorageTankMonthLiquidHydrocarbonEntering>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut storage_tank_month_liquid_hydrocarbon_entering = query_as!(
            StorageTankMonthLiquidHydrocarbonEntering,
            r#"SELECT * FROM storage_tank_month_liquid_hydrocarbon_entering WHERE updated_by_id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        storage_tank_month_liquid_hydrocarbon_entering.sort_by_key(
            |storage_tank_month_liquid_hydrocarbon_entering| {
                storage_tank_month_liquid_hydrocarbon_entering.updated_by_id
            },
        );

        let storage_tank_month_liquid_hydrocarbon_entering =
            storage_tank_month_liquid_hydrocarbon_entering
                .into_iter()
                .group_by(|storage_tank_month_liquid_hydrocarbon_entering| {
                    storage_tank_month_liquid_hydrocarbon_entering.updated_by_id
                })
                .into_iter()
                .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
                .collect();

        Ok(storage_tank_month_liquid_hydrocarbon_entering)
    }
}

pub struct StorageTankMonthLiquidHydrocarbonEnteringByStorageTankLoader {
    pool: Data<PgPool>,
}

impl StorageTankMonthLiquidHydrocarbonEnteringByStorageTankLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for StorageTankMonthLiquidHydrocarbonEnteringByStorageTankLoader {
    type Value = Vec<StorageTankMonthLiquidHydrocarbonEntering>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut storage_tank_month_liquid_hydrocarbon_entering = query_as!(
            StorageTankMonthLiquidHydrocarbonEntering,
            r#"SELECT * FROM storage_tank_month_liquid_hydrocarbon_entering WHERE storage_tank_id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        storage_tank_month_liquid_hydrocarbon_entering.sort_by_key(
            |storage_tank_month_liquid_hydrocarbon_entering| {
                storage_tank_month_liquid_hydrocarbon_entering.storage_tank_id
            },
        );

        let storage_tank_month_liquid_hydrocarbon_entering =
            storage_tank_month_liquid_hydrocarbon_entering
                .into_iter()
                .group_by(|storage_tank_month_liquid_hydrocarbon_entering| {
                    storage_tank_month_liquid_hydrocarbon_entering.storage_tank_id
                })
                .into_iter()
                .map(|(storage_tank_id, group)| (storage_tank_id, group.collect()))
                .collect();

        Ok(storage_tank_month_liquid_hydrocarbon_entering)
    }
}
