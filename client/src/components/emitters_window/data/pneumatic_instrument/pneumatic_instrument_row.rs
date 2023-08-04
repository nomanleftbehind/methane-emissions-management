use crate::{
    models::queries::pneumatic_instrument::get_pneumatic_instruments::GetPneumaticInstrumentsGetPneumaticInstruments,
    utils::row_component::*,
};
use common::{
    SidebarItem::{
        PneumaticInstrumentEmissionRate, PneumaticInstrumentMonthHours,
        PneumaticInstrumentMonthMethaneEmission, PneumaticInstrumentMonthMethaneEmissionOverride,
    },
    UpdateFieldValueEnum::{
        NaiveDateTimeValue, NaiveDateValue, OptionNaiveDateValue, OptionStringValue,
        PneumaticInstrumentTypeValue, UuidValue,
    },
};

row_component!(
component_name: PneumaticInstrumentRowComponent,
id: id,
data_row: pneumatic_instrument,
data_row_type: GetPneumaticInstrumentsGetPneumaticInstruments,
column_start: 3,
delete_entry_variant: DeleteEntryVariant::PNEUMATIC_INSTRUMENT,
data_row_fields: (
        (
            update_field_callback: update_field_callback(UpdateFieldVariant::PNEUMATIC_INSTRUMENT_TYPE),
            dropdown_selection_callback: dropdown_selection_callback(DropdownSelectionVariant::PNEUMATIC_INSTRUMENT_TYPE, dropdown_selection_id_callback()),
            value: (PneumaticInstrumentTypeValue, type_, value_callback()),
            display_value: display_value_callback()
        ),
        (
            update_field_callback: update_field_callback(UpdateFieldVariant::PNEUMATIC_INSTRUMENT_MANUFACTURER_ID),
            dropdown_selection_callback: dropdown_selection_callback(DropdownSelectionVariant::DEVICE_MANUFACTURER_ID, dropdown_selection_id_callback()),
            value: (UuidValue, manufacturer_id, value_callback()),
            display_value: display_value_callback(OptionStringValue, manufacturer, manufacturer)
        ),
        (
            update_field_callback: update_field_callback(UpdateFieldVariant::PNEUMATIC_INSTRUMENT_MODEL),
            dropdown_selection_callback: dropdown_selection_callback(),
            value: (OptionStringValue, model, value_callback()),
            display_value: display_value_callback()
        ),
        (
            update_field_callback: update_field_callback(UpdateFieldVariant::PNEUMATIC_INSTRUMENT_SERIAL_NUMBER),
            dropdown_selection_callback: dropdown_selection_callback(),
            value: (OptionStringValue, serial_number, value_callback()),
            display_value: display_value_callback()
        ),
        (
            update_field_callback: update_field_callback(UpdateFieldVariant::PNEUMATIC_INSTRUMENT_START_DATE),
            dropdown_selection_callback: dropdown_selection_callback(),
            value: (NaiveDateValue, start_date, value_callback()),
            display_value: display_value_callback()
        ),
        (
            update_field_callback: update_field_callback(UpdateFieldVariant::PNEUMATIC_INSTRUMENT_END_DATE),
            dropdown_selection_callback: dropdown_selection_callback(),
            value: (OptionNaiveDateValue, end_date, value_callback()),
            display_value: display_value_callback()
        ),
        (
            update_field_callback: update_field_callback(UpdateFieldVariant::PNEUMATIC_INSTRUMENT_SITE_ID),
            dropdown_selection_callback: dropdown_selection_callback(DropdownSelectionVariant::SITE_ID, dropdown_selection_id_callback()),
            value: (UuidValue, site_id, value_callback()),
            display_value: display_value_callback(OptionStringValue, site, name)
        ),
        (
            update_field_callback: update_field_callback(),
            dropdown_selection_callback: dropdown_selection_callback(),
            value: (OptionStringValue, created_by, value_callback(email)),
            display_value: display_value_callback()
        ),
        (
            update_field_callback: update_field_callback(),
            dropdown_selection_callback: dropdown_selection_callback(),
            value: (NaiveDateTimeValue, created_at, value_callback()),
            display_value: display_value_callback()
        ),
        (
            update_field_callback: update_field_callback(),
            dropdown_selection_callback: dropdown_selection_callback(),
            value: (OptionStringValue, updated_by, value_callback(email)),
            display_value: display_value_callback()
        ),
        (
            update_field_callback: update_field_callback(),
            dropdown_selection_callback: dropdown_selection_callback(),
            value: (NaiveDateTimeValue, updated_at, value_callback()),
            display_value: display_value_callback()
        ),
        (
            update_field_callback: update_field_callback(),
            dropdown_selection_callback: dropdown_selection_callback(),
            value: (UuidValue, id, value_callback()),
            display_value: display_value_callback()
        ),
    )
);
