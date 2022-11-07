use crate::graphql::{context::ContextExt, dataloaders::UserLoader, domain::User};
use async_graphql::dataloader::DataLoader;
use async_graphql::*;
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
// #[graphql(complex)]
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
