use crate::{
    components::emitters_window::{
        data::{
            entry::{EditFieldProp, Entry, IdSelectionProp},
            object_data::ObjectDataComponent,
        },
        delete_entry::DeleteEntryComponent,
        expand_data::ExpandDataComponent,
    },
    models::{
        mutations::manual_mutation::{
            delete_entry::{DeleteEntryVariant, Variables as VariablesDeleteEntry},
            update_field::{UpdateFieldVariant, Variables as VariablesUpdateField},
        },
        queries::{
            get_object::get_object::{
                GetObjectGetObjectCompressors, GetObjectGetObjectControllerChanges,
                GetObjectGetObjectControllerMonthHours, GetObjectGetObjectControllerMonthVent,
                GetObjectGetObjectControllerMonthVentOverride, GetObjectGetObjectControllers,
                GetObjectGetObjectTankFarms,
                GetObjectVariant::{
                    CONTROLLER_CHANGE_BY_CONTROLLER_ID, CONTROLLER_MONTH_HOURS_BY_CONTROLLER_ID,
                    CONTROLLER_MONTH_VENT_BY_CONTROLLER_ID,
                    CONTROLLER_MONTH_VENT_OVERRIDE_BY_CONTROLLER_ID,
                },
            },
            id_selection::id_selection::IdSelectionVariant,
        },
    },
    pages::ModalVariant,
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
    ControllerMonthHours(GetObjectGetObjectControllerMonthHours),
    ControllerMonthVentOverride(GetObjectGetObjectControllerMonthVentOverride),
    ControllerMonthVent(GetObjectGetObjectControllerMonthVent),
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub row_num: usize,
    pub object_data: ObjectDataProp,
    pub handle_update_field: Callback<VariablesUpdateField>,
    pub handle_delete_entry: Callback<VariablesDeleteEntry>,
    pub modal_variant_handle: Callback<Option<ModalVariant>>,
}

