macro_rules! row_component {
    (
        component_name: $component_name:ident,
        id: $id:ident,
        data_row: $data_row:ident,
        data_row_type: $data_row_type:ident,
        column_start: $column_start:literal,
        data_row_fields: (
            $(
                (
                    field: $field:ident,
                    edit_field_callback: $edit_field_callback:ident($($update_field_variant:path)?),
                    dropdown_selection_callback: $dropdown_selection_callback:ident($($dropdown_selection_variant:path, $dropdown_selection_id_callback:ident, $($dropdown_selection_id:ident)?)?),
                    value: $value:path,
                    display_value: $display_value_callback:ident($($display_value:path, $sub_field:ident, $nested_field:ident)?)
                )
            ),* $(,)?
        )
    ) => {

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
                },
            },
            pages::ModalVariant,
        };

        use yew::{function_component, html, use_state_eq, Callback, Html, Properties};

        #[derive(Clone, Debug, PartialEq, Properties)]
        pub struct Props {
            pub row_num: usize,
            pub $data_row: $data_row_type,
            pub handle_update_field: Callback<VariablesUpdateField>,
            pub handle_delete_entry: Callback<VariablesDeleteEntry>,
            pub modal_variant_handle: Callback<Option<ModalVariant>>,
        }

        #[function_component($component_name)]
        pub fn row_component(
            Props {
                row_num,
                $data_row,
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

            let $data_row = $data_row.clone();
            let $id = $data_row.$id;

            // let $($sub_field = $data_row.$sub_field.map(|s| s.name);)*
            // let manufacturer = $data_row.manufacturer.map(|m| m.manufacturer);
            // let created_by = $data_row.created_by.map(|cb| cb.email);
            // let updated_by = $data_row.updated_by.map(|ub| ub.email);



            let sidebar_items = vec![
                PneumaticInstrumentEmissionRate,
                PneumaticInstrumentMonthHours,
                PneumaticInstrumentMonthMethaneEmissionOverride,
                PneumaticInstrumentMonthMethaneEmission,
            ];

            let a = [$(
                (
                    $edit_field_callback!($(handle_update_field, $update_field_variant)?),
                    $dropdown_selection_callback!($($dropdown_selection_variant, $dropdown_selection_id_callback, $($data_row, $dropdown_selection_id)?, modal_variant_handle)?),
                    $value($data_row.$field),
                    $display_value_callback!($($display_value, $data_row, $sub_field, $nested_field)?)
                )
            ),*].into_iter().enumerate().map(|(col_num, (edit_field, dropdown_selection, value, display_value))| html! { <Entry id={$id} {row_num} col_num={col_num + $column_start}
            edit_field={edit_field}
            dropdown_selection={dropdown_selection}
            value={value}
            display_value={display_value}
        /> }).collect::<Html>();

            html! {
                <>
                    <DeleteEntryComponent id={$id} {row_num} col_num={1} delete_entry_variant={DeleteEntryVariant::PNEUMATIC_INSTRUMENT} handle_delete_entry={handle_delete_entry.clone()} />
                    <ExpandDataComponent {row_num} col_num={2} {expanded} {handle_expand_data} />
                //     {[$(($edit_field_callback!(handle_update_field, $update_field_variant), $dropdown_selection_callback!($dropdown_selection_variant, $($dropdown_selection_id)?, $dropdown_selection_id_callback, $data_row, modal_variant_handle))),*].into_iter().enumerate().map(|(col_num, (edit_field, dropdown_selection))| html! { <Entry id={$id} {row_num} col_num={col_num + $column_start}
                //     edit_field={edit_field}
                //     dropdown_selection={dropdown_selection}
                //     value={PneumaticInstrumentTypeValue($data_row.type_)}
                // /> }).collect::<Html>()}
                    // <Entry {id} {row_num} col_num={3}
                    //     edit_field={$edit_field_callback!(handle_update_field, $update_field_variant)}
                    //     dropdown_selection={$dropdown_selection_callback!($dropdown_selection_variant, $dropdown_selection_id, $dropdown_selection_id_callback, modal_variant_handle)}
                    //     value={PneumaticInstrumentTypeValue(pneumatic_instrument.type_)}
                    // />
                    // <Entry {id} {row_num} col_num={4}
                    //     edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: UpdateFieldVariant::PNEUMATIC_INSTRUMENT_MANUFACTURER_ID}}
                    //     dropdown_selection={DropdownSelectionProp {variant: DropdownSelectionVariant::DEVICE_MANUFACTURER_ID, id: None, modal_variant_handle: modal_variant_handle.clone()}}
                    //     value={UuidValue(pneumatic_instrument.manufacturer_id)}
                    //     display_value={OptionStringValue(manufacturer)}
                    // />
                    // <Entry {id} {row_num} col_num={5} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: UpdateFieldVariant::PNEUMATIC_INSTRUMENT_MODEL}} value={OptionStringValue(pneumatic_instrument.model)} />
                    // <Entry {id} {row_num} col_num={6} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: UpdateFieldVariant::PNEUMATIC_INSTRUMENT_SERIAL_NUMBER}} value={OptionStringValue(pneumatic_instrument.serial_number)} />
                    // <Entry {id} {row_num} col_num={7} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: UpdateFieldVariant::PNEUMATIC_INSTRUMENT_START_DATE}} value={NaiveDateValue(pneumatic_instrument.start_date)} />
                    // <Entry {id} {row_num} col_num={8} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: UpdateFieldVariant::PNEUMATIC_INSTRUMENT_END_DATE}} value={OptionNaiveDateValue(pneumatic_instrument.end_date)} />
                    // <Entry {id} {row_num} col_num={9}
                    //     edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: UpdateFieldVariant::PNEUMATIC_INSTRUMENT_SITE_ID}}
                    //     dropdown_selection={DropdownSelectionProp {variant: DropdownSelectionVariant::SITE_ID, id: None, modal_variant_handle: modal_variant_handle.clone()}}
                    //     value={UuidValue(pneumatic_instrument.site_id)}
                    //     display_value={OptionStringValue(site)}
                    // />
                    // <Entry {id} {row_num} col_num={10} value={OptionStringValue(created_by)} />
                    // <Entry {id} {row_num} col_num={11} value={NaiveDateTimeValue(pneumatic_instrument.created_at)} />
                    // <Entry {id} {row_num} col_num={12} value={OptionStringValue(updated_by)} />
                    // <Entry {id} {row_num} col_num={13} value={NaiveDateTimeValue(pneumatic_instrument.updated_at)} />
                    // <Entry {id} {row_num} col_num={14} value={UuidValue(id)} />
                    // if expanded {
                    //     <ObjectDataComponent {id} {sidebar_items} {modal_variant_handle} row_num={row_num + 1} col_num={14} />
                    // }
                </>
            }
        }


    };
}

