use crate::{
    components::{
        emitters_window::{
            data::object_data::ObjectDataComponent,
            delete_entry::DeleteEntryComponent,
            entry::{EditFieldProp, Entry},
            expand_data::ExpandDataComponent,
        },
        modal::modal::Modal,
    },
    models::{
        mutations::manual_mutation::{
            delete_entry::{DeleteEntryVariant, Variables as VariablesDeleteEntry},
            update_field::{UpdateFieldVariant, Variables as VariablesUpdateField},
        },
        queries::get_object::get_object::{
            GetObjectGetObjectCompressors, GetObjectGetObjectControllerChanges,
            GetObjectGetObjectControllers, GetObjectGetObjectTankFarms,
        },
    },
    utils::console_log,
};
use common::UpdateFieldValueEnum::{
    FloatValue, NaiveDateTimeValue, NaiveDateValue, OptionNaiveDateValue, OptionStringValue,
    OptionUuidValue, StringValue, UuidValue,
};
use yew::{function_component, html, use_state_eq, Callback, Html, Properties};

#[derive(Debug, Clone, PartialEq)]
pub enum ObjectDataProp {
    Controller(GetObjectGetObjectControllers),
    Compressor(GetObjectGetObjectCompressors),
    TankFarm(GetObjectGetObjectTankFarms),
    ControllerChange(GetObjectGetObjectControllerChanges),
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub row_num: usize,
    pub object_data: ObjectDataProp,
    pub handle_update_field: Callback<VariablesUpdateField>,
    pub handle_delete_entry: Callback<VariablesDeleteEntry>,
}

