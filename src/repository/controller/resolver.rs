use super::{
    super::user::resolver::find_user_details,
    models::{ControllerForm, ControllerUpdateForm},
    provider,
};
use crate::graphql::context::{get_conn_from_ctx, get_redis_conn_from_ctx, get_redis_conn_manager};
use crate::loader::UserLoader;
use crate::repository::user::resolver::User;
use crate::utils::{
    error::ServiceError,
    kafka,
    redis::{create_connection, get_controller_cache_key},
};
use async_graphql::{dataloader::DataLoader, Error, *};
use chrono::NaiveDateTime;
use redis::{AsyncCommands, Value};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use uuid::Uuid;

#[derive(Default)]
pub struct ControllerQuery;

#[derive(SimpleObject, Serialize, Deserialize, Clone, Debug)]
#[graphql(complex)]
pub struct ControllerObject {
    pub id: ID,
    pub created_by_id: ID,
    pub updated_by_id: ID,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub function: Option<String>,
}

#[ComplexObject]
impl ControllerObject {
    async fn created_by(&self, ctx: &Context<'_>) -> Result<Option<User>, Error> {
        find_user_details(ctx, &self.created_by_id)
    }
    async fn created_by2(&self, ctx: &Context<'_>) -> Result<User> {
        let loader = ctx
            .data_unchecked::<DataLoader<UserLoader>>()
            /* .expect("Can't get data loader") */;
        let user_id = self
            .created_by_id
            .to_string()
            .parse::<Uuid>()
            .expect("Can't convert id");
        let user = loader.load_one(user_id).await?;
        user.ok_or_else(|| "Not found".into())
    }
}

