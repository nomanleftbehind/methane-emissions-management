use std::sync::Mutex;

use super::models::FormController;
use super::models::NEW_POST_USER_CACHE;
use super::{models::Controller, provider};
use crate::graphql_module::context::{get_conn_from_ctx, get_redis_conn_from_ctx};
use crate::graphql_module::modules::utils::rate_limiter::RateLimiter;
use crate::graphql_module::modules::utils::redis::{create_connection, get_post_cache_key};
use crate::{
    graphql_module::modules::{utils::error::ServiceError, utils::kafka},
    graphql_module::{
        context::get_redis_conn_manager,
        schema::{Mutation, Query},
    },
};
use async_graphql::Error;
use async_graphql::*;
use chrono::{Local, NaiveDateTime};
use redis::{aio::ConnectionManager, AsyncCommands, RedisError, Value};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Default)]
pub struct PostQuery;

#[derive(SimpleObject, Serialize, Deserialize, Clone)]
pub struct ControllerObject {
    pub id: ID,
    pub created_by_id: User2,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub function: Option<String>,
}

#[Object]
impl PostQuery {
    /// Resolver Reference
    #[graphql(entity)]
    pub async fn get_user_details(&self, #[graphql(key)] id: ID) -> User2 {
        User2 { id }
    }
    #[graphql(entity)]
    pub async fn get_post_details(
        &self,
        ctx: &Context<'_>,
        #[graphql(key)] post_id: ID,
    ) -> Option<ControllerObject> {
        let cache_key = get_post_cache_key(post_id.to_string().as_str());
        let mut redis_connection_manager = get_redis_conn_manager(ctx).await;
        let cached_post: Value = redis_connection_manager
            .get(cache_key.clone())
            .await
            .expect("");
        //  Check if the data in cache exists, if none, retrieve the data from the database
        //  Chain multiple commands and query it to the connection manager
        match cached_post {
            Value::Nil => {
                let post = get_post_detail(ctx, post_id);
                let _: () = redis::pipe()
                    .atomic()
                    .set(&cache_key, post.clone())
                    .expire(&cache_key, 60)
                    .query_async(&mut redis_connection_manager)
                    .await
                    .expect("Internal Error Occurred while attempting to cache the object");
                return post;
            }
            Value::Data(cache) => serde_json::from_slice(&cache).expect(""),
            _ => None,
        }
    }

    #[graphql(name = "getAllPost")]
    async fn get_post(&self, ctx: &Context<'_>) -> Vec<ControllerObject> {
        let conn = get_conn_from_ctx(ctx);
        provider::get_all(&conn)
            .expect("Cannot get Blog ControllerObject ")
            .iter()
            .map(ControllerObject::from)
            .collect()
    }
    #[graphql(name = "getPostbyId")]
    pub async fn get_post_by_id(&self, ctx: &Context<'_>, post_id: ID) -> Option<ControllerObject> {
        let cache_key = get_post_cache_key(post_id.to_string().as_str());
        let redis_client = get_redis_conn_from_ctx(ctx).await;

        let mut redis_connection = create_connection(redis_client)
            .await
            .expect("Unable to create Redis DB Connection");
        let cached_object = redis_connection.get(cache_key.clone()).await.expect("");

        //  Check If Cache Object is available
        match cached_object {
            Value::Nil => {
                let post = get_post_detail(ctx, post_id);

                let _: () = redis::pipe()
                    .atomic()
                    .set(&cache_key, post.clone())
                    .expire(&cache_key, 60)
                    .query_async(&mut redis_connection)
                    .await
                    .expect("Internal Error Occurred while attempting to cache the object");

                return post;
            }
            Value::Data(cache) => serde_json::from_slice(&cache).expect(""),
            _ => None,
        }
    }
    #[graphql(name = "getPostsbyAuthor")]
    async fn get_post_by_authorid(&self, ctx: &Context<'_>, user_id: ID) -> Vec<ControllerObject> {
        get_posts_user(ctx, user_id)
    }
}

