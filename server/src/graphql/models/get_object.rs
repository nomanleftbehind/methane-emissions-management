use async_graphql::SimpleObject;
use sqlx::FromRow;

#[derive(SimpleObject, Debug, Clone, FromRow, PartialEq)]
pub struct IdSelection {
    pub id: String,
    pub name: String,
}
