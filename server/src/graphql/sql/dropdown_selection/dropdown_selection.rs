use crate::graphql::models::dropdown_selection::DropdownSelection;
use crate::graphql::models::input::GetDropdownSelectionInput;
use common::{
    DropdownSelectionVariant::{
        DeviceManufacturerId, PneumaticInstrumentId,
        PneumaticInstrumentType as PneumaticInstrumentTypeVariant, SiteId, UserId,
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
                        r#"SELECT id::text as "id!", name FROM site WHERE facility_id = $1 ORDER BY name"#,
                        id
                    ).fetch_all(pool).await
                } else {
                query_as!(
                    DropdownSelection,
                    r#"SELECT id::text as "id!", name FROM site ORDER BY name"#
                ).fetch_all(pool).await
                }
        }
        DeviceManufacturerId => {
            query_as!(
                DropdownSelection,
                r#"SELECT id::text as "id!", manufacturer as name FROM device_manufacturer ORDER BY manufacturer"#
            ).fetch_all(pool).await
        }
        PneumaticInstrumentTypeVariant => {
            Ok(PneumaticInstrumentType::iter()
                .map(|pit| {
                    let name = pit.to_string();
                    DropdownSelection {
                    id: name.clone(),
                    name,
                }})
                .collect())
        }
        PneumaticInstrumentId => {
            if let Some(id) = id {
            query_as!(
                DropdownSelection,
                r#"SELECT pi.id::text as "id!",

                dm.manufacturer || ' ' || LOWER(pi.type::text) as "name!"
                
                FROM pneumatic_instrument pi
                LEFT OUTER JOIN device_manufacturer dm ON dm.id = pi.manufacturer_id

                WHERE site_id = $1
                
                ORDER BY dm.manufacturer, pi.type"#,
                id
            ).fetch_all(pool).await
        } else {
            query_as!(
                DropdownSelection,
                r#"SELECT pi.id::text as "id!",

                dm.manufacturer || ' ' || LOWER(pi.type::text) as "name!"
                
                FROM pneumatic_instrument pi
                LEFT OUTER JOIN device_manufacturer dm ON dm.id = pi.manufacturer_id
                
                ORDER BY dm.manufacturer, pi.type"#
                ).fetch_all(pool).await
            }
        }
        UserId => {
            return Err(Error::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Wrong input provided"),
            )))
        }
    }
}