macro_rules! edit_field_callback {
    ($handle_update_field:ident, $update_field_variant:path) => {
        Some(EditFieldProp {
            handle_update_field: $handle_update_field.clone(),
            update_field_variant: $update_field_variant,
        })
    };
    () => {
        None
    };
}

macro_rules! dropdown_selection_callback {
    ($dropdown_selection_variant:path, $dropdown_selection_id_callback:ident, $($data_row:ident, $dropdown_selection_id:ident)?, $modal_variant_handle:ident) => {
        Some(DropdownSelectionProp {
            variant: $dropdown_selection_variant,
            id: $dropdown_selection_id_callback!($($data_row, $dropdown_selection_id)?),
            modal_variant_handle: $modal_variant_handle.clone(),
        })
    };
    () => {
        None
    };
}

macro_rules! dropdown_selection_id_callback {
    ($data_row:ident, $dropdown_selection_id:ident) => {
        Some($data_row.$dropdown_selection_id)
    };
    () => {
        None
    };
}

macro_rules! display_value_callback {
    ($display_value:path, $data_row:ident, $sub_field:ident, $nested_field:ident) => {
        Some($display_value(
            $data_row.$sub_field.map(|field| field.$nested_field),
        ))
    };
    () => {
        None
    };
}

pub(crate) use row_component;

pub(crate) use edit_field_callback;

pub(crate) use dropdown_selection_callback;

pub(crate) use dropdown_selection_id_callback;

pub(crate) use display_value_callback;
