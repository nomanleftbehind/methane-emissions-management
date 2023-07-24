use crate::{
    components::emitters_window::data::{
        insert_entry_button::InsertEntryButton,
        object_row::{ObjectDataProp, ObjectRowComponent},
        pneumatic_instrument::InsertPneumaticInstrumentForm,
    },
    hooks::{lazy_query, use_query_with_deps, QueryResponse},
    models::{
        mutations::{
            manual_mutation::{
                delete_entry::{
                    ResponseData as ResponseDataDeleteEntry, Variables as VariablesDeleteEntry,
                },
                update_field::{
                    ResponseData as ResponseDataUpdateField, Variables as VariablesUpdateField,
                },
                DeleteEntry, UpdateField,
            },
            pneumatic_instrument::{
                insert_pneumatic_instrument_change::{
                    ResponseData as ResponseDataInsertPneumaticInstrumentChange,
                    Variables as VariablesInsertController,
                },
                InsertPneumaticInstrumentChange,
            },
        },
        queries::pneumatic_instrument::{
            get_pneumatic_instrument_changes::{
                GetPneumaticInstrumentChangesGetPneumaticInstrumentChanges,
                GetPneumaticInstrumentChangesInput, PneumaticInstrumentChangesByVariant,
                ResponseData, Variables,
            },
            GetPneumaticInstrumentChanges,
        },
    },
    pages::ModalVariant,
    utils::gen_style::gen_grid_style,
};
use std::rc::Rc;
use uuid::Uuid;
use yew::{classes, function_component, html, use_state_eq, Callback, Html, Properties};

/// In an effort to avoid cloning large amounts of data to create props when re-rendering,
/// a smart pointer is passed in props to only clone a reference to the data instead of the data itself.
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub id: Rc<Uuid>,
    pub modal_variant_handle: Callback<Option<ModalVariant>>,
}

