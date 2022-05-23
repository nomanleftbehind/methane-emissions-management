use crate::schemas::root::{Context, MutationRoot, QueryRoot};
use std::convert::TryInto;

use juniper::{graphql_object, FieldResult};

use crate::db::{
    license_change::{self, LicenseChange, NewLicenseChangeForm},
    user::{self, user::User},
};
use crate::models::thermostat_status::*;

use uuid::Uuid;

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

    fn create_license_change(
        context: &Context,
        new_license_change: NewLicenseChangeForm,
    ) -> FieldResult<LicenseChange> {
        let created_license_change =
            license_change::repository::Repository::insert(&context.db_pool, new_license_change)?;

        Ok(created_license_change.into())
    }

    // fn create_license_change_pg_connection(
    //     context: &Context,
    //     new_license_change: NewLicenseChangeForm,
    // ) -> FieldResult<LicenseChange> {
    //     let connection = &context.db_pool.get()?;
    //     LicenseChange::insert_pg_connection(connection, new_license_change)?;

    //     let result = LicenseChange::get_latest(connection)?;
    //     Ok(result)
    // }

    fn delete_license_change(context: &Context, id: Uuid) -> FieldResult<LicenseChange> {
        let deleted_license_change =
            license_change::repository::Repository::delete(&context.db_pool, id)?;

        Ok(deleted_license_change.into())
    }
}
