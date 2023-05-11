use async_graphql::{Context, Error, Object};

use crate::graphql::{
    context::ContextExt,
    models::facility::{Facility, FacilityBy, LimitOffsetInput},
    sql::{query_all_facilities, query_facilities},
};

#[derive(Default, Clone)]
pub(super) struct FacilityQuery;

#[Object]
impl FacilityQuery {
    async fn facilities_by(
        &self,
        ctx: &Context<'_>,
        by: FacilityBy,
        #[graphql(default = 10)] limit: i64,
        #[graphql(default = 0)] offset: i64,
    ) -> Result<Vec<Facility>, Error> {
        let pool = ctx.db_pool();

        let facilities = query_facilities(pool, by, limit, offset)
            .await
            .map_err(Error::from);

        facilities
    }

    async fn all_facilities(
        &self,
        ctx: &Context<'_>,
        limit_offset_input: Option<LimitOffsetInput>,
    ) -> Result<Vec<Facility>, Error> {
        let pool = ctx.db_pool();
        let facilities = query_all_facilities(pool, limit_offset_input)
            .await
            .map_err(Error::from);
        facilities
    }
}
