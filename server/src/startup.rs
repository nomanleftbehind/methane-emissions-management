use crate::{
    authentication::{utils::SessionCookieNameSecret, SessionManager},
    configuration::{DatabaseSettings, DefaultGasParams, FdcDatabaseSettings, Settings},
    graphql::{
        dataloaders::{get_loaders, LoaderRegistry},
        mutations::full_mutation,
        queries::full_query,
    },
    routes::{graphql, graphql_playground},
    // ssr_render::ssr_render,
    MssqlFdcClient,
};
use actix_cors::Cors;
use actix_web::{cookie::Key, dev::Server, middleware::Logger, web, App, HttpServer};
use actix_web_flash_messages::{storage::CookieMessageStore, FlashMessagesFramework};
use async_graphql::{EmptySubscription, Schema};
use async_redis_session::RedisSessionStore;
// use clap::Parser;
use secrecy::{ExposeSecret, Secret};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{net::TcpListener, /*path::PathBuf,*/ sync::Arc};
use tiberius::Client;
use tokio::{net::TcpStream, sync::Mutex};
use tokio_util::compat::TokioAsyncWriteCompatExt;
// use tracing::log::LevelFilter;

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        let connection_pool = get_connection_pool(&configuration.database);
        let mssql_fdc_client = get_mssql_fdc_client(&configuration.fdc_database).await;

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(&address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(
            listener,
            connection_pool,
            mssql_fdc_client,
            configuration.application.base_url,
            configuration.application.hmac_secret,
            configuration.application.session_cookie_name,
            configuration.redis_uri,
            configuration.default_gas_params,
        )
        .await?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.with_db())
}

/// When getting Microsoft SQL Server client for FDC (Field Data Capture) database, Result is converted to Option because we don't want program to exit in case connection is denied.
///
/// FDC is a third party database which makes it susceptible to uncontrolled outages or denials of service.
///
/// Application is still 99% usable even if connection to FDC is not able to be established.
pub async fn get_mssql_fdc_client(configuration: &FdcDatabaseSettings) -> Option<MssqlFdcClient> {
    let config = configuration.create();
    let tcp = TcpStream::connect(config.get_addr()).await.ok()?;
    tcp.set_nodelay(true).ok()?;
    let client = Client::connect(config, tcp.compat_write()).await.ok();

    client
}

pub struct ApplicationBaseUrl(pub String);
#[derive(Clone)]
pub struct HmacSecret(pub Secret<String>);

// #[derive(Parser, Debug)]
// struct Opt {
//     /// the "dist" created by trunk directory to be served for hydration.
//     #[clap(short, long)]
//     dir: PathBuf,
// }

pub async fn run(
    listener: TcpListener,
    db_pool: PgPool,
    mssql_fdc_client: Option<MssqlFdcClient>,
    base_url: String,
    hmac_secret: Secret<String>,
    session_cookie_name_secret: Secret<String>,
    redis_uri: Secret<String>,
    default_gas_params: DefaultGasParams,
) -> Result<Server, anyhow::Error> {
    // env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    env_logger::init();

    let db_pool = web::Data::new(db_pool);
    let loaders = get_loaders(db_pool.clone()).await;
    let loader_registry_data = web::Data::new(LoaderRegistry { loaders });

    let base_url = web::Data::new(ApplicationBaseUrl(base_url));
    let secret_key = Key::from(hmac_secret.expose_secret().as_bytes());
    let message_store = CookieMessageStore::builder(secret_key.clone()).build();
    let message_framework = FlashMessagesFramework::builder(message_store).build();
    let redis_store = RedisSessionStore::new(redis_uri.expose_secret().as_str())
        .expect("Failed to connect to Redis");
    let session_manager = SessionManager::new(redis_store);
    let session_manager_atomic = web::Data::new(session_manager);

    let session_cookie_name_secret = SessionCookieNameSecret::new(session_cookie_name_secret);
    let session_cookie_name_secret_atomic = web::Data::new(session_cookie_name_secret);

    let schema_builder = Schema::build(full_query(), full_mutation(), EmptySubscription)
        .extension(async_graphql::extensions::Tracing)
        .limit_complexity(1024)
        .data(loader_registry_data)
        .data(db_pool.clone())
        .data(base_url.clone())
        .data(web::Data::new(default_gas_params.clone()))
        .data(web::Data::new(HmacSecret(hmac_secret.clone())))
        .data(session_cookie_name_secret_atomic.clone())
        .data(session_manager_atomic.clone());

    // Append FDC client to schema data in case connection was established, otherwise just finish building the schema without adding any additional data.
    let schema = if let Some(fc) = mssql_fdc_client {
        let atomic_fc = Arc::new(Mutex::new(fc));
        schema_builder.data(atomic_fc).finish()
    } else {
        schema_builder.finish()
    };

    log::info!("starting HTTP server on port 8081");
    log::info!("GraphiQL playground: http://localhost:8081/graphiql");

    // let opts = Opt::parse();
    // let dir_data = web::Data::new(opts.dir.clone());

    let server = HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(message_framework.clone())
            .app_data(web::Data::new(schema.clone()))
            // .app_data(dir_data.clone())
            .app_data(session_cookie_name_secret_atomic.clone())
            .app_data(session_manager_atomic.clone())
            .service(graphql)
            .service(graphql_playground)
            // .service(web::resource("/").route(web::get().to(ssr_render)))
            // .service(web::resource("/users").route(web::get().to(ssr_render)))
            // .service(web::resource("/register").route(web::get().to(ssr_render)))
            // .service(actix_files::Files::new("/", opts.dir.clone()))
            .wrap(cors)
            .wrap(Logger::default())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
