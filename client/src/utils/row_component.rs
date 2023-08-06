macro_rules! row_component {
    (
        component_name: $component_name:ident,
        id: $id:ident,
        data_row: $data_row:ident,
        data_row_type: $data_row_type:ident,
        column_start: $column_start:literal,
        delete_entry_variant: $delete_entry_variant:path,
        data_row_fields: (
            $(
                (
                    update_field_callback: ($($update_field_variant:path)?),
                    dropdown_selection_callback: ($($dropdown_selection_variant:path, ($($dropdown_selection_id:ident)?))?),
                    value: ($value_variant:path, $field:ident, ($($value_nested_field:ident)?)),
                    display_value: ($($display_value_variant:path, $display_value_sub_field:ident, $display_value_nested_field:ident)?)
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

            let sidebar_items = vec![
                PneumaticInstrumentEmissionRate,
                PneumaticInstrumentMonthHours,
                PneumaticInstrumentMonthMethaneEmissionOverride,
                PneumaticInstrumentMonthMethaneEmission,
            ];

            html! {
                <>
                    <DeleteEntryComponent id={$id} {row_num} col_num={1} delete_entry_variant={$delete_entry_variant} handle_delete_entry={handle_delete_entry.clone()} />
                    <ExpandDataComponent {row_num} col_num={2} {expanded} {handle_expand_data} />
                    {
                        [
                            $(
                                (
                                    update_field_callback!($(handle_update_field, $update_field_variant)?),
                                    dropdown_selection_callback!($($dropdown_selection_variant, $($data_row, $dropdown_selection_id)?, modal_variant_handle)?),
                                    value_callback!($value_variant, $data_row, $field, $($value_nested_field)?),
                                    display_value_callback!($($display_value_variant, $data_row, $display_value_sub_field, $display_value_nested_field)?)
                                )
                            ),*
                        ].into_iter().enumerate().map(|(col_num, (edit_field, dropdown_selection, value, display_value))| html! {
                            <Entry id={$id} {row_num} col_num={col_num + $column_start} {edit_field} {dropdown_selection} {value} {display_value} />
                        }).collect::<Html>()
                    }
                    if expanded {
                        <ObjectDataComponent id={$id} {sidebar_items} {modal_variant_handle} row_num={row_num + 1} col_num={count_fields!($($field)*) + $column_start - 1} />
                    }
                </>
            }
        }


    };
}

macro_rules! update_field_callback {
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
    ($dropdown_selection_variant:path, $($data_row:ident, $dropdown_selection_id:ident)?, $modal_variant_handle:ident) => {
        Some(DropdownSelectionProp {
            variant: $dropdown_selection_variant,
            id: dropdown_selection_id_callback!($($data_row, $dropdown_selection_id)?),
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

macro_rules! value_callback {
    ($value_variant:path, $data_row:ident, $field:ident, $value_nested_field:ident) => {
        $value_variant($data_row.$field.map(|field| field.$value_nested_field))
    };
    ($value_variant:path, $data_row:ident, $field:ident, ) => {
        $value_variant($data_row.$field)
    };
}

macro_rules! display_value_callback {
    ($display_value_variant:path, $data_row:ident, $display_value_sub_field:ident, $display_value_nested_field:ident) => {
        Some($display_value_variant(
            $data_row
                .$display_value_sub_field
                .map(|field| field.$display_value_nested_field),
        ))
    };
    () => {
        None
    };
}

macro_rules! count_fields {
    () => { 0 };
    ($odd:ident $($a:ident $b:ident)*) => { (count_fields!($($a)*) << 1) | 1 };
    ($($a:ident $even:ident)*) => { count_fields!($($a)*) << 1 };
}

pub(crate) use count_fields;
pub(crate) use display_value_callback;
pub(crate) use dropdown_selection_callback;
pub(crate) use dropdown_selection_id_callback;
pub(crate) use row_component;
pub(crate) use update_field_callback;
pub(crate) use value_callback;
