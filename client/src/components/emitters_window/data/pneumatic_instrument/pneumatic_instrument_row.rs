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
data_row_fields: (
        (
            field: type_,
            edit_field_callback: edit_field_callback(UpdateFieldVariant::PNEUMATIC_INSTRUMENT_TYPE),
            dropdown_selection_callback: dropdown_selection_callback(DropdownSelectionVariant::PNEUMATIC_INSTRUMENT_TYPE, dropdown_selection_id_callback, ),
            value: PneumaticInstrumentTypeValue,
            display_value: display_value_callback()
        ),
        (
            field: manufacturer_id,
            edit_field_callback: edit_field_callback(UpdateFieldVariant::PNEUMATIC_INSTRUMENT_MANUFACTURER_ID),
            dropdown_selection_callback: dropdown_selection_callback(DropdownSelectionVariant::DEVICE_MANUFACTURER_ID, dropdown_selection_id_callback, site_id),
            value: UuidValue,
            display_value: display_value_callback(OptionStringValue, manufacturer, manufacturer)
        ),
        (
            field: model,
            edit_field_callback: edit_field_callback(UpdateFieldVariant::PNEUMATIC_INSTRUMENT_MODEL),
            dropdown_selection_callback: dropdown_selection_callback(),
            value: OptionStringValue,
            display_value: display_value_callback()
        ),
        (
            field: serial_number,
            edit_field_callback: edit_field_callback(UpdateFieldVariant::PNEUMATIC_INSTRUMENT_SERIAL_NUMBER),
            dropdown_selection_callback: dropdown_selection_callback(),
            value: OptionStringValue,
            display_value: display_value_callback()
        ),
    )
);

// <Entry {id} {row_num} col_num={3}
// edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: UpdateFieldVariant::PNEUMATIC_INSTRUMENT_TYPE}}
// dropdown_selection={DropdownSelectionProp {variant: DropdownSelectionVariant::PNEUMATIC_INSTRUMENT_TYPE, id: None, modal_variant_handle: modal_variant_handle.clone()}}
// value={PneumaticInstrumentTypeValue(pneumatic_instrument.type_)}
// />
// <Entry {id} {row_num} col_num={4}
// edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: UpdateFieldVariant::PNEUMATIC_INSTRUMENT_MANUFACTURER_ID}}
// dropdown_selection={DropdownSelectionProp {variant: DropdownSelectionVariant::DEVICE_MANUFACTURER_ID, id: None, modal_variant_handle: modal_variant_handle.clone()}}
// value={UuidValue(pneumatic_instrument.manufacturer_id)}
// display_value={OptionStringValue(manufacturer)}
// />
// <Entry {id} {row_num} col_num={5} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: UpdateFieldVariant::PNEUMATIC_INSTRUMENT_MODEL}} value={OptionStringValue(pneumatic_instrument.model)} />
// <Entry {id} {row_num} col_num={6} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: UpdateFieldVariant::PNEUMATIC_INSTRUMENT_SERIAL_NUMBER}} value={OptionStringValue(pneumatic_instrument.serial_number)} />
// <Entry {id} {row_num} col_num={7} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: UpdateFieldVariant::PNEUMATIC_INSTRUMENT_START_DATE}} value={NaiveDateValue(pneumatic_instrument.start_date)} />
// <Entry {id} {row_num} col_num={8} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: UpdateFieldVariant::PNEUMATIC_INSTRUMENT_END_DATE}} value={OptionNaiveDateValue(pneumatic_instrument.end_date)} />
// <Entry {id} {row_num} col_num={9}
// edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: UpdateFieldVariant::PNEUMATIC_INSTRUMENT_SITE_ID}}
// dropdown_selection={DropdownSelectionProp {variant: DropdownSelectionVariant::SITE_ID, id: None, modal_variant_handle: modal_variant_handle.clone()}}
// value={UuidValue(pneumatic_instrument.site_id)}
// display_value={OptionStringValue(site)}
// />
// <Entry {id} {row_num} col_num={10} value={OptionStringValue(created_by)} />
// <Entry {id} {row_num} col_num={11} value={NaiveDateTimeValue(pneumatic_instrument.created_at)} />
// <Entry {id} {row_num} col_num={12} value={OptionStringValue(updated_by)} />
// <Entry {id} {row_num} col_num={13} value={NaiveDateTimeValue(pneumatic_instrument.updated_at)} />
// <Entry {id} {row_num} col_num={14} value={UuidValue(id)} />
