use crate::{
    components::emitters_window::{
        delete_entry::DeleteEntryComponent,
        entry::{EditFieldProp, Entry},
    },
    models::{
        mutations::manual_mutation::{
            delete_entry::{
                DeleteEntryVariant::CONTROLLER_CHANGE, Variables as VariablesDeleteEntry,
            },
            update_field::{
                UpdateFieldVariant::{CONTROLLER_CHANGE_DATE, CONTROLLER_CHANGE_RATE},
                Variables as VariablesUpdateField,
            },
        },
        queries::controller_change::get_controller_changes::GetControllerChangesGetControllerChanges,
    },
};
use common::UpdateFieldValueEnum::{
    FloatValue, NaiveDateTimeValue, NaiveDateValue, OptionStringValue, UuidValue,
};
use yew::{function_component, html, Callback, Html, Properties};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub row_num: usize,
    pub controller_change: GetControllerChangesGetControllerChanges,
    pub handle_update_field: Callback<VariablesUpdateField>,
    pub handle_delete_entry: Callback<VariablesDeleteEntry>,
}

#[function_component(ControllerChangeRowComponent)]
pub fn controller_change_row(
    Props {
        row_num,
        controller_change,
        handle_update_field,
        handle_delete_entry,
    }: &Props,
) -> Html {
    let controller_change = controller_change.clone();
    let id = controller_change.id;
    let created_by = controller_change.created_by.map(|cb| cb.email);
    let updated_by = controller_change.updated_by.map(|ub| ub.email);

    html! {
        <>
            <DeleteEntryComponent {id} {row_num} col_num={1} delete_entry_variant={CONTROLLER_CHANGE} handle_delete_entry={handle_delete_entry.clone()} />
            <Entry {id} {row_num} col_num={2} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: CONTROLLER_CHANGE_DATE}} value={NaiveDateValue(controller_change.date)} />
            <Entry {id} {row_num} col_num={3} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: CONTROLLER_CHANGE_RATE}} value={FloatValue(controller_change.rate)} />
            <Entry {id} {row_num} col_num={4} value={OptionStringValue(created_by)} />
            <Entry {id} {row_num} col_num={5} value={NaiveDateTimeValue(controller_change.created_at)} />
            <Entry {id} {row_num} col_num={6} value={OptionStringValue(updated_by)} />
            <Entry {id} {row_num} col_num={7} value={NaiveDateTimeValue(controller_change.updated_at)} />
            <Entry {id} {row_num} col_num={8} value={UuidValue(id)} />
        </>
    }
}
