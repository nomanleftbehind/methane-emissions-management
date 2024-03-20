use crate::graphql::models::routine::compressor_seal::CompressorSealMonthMethaneEmissionOverride;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

pub struct CompressorSealMonthMethaneEmissionOverrideLoader {
    pool: Data<PgPool>,
}

impl CompressorSealMonthMethaneEmissionOverrideLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for CompressorSealMonthMethaneEmissionOverrideLoader {
    type Value = CompressorSealMonthMethaneEmissionOverride;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let compressor_seal_month_methane_emission_overrides = sqlx::query_as!(
            CompressorSealMonthMethaneEmissionOverride,
            r#"SELECT * FROM compressor_seal_month_methane_emission_override WHERE id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|compressor_seal_month_methane_emission_override| {
            (
                compressor_seal_month_methane_emission_override.id,
                compressor_seal_month_methane_emission_override,
            )
        })
        .collect();

        Ok(compressor_seal_month_methane_emission_overrides)
    }
}

pub struct CreatedCompressorSealMonthMethaneEmissionOverridesLoader {
    pool: Data<PgPool>,
}

impl CreatedCompressorSealMonthMethaneEmissionOverridesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for CreatedCompressorSealMonthMethaneEmissionOverridesLoader {
    type Value = Vec<CompressorSealMonthMethaneEmissionOverride>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut compressor_seal_month_methane_emission_overrides = sqlx::query_as!(
            CompressorSealMonthMethaneEmissionOverride,
            r#"SELECT * FROM compressor_seal_month_methane_emission_override WHERE created_by_id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        compressor_seal_month_methane_emission_overrides.sort_by_key(
            |compressor_seal_month_methane_emission_override| {
                compressor_seal_month_methane_emission_override.created_by_id
            },
        );

        let compressor_seal_month_methane_emission_overrides =
            compressor_seal_month_methane_emission_overrides
                .into_iter()
                .group_by(|compressor_seal_month_methane_emission_override| {
                    compressor_seal_month_methane_emission_override.created_by_id
                })
                .into_iter()
                .map(|(created_by_id, group)| (created_by_id, group.collect()))
                .collect();

        Ok(compressor_seal_month_methane_emission_overrides)
    }
}

pub struct UpdatedCompressorSealMonthMethaneEmissionOverridesLoader {
    pool: Data<PgPool>,
}

impl UpdatedCompressorSealMonthMethaneEmissionOverridesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for UpdatedCompressorSealMonthMethaneEmissionOverridesLoader {
    type Value = Vec<CompressorSealMonthMethaneEmissionOverride>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut compressor_seal_month_methane_emission_overrides = sqlx::query_as!(
            CompressorSealMonthMethaneEmissionOverride,
            r#"SELECT * FROM compressor_seal_month_methane_emission_override WHERE updated_by_id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        compressor_seal_month_methane_emission_overrides.sort_by_key(
            |compressor_seal_month_methane_emission_override| {
                compressor_seal_month_methane_emission_override.updated_by_id
            },
        );

        let updated_compressor_month_vent_overrides =
            compressor_seal_month_methane_emission_overrides
                .into_iter()
                .group_by(|compressor_seal_month_methane_emission_override| {
                    compressor_seal_month_methane_emission_override.updated_by_id
                })
                .into_iter()
                .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
                .collect();

        Ok(updated_compressor_month_vent_overrides)
    }
}

pub struct CompressorSealMonthMethaneEmissionOverridesByCompressorSealLoader {
    pool: Data<PgPool>,
}

impl CompressorSealMonthMethaneEmissionOverridesByCompressorSealLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

impl Loader<Uuid> for CompressorSealMonthMethaneEmissionOverridesByCompressorSealLoader {
    type Value = Vec<CompressorSealMonthMethaneEmissionOverride>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut compressor_seal_month_methane_emission_overrides = sqlx::query_as!(
            CompressorSealMonthMethaneEmissionOverride,
            r#"SELECT * FROM compressor_seal_month_methane_emission_override WHERE compressor_seal_id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        compressor_seal_month_methane_emission_overrides.sort_by_key(
            |compressor_seal_month_methane_emission_override| {
                compressor_seal_month_methane_emission_override.compressor_seal_id
            },
        );

        let compressor_month_vent_overrides_by_compressor =
            compressor_seal_month_methane_emission_overrides
                .into_iter()
                .group_by(|compressor_seal_month_methane_emission_override| {
                    compressor_seal_month_methane_emission_override.compressor_seal_id
                })
                .into_iter()
                .map(|(compressor_seal_id, group)| (compressor_seal_id, group.collect()))
                .collect();

        Ok(compressor_month_vent_overrides_by_compressor)
    }
}
