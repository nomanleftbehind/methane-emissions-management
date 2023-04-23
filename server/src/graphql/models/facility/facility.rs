use crate::graphql::{
    context::ContextExt,
    dataloaders::{
        gas_analysis_loader::GasAnalysesByFacilityLoader,
        month_methane_emission::MonthMethaneEmissionsByFacilityLoader, site::FacilitySitesLoader,
        tank_farm_loader::FacilityTankFarmLoader, user::UserLoader,
    },
    models::{
        compressor::Compressor, site::Site, user::User, GasAnalysis, MonthMethaneEmission, TankFarm,
    },
};
use async_graphql::{
    dataloader::DataLoader, ComplexObject, Context, Error, InputObject, OneofObject, SimpleObject,
};
use chrono::NaiveDateTime;
use common::FacilityType;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, OneofObject)]
pub enum FacilityBy {
    Type(FacilityType),
    Name(String),
}

#[derive(SimpleObject, Clone, FromRow, Debug)]
#[graphql(complex)]
pub struct Facility {
    pub id: Uuid,
    pub idpa: String,
    pub name: String,
    pub r#type: FacilityType,
    pub created_by_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_by_id: Uuid,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl Facility {
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

    async fn sites(&self, ctx: &Context<'_>) -> Result<Vec<Site>, Error> {
        let loader = ctx.get_loader::<DataLoader<FacilitySitesLoader>>();
        let sites = loader.load_one(self.id).await?;
        let result = sites.unwrap_or(vec![]);

        Ok(result)
    }

    async fn tank_farm(&self, ctx: &Context<'_>) -> Result<Option<TankFarm>, Error> {
        let loader = ctx.get_loader::<DataLoader<FacilityTankFarmLoader>>();
        let tank_farm = loader.load_one(self.id).await;

        tank_farm
    }

    async fn gas_analyses(&self, ctx: &Context<'_>) -> Result<Vec<GasAnalysis>, Error> {
        let loader = ctx.get_loader::<DataLoader<GasAnalysesByFacilityLoader>>();
        let gas_analyses = loader.load_one(self.id).await?;
        // Need to return empty vector if facility has no associated gas analyses
        let result = gas_analyses.unwrap_or(vec![]);

        Ok(result)
    }

    async fn month_methane_emissions(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<MonthMethaneEmission>, Error> {
        let loader = ctx.get_loader::<DataLoader<MonthMethaneEmissionsByFacilityLoader>>();
        let month_methane_emissions = loader.load_one(self.id).await?;
        let result = month_methane_emissions.unwrap_or(vec![]);

        Ok(result)
    }
}

#[derive(InputObject, Debug)]
pub struct LimitOffsetInput {
    pub limit: i64,
    pub offset: i64,
}
