use super::{super::super::super::user::User, PneumaticPump};
use crate::graphql::{
    context::ContextExt,
    dataloaders::{
        routine::pneumatic_device::pneumatic_pump::PneumaticPumpLoader, user::UserLoader,
    },
};
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, Error, SimpleObject};
use chrono::{NaiveDate, NaiveDateTime};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(SimpleObject, Clone, FromRow, Debug)]
#[graphql(complex)]
pub struct PneumaticPumpMonthHours {
    pub id: Uuid,
    pub pneumatic_pump_id: Uuid,
    pub month: NaiveDate,
    pub hours_on: f64,
    pub created_by_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_by_id: Uuid,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl PneumaticPumpMonthHours {
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

    async fn pneumatic_pump(&self, ctx: &Context<'_>) -> Result<Option<PneumaticPump>, Error> {
        let loader = ctx.get_loader::<DataLoader<PneumaticPumpLoader>>();
        let pneumatic_pump = loader.load_one(self.pneumatic_pump_id).await;

        pneumatic_pump
    }
}
