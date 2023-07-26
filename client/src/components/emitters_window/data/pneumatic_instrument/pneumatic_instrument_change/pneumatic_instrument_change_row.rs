use crate::{
    components::emitters_window::{
        data::{
            entry::{DropdownSelectionProp, EditFieldProp, Entry},
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
            dropdown_selection::get_dropdown_selection::DropdownSelectionVariant,
            pneumatic_instrument::get_pneumatic_instrument_changes::GetPneumaticInstrumentChangesGetPneumaticInstrumentChanges,
        },
    },
    pages::ModalVariant,
};
use common::{
    SidebarItem::{
        PneumaticInstrumentChange, PneumaticInstrumentMonthHours,
        PneumaticInstrumentMonthMethaneEmission, PneumaticInstrumentMonthMethaneEmissionOverride,
    },
    UpdateFieldValueEnum::{
        FloatValue, NaiveDateTimeValue, NaiveDateValue, OptionStringValue, UuidValue,
    },
};
use yew::{function_component, html, use_state_eq, Callback, Html, Properties};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub row_num: usize,
    pub pneumatic_instrument_change: GetPneumaticInstrumentChangesGetPneumaticInstrumentChanges,
    pub handle_update_field: Callback<VariablesUpdateField>,
    pub handle_delete_entry: Callback<VariablesDeleteEntry>,
    pub modal_variant_handle: Callback<Option<ModalVariant>>,
}

#[function_component(PneumaticInstrumentChangeRowComponent)]
pub fn pneumatic_instrument_change_row_component(
    Props {
        row_num,
        pneumatic_instrument_change,
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

    let pneumatic_instrument_change = pneumatic_instrument_change.clone();
    let id = pneumatic_instrument_change.id;
    let created_by = pneumatic_instrument_change.created_by.map(|cb| cb.email);
    let updated_by = pneumatic_instrument_change.updated_by.map(|ub| ub.email);

    let sidebar_items = vec![
        PneumaticInstrumentChange,
        PneumaticInstrumentMonthHours,
        PneumaticInstrumentMonthMethaneEmissionOverride,
        PneumaticInstrumentMonthMethaneEmission,
    ];

    html! {
        <>
            <DeleteEntryComponent {id} {row_num} col_num={1} delete_entry_variant={DeleteEntryVariant::PNEUMATIC_INSTRUMENT_CHANGE} handle_delete_entry={handle_delete_entry.clone()} />
            <ExpandDataComponent {row_num} col_num={2} {expanded} {handle_expand_data} />
            <Entry {id} {row_num} col_num={3}
                edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: UpdateFieldVariant::PNEUMATIC_INSTRUMENT_CHANGE_PNEUMATIC_INSTRUMENT_ID}}
                dropdown_selection={DropdownSelectionProp {variant: DropdownSelectionVariant::PNEUMATIC_INSTRUMENT_ID, id: None, modal_variant_handle: modal_variant_handle.clone()}}
                value={UuidValue(pneumatic_instrument_change.pneumatic_instrument_id)}
            />
            <Entry {id} {row_num} col_num={4} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: UpdateFieldVariant::PNEUMATIC_INSTRUMENT_CHANGE_DATE}} value={NaiveDateValue(pneumatic_instrument_change.date)} />
            <Entry {id} {row_num} col_num={5} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: UpdateFieldVariant::PNEUMATIC_INSTRUMENT_CHANGE_RATE}} value={FloatValue(pneumatic_instrument_change.rate)} />
            <Entry {id} {row_num} col_num={6} value={OptionStringValue(created_by)} />
            <Entry {id} {row_num} col_num={7} value={NaiveDateTimeValue(pneumatic_instrument_change.created_at)} />
            <Entry {id} {row_num} col_num={8} value={OptionStringValue(updated_by)} />
            <Entry {id} {row_num} col_num={9} value={NaiveDateTimeValue(pneumatic_instrument_change.updated_at)} />
            <Entry {id} {row_num} col_num={10} value={UuidValue(id)} />
            if expanded {
                <ObjectDataComponent {id} {sidebar_items} {modal_variant_handle} row_num={row_num + 1} col_num={14} />
            }
        </>
    }
}
