use crate::{
    components::emitters_window::{
        data::entry::{EditFieldProp, Entry, IdSelectionProp},
        delete_entry::DeleteEntryComponent,
        expand_data::ExpandDataComponent,
    },
    models::{
        mutations::manual_mutation::{
            delete_entry::Variables as VariablesDeleteEntry,
            update_field::Variables as VariablesUpdateField,
        },
        queries::{
            id_selection::id_selection::IdSelectionVariant,
            pneumatic_instrument::get_pneumatic_instruments::GetPneumaticInstrumentsGetPneumaticInstruments,
        },
    },
    pages::ModalVariant,
};

use common::{
    DeleteEntryVariant,
    // IdSelectionVariant,
    UpdateFieldValueEnum::{
        NaiveDateTimeValue, NaiveDateValue, OptionNaiveDateValue, OptionStringValue,
        PneumaticInstrumentTypeValue, UuidValue,
    },
    UpdateFieldVariant,
};
use yew::{function_component, html, use_state_eq, Callback, Html, Properties};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub row_num: usize,
    pub pneumatic_instrument: GetPneumaticInstrumentsGetPneumaticInstruments,
    pub handle_update_field: Callback<VariablesUpdateField>,
    pub handle_delete_entry: Callback<VariablesDeleteEntry>,
    pub modal_variant_handle: Callback<Option<ModalVariant>>,
}

#[function_component(PneumaticInstrumentRowComponent)]
pub fn pneumatic_instrument_row_component(
    Props {
        row_num,
        pneumatic_instrument,
        handle_update_field,
        handle_delete_entry,
        modal_variant_handle,
    }: &Props,
) -> Html {
    let expanded_handle = use_state_eq(|| false);
    let expanded = *expanded_handle;

    let handle_expand_data = Callback::from(move |_| {
        expanded_handle.set(!expanded);
    });

    let pneumatic_instrument = pneumatic_instrument.clone();
    let id = pneumatic_instrument.id;
    let manufacturer = pneumatic_instrument.manufacturer.map(|m| m.manufacturer);
    let created_by = pneumatic_instrument.created_by.map(|cb| cb.email);
    let updated_by = pneumatic_instrument.updated_by.map(|ub| ub.email);

    // let sidebar_items = vec![
    //     CONTROLLER_CHANGE_BY_CONTROLLER_ID,
    //     CONTROLLER_MONTH_HOURS_BY_CONTROLLER_ID,
    //     CONTROLLER_MONTH_VENT_OVERRIDE_BY_CONTROLLER_ID,
    //     CONTROLLER_MONTH_VENT_BY_CONTROLLER_ID,
    // ];

    html! {
        <>
            <DeleteEntryComponent {id} {row_num} col_num={1} delete_entry_variant={DeleteEntryVariant::PneumaticInstrument} handle_delete_entry={handle_delete_entry.clone()} />
            <ExpandDataComponent {row_num} col_num={2} {expanded} {handle_expand_data} />
            <Entry {id} {row_num} col_num={6}
            edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: UpdateFieldVariant::PneumaticInstrumentType}}
            // id_selection={IdSelectionProp {variant: IdSelectionVariant::CONTROLLER_APPLICATION_ID, modal_variant_handle: modal_variant_handle.clone()}}
            value={PneumaticInstrumentTypeValue(pneumatic_instrument.type_)}
            />
            <Entry {id} {row_num} col_num={5}
            edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: UpdateFieldVariant::PneumaticInstrumentManufacturerId}}
            id_selection={IdSelectionProp {variant: IdSelectionVariant::DEVICE_MANUFACTURER_ID, modal_variant_handle: modal_variant_handle.clone()}}
            value={UuidValue(pneumatic_instrument.manufacturer_id)}
            display_value={OptionStringValue(manufacturer)}
            />
            <Entry {id} {row_num} col_num={3} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: UpdateFieldVariant::PneumaticInstrumentModel}} value={OptionStringValue(pneumatic_instrument.model)} />
            <Entry {id} {row_num} col_num={4} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: UpdateFieldVariant::PneumaticInstrumentSerialNumber}} value={OptionStringValue(pneumatic_instrument.serial_number)} />
            <Entry {id} {row_num} col_num={7} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: UpdateFieldVariant::PneumaticInstrumentStartDate}} value={NaiveDateValue(pneumatic_instrument.start_date)} />
            <Entry {id} {row_num} col_num={7} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: UpdateFieldVariant::PneumaticInstrumentEndDate}} value={OptionNaiveDateValue(pneumatic_instrument.end_date)} />
            <Entry {id} {row_num} col_num={8} value={OptionStringValue(created_by)} />
            <Entry {id} {row_num} col_num={9} value={NaiveDateTimeValue(pneumatic_instrument.created_at)} />
            <Entry {id} {row_num} col_num={10} value={OptionStringValue(updated_by)} />
            <Entry {id} {row_num} col_num={11} value={NaiveDateTimeValue(pneumatic_instrument.updated_at)} />
            <Entry {id} {row_num} col_num={12} value={UuidValue(id)} />
            // if expanded {
            //     <ObjectDataComponent {id} {sidebar_items} {modal_variant_handle} row_num={row_num + 1} col_num={12} />
            // }
        </>
    }
}
