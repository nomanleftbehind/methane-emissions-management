use crate::graphql::models::input::{UpdateFieldInput, UpdateFieldValue};
use chrono::Datelike;
use common::UpdateFieldVariant::{
    CompressorFdcRecId, CompressorInstallDate, CompressorName, CompressorPower,
    CompressorRemoveDate, CompressorSerialNumber, CompressorSiteId, CompressorThrowCount,
    CompressorType, PneumaticInstrumentChangeDate, PneumaticInstrumentChangePneumaticInstrumentId,
    PneumaticInstrumentChangeRate, PneumaticInstrumentEndDate, PneumaticInstrumentManufacturerId,
    PneumaticInstrumentModel, PneumaticInstrumentMonthHoursHoursOn,
    PneumaticInstrumentMonthHoursMonth, PneumaticInstrumentMonthHoursPneumaticInstrumentId,
    PneumaticInstrumentMonthMethaneEmissionOverrideComment,
    PneumaticInstrumentMonthMethaneEmissionOverrideGasVolume,
    PneumaticInstrumentMonthMethaneEmissionOverrideMonth,
    PneumaticInstrumentMonthMethaneEmissionOverridePneumaticInstrumentId,
    PneumaticInstrumentSerialNumber, PneumaticInstrumentSiteId, PneumaticInstrumentStartDate,
    PneumaticInstrumentType,
};
use sqlx::{query, Error, PgPool};
use uuid::Uuid;

