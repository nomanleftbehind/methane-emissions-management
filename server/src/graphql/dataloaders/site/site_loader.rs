use crate::graphql::models::site::Site;
use actix_web::web::Data;
use async_graphql::dataloader::Loader;
use itertools::Itertools;
use sqlx::{query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub struct SiteLoader {
    pool: Data<PgPool>,
}

impl SiteLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for SiteLoader {
    type Value = Site;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let sites = query_as!(
            Site,
            r#"SELECT
            id, facility_id, fdc_rec_id, name, type as "type: _", description, created_by_id, created_at, updated_by_id, updated_at
            FROM site
            WHERE id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?
        .into_iter()
        .map(|site| (site.id, site))
        .collect();

        Ok(sites)
    }
}

pub struct CreatedSitesLoader {
    pool: Data<PgPool>,
}

impl CreatedSitesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for CreatedSitesLoader {
    type Value = Vec<Site>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut sites = query_as!(
            Site,
            r#"SELECT
            id, facility_id, fdc_rec_id, name, type as "type: _", description, created_by_id, created_at, updated_by_id, updated_at
            FROM site
            WHERE created_by_id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        sites.sort_by_key(|site| site.created_by_id);

        let created_sites = sites
            .into_iter()
            .group_by(|site| site.created_by_id)
            .into_iter()
            .map(|(created_by_id, group)| (created_by_id, group.collect()))
            .collect();

        Ok(created_sites)
    }
}

pub struct UpdatedSitesLoader {
    pool: Data<PgPool>,
}

impl UpdatedSitesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for UpdatedSitesLoader {
    type Value = Vec<Site>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut sites = query_as!(
            Site,
            r#"SELECT
            id, facility_id, fdc_rec_id, name, type as "type: _", description, created_by_id, created_at, updated_by_id, updated_at
            FROM site
            WHERE updated_by_id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;

        sites.sort_by_key(|site| site.updated_by_id);

        let updated_sites = sites
            .into_iter()
            .group_by(|site| site.updated_by_id)
            .into_iter()
            .map(|(updated_by_id, group)| (updated_by_id, group.collect()))
            .collect();

        Ok(updated_sites)
    }
}

pub struct FacilitySitesLoader {
    pool: Data<PgPool>,
}

impl FacilitySitesLoader {
    pub fn new(pool: Data<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for FacilitySitesLoader {
    type Value = Vec<Site>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let mut sites = query_as!(
            Site,
            r#"SELECT id, facility_id, fdc_rec_id, name, type as "type: _", description, created_by_id, created_at, updated_by_id, updated_at FROM site WHERE facility_id = ANY($1)"#,
            keys
        )
        .fetch_all(&**self.pool)
        .await?;
        sites.sort_by_key(|site| site.facility_id);

        let sites = sites
            .into_iter()
            .group_by(|site| site.facility_id)
            .into_iter()
            .map(|(facility_id, group)| (facility_id, group.collect()))
            .collect();

        Ok(sites)
    }
}
