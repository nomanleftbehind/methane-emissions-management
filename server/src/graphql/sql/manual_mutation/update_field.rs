use crate::graphql::models::input::{UpdateFieldInput, UpdateFieldValue};
use chrono::Datelike;
use common::{
    CompressorSealTestingPoint, CompressorType, ControlDevice, ControlDeviceInactivityReason,
    PneumaticInstrumentType, SealType,
    UpdateFieldVariant::{
        CompressorControlDeviceInactivityComment,
        CompressorControlDeviceInactivityCompressorControlledCharacterizationId,
        CompressorControlDeviceInactivityEndDate, CompressorControlDeviceInactivityReason,
        CompressorControlDeviceInactivityStartDate, CompressorControlledCharacterizationComment,
        CompressorControlledCharacterizationCompressorId,
        CompressorControlledCharacterizationControlDevice,
        CompressorControlledCharacterizationEndDate, CompressorControlledCharacterizationStartDate,
        CompressorEmissionSurveyCompressorId, CompressorEmissionSurveyEndDate,
        CompressorEmissionSurveyLeakDuration, CompressorEmissionSurveyRate,
        CompressorEmissionSurveyStartDate, CompressorEmissionSurveySurveyEquipmentId,
        CompressorEmissionSurveySurveyPoint, CompressorFdcRecId, CompressorInstallDate,
        CompressorMonthHoursCompressorId, CompressorMonthHoursMonth,
        CompressorMonthHoursPressurizedHours, CompressorName, CompressorPower,
        CompressorRemoveDate, CompressorSealDescription,
        CompressorSealMonthMethaneEmissionOverrideComment,
        CompressorSealMonthMethaneEmissionOverrideCompressorSealId,
        CompressorSealMonthMethaneEmissionOverrideGasVolume,
        CompressorSealMonthMethaneEmissionOverrideMonth, CompressorSealTestCompressorSealId,
        CompressorSealTestEndDate, CompressorSealTestRate, CompressorSealTestStartDate,
        CompressorSealTestSurveyEquipmentId, CompressorSealTestTestingPoint, CompressorSealType,
        CompressorSerialNumber, CompressorSiteId, CompressorThrowCount,
        CompressorType as CompressorTypeVariant,
        LevelControllerActuationFrequencyActuationFrequency, LevelControllerActuationFrequencyDate,
        LevelControllerActuationFrequencyLevelControllerId, LevelControllerChangeDate,
        LevelControllerChangeLevelControllerId, LevelControllerChangeRate,
        LevelControllerControlDeviceInactivityComment,
        LevelControllerControlDeviceInactivityEndDate,
        LevelControllerControlDeviceInactivityLevelControllerControlledCharacterizationId,
        LevelControllerControlDeviceInactivityReason,
        LevelControllerControlDeviceInactivityStartDate,
        LevelControllerControlledCharacterizationComment,
        LevelControllerControlledCharacterizationControlDevice,
        LevelControllerControlledCharacterizationEndDate,
        LevelControllerControlledCharacterizationLevelControllerId,
        LevelControllerControlledCharacterizationStartDate, LevelControllerEndDate,
        LevelControllerManufacturerId, LevelControllerModel, LevelControllerMonthHoursHoursOn,
        LevelControllerMonthHoursLevelControllerId, LevelControllerMonthHoursMonth,
        LevelControllerMonthMethaneEmissionOverrideComment,
        LevelControllerMonthMethaneEmissionOverrideGasVolume,
        LevelControllerMonthMethaneEmissionOverrideLevelControllerId,
        LevelControllerMonthMethaneEmissionOverrideMonth, LevelControllerSerialNumber,
        LevelControllerSiteId, LevelControllerStartDate, PneumaticInstrumentChangeDate,
        PneumaticInstrumentChangePneumaticInstrumentId, PneumaticInstrumentChangeRate,
        PneumaticInstrumentControlDeviceInactivityComment,
        PneumaticInstrumentControlDeviceInactivityEndDate,
        PneumaticInstrumentControlDeviceInactivityPneumaticInstrumentControlledCharacterizationId,
        PneumaticInstrumentControlDeviceInactivityReason,
        PneumaticInstrumentControlDeviceInactivityStartDate,
        PneumaticInstrumentControlledCharacterizationComment,
        PneumaticInstrumentControlledCharacterizationControlDevice,
        PneumaticInstrumentControlledCharacterizationEndDate,
        PneumaticInstrumentControlledCharacterizationPneumaticInstrumentId,
        PneumaticInstrumentControlledCharacterizationStartDate, PneumaticInstrumentEndDate,
        PneumaticInstrumentManufacturerId, PneumaticInstrumentModel,
        PneumaticInstrumentMonthHoursHoursOn, PneumaticInstrumentMonthHoursMonth,
        PneumaticInstrumentMonthHoursPneumaticInstrumentId,
        PneumaticInstrumentMonthMethaneEmissionOverrideComment,
        PneumaticInstrumentMonthMethaneEmissionOverrideGasVolume,
        PneumaticInstrumentMonthMethaneEmissionOverrideMonth,
        PneumaticInstrumentMonthMethaneEmissionOverridePneumaticInstrumentId,
        PneumaticInstrumentSerialNumber, PneumaticInstrumentSiteId, PneumaticInstrumentStartDate,
        PneumaticInstrumentType as PneumaticInstrumentTypeVariant, PneumaticPumpChangeDate,
        PneumaticPumpChangePneumaticPumpId, PneumaticPumpChangeRate,
        PneumaticPumpControlDeviceInactivityComment, PneumaticPumpControlDeviceInactivityEndDate,
        PneumaticPumpControlDeviceInactivityPneumaticPumpControlledCharacterizationId,
        PneumaticPumpControlDeviceInactivityReason, PneumaticPumpControlDeviceInactivityStartDate,
        PneumaticPumpControlledCharacterizationComment,
        PneumaticPumpControlledCharacterizationControlDevice,
        PneumaticPumpControlledCharacterizationEndDate,
        PneumaticPumpControlledCharacterizationPneumaticPumpId,
        PneumaticPumpControlledCharacterizationStartDate, PneumaticPumpEndDate,
        PneumaticPumpManufacturerId, PneumaticPumpModel, PneumaticPumpMonthHoursHoursOn,
        PneumaticPumpMonthHoursMonth, PneumaticPumpMonthHoursPneumaticPumpId,
        PneumaticPumpMonthMethaneEmissionOverrideComment,
        PneumaticPumpMonthMethaneEmissionOverrideGasVolume,
        PneumaticPumpMonthMethaneEmissionOverrideMonth,
        PneumaticPumpMonthMethaneEmissionOverridePneumaticPumpId, PneumaticPumpSerialNumber,
        PneumaticPumpSiteId, PneumaticPumpStartDate,
    },
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
                control_device_value,
                control_device_inactivity_reason_value,
                seal_type_value,
                compressor_seal_testing_point_value,
            },
    } = input;

    let query = match update_field_variant {
        // Pneumatic Instrument
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
        PneumaticInstrumentTypeVariant => query!(
            "UPDATE pneumatic_instrument
            SET type = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            pneumatic_instrument_type_value as Option<PneumaticInstrumentType>,
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

        // Pneumatic Instrument Change
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
        // Pneumatic Instrument Controlled Characterization
        PneumaticInstrumentControlledCharacterizationPneumaticInstrumentId => query!(
            "UPDATE pneumatic_instrument_controlled_characterization
            SET pneumatic_instrument_id = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            uuid_value,
            user_id,
            updated_at,
        ),
        PneumaticInstrumentControlledCharacterizationStartDate => query!(
            "UPDATE pneumatic_instrument_controlled_characterization
            SET start_date = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            naive_date_value,
            user_id,
            updated_at,
        ),
        PneumaticInstrumentControlledCharacterizationEndDate => query!(
            "UPDATE pneumatic_instrument_controlled_characterization
            SET end_date = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            naive_date_value,
            user_id,
            updated_at,
        ),
        PneumaticInstrumentControlledCharacterizationControlDevice => query!(
            "UPDATE pneumatic_instrument_controlled_characterization
            SET control_device = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            control_device_value as Option<ControlDevice>,
            user_id,
            updated_at,
        ),
        PneumaticInstrumentControlledCharacterizationComment => query!(
            "UPDATE pneumatic_instrument_controlled_characterization
            SET comment = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            string_value,
            user_id,
            updated_at,
        ),
        // Pneumatic Instrument Control Device Inactivity
        PneumaticInstrumentControlDeviceInactivityPneumaticInstrumentControlledCharacterizationId =>
        {
            query!(
                "UPDATE pneumatic_instrument_control_device_inactivity
                SET pneumatic_instrument_controlled_characterization_id = $2,
                    updated_by_id = $3,
                    updated_at = $4
                WHERE id = $1",
                id,
                uuid_value,
                user_id,
                updated_at,
            )
        }
        PneumaticInstrumentControlDeviceInactivityStartDate => {
            query!(
                "UPDATE pneumatic_instrument_control_device_inactivity
                SET start_date = $2,
                    updated_by_id = $3,
                    updated_at = $4
                WHERE id = $1",
                id,
                naive_date_value,
                user_id,
                updated_at,
            )
        }
        PneumaticInstrumentControlDeviceInactivityEndDate => {
            query!(
                "UPDATE pneumatic_instrument_control_device_inactivity
                SET end_date = $2,
                    updated_by_id = $3,
                    updated_at = $4
                WHERE id = $1",
                id,
                naive_date_value,
                user_id,
                updated_at,
            )
        }
        PneumaticInstrumentControlDeviceInactivityReason => {
            query!(
                "UPDATE pneumatic_instrument_control_device_inactivity
                SET reason = $2,
                    updated_by_id = $3,
                    updated_at = $4
                WHERE id = $1",
                id,
                control_device_inactivity_reason_value as Option<ControlDeviceInactivityReason>,
                user_id,
                updated_at,
            )
        }
        PneumaticInstrumentControlDeviceInactivityComment => {
            query!(
                "UPDATE pneumatic_instrument_control_device_inactivity
                SET comment = $2,
                    updated_by_id = $3,
                    updated_at = $4
                WHERE id = $1",
                id,
                string_value,
                user_id,
                updated_at,
            )
        }
        // Pneumatic Instrument Month Hours
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
        // Pneumatic Instrument Month Methane Emission Override
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
        // Level Controller
        LevelControllerSiteId => query!(
            "UPDATE level_controller
            SET site_id = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            uuid_value,
            user_id,
            updated_at,
        ),
        LevelControllerManufacturerId => query!(
            "UPDATE level_controller
            SET manufacturer_id = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            uuid_value,
            user_id,
            updated_at,
        ),
        LevelControllerModel => query!(
            "UPDATE level_controller
            SET model = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            string_value,
            user_id,
            updated_at,
        ),
        LevelControllerSerialNumber => query!(
            "UPDATE level_controller
            SET serial_number = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            string_value,
            user_id,
            updated_at,
        ),
        LevelControllerStartDate => query!(
            "UPDATE level_controller
            SET start_date = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            naive_date_value,
            user_id,
            updated_at,
        ),
        LevelControllerEndDate => query!(
            "UPDATE level_controller
            SET end_date = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            naive_date_value,
            user_id,
            updated_at,
        ),
        // Level Controller ActuationFrequency
        LevelControllerActuationFrequencyLevelControllerId => query!(
            "UPDATE level_controller_actuation_frequency
            SET level_controller_id = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            uuid_value,
            user_id,
            updated_at,
        ),
        LevelControllerActuationFrequencyDate => query!(
            "UPDATE level_controller_actuation_frequency
            SET date = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            naive_date_value,
            user_id,
            updated_at,
        ),
        LevelControllerActuationFrequencyActuationFrequency => query!(
            "UPDATE level_controller_actuation_frequency
            SET actuation_frequency = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            float_value,
            user_id,
            updated_at,
        ),
        // Level Controller Change
        LevelControllerChangeLevelControllerId => query!(
            "UPDATE level_controller_change
            SET level_controller_id = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            uuid_value,
            user_id,
            updated_at,
        ),
        LevelControllerChangeDate => query!(
            "UPDATE level_controller_change
            SET date = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            naive_date_value,
            user_id,
            updated_at,
        ),
        LevelControllerChangeRate => query!(
            "UPDATE level_controller_change
            SET rate = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            float_value,
            user_id,
            updated_at,
        ),
        // Level Controller Controlled Characterization
        LevelControllerControlledCharacterizationLevelControllerId => query!(
            "UPDATE level_controller_controlled_characterization
            SET level_controller_id = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            uuid_value,
            user_id,
            updated_at,
        ),
        LevelControllerControlledCharacterizationStartDate => query!(
            "UPDATE level_controller_controlled_characterization
            SET start_date = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            naive_date_value,
            user_id,
            updated_at,
        ),
        LevelControllerControlledCharacterizationEndDate => query!(
            "UPDATE level_controller_controlled_characterization
            SET end_date = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            naive_date_value,
            user_id,
            updated_at,
        ),
        LevelControllerControlledCharacterizationControlDevice => query!(
            "UPDATE level_controller_controlled_characterization
            SET control_device = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            control_device_value as Option<ControlDevice>,
            user_id,
            updated_at,
        ),
        LevelControllerControlledCharacterizationComment => query!(
            "UPDATE level_controller_controlled_characterization
            SET comment = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            string_value,
            user_id,
            updated_at,
        ),
        // Level Controller Control Device Inactivity
        LevelControllerControlDeviceInactivityLevelControllerControlledCharacterizationId => {
            query!(
                "UPDATE level_controller_control_device_inactivity
                SET level_controller_controlled_characterization_id = $2,
                    updated_by_id = $3,
                    updated_at = $4
                WHERE id = $1",
                id,
                uuid_value,
                user_id,
                updated_at,
            )
        }
        LevelControllerControlDeviceInactivityStartDate => {
            query!(
                "UPDATE level_controller_control_device_inactivity
                SET start_date = $2,
                    updated_by_id = $3,
                    updated_at = $4
                WHERE id = $1",
                id,
                naive_date_value,
                user_id,
                updated_at,
            )
        }
        LevelControllerControlDeviceInactivityEndDate => {
            query!(
                "UPDATE level_controller_control_device_inactivity
                SET end_date = $2,
                    updated_by_id = $3,
                    updated_at = $4
                WHERE id = $1",
                id,
                naive_date_value,
                user_id,
                updated_at,
            )
        }
        LevelControllerControlDeviceInactivityReason => {
            query!(
                "UPDATE level_controller_control_device_inactivity
                SET reason = $2,
                    updated_by_id = $3,
                    updated_at = $4
                WHERE id = $1",
                id,
                control_device_inactivity_reason_value as Option<ControlDeviceInactivityReason>,
                user_id,
                updated_at,
            )
        }
        LevelControllerControlDeviceInactivityComment => {
            query!(
                "UPDATE level_controller_control_device_inactivity
                SET comment = $2,
                    updated_by_id = $3,
                    updated_at = $4
                WHERE id = $1",
                id,
                string_value,
                user_id,
                updated_at,
            )
        }
        // Level Controller Month Hours
        LevelControllerMonthHoursLevelControllerId => query!(
            "UPDATE level_controller_month_hours
            SET level_controller_id = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            uuid_value,
            user_id,
            updated_at,
        ),
        LevelControllerMonthHoursMonth => {
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
                "UPDATE level_controller_month_hours
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
        LevelControllerMonthHoursHoursOn => query!(
            "UPDATE level_controller_month_hours
            SET hours_on = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            float_value,
            user_id,
            updated_at,
        ),
        // Level Controller Month Methane Emission Override
        LevelControllerMonthMethaneEmissionOverrideLevelControllerId => query!(
            "UPDATE level_controller_month_methane_emission_override
            SET level_controller_id = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            uuid_value,
            user_id,
            updated_at,
        ),
        LevelControllerMonthMethaneEmissionOverrideMonth => {
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
                "UPDATE level_controller_month_methane_emission_override
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
        LevelControllerMonthMethaneEmissionOverrideGasVolume => query!(
            "UPDATE level_controller_month_methane_emission_override
            SET gas_volume = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            float_value,
            user_id,
            updated_at,
        ),
        LevelControllerMonthMethaneEmissionOverrideComment => query!(
            "UPDATE level_controller_month_methane_emission_override
            SET comment = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            string_value,
            user_id,
            updated_at,
        ),
        // Pneumatic Pump
        PneumaticPumpSiteId => query!(
            "UPDATE pneumatic_pump
            SET site_id = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            uuid_value,
            user_id,
            updated_at,
        ),
        PneumaticPumpManufacturerId => query!(
            "UPDATE pneumatic_pump
            SET manufacturer_id = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            uuid_value,
            user_id,
            updated_at,
        ),
        PneumaticPumpModel => query!(
            "UPDATE pneumatic_pump
            SET model = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            string_value,
            user_id,
            updated_at,
        ),
        PneumaticPumpSerialNumber => query!(
            "UPDATE pneumatic_pump
            SET serial_number = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            string_value,
            user_id,
            updated_at,
        ),
        PneumaticPumpStartDate => query!(
            "UPDATE pneumatic_pump
            SET start_date = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            naive_date_value,
            user_id,
            updated_at,
        ),
        PneumaticPumpEndDate => query!(
            "UPDATE pneumatic_pump
            SET end_date = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            naive_date_value,
            user_id,
            updated_at,
        ),
        // Pneumatic Pump Change
        PneumaticPumpChangePneumaticPumpId => query!(
            "UPDATE pneumatic_pump_change
            SET pneumatic_pump_id = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            uuid_value,
            user_id,
            updated_at,
        ),
        PneumaticPumpChangeDate => query!(
            "UPDATE pneumatic_pump_change
            SET date = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            naive_date_value,
            user_id,
            updated_at,
        ),
        PneumaticPumpChangeRate => query!(
            "UPDATE pneumatic_pump_change
            SET rate = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            float_value,
            user_id,
            updated_at,
        ),
        // Pneumatic Pump Controlled Characterization
        PneumaticPumpControlledCharacterizationPneumaticPumpId => query!(
            "UPDATE pneumatic_pump_controlled_characterization
            SET pneumatic_pump_id = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            uuid_value,
            user_id,
            updated_at,
        ),
        PneumaticPumpControlledCharacterizationStartDate => query!(
            "UPDATE pneumatic_pump_controlled_characterization
            SET start_date = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            naive_date_value,
            user_id,
            updated_at,
        ),
        PneumaticPumpControlledCharacterizationEndDate => query!(
            "UPDATE pneumatic_pump_controlled_characterization
            SET end_date = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            naive_date_value,
            user_id,
            updated_at,
        ),
        PneumaticPumpControlledCharacterizationControlDevice => query!(
            "UPDATE pneumatic_pump_controlled_characterization
            SET control_device = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            control_device_value as Option<ControlDevice>,
            user_id,
            updated_at,
        ),
        PneumaticPumpControlledCharacterizationComment => query!(
            "UPDATE pneumatic_pump_controlled_characterization
            SET comment = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            string_value,
            user_id,
            updated_at,
        ),
        // Pneumatic Pump Control Device Inactivity
        PneumaticPumpControlDeviceInactivityPneumaticPumpControlledCharacterizationId => {
            query!(
                "UPDATE pneumatic_pump_control_device_inactivity
                SET pneumatic_pump_controlled_characterization_id = $2,
                    updated_by_id = $3,
                    updated_at = $4
                WHERE id = $1",
                id,
                uuid_value,
                user_id,
                updated_at,
            )
        }
        PneumaticPumpControlDeviceInactivityStartDate => {
            query!(
                "UPDATE pneumatic_pump_control_device_inactivity
                SET start_date = $2,
                    updated_by_id = $3,
                    updated_at = $4
                WHERE id = $1",
                id,
                naive_date_value,
                user_id,
                updated_at,
            )
        }
        PneumaticPumpControlDeviceInactivityEndDate => {
            query!(
                "UPDATE pneumatic_pump_control_device_inactivity
                SET end_date = $2,
                    updated_by_id = $3,
                    updated_at = $4
                WHERE id = $1",
                id,
                naive_date_value,
                user_id,
                updated_at,
            )
        }
        PneumaticPumpControlDeviceInactivityReason => {
            query!(
                "UPDATE pneumatic_pump_control_device_inactivity
                SET reason = $2,
                    updated_by_id = $3,
                    updated_at = $4
                WHERE id = $1",
                id,
                control_device_inactivity_reason_value as Option<ControlDeviceInactivityReason>,
                user_id,
                updated_at,
            )
        }
        PneumaticPumpControlDeviceInactivityComment => {
            query!(
                "UPDATE pneumatic_pump_control_device_inactivity
                SET comment = $2,
                    updated_by_id = $3,
                    updated_at = $4
                WHERE id = $1",
                id,
                string_value,
                user_id,
                updated_at,
            )
        }
        // Pneumatic Pump Month Hours
        PneumaticPumpMonthHoursPneumaticPumpId => query!(
            "UPDATE pneumatic_pump_month_hours
            SET pneumatic_pump_id = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            uuid_value,
            user_id,
            updated_at,
        ),
        PneumaticPumpMonthHoursMonth => {
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
                "UPDATE pneumatic_pump_month_hours
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
        PneumaticPumpMonthHoursHoursOn => query!(
            "UPDATE pneumatic_pump_month_hours
            SET hours_on = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            float_value,
            user_id,
            updated_at,
        ),
        // Pneumatic Pump Month Methane Emission Override
        PneumaticPumpMonthMethaneEmissionOverridePneumaticPumpId => query!(
            "UPDATE pneumatic_pump_month_methane_emission_override
            SET pneumatic_pump_id = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            uuid_value,
            user_id,
            updated_at,
        ),
        PneumaticPumpMonthMethaneEmissionOverrideMonth => {
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
                "UPDATE pneumatic_pump_month_methane_emission_override
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
        PneumaticPumpMonthMethaneEmissionOverrideGasVolume => query!(
            "UPDATE pneumatic_pump_month_methane_emission_override
            SET gas_volume = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            float_value,
            user_id,
            updated_at,
        ),
        PneumaticPumpMonthMethaneEmissionOverrideComment => query!(
            "UPDATE pneumatic_pump_month_methane_emission_override
            SET comment = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            string_value,
            user_id,
            updated_at,
        ),
        // Compressor
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
        CompressorTypeVariant => query!(
            "UPDATE compressor
            SET type = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            compressor_type_value as Option<CompressorType>,
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
        CompressorSealType => query!(
            "UPDATE compressor_seal
            SET type = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            seal_type_value as Option<SealType>,
            user_id,
            updated_at,
        ),
        CompressorSealDescription => query!(
            "UPDATE compressor_seal
            SET description = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            string_value,
            user_id,
            updated_at,
        ),
        CompressorSealTestCompressorSealId => query!(
            "UPDATE compressor_seal_test
            SET compressor_seal_id = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            uuid_value,
            user_id,
            updated_at,
        ),
        CompressorSealTestStartDate => query!(
            "UPDATE compressor_seal_test
            SET start_date = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            naive_date_value,
            user_id,
            updated_at,
        ),
        CompressorSealTestEndDate => query!(
            "UPDATE compressor_seal_test
            SET end_date = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            naive_date_value,
            user_id,
            updated_at,
        ),
        CompressorSealTestRate => query!(
            "UPDATE compressor_seal_test
            SET rate = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            float_value,
            user_id,
            updated_at,
        ),
        CompressorSealTestTestingPoint => query!(
            "UPDATE compressor_seal_test
            SET testing_point = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            compressor_seal_testing_point_value as Option<CompressorSealTestingPoint>,
            user_id,
            updated_at,
        ),
        CompressorSealTestSurveyEquipmentId => query!(
            "UPDATE compressor_seal_test
            SET survey_equipment_id = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            uuid_value,
            user_id,
            updated_at,
        ),
        // Compressor Emission Survey
        CompressorEmissionSurveyCompressorId => query!(
            "UPDATE compressor_emission_survey
            SET compressor_id = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            uuid_value,
            user_id,
            updated_at,
        ),
        CompressorEmissionSurveyStartDate => query!(
            "UPDATE compressor_emission_survey
            SET start_date = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            naive_date_value,
            user_id,
            updated_at,
        ),
        CompressorEmissionSurveyEndDate => query!(
            "UPDATE compressor_emission_survey
            SET end_date = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            naive_date_value,
            user_id,
            updated_at,
        ),
        CompressorEmissionSurveyRate => query!(
            "UPDATE compressor_emission_survey
            SET rate = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            float_value,
            user_id,
            updated_at,
        ),
        CompressorEmissionSurveySurveyPoint => query!(
            "UPDATE compressor_emission_survey
            SET survey_point = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            string_value,
            user_id,
            updated_at,
        ),
        CompressorEmissionSurveyLeakDuration => query!(
            "UPDATE compressor_emission_survey
            SET leak_duration = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            float_value,
            user_id,
            updated_at,
        ),
        CompressorEmissionSurveySurveyEquipmentId => query!(
            "UPDATE compressor_emission_survey
            SET survey_equipment_id = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            uuid_value,
            user_id,
            updated_at,
        ),
        CompressorControlledCharacterizationCompressorId => query!(
            "UPDATE compressor_controlled_characterization
            SET compressor_id = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            uuid_value,
            user_id,
            updated_at,
        ),
        CompressorControlledCharacterizationStartDate => query!(
            "UPDATE compressor_controlled_characterization
            SET start_date = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            naive_date_value,
            user_id,
            updated_at,
        ),
        CompressorControlledCharacterizationEndDate => query!(
            "UPDATE compressor_controlled_characterization
            SET end_date = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            naive_date_value,
            user_id,
            updated_at,
        ),
        CompressorControlledCharacterizationControlDevice => query!(
            "UPDATE compressor_controlled_characterization
            SET control_device = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            control_device_value as Option<ControlDevice>,
            user_id,
            updated_at,
        ),
        CompressorControlledCharacterizationComment => query!(
            "UPDATE compressor_controlled_characterization
            SET comment = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            string_value,
            user_id,
            updated_at,
        ),
        CompressorControlDeviceInactivityCompressorControlledCharacterizationId => query!(
            "UPDATE compressor_control_device_inactivity
            SET compressor_controlled_characterization_id = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            uuid_value,
            user_id,
            updated_at,
        ),
        CompressorControlDeviceInactivityStartDate => query!(
            "UPDATE compressor_control_device_inactivity
            SET start_date = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            naive_date_value,
            user_id,
            updated_at,
        ),
        CompressorControlDeviceInactivityEndDate => query!(
            "UPDATE compressor_control_device_inactivity
            SET end_date = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            naive_date_value,
            user_id,
            updated_at,
        ),
        CompressorControlDeviceInactivityReason => query!(
            "UPDATE compressor_control_device_inactivity
            SET reason = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            control_device_inactivity_reason_value as Option<ControlDeviceInactivityReason>,
            user_id,
            updated_at,
        ),
        CompressorControlDeviceInactivityComment => query!(
            "UPDATE compressor_control_device_inactivity
            SET comment = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            string_value,
            user_id,
            updated_at,
        ),
        CompressorMonthHoursCompressorId => query!(
            "UPDATE compressor_month_hours
            SET compressor_id = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            uuid_value,
            user_id,
            updated_at,
        ),
        CompressorMonthHoursMonth => {
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
                "UPDATE compressor_month_hours
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
        CompressorMonthHoursPressurizedHours => query!(
            "UPDATE compressor_month_hours
            SET pressurized_hours = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            float_value,
            user_id,
            updated_at,
        ),
        CompressorSealMonthMethaneEmissionOverrideCompressorSealId => query!(
            "UPDATE compressor_seal_month_methane_emission_override
            SET compressor_seal_id = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            uuid_value,
            user_id,
            updated_at,
        ),
        CompressorSealMonthMethaneEmissionOverrideMonth => {
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
                "UPDATE compressor_seal_month_methane_emission_override
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
        CompressorSealMonthMethaneEmissionOverrideGasVolume => query!(
            "UPDATE compressor_seal_month_methane_emission_override
            SET gas_volume = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            float_value,
            user_id,
            updated_at,
        ),
        CompressorSealMonthMethaneEmissionOverrideComment => query!(
            "UPDATE compressor_seal_month_methane_emission_override
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
