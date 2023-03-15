use crate::models::mutations::user::login::LoginLogin;
use self::me::MeMe;
use super::super::UUID;
use common::Role;
use graphql_client::GraphQLQuery;
use serde::Serialize;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.json",
    query_path = "./graphql/queries.graphql",
    response_derives = "Debug, Clone, PartialEq",
    extern_enums("Role")
)]
#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Me;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.json",
    query_path = "./graphql/queries.graphql",
    response_derives = "Debug, Clone, PartialEq",
    extern_enums("Role")
)]
#[derive(Debug, PartialEq)]
pub struct GetUsers;

impl From<LoginLogin> for MeMe {
    fn from(LoginLogin { id, email, role }: LoginLogin) -> Self {
        Self { id, email, role }
    }
}
