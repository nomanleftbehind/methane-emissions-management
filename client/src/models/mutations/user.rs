use self::login::LoginLogin;
use super::super::UUID;
use crate::models::queries::user::me::MeMe;
use common::Role;
use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/mutations.graphql",
    response_derives = "Debug, Clone, PartialEq, Eq",
    extern_enums("Role")
)]
pub struct Login;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/mutations.graphql",
    response_derives = "Debug, Clone, PartialEq"
)]
pub struct Logout;

impl From<MeMe> for LoginLogin {
    fn from(MeMe { id, email, role }: MeMe) -> Self {
        Self { id, email, role }
    }
}
