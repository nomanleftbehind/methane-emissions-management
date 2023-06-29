use crate::graphql::models::IdSelection;
use common::IdSelectionVariant::{self, ControllerApplicationId, DeviceManufacturerId};
use sqlx::{query_as, Error, PgPool};

pub async fn id_selection(
    pool: &PgPool,
    variant: IdSelectionVariant,
) -> Result<Vec<IdSelection>, Error> {
    println!("calling id selection: {:?}", &variant);
    match variant {
        DeviceManufacturerId => {
            query_as!(
                IdSelection,
                "SELECT id, manufacturer as name FROM device_manufacturer ORDER BY manufacturer"
            )
            .fetch_all(pool)
            .await
        }
        ControllerApplicationId => {
            return Err(Error::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Wrong input provided"),
            )))
        }
    }
}
