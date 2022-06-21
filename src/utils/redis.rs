use super::error::ServiceError;
use crate::repository::{controller::resolver::ControllerObject, user::resolver::User};
use redis::{aio::Connection, Client, RedisError, ToRedisArgs};
use std::env;

lazy_static! {
    static ref BLOG_KEY_PREFIX: String =
        std::env::var("REDIS_KEY_PREFIX").expect("JWT Secret Key Error");
}

pub enum RedisDatabase {
    Example,
    ExampleSet,
}
/// Connect to Redis
pub async fn create_client(cache: RedisDatabase) -> Result<Client, RedisError> {
    let a = env::var("REDIS_URL").expect("Nope didn't get redis url");
    println!("REDIS_URL: {}", a);
    let redis_url = match cache {
        RedisDatabase::Example => env::var("REDIS_URL").expect("Cannot Read Redis URI"),
        _ => env::var("REDIS_URL_TEST").expect("Cannot Redis TEST URL"),
    };

    Ok(Client::open(redis_url)?)
}
/// Establish connection to redis
pub async fn create_connection(redis_client: Client) -> Result<Connection, ServiceError> {
    redis_client
        .get_async_connection()
        .await
        .map_err(|_| ServiceError::InternalError)
}
/// Post Caching Key
pub fn get_controller_cache_key(post_id: &str) -> String {
    format!("{}:{}", BLOG_KEY_PREFIX.as_str(), post_id)
}

/// Write the Value into a vector of bytes, in this case, we are caching a Post
/// and turning into a string so we can use it as an argument for K-V pair
/// 'write_redis_args', each item is a single argument
impl ToRedisArgs for User {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        out.write_arg_fmt(
            serde_json::to_string(self).expect("Cannot Serialize PostObject as String"),
        )
    }
}

/// Write the Value into a vector of bytes, in this case, we are caching a Post
/// and turning into a string so we can use it as an argument for K-V pair
/// 'write_redis_args', each item is a single argument
impl ToRedisArgs for ControllerObject {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        out.write_arg_fmt(
            serde_json::to_string(self).expect("Cannot Serialize ControllerObject as String"),
        )
    }
}

//  Redis Pub/ Sub
//  Senders are not programmed to send their messages to specific receivers
//  Rather, they will publish messages irrespectively without having the
//  knowledge of what subscribers there may be
pub async fn start_pubsub(_client: &Client) -> Result<(), ServiceError> {
    // let mut pubsub_conn = create_connection(client).await?.into_pubsub();

    todo!()
}
