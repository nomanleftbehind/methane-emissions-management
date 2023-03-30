use crate::{
    components::emitters_window::{
        controller_data::ControllerDataComponent,
        delete_entry::DeleteEntryComponent,
        entry::{EditFieldProp, Entry},
        expand_data::ExpandDataComponent,
    },
    models::{
        mutations::manual_mutation::{
            delete_entry::{DeleteEntryVariant::CONTROLLER, Variables as VariablesDeleteEntry},
            update_field::{
                UpdateFieldVariant::{
                    CONTROLLER_APPLICATION_ID, CONTROLLER_FDC_REC_ID, CONTROLLER_MANUFACTURER_ID,
                    CONTROLLER_MODEL, CONTROLLER_SERIAL_NUMBER,
                },
                Variables as VariablesUpdateField,
            },
        },
        queries::controller::get_controllers::GetControllersControllersBy,
    },
};
use common::UpdateFieldValueEnum::{
    NaiveDateTimeValue, OptionStringValue, OptionUuidValue, StringValue, UuidValue,
};
use yew::{function_component, html, use_state_eq, Callback, Html, Properties};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub row_num: usize,
    pub controller: GetControllersControllersBy,
    pub handle_update_field: Callback<VariablesUpdateField>,
    pub handle_delete_entry: Callback<VariablesDeleteEntry>,
}

#[function_component(ControllerRowComponent)]
pub fn controller_row(
    Props {
        row_num,
        handle_update_field,
        handle_delete_entry,
        controller,
    }: &Props,
) -> Html {
    let expanded_handle = use_state_eq(|| false);
    let expanded = *expanded_handle;

    let handle_expand_data = Callback::from(move |_| {
        expanded_handle.set(!expanded);
    });

    let controller = controller.clone();
    let id = controller.id;
    let manufacturer = controller.manufacturer.map(|m| m.manufacturer);
    let application = controller.application.map(|a| a.application);
    let created_by = controller.created_by.map(|cb| cb.email);
    let updated_by = controller.updated_by.map(|ub| ub.email);

    html! {
        <>
            <DeleteEntryComponent {id} {row_num} col_num={1} delete_entry_variant={CONTROLLER} handle_delete_entry={handle_delete_entry.clone()} />
            <ExpandDataComponent {row_num} col_num={2} {expanded} {handle_expand_data} />
            <Entry {id} {row_num} col_num={3} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: CONTROLLER_MODEL}} value={OptionStringValue(controller.model)} />
            <Entry {id} {row_num} col_num={4} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: CONTROLLER_SERIAL_NUMBER}} value={OptionStringValue(controller.serial_number)} />
            <Entry {id} {row_num} col_num={5} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: CONTROLLER_MANUFACTURER_ID}} value={UuidValue(controller.manufacturer_id)} display_value={OptionStringValue(manufacturer)} />
            <Entry {id} {row_num} col_num={6} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: CONTROLLER_APPLICATION_ID}} value={OptionUuidValue(controller.application_id)} display_value={OptionStringValue(application)} />
            <Entry {id} {row_num} col_num={7} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: CONTROLLER_FDC_REC_ID}} value={StringValue(controller.fdc_rec_id)} />
            <Entry {id} {row_num} col_num={8} value={OptionStringValue(created_by)} />
            <Entry {id} {row_num} col_num={9} value={NaiveDateTimeValue(controller.created_at)} />
            <Entry {id} {row_num} col_num={10} value={OptionStringValue(updated_by)} />
            <Entry {id} {row_num} col_num={11} value={NaiveDateTimeValue(controller.updated_at)} />
            <Entry {id} {row_num} col_num={12} value={UuidValue(id)} />
            if expanded {
                <ControllerDataComponent {id} row_num={row_num + 1} col_num={12} />
            }
        </>
    }
}