#[function_component(ObjectRowComponent)]
pub fn object_row_component(
    Props {
        row_num,
        object_data,
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

    let view = match object_data {
        ObjectDataProp::Controller(controller) => {
            let controller = controller.clone();
            let id = controller.id;
            let manufacturer = controller.manufacturer.map(|m| m.manufacturer);
            let application = controller.application.map(|a| a.application);
            let created_by = controller.created_by.map(|cb| cb.email);
            let updated_by = controller.updated_by.map(|ub| ub.email);

            let sidebar_items = vec![
                CONTROLLER_CHANGE_BY_CONTROLLER_ID,
                CONTROLLER_MONTH_HOURS_BY_CONTROLLER_ID,
                CONTROLLER_MONTH_VENT_OVERRIDE_BY_CONTROLLER_ID,
                CONTROLLER_MONTH_VENT_BY_CONTROLLER_ID,
            ];

            html! {
                <>
                    <DeleteEntryComponent {id} {row_num} col_num={1} delete_entry_variant={DeleteEntryVariant::CONTROLLER} handle_delete_entry={handle_delete_entry.clone()} />
                    <ExpandDataComponent {row_num} col_num={2} {expanded} {handle_expand_data} />
                    <Entry {id} {row_num} col_num={3} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: UpdateFieldVariant::CONTROLLER_MODEL}} value={OptionStringValue(controller.model)} />
                    <Entry {id} {row_num} col_num={4} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: UpdateFieldVariant::CONTROLLER_SERIAL_NUMBER}} value={OptionStringValue(controller.serial_number)} />
                    <Entry {id} {row_num} col_num={5}
                        edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: UpdateFieldVariant::CONTROLLER_MANUFACTURER_ID}}
                        id_selection={IdSelectionProp {variant: IdSelectionVariant::CONTROLLER_MANUFACTURER_ID, modal_variant_handle: modal_variant_handle.clone()}}
                        value={UuidValue(controller.manufacturer_id)}
                        display_value={OptionStringValue(manufacturer)}
                    />
                    <Entry {id} {row_num} col_num={6}
                        edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: UpdateFieldVariant::CONTROLLER_APPLICATION_ID}}
                        id_selection={IdSelectionProp {variant: IdSelectionVariant::CONTROLLER_APPLICATION_ID, modal_variant_handle: modal_variant_handle.clone()}}
                        value={OptionUuidValue(controller.application_id)}
                        display_value={OptionStringValue(application)}
                    />
                    <Entry {id} {row_num} col_num={7} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: UpdateFieldVariant::CONTROLLER_FDC_REC_ID}} value={StringValue(controller.fdc_rec_id)} />
                    <Entry {id} {row_num} col_num={8} value={OptionStringValue(created_by)} />
                    <Entry {id} {row_num} col_num={9} value={NaiveDateTimeValue(controller.created_at)} />
                    <Entry {id} {row_num} col_num={10} value={OptionStringValue(updated_by)} />
                    <Entry {id} {row_num} col_num={11} value={NaiveDateTimeValue(controller.updated_at)} />
                    <Entry {id} {row_num} col_num={12} value={UuidValue(id)} />
                    if expanded {
                        <ObjectDataComponent {id} {sidebar_items} {modal_variant_handle} row_num={row_num + 1} col_num={12} />
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
        ObjectDataProp::ControllerMonthHours(controller_month_hours) => {
            let controller_month_hours = controller_month_hours.clone();
            let id = controller_month_hours.id;
            let created_by = controller_month_hours.created_by.map(|cb| cb.email);
            let updated_by = controller_month_hours.updated_by.map(|ub| ub.email);

            html! {
                <>
                    <DeleteEntryComponent {id} {row_num} col_num={1} delete_entry_variant={DeleteEntryVariant::CONTROLLER_MONTH_HOURS} handle_delete_entry={handle_delete_entry.clone()} />
                    <Entry {id} {row_num} col_num={2} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: UpdateFieldVariant::CONTROLLER_MONTH_HOURS_MONTH}} value={NaiveDateValue(controller_month_hours.month)} />
                    <Entry {id} {row_num} col_num={3} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: UpdateFieldVariant::CONTROLLER_MONTH_HOURS_HOURS_ON}} value={FloatValue(controller_month_hours.hours_on)} display_value={StringValue(format!("{:.1}", controller_month_hours.hours_on))}/>
                    <Entry {id} {row_num} col_num={4} value={OptionStringValue(created_by)} />
                    <Entry {id} {row_num} col_num={5} value={NaiveDateTimeValue(controller_month_hours.created_at)} />
                    <Entry {id} {row_num} col_num={6} value={OptionStringValue(updated_by)} />
                    <Entry {id} {row_num} col_num={7} value={NaiveDateTimeValue(controller_month_hours.updated_at)} />
                    <Entry {id} {row_num} col_num={8} value={UuidValue(id)} />
                </>
            }
        }
        ObjectDataProp::ControllerMonthVentOverride(controller_month_vent_override) => {
            let controller_month_vent_override = controller_month_vent_override.clone();
            let id = controller_month_vent_override.id;
            let created_by = controller_month_vent_override.created_by.map(|cb| cb.email);
            let updated_by = controller_month_vent_override.updated_by.map(|ub| ub.email);

            html! {
                <>
                    <DeleteEntryComponent {id} {row_num} col_num={1} delete_entry_variant={DeleteEntryVariant::CONTROLLER_MONTH_VENT_OVERRIDE} handle_delete_entry={handle_delete_entry.clone()} />
                    <Entry {id} {row_num} col_num={2} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: UpdateFieldVariant::CONTROLLER_MONTH_VENT_OVERRIDE_MONTH}} value={NaiveDateValue(controller_month_vent_override.month)} />
                    <Entry {id} {row_num} col_num={3} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: UpdateFieldVariant::CONTROLLER_MONTH_VENT_OVERRIDE_GAS_VOLUME}} value={FloatValue(controller_month_vent_override.gas_volume)} display_value={StringValue(format!("{:.2}", controller_month_vent_override.gas_volume))}/>
                    <Entry {id} {row_num} col_num={4} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: UpdateFieldVariant::CONTROLLER_MONTH_VENT_OVERRIDE_COMMENT}} value={OptionStringValue(controller_month_vent_override.comment)} />
                    <Entry {id} {row_num} col_num={5} value={OptionStringValue(created_by)} />
                    <Entry {id} {row_num} col_num={6} value={NaiveDateTimeValue(controller_month_vent_override.created_at)} />
                    <Entry {id} {row_num} col_num={7} value={OptionStringValue(updated_by)} />
                    <Entry {id} {row_num} col_num={8} value={NaiveDateTimeValue(controller_month_vent_override.updated_at)} />
                    <Entry {id} {row_num} col_num={9} value={UuidValue(id)} />
                </>
            }
        }
        ObjectDataProp::ControllerMonthVent(controller_month_vent) => {
            let controller_month_vent = controller_month_vent.clone();
            let id = controller_month_vent.id;
            let created_by = controller_month_vent.created_by.map(|cb| cb.email);
            let updated_by = controller_month_vent.updated_by.map(|ub| ub.email);

            html! {
                <>
                    <DeleteEntryComponent {id} {row_num} col_num={1} delete_entry_variant={DeleteEntryVariant::CONTROLLER_MONTH_VENT} handle_delete_entry={handle_delete_entry.clone()} />
                    <Entry {id} {row_num} col_num={2} value={NaiveDateValue(controller_month_vent.month)} />
                    <Entry {id} {row_num} col_num={3} value={FloatValue(controller_month_vent.gas_volume)} display_value={StringValue(format!("{:.2}", controller_month_vent.gas_volume))}/>
                    <Entry {id} {row_num} col_num={4} value={FloatValue(controller_month_vent.c1_volume)} display_value={StringValue(format!("{:.2}", controller_month_vent.c1_volume))}/>
                    <Entry {id} {row_num} col_num={5} value={FloatValue(controller_month_vent.co2_volume)} display_value={StringValue(format!("{:.2}", controller_month_vent.co2_volume))}/>
                    <Entry {id} {row_num} col_num={6} value={OptionStringValue(created_by)} />
                    <Entry {id} {row_num} col_num={7} value={NaiveDateTimeValue(controller_month_vent.created_at)} />
                    <Entry {id} {row_num} col_num={8} value={OptionStringValue(updated_by)} />
                    <Entry {id} {row_num} col_num={9} value={NaiveDateTimeValue(controller_month_vent.updated_at)} />
                    <Entry {id} {row_num} col_num={10} value={UuidValue(id)} />
                </>
            }
        }
    };

    view
}
