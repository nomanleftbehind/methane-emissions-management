use crate::graphql::models::input::DeleteEntryInput;
use common::DeleteEntryVariant::{
    Compressor, CompressorBlowdown, CompressorBlowdownOverride, CompressorControlDeviceInactivity,
    CompressorControlledCharacterization, CompressorEmissionSurvey, CompressorMonthHours,
    CompressorSeal, CompressorSealMonthMethaneEmissionOverride, CompressorSealTest,
    DeviceManufacturer, Facility, GasAnalysis, LevelController, LevelControllerActuationFrequency,
    LevelControllerChange, LevelControllerControlDeviceInactivity,
    LevelControllerControlledCharacterization, LevelControllerMonthHours,
    LevelControllerMonthMethaneEmissionOverride, PneumaticInstrument, PneumaticInstrumentChange,
    PneumaticInstrumentControlDeviceInactivity, PneumaticInstrumentControlledCharacterization,
    PneumaticInstrumentMonthHours, PneumaticInstrumentMonthMethaneEmissionOverride, PneumaticPump,
    PneumaticPumpChange, PneumaticPumpControlDeviceInactivity,
    PneumaticPumpControlledCharacterization, PneumaticPumpMonthHours,
    PneumaticPumpMonthMethaneEmissionOverride, Site, StorageTank, StorageTankChange,
    StorageTankControlDeviceInactivity, StorageTankControlledCharacterization,
    StorageTankEmissionSurvey, StorageTankGasInSolutionFactorCalculated,
    StorageTankMonthLiquidHydrocarbonEntering, StorageTankMonthMethaneEmissionOverride,
    SurveyEquipment,
};
use sqlx::{query, Error, PgPool};

