use super::{super::super::super::user::User, StorageTank};
use crate::graphql::{
    context::ContextExt,
    dataloaders::{routine::defined_vent_gas::storage_tank::StorageTankLoader, user::UserLoader},
};
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, Error, SimpleObject};
use chrono::{NaiveDate, NaiveDateTime};
use itertools::MultiUnzip;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(SimpleObject, Clone, FromRow, Debug)]
#[graphql(complex)]
pub struct StorageTankGasInSolutionFactorCalculated {
    pub id: Uuid,
    pub storage_tank_id: Uuid,
    pub date: NaiveDate,
    pub gis_factor: f64,
    pub created_by_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_by_id: Uuid,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl StorageTankGasInSolutionFactorCalculated {
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

    async fn storage_tank(&self, ctx: &Context<'_>) -> Result<Option<StorageTank>, Error> {
        let loader = ctx.get_loader::<DataLoader<StorageTankLoader>>();
        let storage_tank_farm = loader.load_one(self.storage_tank_id).await;

        storage_tank_farm
    }
}

#[derive(SimpleObject, Clone, FromRow, Debug)]
pub struct StorageTankFarmVentFactorCalculatedInterim {
    pub storage_tank_farm_id: Uuid,
    pub date: NaiveDate,
    pub vent_factor: f64,
}

#[derive(Debug)]
pub struct StorageTankFarmVentFactorCalculatedInterimUnnestedRows {
    pub user_id: Uuid,
    pub storage_tank_farm_vent_factors_calculated_interim:
        Vec<StorageTankFarmVentFactorCalculatedInterim>,
}

#[derive(Debug)]
pub struct StorageTankFarmVentFactorCalculatedInterimNestedRows {
    pub id: Vec<Uuid>,
    pub storage_tank_farm_id: Vec<Uuid>,
    pub date: Vec<NaiveDate>,
    pub vent_factor: Vec<f64>,
    pub created_by_id: Vec<Uuid>,
    pub created_at: Vec<NaiveDateTime>,
    pub updated_by_id: Vec<Uuid>,
    pub updated_at: Vec<NaiveDateTime>,
}

impl From<StorageTankFarmVentFactorCalculatedInterimUnnestedRows>
    for StorageTankFarmVentFactorCalculatedInterimNestedRows
{
    fn from(
        StorageTankFarmVentFactorCalculatedInterimUnnestedRows {
            user_id,
            storage_tank_farm_vent_factors_calculated_interim,
        }: StorageTankFarmVentFactorCalculatedInterimUnnestedRows,
    ) -> Self {
        let (
            id,
            storage_tank_farm_id,
            date,
            vent_factor,
            created_by_id,
            created_at,
            updated_by_id,
            updated_at,
        ): (
            Vec<_>,
            Vec<_>,
            Vec<_>,
            Vec<_>,
            Vec<_>,
            Vec<_>,
            Vec<_>,
            Vec<_>,
        ) = storage_tank_farm_vent_factors_calculated_interim
            .into_iter()
            .map(|cmvc| {
                (
                    Uuid::new_v4(),
                    cmvc.storage_tank_farm_id,
                    cmvc.date,
                    cmvc.vent_factor,
                    user_id.clone(),
                    chrono::Utc::now().naive_utc(),
                    user_id.clone(),
                    chrono::Utc::now().naive_utc(),
                )
            })
            .multiunzip();

        StorageTankFarmVentFactorCalculatedInterimNestedRows {
            id,
            storage_tank_farm_id,
            date,
            vent_factor,
            created_by_id,
            created_at,
            updated_by_id,
            updated_at,
        }
    }
}
