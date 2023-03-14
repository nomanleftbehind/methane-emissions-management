use crate::authentication::{cookie::SessionCookie, SessionManager};
use crate::configuration::DefaultGasParams;
use crate::graphql::dataloaders::LoaderRegistry;
use crate::MssqlFdcClient;
use actix_web::web::Data;
use async_graphql::{Context, Error};
use async_redis_session::RedisSessionStore;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::Mutex;

// Sugar that helps make things neater and avoid errors that would only crop up at runtime.
pub trait ContextExt {
    fn get_loader<T: anymap::any::Any + Send + Sync>(&self) -> &T;
    fn db_pool(&self) -> &PgPool;
    fn mssql_fdc_client(&self) -> Result<&Arc<Mutex<MssqlFdcClient>>, Error>;
    fn get_cookie(&self) -> Result<&SessionCookie, Error>;
    fn get_session_manager(&self) -> Result<SessionManager, Error>;
    fn get_default_gas_params(&self) -> &DefaultGasParams;
}

impl<'a> ContextExt for Context<'a> {
    fn get_loader<T: anymap::any::Any + Send + Sync>(&self) -> &T {
        self.data_unchecked::<Data<LoaderRegistry>>().get::<T>()
    }

    fn db_pool(&self) -> &PgPool {
        self.data_unchecked::<Data<PgPool>>()
    }

    fn mssql_fdc_client(&self) -> Result<&Arc<Mutex<MssqlFdcClient>>, Error> {
        self.data::<Arc<Mutex<MssqlFdcClient>>>()
    }

    /// Gets the SessionCookie or errors if no cookie is found.
    fn get_cookie(&self) -> Result<&SessionCookie, Error> {
        let session_cookie = self
            .data::<Option<SessionCookie>>()
            .expect("Auth Cookie Option not found in Context");

        session_cookie
            .as_ref()
            .ok_or_else(|| Error::new("Not logged in"))
    }

    fn get_session_manager(&self) -> Result<SessionManager, Error> {
        let store = self
            .data::<Data<RedisSessionStore>>()
            .expect("Session store not found in Context");
        let authorizer = SessionManager::new(store);
        Ok(authorizer)
    }

    fn get_default_gas_params(&self) -> &DefaultGasParams {
        self.data_unchecked::<Data<DefaultGasParams>>()
    }
}
