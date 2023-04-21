use crate::graphql::{
    context::ContextExt,
    dataloaders::{
        compressor_loader::FacilityCompressorsLoader,
        gas_analysis_loader::GasAnalysesByFacilityLoader,
        pneumatic_device::SitePneumaticDevicesLoader, tank_farm_loader::FacilityTankFarmLoader,
        user::UserLoader,
    },
    models::{
        compressor::Compressor, pneumatic_device::PneumaticDevice, GasAnalysis, TankFarm, User,
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
}