pub async fn update_field(
    pool: &PgPool,
    input: UpdateFieldInput,
    user_id: &Uuid,
) -> Result<u64, Error> {
    let updated_at = chrono::Utc::now().naive_utc();

    println!("input: {:#?}", &input);
    let UpdateFieldInput {
        id,
        update_field_variant,
        value:
            UpdateFieldValue {
                string_value,
                uuid_value,
                integer_value,
                float_value,
                naive_date_value,
                naive_date_time_value: _,
                pneumatic_instrument_type_value,
                compressor_type_value,
            },
    } = input;

    let query = match update_field_variant {
        PneumaticInstrumentSiteId => query!(
            "UPDATE pneumatic_instrument
            SET site_id = $2,
            updated_by_id = $3,
            updated_at = $4
            WHERE id = $1",
            id,
            uuid_value,
            user_id,
            updated_at,
        ),
        PneumaticInstrumentType => query!(
            "UPDATE pneumatic_instrument
            SET type = $2,
            updated_by_id = $3,
            updated_at = $4
            WHERE id = $1",
            id,
            pneumatic_instrument_type_value as _,
            user_id,
            updated_at,
        ),
        PneumaticInstrumentManufacturerId => query!(
            "UPDATE pneumatic_instrument
            SET manufacturer_id = $2,
                updated_by_id = $3,
                updated_at = $4
                WHERE id = $1",
            id,
            uuid_value,
            user_id,
            updated_at,
        ),
        PneumaticInstrumentModel => query!(
            "UPDATE pneumatic_instrument
            SET model = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            string_value,
            user_id,
            updated_at,
        ),
        PneumaticInstrumentSerialNumber => query!(
            "UPDATE pneumatic_instrument
            SET serial_number = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            string_value,
            user_id,
            updated_at,
        ),
        PneumaticInstrumentStartDate => query!(
            "UPDATE pneumatic_instrument
            SET start_date = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            naive_date_value,
            user_id,
            updated_at,
        ),
        PneumaticInstrumentEndDate => query!(
            "UPDATE pneumatic_instrument
            SET end_date = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            naive_date_value,
            user_id,
            updated_at,
        ),
        CompressorSiteId => query!(
            "UPDATE compressor
            SET site_id = $2,
            updated_by_id = $3,
            updated_at = $4
            WHERE id = $1",
            id,
            uuid_value,
            user_id,
            updated_at,
        ),
        CompressorFdcRecId => query!(
            "UPDATE compressor
            SET fdc_rec_id = $2,
            updated_by_id = $3,
            updated_at = $4
            WHERE id = $1",
            id,
            string_value,
            user_id,
            updated_at,
        ),
        CompressorType => query!(
            "UPDATE compressor
            SET type = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            compressor_type_value as _,
            user_id,
            updated_at,
        ),
        CompressorName => query!(
            "UPDATE compressor
            SET name = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            string_value,
            user_id,
            updated_at,
        ),
        CompressorSerialNumber => query!(
            "UPDATE compressor
            SET serial_number = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            string_value,
            user_id,
            updated_at,
        ),
        CompressorPower => query!(
            "UPDATE compressor
            SET power = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            float_value,
            user_id,
            updated_at,
        ),
        CompressorThrowCount => query!(
            "UPDATE compressor
            SET throw_count = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            integer_value,
            user_id,
            updated_at,
        ),
        CompressorInstallDate => query!(
            "UPDATE compressor
            SET install_date = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            naive_date_value,
            user_id,
            updated_at,
        ),
        CompressorRemoveDate => query!(
            "UPDATE compressor
            SET remove_date = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            naive_date_value,
            user_id,
            updated_at,
        ),
        PneumaticInstrumentChangePneumaticInstrumentId => query!(
            "UPDATE pneumatic_instrument_change
            SET pneumatic_instrument_id = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            uuid_value,
            user_id,
            updated_at,
        ),
        PneumaticInstrumentChangeDate => query!(
            "UPDATE pneumatic_instrument_change
            SET date = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            naive_date_value,
            user_id,
            updated_at,
        ),
        PneumaticInstrumentChangeRate => query!(
            "UPDATE pneumatic_instrument_change
            SET rate = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            float_value,
            user_id,
            updated_at,
        ),
        PneumaticInstrumentMonthHoursPneumaticInstrumentId => query!(
            "UPDATE pneumatic_instrument_month_hours
            SET pneumatic_instrument_id = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            uuid_value,
            user_id,
            updated_at,
        ),
        PneumaticInstrumentMonthHoursMonth => {
            if let Some(value) = &naive_date_value {
                if value.day() != 1 {
                    let error = Error::Io(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Expected first day of the month, got `{}`", value),
                    ));
                    return Err(error);
                }
            }

            query!(
                "UPDATE pneumatic_instrument_month_hours
                SET month = $2,
                    updated_by_id = $3,
                    updated_at = $4
                WHERE id = $1",
                id,
                naive_date_value,
                user_id,
                updated_at,
            )
        }
        PneumaticInstrumentMonthHoursHoursOn => query!(
            "UPDATE pneumatic_instrument_month_hours
            SET hours_on = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            float_value,
            user_id,
            updated_at,
        ),
        PneumaticInstrumentMonthMethaneEmissionOverridePneumaticInstrumentId => query!(
            "UPDATE pneumatic_instrument_month_methane_emission_override
            SET pneumatic_instrument_id = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            uuid_value,
            user_id,
            updated_at,
        ),
        PneumaticInstrumentMonthMethaneEmissionOverrideMonth => {
            if let Some(value) = &naive_date_value {
                if value.day() != 1 {
                    let error = Error::Io(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Expected first day of the month, got `{}`", value),
                    ));
                    return Err(error);
                }
            }

            query!(
                "UPDATE pneumatic_instrument_month_methane_emission_override
                SET month = $2,
                    updated_by_id = $3,
                    updated_at = $4
                WHERE id = $1",
                id,
                naive_date_value,
                user_id,
                updated_at,
            )
        }
        PneumaticInstrumentMonthMethaneEmissionOverrideGasVolume => query!(
            "UPDATE pneumatic_instrument_month_methane_emission_override
            SET gas_volume = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            float_value,
            user_id,
            updated_at,
        ),
        PneumaticInstrumentMonthMethaneEmissionOverrideComment => query!(
            "UPDATE pneumatic_instrument_month_methane_emission_override
            SET comment = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            string_value,
            user_id,
            updated_at,
        ),
    };

    let res = query.execute(pool).await?.rows_affected();

    println!("rows affected: {}", res);

    Ok(res)

    // Ok(query.execute(pool).await?.rows_affected())
}
