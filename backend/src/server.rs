use crate::graphql::context::{configure_service, create_schema, run_migrations};
use crate::repository::db::{establish_connection, DatabaseKind};
use crate::utils::{
    rate_limiter::RateLimiter,
    redis::{create_client, RedisDatabase},
};
use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};

pub async fn new_server(port: u32) -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let db_pool = establish_connection(DatabaseKind::Example);
    run_migrations(&db_pool);

    //  Create a Redis Client `
    let redis_client = create_client(RedisDatabase::Example)
        .await
        .expect("Unable to create Redis Client Connection");

    //  Redis Config
    let redis_connection_manager = redis_client
        .get_tokio_connection_manager()
        .await
        .expect("Cannot Create Redis Connection Manager");

    //  GraphQl Schema
    let schema = web::Data::new(
        create_schema(
            db_pool,
            redis_client.clone(),
            redis_connection_manager.clone(),
        )
        .await,
    );

    //  In Memory API Limiter
    let redis_api_limiter = web::Data::new(RateLimiter::new(redis_connection_manager));

    // start_pubsub(&redis_client)
    //     .await
    //     .expect("Unable to start Redis Pub/ Sub");

    log::info!("{}", &schema.sdl());
    log::info!("🚀 Starting HTTP server on port {} ", port);
    log::info!("📭 GraphiQL playground: http://localhost:{}/graphiql", port);
    log::info!("📢 Query at https://studio.apollographql.com/dev");

    HttpServer::new(move || {
        App::new()
            .app_data(redis_api_limiter.clone())
            .app_data(schema.clone())
            .configure(configure_service)
            .wrap(Cors::permissive())
            .wrap(Logger::default())
    })
    .workers(2)
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}
