use super::{
    CompressorBlowdown, CompressorChange, CompressorMonthHours, CompressorMonthVentOverride,
};
use crate::graphql::{
    context::ContextExt,
    dataloaders::{
        compressor::{
            CompressorBlowdownsByCompressorLoader, CompressorChangesByCompressorLoader,
            CompressorMonthHoursByCompressorLoader, CompressorMonthVentOverridesByCompressorLoader,
        },
        site::SiteLoader,
        user::UserLoader,
    },
    models::{site::Site, user::User},
};
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, Error, SimpleObject};
use chrono::{NaiveDate, NaiveDateTime};
use common::CompressorType;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(SimpleObject, Clone, FromRow, Debug)]
#[graphql(complex)]
pub struct Compressor {
    pub id: Uuid,
    pub site_id: Uuid,
    pub fdc_rec_id: String,
    pub r#type: CompressorType,
    pub controlled: bool,
    pub name: String,
    pub serial_number: String,
    pub throw_count: Option<i32>,
    pub install_date: NaiveDate,
    pub remove_date: Option<NaiveDate>,
    pub created_by_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_by_id: Uuid,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl Compressor {
    pub(in crate::graphql) async fn created_by(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Option<User>, Error> {
        let loader = ctx.get_loader::<DataLoader<UserLoader>>();
        let created_by = loader.load_one(self.created_by_id).await;

        created_by
    }

    pub(in crate::graphql) async fn updated_by(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Option<User>, Error> {
        let loader = ctx.get_loader::<DataLoader<UserLoader>>();
        let updated_by = loader.load_one(self.updated_by_id).await;

        updated_by
    }

    async fn site(&self, ctx: &Context<'_>) -> Result<Option<Site>, Error> {
        let loader = ctx.get_loader::<DataLoader<SiteLoader>>();
        let site = loader.load_one(self.site_id).await;

        site
    }

    async fn compressor_changes(&self, ctx: &Context<'_>) -> Result<Vec<CompressorChange>, Error> {
        let loader = ctx.get_loader::<DataLoader<CompressorChangesByCompressorLoader>>();
        let compressor_changes = loader.load_one(self.id).await?;
        let result = compressor_changes.unwrap_or(vec![]);

        Ok(result)
    }

    async fn compressor_month_hours(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<CompressorMonthHours>, Error> {
        let loader = ctx.get_loader::<DataLoader<CompressorMonthHoursByCompressorLoader>>();
        let compressor_month_hours = loader.load_one(self.id).await?;
        let result = compressor_month_hours.unwrap_or(vec![]);

        Ok(result)
    }

    async fn compressor_blowdowns(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<CompressorBlowdown>, Error> {
        let loader = ctx.get_loader::<DataLoader<CompressorBlowdownsByCompressorLoader>>();
        let compressor_blowdowns = loader.load_one(self.id).await?;
        let result = compressor_blowdowns.unwrap_or(vec![]);

        Ok(result)
    }
}
