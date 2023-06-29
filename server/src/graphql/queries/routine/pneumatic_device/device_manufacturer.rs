use crate::graphql::{
    context::ContextExt, models::routine::pneumatic_device::DeviceManufacturer,
    sql::routine::pneumatic_device,
};
use async_graphql::{Context, Error, Object};

#[derive(Default, Clone)]
pub struct DeviceManufacturerQuery;

#[Object]
impl DeviceManufacturerQuery {
    async fn get_device_manufacturers(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<DeviceManufacturer>, Error> {
        let pool = ctx.db_pool();

        let device_manufacturer_selection = pneumatic_device::get_device_manufacturers(pool)
            .await
            .map_err(Error::from);

        device_manufacturer_selection
    }
}
