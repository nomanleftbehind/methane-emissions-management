use crate::graphql::models::MonthMethaneEmission;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct MonthMethaneEmissionLoader {
    pool: Data<PgPool>,
}

impl MonthMethaneEmissionLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for MonthMethaneEmissionLoader {
    type Value = MonthMethaneEmission;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let month_methane_emissions = query_as!(
            MonthMethaneEmission,
            r#"
            SELECT
            id, facility_id, site_id, source_table as "source_table: _", source_table_id, category as "category: _", source as "source: _", month, gas_volume, c1_volume, co2_volume, created_by_id, created_at, updated_by_id, updated_at
            FROM month_methane_emission
            WHERE id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|month_methane_emission| (month_methane_emission.id, month_methane_emission))
        .collect();

        Ok(month_methane_emissions)
    }
}

pub struct CreatedMonthMethaneEmissionsLoader {
    pool: Data<PgPool>,
}

impl CreatedMonthMethaneEmissionsLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedMonthMethaneEmissionsLoader {
    type Value = Vec<MonthMethaneEmission>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut month_methane_emissions = query_as!(
            MonthMethaneEmission,
            r#"
            SELECT
            id, facility_id, site_id, source_table as "source_table: _", source_table_id, category as "category: _", source as "source: _", month, gas_volume, c1_volume, co2_volume, created_by_id, created_at, updated_by_id, updated_at
            FROM month_methane_emission
            WHERE created_by_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        month_methane_emissions
            .sort_by_key(|month_methane_emission| month_methane_emission.created_by_id);

        let created_month_methane_emissions = month_methane_emissions
            .into_iter()
            .group_by(|month_methane_emission| month_methane_emission.created_by_id)
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(created_month_methane_emissions)
    }
}

pub struct UpdatedMonthMethaneEmissionsLoader {
    pool: Data<PgPool>,
}

impl UpdatedMonthMethaneEmissionsLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedMonthMethaneEmissionsLoader {
    type Value = Vec<MonthMethaneEmission>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut month_methane_emissions = query_as!(
            MonthMethaneEmission,
            r#"
            SELECT
            id, facility_id, site_id, source_table as "source_table: _", source_table_id, category as "category: _", source as "source: _", month, gas_volume, c1_volume, co2_volume, created_by_id, created_at, updated_by_id, updated_at
            FROM month_methane_emission
            WHERE updated_by_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        month_methane_emissions
            .sort_by_key(|month_methane_emission| month_methane_emission.updated_by_id);

        let updated_month_methane_emissions = month_methane_emissions
            .into_iter()
            .group_by(|month_methane_emission| month_methane_emission.updated_by_id)
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(updated_month_methane_emissions)
    }
}

pub struct MonthMethaneEmissionsByFacilityLoader {
    pool: Data<PgPool>,
}

impl MonthMethaneEmissionsByFacilityLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for MonthMethaneEmissionsByFacilityLoader {
    type Value = Vec<MonthMethaneEmission>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut month_methane_emissions = query_as!(
            MonthMethaneEmission,
            r#"
            SELECT
            id, facility_id, site_id, source_table as "source_table: _", source_table_id, category as "category: _", source as "source: _", month, gas_volume, c1_volume, co2_volume, created_by_id, created_at, updated_by_id, updated_at
            FROM month_methane_emission
            WHERE facility_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        month_methane_emissions
            .sort_by_key(|month_methane_emission| month_methane_emission.facility_id);

        let month_methane_emissions = month_methane_emissions
            .into_iter()
            .group_by(|month_methane_emission| month_methane_emission.facility_id)
            .into_iter()
            .map(|(facility_id, group)| (facility_id, group.collect()))
            .collect();

        Ok(month_methane_emissions)
    }
}

pub struct MonthMethaneEmissionsBySiteLoader {
    pool: Data<PgPool>,
}

impl MonthMethaneEmissionsBySiteLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for MonthMethaneEmissionsBySiteLoader {
    type Value = Vec<MonthMethaneEmission>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut month_methane_emissions = query_as!(
            MonthMethaneEmission,
            r#"
            SELECT
            id, facility_id, site_id, source_table as "source_table: _", source_table_id, category as "category: _", source as "source: _", month, gas_volume, c1_volume, co2_volume, created_by_id, created_at, updated_by_id, updated_at
            FROM month_methane_emission
            WHERE site_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        month_methane_emissions
            .sort_by_key(|month_methane_emission| month_methane_emission.site_id);

        let month_methane_emissions = month_methane_emissions
            .into_iter()
            .group_by(|month_methane_emission| month_methane_emission.site_id)
            .into_iter()
            .map(|(site_id, group)| (site_id, group.collect()))
            .collect();

        Ok(month_methane_emissions)
    }
}

pub struct MonthMethaneEmissionsBySourceTableLoader {
    pool: Data<PgPool>,
}

impl MonthMethaneEmissionsBySourceTableLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for MonthMethaneEmissionsBySourceTableLoader {
    type Value = Vec<MonthMethaneEmission>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut month_methane_emissions = sqlx::query_as!(
            MonthMethaneEmission,
            r#"
            SELECT
            id, facility_id, site_id, source_table as "source_table: _", source_table_id, category as "category: _", source as "source: _", month, gas_volume, c1_volume, co2_volume, created_by_id, created_at, updated_by_id, updated_at
            FROM month_methane_emission
            WHERE source_table_id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        month_methane_emissions
            .sort_by_key(|month_methane_emission| month_methane_emission.source_table_id);

        let month_methane_emissions = month_methane_emissions
            .into_iter()
            .group_by(|month_methane_emission| month_methane_emission.source_table_id)
            .into_iter()
            .map(|(source_table_id, group)| (source_table_id, group.collect()))
            .collect();

        Ok(month_methane_emissions)
    }
}
