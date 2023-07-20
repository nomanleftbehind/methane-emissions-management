use crate::graphql::models::IdSelection;
use common::IdSelectionVariant::{
    self, DeviceManufacturerId, PneumaticInstrumentType as PneumaticInstrumentTypeVariant, SiteId,
    UserId,
};
use common::PneumaticInstrumentType;
use sqlx::{query_as, Error, PgPool};
use strum::IntoEnumIterator;

pub async fn id_selection(
    pool: &PgPool,
    variant: IdSelectionVariant,
) -> Result<Vec<IdSelection>, Error> {
    println!("calling id selection: {:?}", &variant);

    match variant {
        SiteId => {
            query_as!(
                IdSelection,
                r#"SELECT id::text as "id!", name FROM site ORDER BY name"#
            ).fetch_all(pool).await
        }
        DeviceManufacturerId => {
            query_as!(
                IdSelection,
                r#"SELECT id::text as "id!", manufacturer as name FROM device_manufacturer ORDER BY manufacturer"#
            ).fetch_all(pool).await
        }
        PneumaticInstrumentTypeVariant => {
            Ok(PneumaticInstrumentType::iter()
                .map(|pit| IdSelection {
                    id: pit.to_string(),
                    name: pit.to_string(),
                })
                .collect())
        }
        UserId => {
            return Err(Error::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Wrong input provided"),
            )))
        }
    }
}
