use super::super::{
    facility::Facility,
    month_methane_emission::MonthMethaneEmission,
    routine::{
        compressor_seal::Compressor,
        defined_vent_gas::storage_tank::StorageTank,
        pneumatic_device::{
            level_controller::LevelController, non_level_controller::NonLevelController,
        },
    },
    user::User,
};
use crate::graphql::{
    context::ContextExt,
    dataloaders::{
        facility::FacilityLoader,
        month_methane_emission::MonthMethaneEmissionsBySiteLoader,
        routine::{
            compressor_seal::SiteCompressorsLoader,
            defined_vent_gas::storage_tank::SiteStorageTanksLoader,
            pneumatic_device::{
                level_controller::SiteLevelControllersLoader,
                non_level_controller::SiteNonLevelControllersLoader,
            },
        },
        user::UserLoader,
    },
};
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, Error, SimpleObject};
use chrono::NaiveDateTime;
use common::SiteType;
use sqlx::FromRow;
use uuid::Uuid;

/// The area defined by the boundaries of a surface lease for upstream oil and gas facilities and wells (pads counted as one lease).
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

    async fn non_level_controllers(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<NonLevelController>, Error> {
        let loader = ctx.get_loader::<DataLoader<SiteNonLevelControllersLoader>>();
        let non_level_controllers = loader.load_one(self.id).await?;
        let result = non_level_controllers.unwrap_or(vec![]);

        Ok(result)
    }

    async fn level_controllers(&self, ctx: &Context<'_>) -> Result<Vec<LevelController>, Error> {
        let loader = ctx.get_loader::<DataLoader<SiteLevelControllersLoader>>();
        let level_controllers = loader.load_one(self.id).await?;
        let result = level_controllers.unwrap_or(vec![]);

        Ok(result)
    }

    async fn compressors(&self, ctx: &Context<'_>) -> Result<Vec<Compressor>, Error> {
        let loader = ctx.get_loader::<DataLoader<SiteCompressorsLoader>>();
        let compressors = loader.load_one(self.id).await?;
        let result = compressors.unwrap_or(vec![]);

        Ok(result)
    }

    async fn storage_tanks(&self, ctx: &Context<'_>) -> Result<Vec<StorageTank>, Error> {
        let loader = ctx.get_loader::<DataLoader<SiteStorageTanksLoader>>();
        let storage_tanks = loader.load_one(self.id).await?;
        let result = storage_tanks.unwrap_or(vec![]);

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
