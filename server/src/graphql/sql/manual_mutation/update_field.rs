use crate::graphql::models::input::{UpdateFieldInput, UpdateFieldValue};
use chrono::{Datelike, NaiveDate};
use common::{
    CalculationMethod, CompressorSealTestingPoint, CompressorType, ControlDevice,
    ControlDeviceInactivityReason, FacilityType, PneumaticInstrumentType, SealType, SiteType,
    UpdateFieldVariant::{
        CompressorBlowdownOverrideComment, CompressorBlowdownOverrideCompressorId,
        CompressorBlowdownOverrideDate, CompressorBlowdownOverrideGasVolume,
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
        CompressorType as CompressorTypeVariant, DeviceManufacturerManufacturer, FacilityIdpa,
        FacilityName, FacilityType as FacilityTypeVariant, GasAnalysisC1, GasAnalysisC2,
        GasAnalysisC3, GasAnalysisC4I, GasAnalysisC4N, GasAnalysisC5I, GasAnalysisC5N,
        GasAnalysisC6, GasAnalysisC7Plus, GasAnalysisCo2, GasAnalysisDate, GasAnalysisH2,
        GasAnalysisH2s, GasAnalysisHe, GasAnalysisN2,
        LevelControllerActuationFrequencyActuationFrequency, LevelControllerActuationFrequencyDate,
        LevelControllerActuationFrequencyLevelControllerId,
        LevelControllerControlDeviceInactivityComment,
        LevelControllerControlDeviceInactivityEndDate,
        LevelControllerControlDeviceInactivityLevelControllerControlledCharacterizationId,
        LevelControllerControlDeviceInactivityReason,
        LevelControllerControlDeviceInactivityStartDate,
        LevelControllerControlledCharacterizationComment,
        LevelControllerControlledCharacterizationControlDevice,
        LevelControllerControlledCharacterizationEndDate,
        LevelControllerControlledCharacterizationLevelControllerId,
        LevelControllerControlledCharacterizationStartDate, LevelControllerEmissionRateDate,
        LevelControllerEmissionRateLevelControllerId, LevelControllerEmissionRateRate,
        LevelControllerEndDate, LevelControllerManufacturerId, LevelControllerModel,
        LevelControllerMonthHoursHoursOn, LevelControllerMonthHoursLevelControllerId,
        LevelControllerMonthHoursMonth, LevelControllerMonthMethaneEmissionOverrideComment,
        LevelControllerMonthMethaneEmissionOverrideGasVolume,
        LevelControllerMonthMethaneEmissionOverrideLevelControllerId,
        LevelControllerMonthMethaneEmissionOverrideMonth, LevelControllerSerialNumber,
        LevelControllerSiteId, LevelControllerStartDate,
        PneumaticInstrumentControlDeviceInactivityComment,
        PneumaticInstrumentControlDeviceInactivityEndDate,
        PneumaticInstrumentControlDeviceInactivityPneumaticInstrumentControlledCharacterizationId,
        PneumaticInstrumentControlDeviceInactivityReason,
        PneumaticInstrumentControlDeviceInactivityStartDate,
        PneumaticInstrumentControlledCharacterizationComment,
        PneumaticInstrumentControlledCharacterizationControlDevice,
        PneumaticInstrumentControlledCharacterizationEndDate,
        PneumaticInstrumentControlledCharacterizationPneumaticInstrumentId,
        PneumaticInstrumentControlledCharacterizationStartDate,
        PneumaticInstrumentEmissionRateDate, PneumaticInstrumentEmissionRatePneumaticInstrumentId,
        PneumaticInstrumentEmissionRateRate, PneumaticInstrumentEndDate,
        PneumaticInstrumentManufacturerId, PneumaticInstrumentModel,
        PneumaticInstrumentMonthHoursHoursOn, PneumaticInstrumentMonthHoursMonth,
        PneumaticInstrumentMonthHoursPneumaticInstrumentId,
        PneumaticInstrumentMonthMethaneEmissionOverrideComment,
        PneumaticInstrumentMonthMethaneEmissionOverrideGasVolume,
        PneumaticInstrumentMonthMethaneEmissionOverrideMonth,
        PneumaticInstrumentMonthMethaneEmissionOverridePneumaticInstrumentId,
        PneumaticInstrumentSerialNumber, PneumaticInstrumentSiteId, PneumaticInstrumentStartDate,
        PneumaticInstrumentType as PneumaticInstrumentTypeVariant,
        PneumaticPumpControlDeviceInactivityComment, PneumaticPumpControlDeviceInactivityEndDate,
        PneumaticPumpControlDeviceInactivityPneumaticPumpControlledCharacterizationId,
        PneumaticPumpControlDeviceInactivityReason, PneumaticPumpControlDeviceInactivityStartDate,
        PneumaticPumpControlledCharacterizationComment,
        PneumaticPumpControlledCharacterizationControlDevice,
        PneumaticPumpControlledCharacterizationEndDate,
        PneumaticPumpControlledCharacterizationPneumaticPumpId,
        PneumaticPumpControlledCharacterizationStartDate, PneumaticPumpEmissionRateDate,
        PneumaticPumpEmissionRatePneumaticPumpId, PneumaticPumpEmissionRateRate,
        PneumaticPumpEndDate, PneumaticPumpManufacturerId, PneumaticPumpModel,
        PneumaticPumpMonthHoursHoursOn, PneumaticPumpMonthHoursMonth,
        PneumaticPumpMonthHoursPneumaticPumpId, PneumaticPumpMonthMethaneEmissionOverrideComment,
        PneumaticPumpMonthMethaneEmissionOverrideGasVolume,
        PneumaticPumpMonthMethaneEmissionOverrideMonth,
        PneumaticPumpMonthMethaneEmissionOverridePneumaticPumpId, PneumaticPumpSerialNumber,
        PneumaticPumpSiteId, PneumaticPumpStartDate, SiteDescription, SiteFacilityId, SiteFdcRecId,
        SiteName, SiteType as SiteTypeVariant, StorageTankChangeApiDensity,
        StorageTankChangeCalculationMethod, StorageTankChangeDate, StorageTankChangeIA,
        StorageTankChangePressure, StorageTankChangeStorageTankId, StorageTankChangeTemperature,
        StorageTankControlDeviceInactivityComment, StorageTankControlDeviceInactivityEndDate,
        StorageTankControlDeviceInactivityReason, StorageTankControlDeviceInactivityStartDate,
        StorageTankControlDeviceInactivityStorageTankControlledCharacterizationId,
        StorageTankControlledCharacterizationComment,
        StorageTankControlledCharacterizationControlDevice,
        StorageTankControlledCharacterizationEndDate,
        StorageTankControlledCharacterizationStartDate,
        StorageTankControlledCharacterizationStorageTankId, StorageTankEmissionSurveyEndDate,
        StorageTankEmissionSurveyLeakDuration, StorageTankEmissionSurveyRate,
        StorageTankEmissionSurveyStartDate, StorageTankEmissionSurveyStorageTankId,
        StorageTankEmissionSurveySurveyEquipmentId, StorageTankEmissionSurveySurveyPoint,
        StorageTankEndDate, StorageTankMonthLiquidHydrocarbonEnteringLiquidHydrocarbonVolume,
        StorageTankMonthLiquidHydrocarbonEnteringMonth,
        StorageTankMonthLiquidHydrocarbonEnteringStorageTankId,
        StorageTankMonthMethaneEmissionOverrideComment,
        StorageTankMonthMethaneEmissionOverrideGasVolume,
        StorageTankMonthMethaneEmissionOverrideMonth,
        StorageTankMonthMethaneEmissionOverrideStorageTankId, StorageTankSiteId,
        StorageTankStartDate, SurveyEquipmentMake, SurveyEquipmentModel,
    },
};
use sqlx::{query, Error, PgPool};
use uuid::Uuid;

