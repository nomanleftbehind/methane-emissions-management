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
            update_field_callback: (UpdateFieldVariant::PNEUMATIC_INSTRUMENT_TYPE),
            dropdown_selection_callback: (DropdownSelectionVariant::PNEUMATIC_INSTRUMENT_TYPE, ()),
            value: (PneumaticInstrumentTypeValue, type_, ()),
            display_value: ()
        ),
        (
            update_field_callback: (UpdateFieldVariant::PNEUMATIC_INSTRUMENT_MANUFACTURER_ID),
            dropdown_selection_callback: (DropdownSelectionVariant::DEVICE_MANUFACTURER_ID, ()),
            value: (UuidValue, manufacturer_id, ()),
            display_value: (OptionStringValue, manufacturer, manufacturer)
        ),
        (
            update_field_callback: (UpdateFieldVariant::PNEUMATIC_INSTRUMENT_MODEL),
            dropdown_selection_callback: (),
            value: (OptionStringValue, model, ()),
            display_value: ()
        ),
        (
            update_field_callback: (UpdateFieldVariant::PNEUMATIC_INSTRUMENT_SERIAL_NUMBER),
            dropdown_selection_callback: (),
            value: (OptionStringValue, serial_number, ()),
            display_value: ()
        ),
        (
            update_field_callback: (UpdateFieldVariant::PNEUMATIC_INSTRUMENT_START_DATE),
            dropdown_selection_callback: (),
            value: (NaiveDateValue, start_date, ()),
            display_value: ()
        ),
        (
            update_field_callback: (UpdateFieldVariant::PNEUMATIC_INSTRUMENT_END_DATE),
            dropdown_selection_callback: (),
            value: (OptionNaiveDateValue, end_date, ()),
            display_value: ()
        ),
        (
            update_field_callback: (UpdateFieldVariant::PNEUMATIC_INSTRUMENT_SITE_ID),
            dropdown_selection_callback: (DropdownSelectionVariant::SITE_ID, ()),
            value: (UuidValue, site_id, ()),
            display_value: (OptionStringValue, site, name)
        ),
        (
            update_field_callback: (),
            dropdown_selection_callback: (),
            value: (OptionStringValue, created_by, (email)),
            display_value: ()
        ),
        (
            update_field_callback: (),
            dropdown_selection_callback: (),
            value: (NaiveDateTimeValue, created_at, ()),
            display_value: ()
        ),
        (
            update_field_callback: (),
            dropdown_selection_callback: (),
            value: (OptionStringValue, updated_by, (email)),
            display_value: ()
        ),
        (
            update_field_callback: (),
            dropdown_selection_callback: (),
            value: (NaiveDateTimeValue, updated_at, ()),
            display_value: ()
        ),
        (
            update_field_callback: (),
            dropdown_selection_callback: (),
            value: (UuidValue, id, ()),
            display_value: ()
        ),
    )
);
