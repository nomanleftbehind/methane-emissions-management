use crate::graphql::models::{
    Compressor, Controller, ControllerChange, ControllerMonthHours, GetObject, GetObjectInput,
    TankFarm,
};
use common::GetObjectVariant::{
    CompressorByFacilityId, ControllerByFacilityId, ControllerChangeByControllerId,
    ControllerMonthHoursByControllerId, TankFarmByFacilityId,
};
use sqlx::{query_as, Error, PgPool};

pub async fn get_object(
    pool: &PgPool,
    GetObjectInput {
        id,
        get_object_variant,
    }: GetObjectInput,
) -> Result<GetObject, Error> {
    let object = match get_object_variant {
        ControllerByFacilityId => GetObject {
            controllers: Some(
                query_as!(
                    Controller,
                    "SELECT * FROM controllers WHERE facility_id = $1 ORDER BY id",
                    id
                )
                .fetch_all(pool)
                .await?,
            ),
            compressors: None,
            tank_farms: None,
            controller_changes: None,
            controller_month_hours: None,
        },
        CompressorByFacilityId => GetObject {
            controllers: None,
            compressors: Some(
                query_as!(
                    Compressor,
                    "SELECT * FROM compressors WHERE facility_id = $1 ORDER BY id",
                    id
                )
                .fetch_all(pool)
                .await?,
            ),
            tank_farms: None,
            controller_changes: None,
            controller_month_hours: None,
        },
        TankFarmByFacilityId => GetObject {
            compressors: None,
            controllers: None,
            tank_farms: Some(
                query_as!(
                    TankFarm,
                    "SELECT * FROM tank_farms WHERE facility_id = $1 ORDER BY id",
                    id
                )
                .fetch_all(pool)
                .await?,
            ),
            controller_changes: None,
            controller_month_hours: None,
        },
        ControllerChangeByControllerId => GetObject {
            compressors: None,
            controllers: None,
            tank_farms: None,
            controller_changes: Some(
                query_as!(
                    ControllerChange,
                    "SELECT * FROM controller_changes WHERE controller_id = $1 ORDER BY id",
                    id
                )
                .fetch_all(pool)
                .await?,
            ),
            controller_month_hours: None,
        },
        ControllerMonthHoursByControllerId => GetObject {
            compressors: None,
            controllers: None,
            tank_farms: None,
            controller_changes: None,
            controller_month_hours: Some(
                query_as!(
                    ControllerMonthHours,
                    "SELECT * FROM controller_month_hours WHERE controller_id = $1 ORDER BY id",
                    id
                )
                .fetch_all(pool)
                .await?,
            ),
        },
    };

    Ok(object)
}
