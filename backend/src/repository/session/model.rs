use async_graphql::{Context, Error, Object, SimpleObject, ID};
use chrono::NaiveDateTime;
use diesel::{PgConnection, QueryResult, RunQueryDsl};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::graphql::context::get_conn_from_ctx;
use crate::repository::schema::active_sessions;

#[derive(Queryable, Debug, Serialize, Deserialize, PartialEq, Clone, Identifiable)]
#[primary_key(session_user_id, token_hash)]
#[table_name = "active_sessions"]
pub struct ActiveSessionDb {
    pub session_user_id: Uuid,
    pub token_hash: Vec<u8>,
    pub created_at: NaiveDateTime,
    pub expires_at: Option<NaiveDateTime>,
}

impl From<&ActiveSessionDb> for ActiveSession {
    fn from(oop: &ActiveSessionDb) -> Self {
        Self {
            session_user_id: oop.session_user_id.into(),
            token_hash: oop.token_hash.clone(),
            created_at: oop.created_at.clone(),
            expires_at: oop.expires_at.clone(),
        }
    }
}

#[derive(SimpleObject, Serialize, Deserialize, Clone, Debug)]
pub struct ActiveSession {
    pub session_user_id: ID,
    pub token_hash: Vec<u8>,
    pub created_at: NaiveDateTime,
    pub expires_at: Option<NaiveDateTime>,
}

#[derive(Default)]
pub struct ActiveSessionQuery;

#[Object]
impl ActiveSessionQuery {
    #[graphql(name = "allActiveSessions")]
    pub async fn all_active_sessions(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<ActiveSession>, Error> {
        let conn = get_conn_from_ctx(ctx);

        let active_sessions = get_all_active_sessions(&conn)
            .expect("Cannot get Active Sessions")
            .iter()
            .map(ActiveSession::from)
            .collect();
        Ok(active_sessions)
    }
}

pub fn get_all_active_sessions(conn: &PgConnection) -> QueryResult<Vec<ActiveSessionDb>> {
    use crate::repository::schema::active_sessions::dsl::*;
    active_sessions.load(conn)
}
