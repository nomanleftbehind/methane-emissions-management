use crate::{
    authentication::utils::SessionCookieNameSecret, graphql::context::ContextExt,
    utils::SessionTokenExtractorError,
};
use actix_web::{
    dev::Payload,
    web::{self, Data},
    FromRequest, HttpRequest,
};
use async_graphql::{Context, Error};
use async_redis_session::RedisSessionStore;
use async_session::{Session, SessionStore};
use http::header::SET_COOKIE;
use std::future::{ready, Ready};

#[derive(Debug)]
pub struct SessionCookie(pub String);

impl FromRequest for SessionCookie {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let session_cookie = {
            req.app_data::<web::Data<SessionCookieNameSecret>>()
                .map_or_else(
                    move || {
                        let e = SessionTokenExtractorError::NoSessionToken;
                        ready(Err(e.into()))
                    },
                    |session_cookie_name_secret| {
                        req.cookie(session_cookie_name_secret.get_session_cookie_name())
                            .map_or_else(
                                move || {
                                    let e = SessionTokenExtractorError::NoSessionToken;
                                    ready(Err(e.into()))
                                },
                                |cookie| ready(Ok(Self(cookie.value().to_string()))),
                            )
                    },
                )
        };

        session_cookie
    }
}

impl SessionCookie {
    /// Uses GraphQL Context to set session cookie on the browser.
    pub async fn set_cookie(&self, ctx: &Context<'_>) -> Result<(), Error> {
        let session_cookie_name = ctx
            .get_session_cookie_name_secret()?
            .get_session_cookie_name();

        ctx.append_http_header(
            SET_COOKIE,
            format!(
                "{}={}; SameSite=None; Secure; Max-Age=3600; Path=/",
                session_cookie_name, self.0
            ),
        );

        Ok(())
    }

    /// Load actual session from Redis/Session Store.
    pub async fn load_session(
        &self,
        session_store: Data<RedisSessionStore>,
    ) -> Result<Session, Error> {
        session_store
            .load_session(self.0.clone())
            .await
            .map_err(|e| Error::new(e.to_string()))?
            .ok_or_else(|| Error::new("Session present but not found on Redis"))
    }
}
