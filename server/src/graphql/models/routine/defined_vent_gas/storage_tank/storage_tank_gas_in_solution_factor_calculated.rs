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
        let storage_tank = loader.load_one(self.storage_tank_id).await;

        storage_tank
    }
}

#[derive(SimpleObject, Clone, FromRow, Debug)]
pub struct StorageTankGasInSolutionFactorCalculatedInterim {
    pub storage_tank_id: Uuid,
    pub date: NaiveDate,
    pub gis_factor: f64,
}

#[derive(Debug)]
pub struct StorageTankGasInSolutionFactorCalculatedInterimUnnestedRows(
    pub Vec<StorageTankGasInSolutionFactorCalculatedInterim>,
);

#[derive(Debug)]
pub struct StorageTankGasInSolutionFactorCalculatedInterimNestedRows {
    pub id: Vec<Uuid>,
    pub storage_tank_id: Vec<Uuid>,
    pub date: Vec<NaiveDate>,
    pub gis_factor: Vec<f64>,
    pub created_at: Vec<NaiveDateTime>,
}

impl From<StorageTankGasInSolutionFactorCalculatedInterimUnnestedRows>
    for StorageTankGasInSolutionFactorCalculatedInterimNestedRows
{
    fn from(
        StorageTankGasInSolutionFactorCalculatedInterimUnnestedRows(
            storage_tank_gas_in_solution_factors_calculated_interim,
        ): StorageTankGasInSolutionFactorCalculatedInterimUnnestedRows,
    ) -> Self {
        let (id, storage_tank_id, date, gis_factor, created_at): (
            Vec<_>,
            Vec<_>,
            Vec<_>,
            Vec<_>,
            Vec<_>,
        ) = storage_tank_gas_in_solution_factors_calculated_interim
            .into_iter()
            .map(|cmvc| {
                (
                    Uuid::new_v4(),
                    cmvc.storage_tank_id,
                    cmvc.date,
                    cmvc.gis_factor,
                    chrono::Utc::now().naive_utc(),
                )
            })
            .multiunzip();

        StorageTankGasInSolutionFactorCalculatedInterimNestedRows {
            id,
            storage_tank_id,
            date,
            gis_factor,
            created_at,
        }
    }
}
