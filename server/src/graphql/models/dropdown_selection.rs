use async_graphql::SimpleObject;
use sqlx::FromRow;

#[derive(SimpleObject, Debug, Clone, FromRow, PartialEq)]
pub struct DropdownSelection {
    pub id: String,
    pub name: Option<String>,
}