pub async fn delete_entry(
    pool: &PgPool,
    DeleteEntryInput {
        id,
        delete_entry_variant,
    }: DeleteEntryInput,
) -> Result<u64, Error> {
    let query = match delete_entry_variant {
        Facility => query!("DELETE FROM facility WHERE id = $1", id,),
        Site => query!("DELETE FROM site WHERE id = $1", id,),
        DeviceManufacturer => query!("DELETE FROM device_manufacturer WHERE id = $1", id,),
        SurveyEquipment => query!("DELETE FROM survey_equipment WHERE id = $1", id,),
        GasAnalysis => query!("DELETE FROM gas_analysis WHERE id = $1", id,),
        // Pneumatic Instrument
        PneumaticInstrument => query!("DELETE FROM pneumatic_instrument WHERE id = $1", id,),
        PneumaticInstrumentChange => {
            query!("DELETE FROM pneumatic_instrument_change WHERE id = $1", id,)
        }
        PneumaticInstrumentControlledCharacterization => query!(
            "DELETE FROM pneumatic_instrument_controlled_characterization WHERE id = $1",
            id,
        ),
        PneumaticInstrumentControlDeviceInactivity => query!(
            "DELETE FROM pneumatic_instrument_control_device_inactivity WHERE id = $1",
            id,
        ),
        PneumaticInstrumentMonthHours => {
            query!(
                "DELETE FROM pneumatic_instrument_month_hours WHERE id = $1",
                id,
            )
        }
        PneumaticInstrumentMonthMethaneEmissionOverride => query!(
            "DELETE FROM pneumatic_instrument_month_methane_emission_override WHERE id = $1",
            id,
        ),
        // Level Controller
        LevelController => query!("DELETE FROM level_controller WHERE id = $1", id,),
        LevelControllerActuationFrequency => {
            query!(
                "DELETE FROM level_controller_actuation_frequency WHERE id = $1",
                id,
            )
        }
        LevelControllerChange => {
            query!("DELETE FROM level_controller_change WHERE id = $1", id,)
        }
        LevelControllerControlledCharacterization => query!(
            "DELETE FROM level_controller_controlled_characterization WHERE id = $1",
            id,
        ),
        LevelControllerControlDeviceInactivity => query!(
            "DELETE FROM level_controller_control_device_inactivity WHERE id = $1",
            id,
        ),
        LevelControllerMonthHours => {
            query!("DELETE FROM level_controller_month_hours WHERE id = $1", id,)
        }
        LevelControllerMonthMethaneEmissionOverride => query!(
            "DELETE FROM level_controller_month_methane_emission_override WHERE id = $1",
            id,
        ),
        // Pneumatic Pump
        PneumaticPump => query!("DELETE FROM pneumatic_pump WHERE id = $1", id,),
        PneumaticPumpChange => {
            query!("DELETE FROM pneumatic_pump_change WHERE id = $1", id,)
        }
        PneumaticPumpControlledCharacterization => query!(
            "DELETE FROM pneumatic_pump_controlled_characterization WHERE id = $1",
            id,
        ),
        PneumaticPumpControlDeviceInactivity => query!(
            "DELETE FROM pneumatic_pump_control_device_inactivity WHERE id = $1",
            id,
        ),
        PneumaticPumpMonthHours => {
            query!("DELETE FROM pneumatic_pump_month_hours WHERE id = $1", id,)
        }
        PneumaticPumpMonthMethaneEmissionOverride => query!(
            "DELETE FROM pneumatic_pump_month_methane_emission_override WHERE id = $1",
            id,
        ),
        // Compressor Seal
        Compressor => query!("DELETE FROM compressor WHERE id = $1", id,),
        CompressorSeal => query!("DELETE FROM compressor_seal WHERE id = $1", id,),
        CompressorSealTest => query!("DELETE FROM compressor_seal_test WHERE id = $1", id,),
        CompressorEmissionSurvey => {
            query!("DELETE FROM compressor_emission_survey WHERE id = $1", id,)
        }
        CompressorControlledCharacterization => query!(
            "DELETE FROM compressor_controlled_characterization WHERE id = $1",
            id,
        ),
        CompressorControlDeviceInactivity => query!(
            "DELETE FROM compressor_control_device_inactivity WHERE id = $1",
            id,
        ),
        CompressorMonthHours => query!("DELETE FROM compressor_month_hours WHERE id = $1", id,),
        CompressorSealMonthMethaneEmissionOverride => query!(
            "DELETE FROM compressor_seal_month_methane_emission_override WHERE id = $1",
            id,
        ),
        CompressorBlowdown => query!("DELETE FROM compressor_blowdown WHERE id = $1", id,),
        CompressorBlowdownOverride => {
            query!("DELETE FROM compressor_blowdown_override WHERE id = $1", id,)
        }
        // Storage Tank
        StorageTank => query!("DELETE FROM storage_tank WHERE id = $1", id,),
        StorageTankChange => query!("DELETE FROM storage_tank_change WHERE id = $1", id,),
        StorageTankEmissionSurvey => {
            query!("DELETE FROM storage_tank_emission_survey WHERE id = $1", id,)
        }
        StorageTankControlledCharacterization => query!(
            "DELETE FROM storage_tank_controlled_characterization WHERE id = $1",
            id,
        ),
        StorageTankControlDeviceInactivity => query!(
            "DELETE FROM storage_tank_control_device_inactivity WHERE id = $1",
            id,
        ),
        StorageTankGasInSolutionFactorCalculated => query!(
            "DELETE FROM storage_tank_gas_in_solution_factor_calculated WHERE id = $1",
            id,
        ),
        StorageTankMonthLiquidHydrocarbonEntering => query!(
            "DELETE FROM storage_tank_month_liquid_hydrocarbon_entering WHERE id = $1",
            id,
        ),
        StorageTankMonthMethaneEmissionOverride => query!(
            "DELETE FROM storage_tank_month_methane_emission_override WHERE id = $1",
            id,
        ),
    };

    Ok(query.execute(pool).await?.rows_affected())
}
