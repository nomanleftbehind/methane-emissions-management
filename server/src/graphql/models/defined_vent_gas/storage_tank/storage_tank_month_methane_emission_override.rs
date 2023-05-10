use super::StorageTank;
use crate::graphql::{
    context::ContextExt,
    dataloaders::{defined_vent_gas::storage_tank::StorageTankLoader, user::UserLoader},
    models::User,
};
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, Error, SimpleObject};
use chrono::{NaiveDate, NaiveDateTime};
use sqlx::FromRow;
use uuid::Uuid;

/// Model representing user overrides of calculated monthly vented volumes from storage tanks.
///
/// Field `month` is a [`NaiveDate`](chrono::NaiveDate), which must be first day of the month. This is impossible to enforce on database level, but is instead guaranteed through [`MonthBeginningValidator`](crate::graphql::models::validators::MonthBeginningValidator).
///
/// Field `gas_volume` is in m³.
#[derive(SimpleObject, Clone, FromRow, Debug)]
#[graphql(complex)]
pub struct StorageTankMonthMethaneEmissionOverride {
    pub id: Uuid,
    pub storage_tank_id: Uuid,
    pub month: NaiveDate,
    pub gas_volume: f64,
    pub comment: Option<String>,
    pub created_by_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_by_id: Uuid,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl StorageTankMonthMethaneEmissionOverride {
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
