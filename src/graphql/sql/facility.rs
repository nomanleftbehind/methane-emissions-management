use crate::graphql::domain::{Facility, FacilityBy, FacilityType};
use sqlx::PgPool;

pub async fn query_facilities(
    pool: &PgPool,
    by: FacilityBy,
    limit: i64,
    offset: i64,
) -> Result<Vec<Facility>, sqlx::Error> {
    let facilities = match by {
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
