use crate::graphql::{
    context::ContextExt,
    dataloaders::{
        gas_analysis::GasAnalysesByFacilityLoader,
        month_methane_emission::MonthMethaneEmissionsByFacilityLoader, site::FacilitySitesLoader,
        user::UserLoader,
    },
    models::{
        gas_analysis::GasAnalysis, month_methane_emission::MonthMethaneEmission, site::Site,
        user::User,
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

/// Any building, structure, installation, equipment or appurtenance over which the Regulator has jurisdiction and that is connected to or associated with the recovery, development, production, handling, processing, treatment or disposal of hydrocarbon‑based resources, including synthetic coal gas and synthetic coal liquid, or any associated substances or wastes or the disposal of captured carbon dioxide.
#[derive(SimpleObject, Clone, FromRow, Debug)]
#[graphql(complex)]
pub struct Facility {
    pub id: Uuid,
    /// As defined in Directive 047, a unique facility identification code, with 4 letters and 7 numbers (e.g., ABWP1234567), assigned by Petrinex to each facility.
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

    async fn gas_analyses(&self, ctx: &Context<'_>) -> Result<Vec<GasAnalysis>, Error> {
        let loader = ctx.get_loader::<DataLoader<GasAnalysesByFacilityLoader>>();
        let gas_analyses = loader.load_one(self.id).await?;
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
