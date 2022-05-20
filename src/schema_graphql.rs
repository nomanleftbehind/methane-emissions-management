use std::convert::TryInto;
// use std::sync::Arc;

use juniper::{graphql_object, EmptySubscription, FieldResult, RootNode};

use crate::db::DbPool;
use crate::db2::user::{self, user::User};
use crate::models::thermostat_status::*;
use actix_web::web::Data;

#[derive(Clone)]
pub struct Context {
    pub db_pool: Data<DbPool>,
}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[graphql_object(context = Context)]
impl QueryRoot {
    #[graphql(description = "Query the current (latest) thermostat status")]
    fn thermostat_status(context: &Context) -> FieldResult<ThermostatStatus> {
        let connection = &context.db_pool.get()?;

        let result = ThermostatStatus::get_latest(connection)?;
        Ok(result)
    }

    #[graphql(description = "Query the thermostat status history")]
    fn thermostat_status_history(context: &Context) -> FieldResult<Vec<ThermostatStatus>> {
        let connection = &context.db_pool.get()?;

        let results = ThermostatStatus::get_history(connection)?;
        Ok(results)
    }

    #[graphql(arguments(start(default = 0), range(default = 20)))]
    async fn list_user(
        context: &Context,
        first_name: String,
        start: i32,
        range: i32,
    ) -> FieldResult<Vec<User>> {
        let start: usize = start.try_into()?;
        let range: usize = range.try_into()?;
        let end = start + range;

        let users = user::repository::Repository::find_by_name(&context.db_pool, &first_name)?;

        // for user in &users {
        //     context
        //         .loaders
        //         .users
        //         .prime(user.id.clone(), user.clone())
        //         .await;
        // }

        let users = match users.len() {
            n if n > end => users[start..end].to_vec(),
            n if n > start => users[start..].to_vec(),
            _ => Vec::new(),
        };

        Ok(users.into_iter().map(|u| u.into()).collect())
    }
}

pub struct MutationRoot;

#[graphql_object(context = Context)]
impl MutationRoot {
    #[graphql(description = "Set the thermostat status")]
    fn set_thermostat_status(
        context: &Context,
        data: NewThermostatStatus,
    ) -> FieldResult<ThermostatStatus> {
        let connection = &context.db_pool.get()?;

        ThermostatStatus::insert(connection, data)?;

        let result = ThermostatStatus::get_latest(connection)?;
        Ok(result)
    }
}

pub type SchemaGraphQL = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> SchemaGraphQL {
    SchemaGraphQL::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}

pub fn create_context(db_pool: Data<DbPool>) -> Context {
    Context { db_pool }
}
