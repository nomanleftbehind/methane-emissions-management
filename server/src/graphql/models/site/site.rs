use crate::graphql::{
    context::ContextExt,
    dataloaders::{
        compressor::SiteCompressorsLoader, facility::FacilityLoader,
        gas_analysis_loader::GasAnalysesByFacilityLoader,
        month_methane_emission::MonthMethaneEmissionsBySiteLoader,
        pneumatic_device::SitePneumaticDevicesLoader, tank_farm_loader::FacilityTankFarmLoader,
        user::UserLoader,
    },
    models::{
        compressor::Compressor, facility::Facility, pneumatic_device::PneumaticDevice, user::User,
        GasAnalysis, MonthMethaneEmission, TankFarm,
    },
};
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, Error, SimpleObject};
use chrono::NaiveDateTime;
use common::SiteType;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(SimpleObject, Clone, FromRow, Debug)]
#[graphql(complex)]
pub struct Site {
    pub id: Uuid,
    pub facility_id: Uuid,
    pub fdc_rec_id: String,
    pub name: String,
    pub r#type: SiteType,
    pub description: Option<String>,
    pub created_by_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_by_id: Uuid,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl Site {
    async fn created_by(&self, ctx: &Context<'_>) -> Result<Option<User>, Error> {
        let loader = ctx.get_loader::<DataLoader<UserLoader>>();
        let created_by = loader.load_one(self.created_by_id).await;

        created_by
    }

    async fn updated_by(&self, ctx: &Context<'_>) -> Result<Option<User>, Error> {
        let loader = ctx.get_loader::<DataLoader<UserLoader>>();
        let updated_by = loader.load_one(self.updated_by_id).await;

        updated_by
    }

    async fn facility(&self, ctx: &Context<'_>) -> Result<Option<Facility>, Error> {
        let loader = ctx.get_loader::<DataLoader<FacilityLoader>>();
        let facility = loader.load_one(self.facility_id).await;

        facility
    }

    async fn pneumatic_devices(&self, ctx: &Context<'_>) -> Result<Vec<PneumaticDevice>, Error> {
        let loader = ctx.get_loader::<DataLoader<SitePneumaticDevicesLoader>>();
        let pneumatic_devices = loader.load_one(self.id).await?;
        let result = pneumatic_devices.unwrap_or(vec![]);

        Ok(result)
    }

    async fn compressors(&self, ctx: &Context<'_>) -> Result<Vec<Compressor>, Error> {
        let loader = ctx.get_loader::<DataLoader<SiteCompressorsLoader>>();
        let compressors = loader.load_one(self.id).await?;
        let result = compressors.unwrap_or(vec![]);

        Ok(result)
    }

    async fn month_methane_emissions(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<MonthMethaneEmission>, Error> {
        let loader = ctx.get_loader::<DataLoader<MonthMethaneEmissionsBySiteLoader>>();
        let month_methane_emissions = loader.load_one(self.id).await?;
        let result = month_methane_emissions.unwrap_or(vec![]);

        Ok(result)
    }
}
