use crate::{
    components::emitters_window::{
        data::entry::{EditFieldProp, Entry},
        delete_entry::DeleteEntryComponent,
    },
    models::{
        mutations::manual_mutation::{
            delete_entry::{DeleteEntryVariant, Variables as VariablesDeleteEntry},
            update_field::{UpdateFieldVariant, Variables as VariablesUpdateField},
        },
        queries::pneumatic_instrument::get_pneumatic_instrument_changes::GetPneumaticInstrumentChangesGetPneumaticInstrumentChanges,
    },
};
use common::UpdateFieldValueEnum::{
    FloatValue, NaiveDateTimeValue, NaiveDateValue, OptionStringValue, StringValue, UuidValue,
};
use yew::{function_component, html, Callback, Html, Properties};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub row_num: usize,
    pub pneumatic_instrument_change: GetPneumaticInstrumentChangesGetPneumaticInstrumentChanges,
    pub handle_update_field: Callback<VariablesUpdateField>,
    pub handle_delete_entry: Callback<VariablesDeleteEntry>,
}

#[function_component(PneumaticInstrumentChangeRowComponent)]
pub fn pneumatic_instrument_change_row_component(
    Props {
        row_num,
        pneumatic_instrument_change,
        handle_update_field,
        handle_delete_entry,
    }: &Props,
) -> Html {
    let pneumatic_instrument_change = pneumatic_instrument_change.clone();
    let id = pneumatic_instrument_change.id;
    let created_by = pneumatic_instrument_change.created_by.map(|cb| cb.email);
    let updated_by = pneumatic_instrument_change.updated_by.map(|ub| ub.email);

    html! {
        <>
            <DeleteEntryComponent {id} {row_num} col_num={1} delete_entry_variant={DeleteEntryVariant::PNEUMATIC_INSTRUMENT_CHANGE} handle_delete_entry={handle_delete_entry.clone()} />
            <Entry {id} {row_num} col_num={2} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: UpdateFieldVariant::PNEUMATIC_INSTRUMENT_CHANGE_DATE}} value={NaiveDateValue(pneumatic_instrument_change.date)} />
            <Entry {id} {row_num} col_num={3} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: UpdateFieldVariant::PNEUMATIC_INSTRUMENT_CHANGE_RATE}} value={FloatValue(pneumatic_instrument_change.rate)} display_value={StringValue(format!("{:.4}", pneumatic_instrument_change.rate))} />
            <Entry {id} {row_num} col_num={4} value={OptionStringValue(created_by)} />
            <Entry {id} {row_num} col_num={5} value={NaiveDateTimeValue(pneumatic_instrument_change.created_at)} />
            <Entry {id} {row_num} col_num={6} value={OptionStringValue(updated_by)} />
            <Entry {id} {row_num} col_num={7} value={NaiveDateTimeValue(pneumatic_instrument_change.updated_at)} />
            <Entry {id} {row_num} col_num={8} value={UuidValue(id)} />
        </>
    }
}