/// Gets the Post Information by using the Post Id
pub fn get_post_detail(ctx: &Context<'_>, post_id: ID) -> Option<ControllerObject> {
    provider::get_post_by_id(parse_id(post_id), &get_conn_from_ctx(ctx))
        .ok()
        .map(|f| ControllerObject::from(&f))
}
/// Gets the Post under the author: UserId
pub fn get_posts_user(ctx: &Context<'_>, user_id: ID) -> Vec<ControllerObject> {
    provider::get_by_posts_by_author(parse_id(user_id), &get_conn_from_ctx(ctx))
        .expect("Cannot get any User2 Posts")
        .iter()
        .map(|s| ControllerObject::from(s))
        .collect()
}
#[derive(Serialize, Deserialize, Clone)]
pub struct User2 {
    pub id: ID,
}
#[Object(extends)]
impl User2 {
    /// The Key needed for
    #[graphql(external)]
    pub async fn id(&self, id: ID) -> User2 {
        User2 { id }
    }
    /// Load all Posts under User2
    #[graphql(name = "getPostsByUser")]
    pub async fn get_user_posts(&self, ctx: &Context<'_>, id: ID) -> Vec<ControllerObject> {
        get_posts_user(ctx, id)
    }
}

#[derive(Default)]
pub struct PostMutation;

#[derive(InputObject)]
pub struct ControllerInput {
    pub created_by_id: ID,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub function: Option<String>,
}
#[Object]
impl PostMutation {
    /// Create A New Post
    /// The server responds by caching the new Post with Default
    #[graphql(name = "createPost")]
    async fn create_post(
        &self,
        ctx: &Context<'_>,
        form: ControllerInput,
    ) -> Result<ControllerObject, Error> {
        let post = provider::create_post(FormController::from(&form), &get_conn_from_ctx(ctx))?;
        let serialized_post = serde_json::to_string(&ControllerObject::from(&post))
            .map_err(|_| ServiceError::InternalError)?;

        //  In the mutation, post creation a messgage is sent to the kafka.
        let producer = ctx
            .data::<FutureProducer>()
            .expect("Cannot get Kafka Producer");
        kafka::send_message(producer, serialized_post).await;
        Ok(ControllerObject::from(&post))
    }
    #[graphql(name = "updatePosts")]
    async fn update_post(
        &self,
        ctx: &Context<'_>,
        form: ControllerInput,
        post_id: ID,
        user_id: ID,
    ) -> Result<ControllerObject, Error> {
        //  Convert the grahql input into readable database input
        let new_post = provider::update_post(
            parse_id(post_id.clone()),
            parse_id(user_id),
            FormController::from(&form),
            &get_conn_from_ctx(ctx),
        )
        .expect("");
        //  Delete the cache under this value
        let cache_key = get_post_cache_key(post_id.to_string().as_str());
        let redis_connection_manager = get_redis_conn_manager(ctx);
        redis_connection_manager.await.del(cache_key).await?;

        //  Convert Post (from the database), into Graphql object
        Ok(ControllerObject::from(&new_post))
    }
    #[graphql(name = "deletePosts")]
    async fn delete_post(
        &self,
        ctx: &Context<'_>,
        post_author: ID,
        post_id: ID,
    ) -> Result<bool, Error> {
        let conn = get_conn_from_ctx(ctx);
        provider::delete_post(parse_id(post_author), parse_id(post_id.clone()), &conn)
            .expect("Couldn't delete Post");

        //  Deletes the cache under this postid
        let cache_key = get_post_cache_key(post_id.to_string().as_str());
        get_redis_conn_manager(ctx).await.del(cache_key).await?;

        Ok(true)
    }
}

//  Get the latest Posts
//  Subscriptions
use crate::graphql_module::modules::utils::kafka::{
    create_consumer, create_producer, get_kafka_consumer_id, send_message,
};
use futures::{Stream, StreamExt};
use rdkafka::{producer::FutureProducer, Message};
pub struct Subscription;
//  The API client can be notified of the event by a subscription that listens to Kafka consumer
#[Subscription]
impl Subscription {
    async fn latest_post<'ctx>(
        &self,
        ctx: &'ctx Context<'_>,
    ) -> impl Stream<Item = ControllerObject> + 'ctx {
        let kafka_consumer = ctx
            .data::<Mutex<Uuid>>()
            .expect("Cannot get the Kafka Consumer counter");
        let consumer_id = get_kafka_consumer_id(kafka_consumer);
        let consumer = create_consumer(consumer_id);
        // stream! macros returns an anonymous type implementing the Stream trait.
        async_stream::stream! {
            let mut stream = consumer.stream();
            while let Some(val) = stream.next().await {
                yield match val {
                    Ok(msg) => {
                        let payload = msg.payload().expect("Kafka msg should contain payload");
                        let msg = String::from_utf8_lossy(payload).to_string();
                        serde_json::from_str(&msg).expect("Cannot Deserialize a Message")
                    }
                    Err(e) => panic!("Error while Kafka message processing: {}", e)
                };
            }
        }
    }
}
//  Helper Parser
pub fn parse_id(id: ID) -> Uuid {
    id.parse::<Uuid>().expect("ParseIntError")
}
