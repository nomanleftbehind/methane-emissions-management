use async_graphql::SimpleObject;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(SimpleObject, Debug, Clone, FromRow, PartialEq)]
pub struct IdSelection {
    pub id: Uuid,
    pub name: String,
}
