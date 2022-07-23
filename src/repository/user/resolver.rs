use super::{model::NewUser, provider};
use crate::graphql::context::{get_conn_from_ctx, get_redis_conn_from_ctx};
use crate::loader::ControllerLoader;
use crate::repository::controller::resolver::ControllerObject;
use crate::utils::redis::{create_connection, get_controller_cache_key};
use crate::utils::token::Role as AuthRole;
use crate::utils::user_utils::{hash_password, is_admin, verify_password, Role, RoleGuard};
use async_graphql::{dataloader::DataLoader, Error, *};
use chrono::NaiveDateTime;
use redis::{AsyncCommands, Value};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Default)]
pub struct UserQuery;

#[derive(
    SimpleObject, /* directly maps all the fields of a struct to GraphQL object */
    Serialize,
    Deserialize,
    Clone,
    Debug,
)]
#[graphql(complex)] // `complex` attribute required for `ComplexObject` macro to take effect.
pub struct User {
    pub id: ID,
    pub created_at: NaiveDateTime,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub location: Option<String>,
    pub email: String,
    pub hash: String,
    pub role: String,
}

#[ComplexObject] // let's you write user-defined resolvers for the calculated fields
impl User {
    async fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    async fn created_controllers(&self, ctx: &Context<'_>) -> Result<Vec<ControllerObject>> {
        let loader = ctx.data_unchecked::<DataLoader<ControllerLoader>>();

        let user_id = self
            .id
            .to_string()
            .parse::<Uuid>()
            .expect("Can't convert id");
        let controllers_option = loader.load_one(user_id).await?;

        let result = match controllers_option {
            None => Vec::new(),
            Some(controllers) => controllers,
        };

        Ok(result)
    }
}

#[Object]
impl UserQuery {
    /// Reference Resolver for User Details
    #[graphql(entity)]
    pub async fn get_user_details(&self, ctx: &Context<'_>, id: ID) -> Result<Option<User>, Error> {
        find_user_details(ctx, &id)
    }

    #[graphql(name = "getAllUsers")]
    pub async fn get_all(&self, ctx: &Context<'_>) -> Result<Vec<User>, Error> {
        let conn = &get_conn_from_ctx(ctx);
        let users = provider::get_all_users(conn)
            .expect("Cannot get Users")
            .iter()
            .map(User::from)
            .collect();
        Ok(users)
    }

    #[graphql(name = "getAllbyEmail")]
    pub async fn get_users_by_email(
        &self,
        ctx: &Context<'_>,
        user_email: String,
    ) -> Result<Option<User>, Error> {
        let conn = &get_conn_from_ctx(ctx);
        let user = provider::get_user_by_email(user_email, conn)
            .ok()
            .map(|x| User::from(&x));
        Ok(user)
    }

    #[graphql(
        name = "getAllbyId", // guard = "RoleGuard::new(AuthRole::Admin)", // visible = "is_admin"
    )]
    /// This is the test of a description
    pub async fn get_users_by_id(&self, ctx: &Context<'_>, id: ID) -> Option<User> {
        let cache_key = get_controller_cache_key(id.to_string().as_str());
        let redis_client = get_redis_conn_from_ctx(ctx).await;
        let mut redis_connection = create_connection(redis_client)
            .await
            .expect("Unable to create Redis DB connection");
        let cached_object = redis_connection
            .get(cache_key.clone())
            .await
            .expect("Redis Error on Client ");

        //  Check If Cache Object is available
        match cached_object {
            Value::Nil => {
                log::info!("Unable to find cache under this id, accessing Database.. 😂");

                let user = find_user_details(ctx, &id).expect("Unable to get User Details");
                let _: () = redis::pipe()
                    .atomic()
                    .set(&cache_key, user.clone())
                    .expire(&cache_key, 60)
                    .query_async(&mut redis_connection)
                    .await
                    .expect("Internal Error Occurred while attempting to cache the object");
                return user;
            }
            Value::Data(cache) => {
                log::info!("Cache Found Under this Id! 👌");
                serde_json::from_slice(&cache).expect("Unable to Deserialize Struct")
            }
            _ => None,
        }
    }
    #[graphql(
        name = "getAllbyusername",
        guard = "RoleGuard::new(AuthRole::Admin)",
        visible = "is_admin"
    )]
    pub async fn get_users_by_username(
        &self,
        ctx: &Context<'_>,
        user_username: String,
    ) -> Result<Option<User>, Error> {
        let conn = &get_conn_from_ctx(ctx);
        let user = provider::get_user_by_username(user_username, conn)
            .ok()
            .map(|x| User::from(&x));

        Ok(user)
    }
}
pub fn find_user_details(ctx: &Context<'_>, id: &ID) -> Result<Option<User>, Error> {
    let conn = &get_conn_from_ctx(ctx);
    let id = id
        .to_string()
        .parse::<Uuid>()
        .expect("Unable to get Id from String");
    let user = provider::get_user_by_id(&id, conn)
        .ok()
        .map(|x| User::from(&x));
    Ok(user)
}

#[derive(Default)]
pub struct UserMutation;
///  User Mutation Classes types
#[derive(InputObject, Deserialize, Serialize, Clone)]
#[graphql(input_name = "registerInput")]
pub struct RegisterInput {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub location: Option<String>,
    pub email: String,
    pub password: String,
    pub role: Role,
}

#[derive(InputObject, Deserialize, Serialize, Clone)]
#[graphql(input_name = "signInInput")]
pub struct SignInInput {
    pub username: String,
    pub password: String,
}

#[Object]
impl UserMutation {
    // #[graphql(name = "registerUsers", guard = "RoleGuard::new(AuthRole::Admin)", visible = "is_admin")]
    #[graphql(name = "registerUser")]
    /// This will register the users in our database
    pub async fn register_user(
        &self,
        ctx: &Context<'_>,
        register: RegisterInput,
    ) -> Result<User, Error> {
        let conn = &get_conn_from_ctx(ctx);
        let new_user = NewUser {
            first_name: register.first_name,
            last_name: register.last_name,
            username: register.username,
            location: register.location,
            email: register.email,
            hash: hash_password(register.password.as_str()).expect("Unable to hash user password"),
            role: register.role.to_string(),
        };
        let user_created =
            provider::create_user(new_user, conn).expect("Cannot create user right now");
        Ok(User::from(&user_created))
    }
    #[graphql(name = "signInUser")]
    /// The user should be able to
    pub async fn sign_in(&self, ctx: &Context<'_>, input: SignInInput) -> Result<String, Error> {
        let conn = &get_conn_from_ctx(ctx);
        let get_user = provider::get_user_by_username(input.username, conn);
        if let Some(user) = get_user.ok() {
            if let Ok(matching) = verify_password(&user.hash, &input.password) {
                if matching {
                    let role = AuthRole::from_str(user.role.as_str())
                        .expect("Unable to convert to AuthRole");
                    return Ok(crate::utils::token::generate_token(user.username, role));
                }
            }
        }
        Err(Error::new("Unable to Authenticate the User"))
    }
}