#[function_component(ObjectRowComponent)]
pub fn object_row_component(
    Props {
        row_num,
        object_data,
        handle_update_field,
        handle_delete_entry,
    }: &Props,
) -> Html {
    let expanded_handle = use_state_eq(|| false);
    let expanded = *expanded_handle;

    let handle_expand_data = Callback::from(move |_| {
        expanded_handle.set(!expanded);
    });

    let view = match object_data {
        ObjectDataProp::Controller(controller) => {
            let controller = controller.clone();
            let id = controller.id;
            let manufacturer = controller.manufacturer.map(|m| m.manufacturer);
            let application = controller.application.map(|a| a.application);
            let created_by = controller.created_by.map(|cb| cb.email);
            let updated_by = controller.updated_by.map(|ub| ub.email);

            html! {
                <>
                    if expanded {
                        <Modal>
                            <div>{ "Bitch" }</div>
                        </Modal>
                    }
                    <DeleteEntryComponent {id} {row_num} col_num={1} delete_entry_variant={DeleteEntryVariant::CONTROLLER} handle_delete_entry={handle_delete_entry.clone()} />
                    <ExpandDataComponent {row_num} col_num={2} {expanded} {handle_expand_data} />
                    <Entry {id} {row_num} col_num={3} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: UpdateFieldVariant::CONTROLLER_MODEL}} value={OptionStringValue(controller.model)} />
                    <Entry {id} {row_num} col_num={4} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: UpdateFieldVariant::CONTROLLER_SERIAL_NUMBER}} value={OptionStringValue(controller.serial_number)} />
                    <Entry {id} {row_num} col_num={5} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: UpdateFieldVariant::CONTROLLER_MANUFACTURER_ID}} value={UuidValue(controller.manufacturer_id)} display_value={OptionStringValue(manufacturer)} />
                    <Entry {id} {row_num} col_num={6} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: UpdateFieldVariant::CONTROLLER_APPLICATION_ID}} value={OptionUuidValue(controller.application_id)} display_value={OptionStringValue(application)} />
                    <Entry {id} {row_num} col_num={7} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: UpdateFieldVariant::CONTROLLER_FDC_REC_ID}} value={StringValue(controller.fdc_rec_id)} />
                    <Entry {id} {row_num} col_num={8} value={OptionStringValue(created_by)} />
                    <Entry {id} {row_num} col_num={9} value={NaiveDateTimeValue(controller.created_at)} />
                    <Entry {id} {row_num} col_num={10} value={OptionStringValue(updated_by)} />
                    <Entry {id} {row_num} col_num={11} value={NaiveDateTimeValue(controller.updated_at)} />
                    <Entry {id} {row_num} col_num={12} value={UuidValue(id)} />
                    if expanded {
                        <ObjectDataComponent {id} row_num={row_num + 1} col_num={12} />
                    }
                </>
            }
        }
        ObjectDataProp::Compressor(compressor) => {
            let compressor = compressor.clone();
            let id = compressor.id;
            let created_by = compressor.created_by.map(|cb| cb.email);
            let updated_by = compressor.updated_by.map(|ub| ub.email);

            html! {
                <>
                    <DeleteEntryComponent {id} col_num={1} {row_num} delete_entry_variant={DeleteEntryVariant::COMPRESSOR} handle_delete_entry={handle_delete_entry.clone()} />
                    <ExpandDataComponent {row_num} col_num={2} {expanded} {handle_expand_data} />
                    <Entry {id} col_num={3} {row_num} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: UpdateFieldVariant::COMPRESSOR_NAME}} value={StringValue(compressor.name)} />
                    <Entry {id} col_num={4} {row_num} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: UpdateFieldVariant::COMPRESSOR_SERIAL_NUMBER}} value={StringValue(compressor.serial_number)} />
                    <Entry {id} col_num={5} {row_num} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: UpdateFieldVariant::COMPRESSOR_INSTALL_DATE}} value={NaiveDateValue(compressor.install_date)} />
                    <Entry {id} col_num={6} {row_num} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: UpdateFieldVariant::COMPRESSOR_REMOVE_DATE}} value={OptionNaiveDateValue(compressor.remove_date)} />
                    <Entry {id} col_num={7} {row_num} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: UpdateFieldVariant::COMPRESSOR_FDC_REC_ID}} value={StringValue(compressor.fdc_rec_id)} />
                    <Entry {id} col_num={8} {row_num} value={OptionStringValue(created_by)} />
                    <Entry {id} col_num={9} {row_num} value={NaiveDateTimeValue(compressor.created_at)} />
                    <Entry {id} col_num={10} {row_num} value={OptionStringValue(updated_by)} />
                    <Entry {id} col_num={11} {row_num} value={NaiveDateTimeValue(compressor.updated_at)} />
                    <Entry {id} col_num={12} {row_num} value={UuidValue(id)} />
                </>
            }
        }
        ObjectDataProp::TankFarm(tank_farm) => {
            let tank_farm = tank_farm.clone();
            let id = tank_farm.id;
            let created_by = tank_farm.created_by.map(|cb| cb.email);
            let updated_by = tank_farm.updated_by.map(|ub| ub.email);

            html! {
                <>
                    <DeleteEntryComponent {id} col_num={1} {row_num} delete_entry_variant={DeleteEntryVariant::TANK_FARM} handle_delete_entry={handle_delete_entry.clone()} />
                    <ExpandDataComponent {row_num} col_num={2} {expanded} {handle_expand_data} />
                    <Entry {id} col_num={3} {row_num} value={UuidValue(id)} />
                    <Entry {id} col_num={4} {row_num} value={OptionStringValue(created_by)} />
                    <Entry {id} col_num={5} {row_num} value={NaiveDateTimeValue(tank_farm.created_at)} />
                    <Entry {id} col_num={6} {row_num} value={OptionStringValue(updated_by)} />
                    <Entry {id} col_num={7} {row_num} value={NaiveDateTimeValue(tank_farm.updated_at)} />
                </>
            }
        }
        ObjectDataProp::ControllerChange(controller_change) => {
            let controller_change = controller_change.clone();
            let id = controller_change.id;
            let created_by = controller_change.created_by.map(|cb| cb.email);
            let updated_by = controller_change.updated_by.map(|ub| ub.email);

            html! {
                <>
                    <DeleteEntryComponent {id} {row_num} col_num={1} delete_entry_variant={DeleteEntryVariant::CONTROLLER_CHANGE} handle_delete_entry={handle_delete_entry.clone()} />
                    <Entry {id} {row_num} col_num={2} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: UpdateFieldVariant::CONTROLLER_CHANGE_DATE}} value={NaiveDateValue(controller_change.date)} />
                    <Entry {id} {row_num} col_num={3} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: UpdateFieldVariant::CONTROLLER_CHANGE_RATE}} value={FloatValue(controller_change.rate)} display_value={StringValue(format!("{:.4}", controller_change.rate))}/>
                    <Entry {id} {row_num} col_num={4} value={OptionStringValue(created_by)} />
                    <Entry {id} {row_num} col_num={5} value={NaiveDateTimeValue(controller_change.created_at)} />
                    <Entry {id} {row_num} col_num={6} value={OptionStringValue(updated_by)} />
                    <Entry {id} {row_num} col_num={7} value={NaiveDateTimeValue(controller_change.updated_at)} />
                    <Entry {id} {row_num} col_num={8} value={UuidValue(id)} />
                </>
            }
        }
    };

    view
}
