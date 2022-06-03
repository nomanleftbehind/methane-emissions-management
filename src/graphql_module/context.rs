use super::models::controller_model::resolver::Subscription;
use super::models::user_model::provider;
use super::schema::{AppSchema, AppSchemaBuilder, Mutation, Query};
use crate::graphql_module::loader::user::UserLoader;
use async_graphql::dataloader::DataLoader;
// use super::schema::{
//     AppSchema, AppSchemaBuilder, Mutation as SchemaMutation, Query as SchemaQuery,
// };
use crate::db::{DbPool, DbPooledConnection};
use crate::graphql_module::common_utils::token::get_role;
use crate::graphql_module::utils::{error::ServiceError, kafka::create_producer};
use actix_cors::Cors;
use actix_web::{
    get, guard, middleware::Logger, route, web, App, Error, HttpRequest, HttpResponse, HttpServer,
    Responder,
};
use actix_web_lab::respond::Html;
use async_graphql::{
    extensions::ApolloTracing,
    http::{playground_source, GraphQLPlaygroundConfig},
    Context, EmptyMutation, EmptySubscription, Schema,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use diesel::{result::Error as DbError, QueryDsl};
use diesel_migrations::{embed_migrations, MigrationError};
use redis::{
    aio::Connection as RedisConnection, aio::ConnectionManager as RedisManager,
    Client as RedisClient,
};
use std::env::var;
use std::sync::{Arc, Mutex};

pub fn configure_service(cfg: &mut web::ServiceConfig) {
    cfg.service(graphql).service(graphql_playground).service(
        web::resource("/graphiql").route(
            web::get()
                .guard(guard::Header("upgrade", "websocket"))
                .to(index_ws),
        ),
    );
}

/// GraphQL endpoint
#[route("/graphql", method = "GET", method = "POST")]
pub async fn graphql(
    schema: web::Data<AppSchema>,
    req: GraphQLRequest,
    http: HttpRequest,
) -> GraphQLResponse {
    let (role, mut request) = (get_role(http), req.into_inner());
    if let Some(user) = role {
        request = request.data(user);
    }
    schema.execute(request).await.into()
}
/// GraphiQL playground UI
#[get("/graphiql")]
pub async fn graphql_playground() -> impl Responder {
    Html(playground_source(
        GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql"),
    ))
}

pub async fn index_ws(
    schema: web::Data<AppSchema>,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse, Error> {
    GraphQLSubscription::new(Schema::clone(&*schema)).start(&req, payload)
}

embed_migrations!();

// pub fn create_schema(pool: DbPool) -> AppSchema {
//     Schema::build(
//         SchemaQuery::default(),
//         SchemaMutation::default(),
//         EmptySubscription,
//     )
//     .enable_federation()
//     // Add a global data that can be accessed in the Schema
//     .data(pool)
//     .extension(ApolloTracing)
//     .finish()
// }
pub fn create_schema(
    pool: DbPool,
    redis_pool: RedisClient,
    redis_connection: RedisManager,
) -> AppSchema {
    //  SQL Database
    // let arc_pool = Arc::new(pool);
    //  Kafka Queue
    let kafka_consumer = Mutex::new(0);
    // Caching Service
    let arc_redis_connection = Arc::new(redis_connection);

    let user_data_loader = DataLoader::new(UserLoader { pool: pool.clone() }, async_std::task::spawn,);

    Schema::build(Query::default(), Mutation::default(), Subscription)
        .enable_federation()
        .data(arc_redis_connection)
        // Add a global data that can be accessed in the Schema
        //  Redis Caching Client
        .data(redis_pool)
        //  SQL Database Pool
        // changed from arc_pool to pool because was getting
        // Error { message: "Data `r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::pg::connection::PgConnection>>` does not exist.", extensions: None }'
        .data(pool)
        .data(user_data_loader)
        //  Kafka Queue
        .data(create_producer())
        .data(kafka_consumer)
        //  Apollo Tracing
        .extension(ApolloTracing)
        //  Build Schema
        .finish()
}
pub fn run_migrations(pool: &DbPool) {
    let conn = pool
        .get()
        .expect("Database Connection Pool - Migrations error!");
    embedded_migrations::run(&conn).expect("Failed to run database migrations");

    // This step makes absolutely no sense
    // if let Ok(hash) = var("PASSWORD_SECRET_KEY") {
    //     provider::update_password(hash, &conn);
    // };
}
pub fn get_conn_from_ctx(ctx: &Context<'_>) -> DbPooledConnection {
    ctx.data::<DbPool>()
        .expect("Failed to get Db Pool")
        .get()
        .expect("Failed to Connect to Database")
}

/// Access Redis from the Context, use 'create_connection' to establish connection asynchronously
pub async fn get_redis_conn_from_ctx(ctx: &Context<'_>) -> RedisClient {
    ctx.data::<RedisClient>()
        .expect("Failed to get Redis Client")
        .clone()
}
/// Access Redis Database Connection
pub async fn get_redis_conn_manager(ctx: &Context<'_>) -> RedisManager {
    ctx.data::<RedisManager>()
        .expect("Failed to get Redis Connection Manager")
        .clone()
}
