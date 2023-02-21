use crate::graphql::{
    context::ContextExt,
    dataloaders::{
        compressor_blowdown_loader::CompressorBlowdownsByCompressorLoader,
        compressor_change_loader::CompressorChangesByCompressorLoader,
        compressor_month_hours_loader::CompressorMonthHoursByCompressorLoader,
        compressor_month_vent_loader::CompressorMonthVentsByCompressorLoader,
        facility_loader::FacilityLoader, user_loader::UserLoader,
    },
    models::{
        CompressorBlowdown, CompressorChange, CompressorMonthHours, CompressorMonthVent, Facility,
        User,
    },
};
use async_graphql::{
    dataloader::DataLoader, ComplexObject, Context, Error, OneofObject, SimpleObject,
};
use chrono::{NaiveDate, NaiveDateTime};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(SimpleObject, Clone, FromRow, Debug)]
#[graphql(complex)]
pub struct Compressor {
    pub id: Uuid,
    pub fdc_rec_id: String,
    pub facility_id: Uuid,
    pub name: String,
    pub serial_number: String,
    pub install_date: NaiveDate,
    pub remove_date: Option<NaiveDate>,
    pub created_by_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_by_id: Uuid,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl Compressor {
    pub async fn created_by(&self, ctx: &Context<'_>) -> Result<Option<User>, Error> {
        let loader = ctx.get_loader::<DataLoader<UserLoader>>();
        let created_by = loader.load_one(self.created_by_id).await;

        created_by
    }

    pub async fn updated_by(&self, ctx: &Context<'_>) -> Result<Option<User>, Error> {
        let loader = ctx.get_loader::<DataLoader<UserLoader>>();
        let updated_by = loader.load_one(self.updated_by_id).await;

        updated_by
    }

    pub async fn facility(&self, ctx: &Context<'_>) -> Result<Option<Facility>, Error> {
        let loader = ctx.get_loader::<DataLoader<FacilityLoader>>();
        let facility = loader.load_one(self.facility_id).await;

        facility
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

    async fn compressor_month_vents(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<CompressorMonthVent>, Error> {
        let loader = ctx.get_loader::<DataLoader<CompressorMonthVentsByCompressorLoader>>();
        let compressor_month_vents = loader.load_one(self.id).await?;
        let result = compressor_month_vents.unwrap_or(vec![]);

        Ok(result)
    }
}

#[derive(Debug, OneofObject)]
pub enum CompressorsBy {
    FacilityId(Uuid),
    CreatedById(Uuid),
    UpdatedById(Uuid),
}
