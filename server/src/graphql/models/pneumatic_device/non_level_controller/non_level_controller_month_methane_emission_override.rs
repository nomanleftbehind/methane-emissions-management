use super::{super::super::user::User, NonLevelController};
use crate::graphql::{
    context::ContextExt,
    dataloaders::{
        pneumatic_device::non_level_controller::NonLevelControllerLoader, user::UserLoader,
    },
};
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, Error, SimpleObject};
use chrono::{NaiveDate, NaiveDateTime};
use sqlx::FromRow;
use uuid::Uuid;

/// Model representing user overrides of calculated monthly methane emission volumes from non-level controllers.
///
/// Field `month` is a [`NaiveDate`](chrono::NaiveDate), which must be first day of the month. This is impossible to enforce on database level, but is instead guaranteed through [`MonthBeginningValidator`](crate::graphql::models::validators::MonthBeginningValidator).
///
/// Field `gas_volume` is in m³.
#[derive(SimpleObject, Clone, FromRow, Debug)]
#[graphql(complex)]
pub struct NonLevelControllerMonthMethaneEmissionOverride {
    pub id: Uuid,
    pub non_level_controller_id: Uuid,
    pub month: NaiveDate,
    pub gas_volume: f64,
    pub comment: Option<String>,
    pub created_by_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_by_id: Uuid,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl NonLevelControllerMonthMethaneEmissionOverride {
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

    async fn non_level_controller(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Option<NonLevelController>, Error> {
        let loader = ctx.get_loader::<DataLoader<NonLevelControllerLoader>>();
        let non_level_controller = loader.load_one(self.non_level_controller_id).await;

        non_level_controller
    }
}
