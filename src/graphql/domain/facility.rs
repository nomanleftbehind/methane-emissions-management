use crate::graphql::{
    context::ContextExt,
    dataloaders::{FacilityControllersLoader, UserLoader},
    domain::{Controller, User},
};
use async_graphql::{
    dataloader::DataLoader, ComplexObject, Context, Enum, Error, OneofObject, SimpleObject,
};
use sqlx::{types::time::PrimitiveDateTime, FromRow};
use uuid::Uuid;

#[derive(Debug, OneofObject)]
pub enum FacilityBy {
    Type(FacilityType),
    Name(String),
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug, sqlx::Type)]
#[sqlx(type_name = "facility_type")]
pub enum FacilityType {
    TM,
    WT,
    CT,
    DS,
    GS,
    MS,
    GP,
    IF,
    PL,
    WP,
    WS,
    BT,
}

#[derive(SimpleObject, Clone, FromRow, Debug)]
#[graphql(complex)]
pub struct Facility {
    pub id: Uuid,
    pub idpa: String,
    pub name: String,
    pub r#type: FacilityType,
    pub created_by_id: Uuid,
    pub created_at: PrimitiveDateTime,
    pub updated_by_id: Uuid,
    pub updated_at: PrimitiveDateTime,
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

    async fn controllers(&self, ctx: &Context<'_>) -> Result<Vec<Controller>, Error> {
        let loader = ctx.get_loader::<DataLoader<FacilityControllersLoader>>();
        let controllers = loader.load_one(self.id).await?;
        // Need to return empty vector if user has no written controllers
        let result = controllers.unwrap_or(vec![]);

        Ok(result)
    }
}
