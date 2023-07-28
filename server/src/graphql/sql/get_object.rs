use crate::graphql::models::{
    compressor::Compressor,
    pneumatic_device::{
        ControllerMonthHours, ControllerMonthVent, ControllerMonthVentOverride, NonLevelController,
        NonLevelControllerEmissionRate,
    },
    GetObject, GetObjectInput, TankFarm,
};
use common::GetObjectVariant::{
    CompressorByFacilityId, ControllerByFacilityId, ControllerEmissionRateByControllerId,
    ControllerMonthHoursByControllerId, ControllerMonthVentByControllerId,
    ControllerMonthVentOverrideByControllerId, TankFarmByFacilityId,
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
                    NonLevelController,
                    "SELECT * FROM controllers WHERE facility_id = $1 ORDER BY created_at DESC",
                    id
                )
                .fetch_all(pool)
                .await?,
            ),
            compressors: None,
            tank_farms: None,
            controller_emission_rates: None,
            controller_month_hours: None,
            controller_month_vent_override: None,
            controller_month_vent: None,
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
            controller_emission_rates: None,
            controller_month_hours: None,
            controller_month_vent_override: None,
            controller_month_vent: None,
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
            controller_emission_rates: None,
            controller_month_hours: None,
            controller_month_vent_override: None,
            controller_month_vent: None,
        },
        ControllerEmissionRateByControllerId => GetObject {
            compressors: None,
            controllers: None,
            tank_farms: None,
            controller_emission_rates: Some(
                query_as!(
                    NonLevelControllerEmissionRate,
                    "SELECT * FROM controller_emission_rates WHERE controller_id = $1 ORDER BY id",
                    id
                )
                .fetch_all(pool)
                .await?,
            ),
            controller_month_hours: None,
            controller_month_vent_override: None,
            controller_month_vent: None,
        },
        ControllerMonthHoursByControllerId => GetObject {
            compressors: None,
            controllers: None,
            tank_farms: None,
            controller_emission_rates: None,
            controller_month_hours: Some(
                query_as!(
                    ControllerMonthHours,
                    "SELECT * FROM controller_month_hours WHERE controller_id = $1 ORDER BY id",
                    id
                )
                .fetch_all(pool)
                .await?,
            ),
            controller_month_vent_override: None,
            controller_month_vent: None,
        },
        ControllerMonthVentByControllerId => GetObject {
            compressors: None,
            controllers: None,
            tank_farms: None,
            controller_emission_rates: None,
            controller_month_hours: None,
            controller_month_vent_override: None,
            controller_month_vent: Some(
                query_as!(
                    ControllerMonthVent,
                    "SELECT * FROM controller_month_vent WHERE controller_id = $1 ORDER BY id",
                    id
                )
                .fetch_all(pool)
                .await?,
            ),
        },
        ControllerMonthVentOverrideByControllerId => GetObject {
            compressors: None,
            controllers: None,
            tank_farms: None,
            controller_emission_rates: None,
            controller_month_hours: None,
            controller_month_vent_override: Some(
                query_as!(
                    ControllerMonthVentOverride,
                    "SELECT * FROM controller_month_vent_override WHERE controller_id = $1 ORDER BY id",
                    id
                )
                .fetch_all(pool)
                .await?,
            ),
            controller_month_vent: None,
        },
    };

    Ok(object)
}