#[function_component(ObjectsComponent)]
pub fn objects_component(
    Props {
        id,
        modal_variant_handle,
    }: &Props,
) -> Html {
    let number_of_updated_fields_handle = use_state_eq(|| 0);
    let number_of_updated_fields = *number_of_updated_fields_handle;

    let insert_form_is_open_handle = use_state_eq(|| false);
    let insert_form_is_open = *insert_form_is_open_handle;

    let toggle_insert_form_is_open = {
        let insert_form_is_open_handle = insert_form_is_open_handle.clone();
        Callback::from(move |_| {
            insert_form_is_open_handle.set(!insert_form_is_open);
        })
    };

    let close_insert_form = Callback::from(move |_| {
        insert_form_is_open_handle.set(false);
    });

    let get_pneumatic_instrument_changes = {
        let variables = Variables {
            get_pneumatic_instrument_changes_input: GetPneumaticInstrumentChangesInput {
                id: **id,
                by: PneumaticInstrumentChangesByVariant::PNEUMATIC_INSTRUMENT_ID,
            },
        };
        use_query_with_deps::<GetPneumaticInstrumentChanges, _>(
            variables,
            (id.clone(), number_of_updated_fields),
        )
    };

    // use_effect_with_deps(
    //     move |u| {
    //         console_log!("object_variant: {:#?}", u);
    //     },
    //     object_variant.clone(),
    // );

    let handle_insert_pneumatic_instrument_change = {
        let number_of_updated_fields_handle = number_of_updated_fields_handle.clone();
        let modal_variant_handle = modal_variant_handle.clone();
        Callback::from(move |variables: VariablesInsertController| {
            let number_of_updated_fields_handle = number_of_updated_fields_handle.clone();
            let modal_variant_handle = modal_variant_handle.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match lazy_query::<InsertPneumaticInstrumentChange>(variables).await {
                    QueryResponse {
                        data:
                            Some(ResponseDataInsertPneumaticInstrumentChange {
                                insert_pneumatic_instrument_change,
                            }),
                        ..
                    } => {
                        number_of_updated_fields_handle
                            .set(number_of_updated_fields + insert_pneumatic_instrument_change);
                    }
                    QueryResponse {
                        error: Some(error), ..
                    } => {
                        modal_variant_handle.emit(Some(ModalVariant::Error(error)));
                    }
                    _ => (),
                };
            });
        })
    };

    let handle_update_field = {
        let number_of_updated_fields_handle = number_of_updated_fields_handle.clone();
        let modal_variant_handle = modal_variant_handle.clone();
        Callback::from(move |variables: VariablesUpdateField| {
            let number_of_updated_fields_handle = number_of_updated_fields_handle.clone();
            let modal_variant_handle = modal_variant_handle.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match lazy_query::<UpdateField>(variables).await {
                    QueryResponse {
                        data: Some(ResponseDataUpdateField { update_field }),
                        ..
                    } => {
                        number_of_updated_fields_handle
                            .set(number_of_updated_fields + update_field);
                    }
                    QueryResponse {
                        error: Some(error), ..
                    } => {
                        modal_variant_handle.emit(Some(ModalVariant::Error(error)));
                    }
                    _ => (),
                };
            });
        })
    };

    // The reason for having callback within callback is to be able to modify both local and parent component's state.
    // Outer callback modifies parent component's state (opens modal to confirm deletion) and passes the inner callback to parent component.
    // Inner callback modifies local state (updates number of modified fields once deletion has been confirmed and returned successfully).
    let handle_delete_entry = {
        let modal_variant_handle = modal_variant_handle.clone();
        let number_of_updated_fields_handle = number_of_updated_fields_handle.clone();

        Callback::from(move |variables: VariablesDeleteEntry| {
            let delete_entry_callback = {
                let number_of_updated_fields_handle = number_of_updated_fields_handle.clone();
                let modal_variant_handle = modal_variant_handle.clone();

                Callback::from(move |_| {
                    let variables = variables.clone();
                    let number_of_updated_fields_handle = number_of_updated_fields_handle.clone();
                    let modal_variant_handle = modal_variant_handle.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        match lazy_query::<DeleteEntry>(variables).await {
                            QueryResponse {
                                data: Some(ResponseDataDeleteEntry { delete_entry }),
                                ..
                            } => {
                                number_of_updated_fields_handle
                                    .set(number_of_updated_fields + delete_entry);
                            }
                            QueryResponse {
                                error: Some(error), ..
                            } => {
                                modal_variant_handle.emit(Some(ModalVariant::Error(error)));
                            }
                            _ => (),
                        };
                    });
                })
            };

            modal_variant_handle.emit(Some(ModalVariant::ConfirmDelete(delete_entry_callback)));
        })
    };

    // use_effect_with_deps(
    //     move |u| {
    //         console_log!("number_of_updated_fields: {:#?}", u);
    //     },
    //     number_of_updated_fields,
    // );

    let view = match get_pneumatic_instrument_changes {
        QueryResponse {
            data:
                Some(ResponseData {
                    get_pneumatic_instrument_changes,
                }),
            ..
        } => {
            let pneumatic_instrument_changes_iter = get_pneumatic_instrument_changes.into_iter().enumerate().map(|(mut row_num, pneumatic_instrument_change)| {
                row_num = (row_num + 2) * 2 - 1;
                html! {
                    <ObjectRowComponent {row_num} {modal_variant_handle} object_data={ObjectDataProp::ControllerChange(pneumatic_instrument_change)} handle_update_field={handle_update_field.clone()} handle_delete_entry={handle_delete_entry.clone()} />
                }
            });

            html! {
                <div class={classes!("emitters", "controller-changes")}>
                    <div class={classes!("sticky")} style={gen_grid_style(1, 1)}/>
                    <div class={classes!("sticky")} style={gen_grid_style(2, 1)}>{ "Date" }</div>
                    <div class={classes!("sticky")} style={gen_grid_style(3, 1)}>{ "Rate" }</div>
                    <div class={classes!("sticky")} style={gen_grid_style(4, 1)}>{ "Created By" }</div>
                    <div class={classes!("sticky")} style={gen_grid_style(5, 1)}>{ "Created At" }</div>
                    <div class={classes!("sticky")} style={gen_grid_style(6, 1)}>{ "Updated By" }</div>
                    <div class={classes!("sticky")} style={gen_grid_style(7, 1)}>{ "Updated At" }</div>
                    <div class={classes!("sticky")} style={gen_grid_style(8, 1)}>{ "ID" }</div>
                    { for pneumatic_instrument_changes_iter }
                </div>
            }
        }
        QueryResponse { error: Some(e), .. } => {
            modal_variant_handle.emit(Some(ModalVariant::Error(e)));
            Html::default()
        }
        _ => Html::default(),
    };

    view
}