fn not_first_day_of_month_error(date: &NaiveDate) -> Error {
    Error::Io(std::io::Error::new(
        std::io::ErrorKind::InvalidInput,
        format!("Expected first day of the month, got `{}`", date),
    ))
}

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
                integer_value,
                float_value,
                uuid_value,
                bool_value,
                naive_date_value,
                naive_date_time_value: _,
                facility_type_value,
                site_type_value,
                pneumatic_instrument_type_value,
                compressor_type_value,
                control_device_value,
                control_device_inactivity_reason_value,
                seal_type_value,
                compressor_seal_testing_point_value,
                calculation_method_value,
            },
    } = input;

    let query = match update_field_variant {
        // Facility
        FacilityIdpa => query!(
            "UPDATE facility
            SET idpa = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            string_value,
            user_id,
            updated_at,
        ),
        FacilityName => query!(
            "UPDATE facility
            SET name = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            string_value,
            user_id,
            updated_at,
        ),
        FacilityTypeVariant => query!(
            "UPDATE facility
            SET type = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            facility_type_value as Option<FacilityType>,
            user_id,
            updated_at,
        ),
        // Site
        SiteFacilityId => query!(
            "UPDATE site
            SET facility_id = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            uuid_value,
            user_id,
            updated_at,
        ),
        SiteFdcRecId => query!(
            "UPDATE site
            SET fdc_rec_id = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            string_value,
            user_id,
            updated_at,
        ),
        SiteName => query!(
            "UPDATE site
            SET name = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            string_value,
            user_id,
            updated_at,
        ),
        SiteTypeVariant => query!(
            "UPDATE site
            SET type = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            site_type_value as Option<SiteType>,
            user_id,
            updated_at,
        ),
        SiteDescription => query!(
            "UPDATE site
            SET description = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            string_value,
            user_id,
            updated_at,
        ),
        // Device Manufacturer
        DeviceManufacturerManufacturer => query!(
            "UPDATE device_manufacturer
            SET manufacturer = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            string_value,
            user_id,
            updated_at,
        ),
        // Survey Equipment
        SurveyEquipmentMake => query!(
            "UPDATE survey_equipment
            SET make = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            string_value,
            user_id,
            updated_at,
        ),
        SurveyEquipmentModel => query!(
            "UPDATE survey_equipment
            SET model = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            string_value,
            user_id,
            updated_at,
        ),
        GasAnalysisDate => query!(
            "UPDATE gas_analysis
            SET date = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            naive_date_value,
            user_id,
            updated_at,
        ),
        GasAnalysisH2 => query!(
            "UPDATE gas_analysis
            SET h2 = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            float_value,
            user_id,
            updated_at,
        ),
        GasAnalysisHe => query!(
            "UPDATE gas_analysis
            SET he = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            float_value,
            user_id,
            updated_at,
        ),
        GasAnalysisN2 => query!(
            "UPDATE gas_analysis
            SET n2 = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            float_value,
            user_id,
            updated_at,
        ),
        GasAnalysisCo2 => query!(
            "UPDATE gas_analysis
            SET co2 = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            float_value,
            user_id,
            updated_at,
        ),
        GasAnalysisH2s => query!(
            "UPDATE gas_analysis
            SET h2s = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            float_value,
            user_id,
            updated_at,
        ),
        GasAnalysisC1 => query!(
            "UPDATE gas_analysis
            SET c1 = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            float_value,
            user_id,
            updated_at,
        ),
        GasAnalysisC2 => query!(
            "UPDATE gas_analysis
            SET c2 = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            float_value,
            user_id,
            updated_at,
        ),
        GasAnalysisC3 => query!(
            "UPDATE gas_analysis
            SET c3 = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            float_value,
            user_id,
            updated_at,
        ),
        GasAnalysisC4I => query!(
            "UPDATE gas_analysis
            SET c4_i = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            float_value,
            user_id,
            updated_at,
        ),
        GasAnalysisC4N => query!(
            "UPDATE gas_analysis
            SET c4_n = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            float_value,
            user_id,
            updated_at,
        ),
        GasAnalysisC5I => query!(
            "UPDATE gas_analysis
            SET c5_i = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            float_value,
            user_id,
            updated_at,
        ),
        GasAnalysisC5N => query!(
            "UPDATE gas_analysis
            SET c5_n = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            float_value,
            user_id,
            updated_at,
        ),
        GasAnalysisC6 => query!(
            "UPDATE gas_analysis
            SET c6 = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            float_value,
            user_id,
            updated_at,
        ),
        GasAnalysisC7Plus => query!(
            "UPDATE gas_analysis
            SET c7_plus = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            float_value,
            user_id,
            updated_at,
        ),
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

        // Pneumatic Instrument Emission Rate
        PneumaticInstrumentEmissionRatePneumaticInstrumentId => query!(
            "UPDATE pneumatic_instrument_emission_rate
            SET pneumatic_instrument_id = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            uuid_value,
            user_id,
            updated_at,
        ),
        PneumaticInstrumentEmissionRateDate => query!(
            "UPDATE pneumatic_instrument_emission_rate
            SET date = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            naive_date_value,
            user_id,
            updated_at,
        ),
        PneumaticInstrumentEmissionRateRate => query!(
            "UPDATE pneumatic_instrument_emission_rate
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
                    return Err(not_first_day_of_month_error(value));
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
                    return Err(not_first_day_of_month_error(value));
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
        // Level Controller Emission Rate
        LevelControllerEmissionRateLevelControllerId => query!(
            "UPDATE level_controller_emission_rate
            SET level_controller_id = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            uuid_value,
            user_id,
            updated_at,
        ),
        LevelControllerEmissionRateDate => query!(
            "UPDATE level_controller_emission_rate
            SET date = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            naive_date_value,
            user_id,
            updated_at,
        ),
        LevelControllerEmissionRateRate => query!(
            "UPDATE level_controller_emission_rate
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
                    return Err(not_first_day_of_month_error(value));
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
                    return Err(not_first_day_of_month_error(value));
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
        // Pneumatic Pump Emission Rate
        PneumaticPumpEmissionRatePneumaticPumpId => query!(
            "UPDATE pneumatic_pump_emission_rate
            SET pneumatic_pump_id = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            uuid_value,
            user_id,
            updated_at,
        ),
        PneumaticPumpEmissionRateDate => query!(
            "UPDATE pneumatic_pump_emission_rate
            SET date = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            naive_date_value,
            user_id,
            updated_at,
        ),
        PneumaticPumpEmissionRateRate => query!(
            "UPDATE pneumatic_pump_emission_rate
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
                    return Err(not_first_day_of_month_error(value));
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
                    return Err(not_first_day_of_month_error(value));
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
        // Compressor Controlled Characterization
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
        // Compressor Control Device Inactivity
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
                    return Err(not_first_day_of_month_error(value));
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
        // Compressor Seal Month Methane Emission Override
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
                    return Err(not_first_day_of_month_error(value));
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
        // Compressor Blowdown Override
        CompressorBlowdownOverrideCompressorId => query!(
            "UPDATE compressor_blowdown_override
            SET compressor_id = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            uuid_value,
            user_id,
            updated_at,
        ),
        CompressorBlowdownOverrideDate => query!(
            "UPDATE compressor_blowdown_override
            SET date = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            naive_date_value,
            user_id,
            updated_at,
        ),
        CompressorBlowdownOverrideGasVolume => query!(
            "UPDATE compressor_blowdown_override
            SET gas_volume = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            float_value,
            user_id,
            updated_at,
        ),
        CompressorBlowdownOverrideComment => query!(
            "UPDATE compressor_blowdown_override
            SET comment = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            string_value,
            user_id,
            updated_at,
        ),
        StorageTankSiteId => query!(
            "UPDATE storage_tank
            SET site_id = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            uuid_value,
            user_id,
            updated_at,
        ),
        StorageTankStartDate => query!(
            "UPDATE storage_tank
            SET start_date = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            naive_date_value,
            user_id,
            updated_at,
        ),
        StorageTankEndDate => query!(
            "UPDATE storage_tank
            SET end_date = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            naive_date_value,
            user_id,
            updated_at,
        ),
        StorageTankChangeStorageTankId => query!(
            "UPDATE storage_tank_change
            SET storage_tank_id = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            uuid_value,
            user_id,
            updated_at,
        ),
        StorageTankChangeDate => query!(
            "UPDATE storage_tank_change
            SET date = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            naive_date_value,
            user_id,
            updated_at,
        ),
        StorageTankChangeIA => query!(
            "UPDATE storage_tank_change
            SET ia = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            bool_value,
            user_id,
            updated_at,
        ),
        StorageTankChangeApiDensity => query!(
            "UPDATE storage_tank_change
            SET api_density = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            float_value,
            user_id,
            updated_at,
        ),
        StorageTankChangeTemperature => query!(
            "UPDATE storage_tank_change
            SET temperature = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            float_value,
            user_id,
            updated_at,
        ),
        StorageTankChangePressure => query!(
            "UPDATE storage_tank_change
            SET pressure = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            float_value,
            user_id,
            updated_at,
        ),
        StorageTankChangeCalculationMethod => query!(
            "UPDATE storage_tank_change
            SET calculation_method = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            calculation_method_value as Option<CalculationMethod>,
            user_id,
            updated_at,
        ),
        StorageTankEmissionSurveyStorageTankId => query!(
            "UPDATE storage_tank_emission_survey
            SET storage_tank_id = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            uuid_value,
            user_id,
            updated_at,
        ),
        StorageTankEmissionSurveyStartDate => query!(
            "UPDATE storage_tank_emission_survey
            SET start_date = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            naive_date_value,
            user_id,
            updated_at,
        ),
        StorageTankEmissionSurveyEndDate => query!(
            "UPDATE storage_tank_emission_survey
            SET end_date = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            naive_date_value,
            user_id,
            updated_at,
        ),
        StorageTankEmissionSurveyRate => query!(
            "UPDATE storage_tank_emission_survey
            SET rate = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            float_value,
            user_id,
            updated_at,
        ),
        StorageTankEmissionSurveySurveyPoint => query!(
            "UPDATE storage_tank_emission_survey
            SET survey_point = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            string_value,
            user_id,
            updated_at,
        ),
        StorageTankEmissionSurveyLeakDuration => query!(
            "UPDATE storage_tank_emission_survey
            SET leak_duration = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            float_value,
            user_id,
            updated_at,
        ),
        StorageTankEmissionSurveySurveyEquipmentId => query!(
            "UPDATE storage_tank_emission_survey
            SET survey_equipment_id = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            uuid_value,
            user_id,
            updated_at,
        ),
        // Storage Tank Controlled Characterization
        StorageTankControlledCharacterizationStorageTankId => query!(
            "UPDATE storage_tank_controlled_characterization
            SET storage_tank_id = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            uuid_value,
            user_id,
            updated_at,
        ),
        StorageTankControlledCharacterizationStartDate => query!(
            "UPDATE storage_tank_controlled_characterization
            SET start_date = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            naive_date_value,
            user_id,
            updated_at,
        ),
        StorageTankControlledCharacterizationEndDate => query!(
            "UPDATE storage_tank_controlled_characterization
            SET end_date = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            naive_date_value,
            user_id,
            updated_at,
        ),
        StorageTankControlledCharacterizationControlDevice => query!(
            "UPDATE storage_tank_controlled_characterization
            SET control_device = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            control_device_value as Option<ControlDevice>,
            user_id,
            updated_at,
        ),
        StorageTankControlledCharacterizationComment => query!(
            "UPDATE storage_tank_controlled_characterization
            SET comment = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            string_value,
            user_id,
            updated_at,
        ),
        // Storage Tank Control Device Inactivity
        StorageTankControlDeviceInactivityStorageTankControlledCharacterizationId => query!(
            "UPDATE storage_tank_control_device_inactivity
            SET storage_tank_controlled_characterization_id = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            uuid_value,
            user_id,
            updated_at,
        ),
        StorageTankControlDeviceInactivityStartDate => query!(
            "UPDATE storage_tank_control_device_inactivity
            SET start_date = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            naive_date_value,
            user_id,
            updated_at,
        ),
        StorageTankControlDeviceInactivityEndDate => query!(
            "UPDATE storage_tank_control_device_inactivity
            SET end_date = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            naive_date_value,
            user_id,
            updated_at,
        ),
        StorageTankControlDeviceInactivityReason => query!(
            "UPDATE storage_tank_control_device_inactivity
            SET reason = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            control_device_inactivity_reason_value as Option<ControlDeviceInactivityReason>,
            user_id,
            updated_at,
        ),
        StorageTankControlDeviceInactivityComment => query!(
            "UPDATE storage_tank_control_device_inactivity
            SET comment = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            string_value,
            user_id,
            updated_at,
        ),
        StorageTankMonthLiquidHydrocarbonEnteringStorageTankId => query!(
            "UPDATE storage_tank_month_liquid_hydrocarbon_entering
            SET storage_tank_id = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            uuid_value,
            user_id,
            updated_at,
        ),
        StorageTankMonthLiquidHydrocarbonEnteringMonth => {
            if let Some(value) = &naive_date_value {
                if value.day() != 1 {
                    return Err(not_first_day_of_month_error(value));
                }
            }
            query!(
                "UPDATE storage_tank_month_liquid_hydrocarbon_entering
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
        StorageTankMonthLiquidHydrocarbonEnteringLiquidHydrocarbonVolume => query!(
            "UPDATE storage_tank_month_liquid_hydrocarbon_entering
            SET liquid_hydrocarbon_volume = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            float_value,
            user_id,
            updated_at,
        ),
        // Storage Tank Month Methane Emission Override
        StorageTankMonthMethaneEmissionOverrideStorageTankId => query!(
            "UPDATE storage_tank_month_methane_emission_override
            SET storage_tank_id = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            uuid_value,
            user_id,
            updated_at,
        ),
        StorageTankMonthMethaneEmissionOverrideMonth => {
            if let Some(value) = &naive_date_value {
                if value.day() != 1 {
                    return Err(not_first_day_of_month_error(value));
                }
            }
            query!(
                "UPDATE storage_tank_month_methane_emission_override
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
        StorageTankMonthMethaneEmissionOverrideGasVolume => query!(
            "UPDATE storage_tank_month_methane_emission_override
            SET gas_volume = $2,
                updated_by_id = $3,
                updated_at = $4
            WHERE id = $1",
            id,
            float_value,
            user_id,
            updated_at,
        ),
        StorageTankMonthMethaneEmissionOverrideComment => query!(
            "UPDATE storage_tank_month_methane_emission_override
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
