use crate::graphql::models::dropdown_selection::DropdownSelection;
use crate::graphql::models::input::GetDropdownSelectionInput;
use common::{
    DropdownSelectionVariant::{
        DeviceManufacturerId, PneumaticInstrumentType as PneumaticInstrumentTypeVariant, SiteId,
        UserId,
    },
    PneumaticInstrumentType,
};
use sqlx::{query_as, Error, PgPool};
use strum::IntoEnumIterator;

pub async fn get_dropdown_selection(
    pool: &PgPool,
    GetDropdownSelectionInput { variant, id }: GetDropdownSelectionInput,
) -> Result<Vec<DropdownSelection>, Error> {
    match variant {
        SiteId => {
            if let Some(id) = id {
                query_as!(
                        DropdownSelection,
                        r#"SELECT id::text as "id!", name as "name?" FROM site WHERE facility_id = $1 ORDER BY name"#,
                        id
                    ).fetch_all(pool).await
                } else {
                query_as!(
                    DropdownSelection,
                    r#"SELECT id::text as "id!", name as "name?" FROM site ORDER BY name"#
                ).fetch_all(pool).await
                }
        }
        DeviceManufacturerId => {
            query_as!(
                DropdownSelection,
                r#"SELECT id::text as "id!", manufacturer as "name?" FROM device_manufacturer ORDER BY manufacturer"#
            ).fetch_all(pool).await
        }
        PneumaticInstrumentTypeVariant => {
            Ok(PneumaticInstrumentType::iter()
                .map(|pit| DropdownSelection {
                    id: pit.to_string(),
                    name: None,
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
