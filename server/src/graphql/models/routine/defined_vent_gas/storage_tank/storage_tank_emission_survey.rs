use super::{
    super::super::super::{survey_equipment::SurveyEquipment, user::User},
    StorageTank,
};
use crate::graphql::{
    context::ContextExt,
    dataloaders::{
        routine::defined_vent_gas::storage_tank::StorageTankLoader,
        survey_equipment::SurveyEquipmentLoader, user::UserLoader,
    },
};
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, Error, SimpleObject};
use chrono::{NaiveDate, NaiveDateTime};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(SimpleObject, Clone, FromRow, Debug)]
#[graphql(complex)]
pub struct StorageTankEmissionSurvey {
    pub id: Uuid,
    pub storage_tank_id: Uuid,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    /// standard cubic feet per hour of methane
    pub rate: f64,
    pub survey_point: String,
    pub leak_duration: Option<f64>,
    pub survey_equipment_id: Uuid,
    pub created_by_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_by_id: Uuid,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl StorageTankEmissionSurvey {
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

    async fn survey_equipment(&self, ctx: &Context<'_>) -> Result<Option<SurveyEquipment>, Error> {
        let loader = ctx.get_loader::<DataLoader<SurveyEquipmentLoader>>();
        let survey_equipment = loader.load_one(self.survey_equipment_id).await;

        survey_equipment
    }
}