#[Object]
impl ControllerQuery {
    pub async fn get_controller_details(
        &self,
        ctx: &Context<'_>,
        controller_id: ID,
    ) -> Option<ControllerObject> {
        let cache_key = get_controller_cache_key(controller_id.to_string().as_str());
        let mut redis_connection_manager = get_redis_conn_manager(ctx).await;
        let cached_post: Value = redis_connection_manager
            .get(cache_key.clone())
            .await
            .expect("");
        //  Check if the data in cache exists, if none, retrieve the data from the database
        //  Chain multiple commands and query it to the connection manager
        match cached_post {
            Value::Nil => {
                let post = get_controller_detail(ctx, controller_id);
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

    #[graphql(name = "getAllControllers")]
    async fn get_controller(&self, ctx: &Context<'_>) -> Vec<ControllerObject> {
        let conn = get_conn_from_ctx(ctx);
        provider::get_all(&conn)
            .expect("Cannot get Blog ControllerObject ")
            .iter()
            .map(ControllerObject::from)
            .collect()
    }
    #[graphql(name = "getControllerById")]
    pub async fn get_controller_by_id(
        &self,
        ctx: &Context<'_>,
        controller_id: ID,
    ) -> Option<ControllerObject> {
        let cache_key = get_controller_cache_key(controller_id.to_string().as_str());
        println!("cache_key: {:?}", cache_key);
        let redis_client = get_redis_conn_from_ctx(ctx).await;

        println!("redis_client: {:?}", redis_client);
        
        let mut redis_connection = create_connection(redis_client)
        .await
        .expect("Unable to create Redis DB Connection");
        let cached_object = redis_connection.get(cache_key.clone()).await.expect("");
        println!("cached_object: {:?}", cached_object);

        //  Check If Cache Object is available
        match cached_object {
            Value::Nil => {
                let post = get_controller_detail(ctx, controller_id);

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
    #[graphql(name = "getControllersbyAuthor")]
    async fn get_controller_by_authorid(
        &self,
        ctx: &Context<'_>,
        user_id: ID,
    ) -> Vec<ControllerObject> {
        get_controllers_user(ctx, user_id)
    }
}

pub fn get_controller_detail(ctx: &Context<'_>, controller_id: ID) -> Option<ControllerObject> {
    provider::get_controller_by_id(parse_id(controller_id), &get_conn_from_ctx(ctx))
        .ok()
        .map(|f| ControllerObject::from(&f))
}
/// Gets the Post under the author: UserId
pub fn get_controllers_user(ctx: &Context<'_>, user_id: ID) -> Vec<ControllerObject> {
    provider::get_by_controllers_by_author(parse_id(user_id), &get_conn_from_ctx(ctx))
        .expect("Cannot get any User Posts")
        .iter()
        .map(|s| ControllerObject::from(s))
        .collect()
}

#[derive(Default)]
pub struct ControllerMutation;

#[derive(InputObject)]
pub struct ControllerInput {
    pub created_by_id: ID,
    pub updated_by_id: ID,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub function: Option<String>,
}

#[derive(InputObject)]
pub struct ControllerUpdateInput {
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub function: Option<String>,
}

#[Object]
impl ControllerMutation {
    /// Create A New Controller
    /// The server responds by caching the new Controller with Default
    #[graphql(name = "createController")]
    async fn create_controller(
        &self,
        ctx: &Context<'_>,
        form: ControllerInput,
    ) -> Result<ControllerObject, Error> {
        let post =
            provider::create_controller(ControllerForm::from(&form), &get_conn_from_ctx(ctx))?;
        let serialized_controller = serde_json::to_string(&ControllerObject::from(&post))
            .map_err(|_| ServiceError::InternalError)?;

        //  In the mutation, post creation a messgage is sent to the kafka.
        let producer = ctx
            .data::<FutureProducer>()
            .expect("Cannot get Kafka Producer");
        kafka::send_message(producer, serialized_controller).await;
        Ok(ControllerObject::from(&post))
    }
    #[graphql(name = "updateController")]
    async fn update_controller(
        &self,
        ctx: &Context<'_>,
        form: ControllerInput,
        controller_id: ID,
        user_id: ID,
    ) -> Result<ControllerObject, Error> {
        //  Convert the grahql input into readable database input
        let new_post = provider::update_controller(
            parse_id(controller_id.clone()),
            parse_id(user_id),
            ControllerForm::from(&form),
            &get_conn_from_ctx(ctx),
        )
        .expect("");
        //  Delete the cache under this value
        let cache_key = get_controller_cache_key(controller_id.to_string().as_str());
        let redis_connection_manager = get_redis_conn_manager(ctx);
        redis_connection_manager.await.del(cache_key).await?;

        //  Convert Controller (from the database), into Graphql object
        Ok(ControllerObject::from(&new_post))
    }
    #[graphql(name = "deleteController")]
    async fn delete_controller(
        &self,
        ctx: &Context<'_>,
        controller_author: ID,
        controller_id: ID,
    ) -> Result<bool, Error> {
        let conn = get_conn_from_ctx(ctx);
        provider::delete_controller(
            parse_id(controller_author),
            parse_id(controller_id.clone()),
            &conn,
        )
        .expect("Couldn't delete Controller");

        //  Deletes the cache under this postid
        let cache_key = get_controller_cache_key(controller_id.to_string().as_str());
        get_redis_conn_manager(ctx).await.del(cache_key).await?;

        Ok(true)
    }

    #[graphql(name = "updateController2")]
    async fn update_controller2(
        &self,
        ctx: &Context<'_>,
        form: ControllerUpdateInput,
        controller_id: ID,
    ) -> Result<ControllerObject, Error> {
        println!("controller_id: {:?}, form: {:?}", controller_id, form.manufacturer);
        //  Convert the grahql input into readable database input
        let new_post = provider::update_controller2(
            parse_id(controller_id.clone()),
            ControllerUpdateForm::from(&form),
            &get_conn_from_ctx(ctx),
        )
        .expect("");

        println!("new_post: {:?}", &new_post);
        //  Delete the cache under this value
        let cache_key = get_controller_cache_key(controller_id.to_string().as_str());
        println!("cache_key: {:?}", &cache_key);
        let redis_connection_manager = get_redis_conn_manager(ctx);
        redis_connection_manager.await.del(cache_key).await?;

        //  Convert Controller (from the database), into Graphql object
        Ok(ControllerObject::from(&new_post))
    }
}

//  Get the latest Posts
//  Subscriptions
use crate::utils::kafka::{create_consumer, get_kafka_consumer_id};
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
            .data::<Mutex<i32>>()
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
