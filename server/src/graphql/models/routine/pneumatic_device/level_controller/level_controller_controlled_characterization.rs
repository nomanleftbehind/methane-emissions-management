use super::{
    super::super::super::user::User, LevelController, LevelControllerControlDeviceInactivity,
};
use crate::graphql::{
    context::ContextExt,
    dataloaders::{
        routine::pneumatic_device::level_controller::{
            LevelControllerControlDeviceInactivitiesByLevelControllerControlledCharacterizationLoader,
            LevelControllerLoader,
        },
        user::UserLoader,
    },
};
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, Error, SimpleObject};
use chrono::{NaiveDate, NaiveDateTime};
use common::ControlDevice;
use sqlx::FromRow;
use uuid::Uuid;

/// Object representing changes to controlled or uncontrolled characterization of level controller as defined in AER Manual 015 section [`1.1.2`](https://static.aer.ca/prd/documents/manuals/Manual015.pdf#page=10).
#[derive(SimpleObject, Clone, FromRow, Debug)]
#[graphql(complex)]
pub struct LevelControllerControlledCharacterization {
    pub id: Uuid,
    pub level_controller_id: Uuid,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub control_device: ControlDevice,
    pub comment: Option<String>,
    pub created_by_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_by_id: Uuid,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl LevelControllerControlledCharacterization {
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

    async fn level_controller(&self, ctx: &Context<'_>) -> Result<Option<LevelController>, Error> {
        let loader = ctx.get_loader::<DataLoader<LevelControllerLoader>>();
        let level_controller = loader.load_one(self.level_controller_id).await;

        level_controller
    }

    async fn level_controller_control_device_inactivities(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<LevelControllerControlDeviceInactivity>, Error> {
        let loader = ctx.get_loader::<DataLoader<
            LevelControllerControlDeviceInactivitiesByLevelControllerControlledCharacterizationLoader,
        >>();
        let level_controller_control_device_inactivities = loader.load_one(self.id).await?;
        let result = level_controller_control_device_inactivities.unwrap_or(vec![]);

        Ok(result)
    }
}
