use async_graphql::{Context, Error, Object};

use crate::graphql::{
    context::ContextExt,
    models::{EmittersByInput, TankFarm},
    sql::query_tank_farms,
};

#[derive(Default, Clone)]
pub(super) struct TankFarmQuery;

#[Object]
impl TankFarmQuery {
    async fn tank_farms_by(
        &self,
        ctx: &Context<'_>,
        by: EmittersByInput,
    ) -> Result<Vec<TankFarm>, Error> {
        let pool = ctx.db_pool();
        let tank_farms = query_tank_farms(pool, by).await.map_err(Error::from);

        tank_farms
    }
}
