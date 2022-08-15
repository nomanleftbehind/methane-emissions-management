use async_graphql::Context;
use cookie::{time::OffsetDateTime, Cookie};
use diesel::{result::Error, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tracing::debug;
use uuid::Uuid;

use crate::graphql::context::SessionCookieName;
use crate::repository::{schema::active_sessions, session::model::ActiveSessionDb};

#[derive(Clone, Debug)]
pub struct SessionCookie<'a>(pub Option<Cookie<'a>>);

pub struct UserCredential(Option<SessionData>);

impl UserCredential {
    pub fn new(session: Option<SessionData>) -> Self {
        Self(session)
    }
    pub fn is_anonymous(&self) -> bool {
        self.0.is_none()
    }
    pub fn user_id(&self) -> Option<Uuid> {
        Some(self.0.as_ref()?.user_id)
    }
    pub fn session(&self) -> Option<&SessionData> {
        self.0.as_ref()
    }
}

#[derive(Serialize, Deserialize)]
pub struct SessionData {
    pub user_id: Uuid,
    pub secret: String,
}

impl<'a> TryFrom<&Cookie<'a>> for SessionData {
    type Error = String;

    fn try_from(cookie: &Cookie) -> Result<Self, Self::Error> {
        let val = cookie.value();
        serde_json::from_str(val).map_err(|_| "invalid session cookie".to_owned())
    }
}

/// Try to get verified session data from a session cookie
pub async fn try_get_verified_session_data<'a>(
    conn: &PgConnection,
    session_cookie: &SessionCookie<'_>,
) -> Option<SessionData> {
    let cookie = session_cookie.0.as_ref()?;
    let session = SessionData::try_from(cookie).ok()?;
    if verify_session(conn, &session).await {
        debug!("Valid session for user_id: {}", session.user_id);
        Some(session)
    } else {
        debug!("Invalid session for user_id: {}", session.user_id);
        None
    }
}

async fn verify_session(conn: &PgConnection, session: &SessionData) -> bool {
    let hash = &Sha256::digest(session.secret.as_bytes())[..];
    active_sessions::table
        .filter(active_sessions::session_user_id.eq(session.user_id))
        .filter(active_sessions::token_hash.eq(hash))
        .first::<ActiveSessionDb>(conn)
        .ok()
        .is_some()
}

pub async fn insert_session(conn: &PgConnection, session: SessionData) -> Result<(), Error> {
    debug!("NEW SESSION: {:?}", session.secret);
    let hash = &Sha256::digest(session.secret.as_bytes())[..];

    diesel::insert_into(active_sessions::table)
        .values((
            active_sessions::session_user_id.eq(session.user_id),
            active_sessions::token_hash.eq(hash),
        ))
        .execute(conn)?;
    Ok(())
}

pub async fn delete_session(
    conn: &PgConnection,
    user_id: Uuid,
    secret: Secret<String>,
) -> Result<(), Error> {
    let hash = &Sha256::digest(secret.expose_secret().as_bytes())[..];

    diesel::delete(
        active_sessions::table
            .filter(active_sessions::session_user_id.eq(user_id))
            .filter(active_sessions::token_hash.eq(hash)),
    )
    .execute(conn)?;
    Ok(())
}

pub async fn invalidate_session(
    ctx: &Context<'_>,
    pool: &PgConnection,
    cred: &UserCredential,
) -> Result<(), async_graphql::Error> {
    if let Some(session) = cred.session() {
        let session_cookie_name = ctx.data::<SessionCookieName>().unwrap();
        // Remove the cookie
        let cookie = Cookie::build(session_cookie_name.0.clone(), "")
            .http_only(true)
            .secure(true)
            .same_site(cookie::SameSite::Strict)
            .expires(OffsetDateTime::now_utc())
            .finish();
        ctx.append_http_header("Set-Cookie", cookie.to_string());
        delete_session(pool, session.user_id, Secret::new(session.secret.clone())).await?;
        Ok(())
    } else {
        Err(async_graphql::Error::new("Already logged out"))
    }
}
