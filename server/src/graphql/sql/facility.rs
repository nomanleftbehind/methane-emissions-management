use crate::graphql::models::{Facility, FacilityBy, FacilityType, LimitOffsetInput};
use sqlx::{PgExecutor, PgPool};

pub async fn query_facilities(
    pool: &PgPool,
    facility_by: FacilityBy,
    limit: i64,
    offset: i64,
) -> Result<Vec<Facility>, sqlx::Error> {
    let facilities = match facility_by {
        FacilityBy::Name(name) => {
            sqlx::query_as!(
                Facility,
                r#"
                SELECT
                id, idpa, name, type as "type: _", created_by_id, created_at, updated_by_id, updated_at
                FROM facilities
                WHERE name = $1
                LIMIT $2
                OFFSET $3
                "#,
                name, limit, offset
            )
            .fetch_all(pool)
            .await
        }

        FacilityBy::Type(t) => {
            sqlx::query_as!(
                Facility,
                r#"
                SELECT
                id, idpa, name, type as "type: _", created_by_id, created_at, updated_by_id, updated_at
                FROM facilities
                WHERE type = $1
                LIMIT $2
                OFFSET $3
                "#,
                t as FacilityType, limit, offset
            )
            .fetch_all(pool)
            .await
        }
    };

    facilities
}

pub async fn query_all_facilities<'e, E: PgExecutor<'e>>(
    executor: E,
    limit_offset_input: Option<LimitOffsetInput>,
) -> Result<Vec<Facility>, sqlx::Error> {
    if let Some(LimitOffsetInput { limit, offset }) = limit_offset_input {
        sqlx::query_as!(
            Facility,
            r#"
            SELECT
            id, idpa, name, type as "type: _", created_by_id, created_at, updated_by_id, updated_at
            FROM facilities
            LIMIT $1
            OFFSET $2
            "#,
            limit,
            offset
        )
        .fetch_all(executor)
        .await
    } else {
        sqlx::query_as!(
            Facility,
            r#"
            SELECT
            id, idpa, name, type as "type: _", created_by_id, created_at, updated_by_id, updated_at
            FROM facilities
            WHERE id IN (SELECT facility_id FROM controllers)
            "#,
        )
        .fetch_all(executor)
        .await
    }
}
