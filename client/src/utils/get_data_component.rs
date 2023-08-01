macro_rules! get_data_component {
    (component_name: $component_name:ident,
    id: $id:ident,
    get_data_input: ($get_data:ident, $get_data_variable:ident, $get_data_input:ident, $get_data_input_type:ident, $get_data_by:path, $query:ident, $get_data_response:ident),
    insert_input: ($insert_variable:ident, $insert_query:ident, $insert_response_data:ident, $insert_response_data_field:ident)
    ) => {


        use super::{
            insert_pneumatic_instrument_form::InsertPneumaticInstrumentForm,
            pneumatic_instrument_row::PneumaticInstrumentRowComponent,
        };
        use crate::{
            components::emitters_window::data::insert_entry_button::InsertEntryButton,
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
                },
            },
            pages::ModalVariant,
            utils::gen_style::gen_grid_style,
        };
        use std::rc::Rc;
        use uuid::Uuid;
        use yew::{
            classes, function_component, html, use_effect_with_deps, use_state_eq, Callback, Html,
            Properties,
        };

        /// In an effort to avoid cloning large amounts of data to create props when re-rendering,
        /// a smart pointer is passed in props to only clone a reference to the data instead of the data itself.
        #[derive(Clone, Debug, PartialEq, Properties)]
        pub struct Props {
            pub $id: Rc<Uuid>,
            pub modal_variant_handle: Callback<Option<ModalVariant>>,
        }

        #[function_component($component_name)]
        pub fn get_data(
            Props {
                $id,
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

            let close_insert_form = {
                let insert_form_is_open_handle = insert_form_is_open_handle.clone();
                Callback::from(move |_| {
                    insert_form_is_open_handle.set(false);
                })
            };

            let $get_data = {
                let variables = $get_data_variable {
                    $get_data_input: $get_data_input_type {
                        by: $get_data_by,
                        id: **$id,
                    },
                };
                use_query_with_deps::<$query, _>(
                    variables,
                    ($id.clone(), number_of_updated_fields),
                )
            };

            // Close insert form every time facility is changed so that new sites are loaded when new insert form is opened.
            use_effect_with_deps(
                move |_| {
                    insert_form_is_open_handle.set(false);
                },
                $id.clone(),
            );

            let handle_insert = {
                let number_of_updated_fields_handle = number_of_updated_fields_handle.clone();
                let modal_variant_handle = modal_variant_handle.clone();
                Callback::from(move |variables: $insert_variable| {
                    let number_of_updated_fields_handle = number_of_updated_fields_handle.clone();
                    let modal_variant_handle = modal_variant_handle.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        match lazy_query::<$insert_query>(variables).await {
                            QueryResponse {
                                data:
                                    Some($insert_response_data {
                                        $insert_response_data_field,
                                    }),
                                ..
                            } => {
                                number_of_updated_fields_handle
                                    .set(number_of_updated_fields + $insert_response_data_field);
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

            let view = match $get_data {
                QueryResponse {
                    data: Some($get_data_response {
                        $get_data,
                    }),
                    ..
                } => {
                    let get_data_iter = $get_data.into_iter().enumerate().map(|(mut row_num, pneumatic_instrument)| {
                        row_num = (row_num + 2) * 2 - 1;
                        html! {
                            <PneumaticInstrumentRowComponent {row_num} {modal_variant_handle} {pneumatic_instrument} handle_update_field={handle_update_field.clone()} handle_delete_entry={handle_delete_entry.clone()} />
                        }
                    });

                    html! {
                        <div class={classes!("emitters", "pneumatic-instruments")}>
                            <InsertEntryButton {insert_form_is_open} {toggle_insert_form_is_open}/>
                            <div class={classes!("sticky")} style={gen_grid_style(2, 1)}/>
                            <div class={classes!("sticky")} style={gen_grid_style(3, 1)}>{ "Type" }</div>
                            <div class={classes!("sticky")} style={gen_grid_style(4, 1)}>{ "Manufacturer" }</div>
                            <div class={classes!("sticky")} style={gen_grid_style(5, 1)}>{ "Model" }</div>
                            <div class={classes!("sticky")} style={gen_grid_style(6, 1)}>{ "Serial Number" }</div>
                            <div class={classes!("sticky")} style={gen_grid_style(7, 1)}>{ "StartDate" }</div>
                            <div class={classes!("sticky")} style={gen_grid_style(8, 1)}>{ "End Date" }</div>
                            <div class={classes!("sticky")} style={gen_grid_style(9, 1)}>{ "Site" }</div>
                            <div class={classes!("sticky")} style={gen_grid_style(10, 1)}>{ "Created By" }</div>
                            <div class={classes!("sticky")} style={gen_grid_style(11, 1)}>{ "Created At" }</div>
                            <div class={classes!("sticky")} style={gen_grid_style(12, 1)}>{ "Updated By" }</div>
                            <div class={classes!("sticky")} style={gen_grid_style(13, 1)}>{ "Updated At" }</div>
                            <div class={classes!("sticky")} style={gen_grid_style(14, 1)}>{ "ID" }</div>
                            if insert_form_is_open {
                                <InsertPneumaticInstrumentForm {$id} {close_insert_form} {handle_insert} {modal_variant_handle} />
                            }
                            { for get_data_iter }
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



    };
}

pub(crate) use get_data_component;
