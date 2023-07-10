use crate::graphql::models::IdSelection;
use common::IdSelectionVariant::{self, DeviceManufacturerId, UserId};
use sqlx::{query_as, Error, PgPool};

pub async fn id_selection(
    pool: &PgPool,
    variant: IdSelectionVariant,
) -> Result<Vec<IdSelection>, Error> {
    println!("calling id selection: {:?}", &variant);
    let query = match variant {
        DeviceManufacturerId => {
            query_as!(
                IdSelection,
                "SELECT id, manufacturer as name FROM device_manufacturer ORDER BY manufacturer"
            )
        }
        UserId => {
            return Err(Error::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Wrong input provided"),
            )))
        }
    };

    Ok(query.fetch_all(pool).await?)
}
