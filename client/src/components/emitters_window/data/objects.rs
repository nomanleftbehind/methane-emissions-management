use crate::{
    components::{
        emitters_window::data::object_row::{ObjectDataProp, ObjectRowComponent},
        modal::{error::Error, modal::Modal},
    },
    hooks::{lazy_query, use_query_with_deps, QueryResponse},
    models::{
        mutations::manual_mutation::{
            delete_entry::{
                ResponseData as ResponseDataDeleteEntry, Variables as VariablesDeleteEntry,
            },
            update_field::{
                ResponseData as ResponseDataUpdateField, Variables as VariablesUpdateField,
            },
            DeleteEntry, UpdateField,
        },
        queries::get_object::{
            get_object::{
                GetObjectGetObject, GetObjectInput, GetObjectVariant, ResponseData, Variables,
            },
            GetObject,
        },
    },
    utils::{console_log, gen_style::gen_grid_style},
};
use std::rc::Rc;
use uuid::Uuid;
use wasm_bindgen::JsCast;
use web_sys::HtmlDialogElement;
use yew::{
    classes, function_component, html, use_effect, use_effect_with_deps, use_state_eq, Callback,
    Html, MouseEvent, Properties,
};

/// In an effort to avoid cloning large amounts of data to create props when re-rendering,
/// a smart pointer is passed in props to only clone a reference to the data instead of the data itself.
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub id: Rc<Uuid>,
    pub object_variant: GetObjectVariant,
}

#[function_component(ObjectsComponent)]
pub fn objects_component(Props { id, object_variant }: &Props) -> Html {
    let number_of_updated_fields_handle = use_state_eq(|| 0);
    let number_of_updated_fields = *number_of_updated_fields_handle;

    let error_handle = use_state_eq(|| None);
    let error = (*error_handle).clone();

    let modal_root_handle = use_state_eq(|| Rc::new(None));
    let modal_root = (*modal_root_handle).clone();
    use_effect(move || {
        let modal_root = gloo::utils::document()
            .get_element_by_id("modal-root")
            .expect("Expected to find a #modal-root element")
            .dyn_into::<HtmlDialogElement>()
            .expect("#modal-root is not a <dialog> element");

        modal_root_handle.set(Rc::new(Some(modal_root)));
    });

    // let modal_root = Rc::new(
    //     gloo::utils::document()
    //         .get_element_by_id("modal-root")
    //         .expect("Expected to find a #modal-root element")
    //         .dyn_into::<HtmlDialogElement>()
    //         .expect("#modal-root is not a <dialog> element"),
    // );

    // let open_error_dialog_handle = use_state_eq(|| false);
    // let open_error_dialog = *open_error_dialog_handle;
    let on_error_dialog_button_click = {
        let error_handle = error_handle.clone();
        Callback::from(move |_: MouseEvent| {
            error_handle.set(None);
        })
    };

    let get_objects = {
        let variables = Variables {
            get_object_input: GetObjectInput {
                id: **id,
                get_object_variant: object_variant.clone(),
            },
        };
        use_query_with_deps::<GetObject, _>(
            variables,
            (id.clone(), number_of_updated_fields, object_variant.clone()),
        )
    };

    // use_effect_with_deps(
    //     move |u| {
    //         console_log!("object_variant: {:#?}", u);
    //     },
    //     object_variant.clone(),
    // );

    let handle_update_field = {
        let number_of_updated_fields_handle = number_of_updated_fields_handle.clone();
        let error_handle = error_handle.clone();
        let modal_root = Rc::clone(&modal_root);
        Callback::from(move |variables: VariablesUpdateField| {
            let number_of_updated_fields_handle = number_of_updated_fields_handle.clone();
            let error_handle = error_handle.clone();
            let modal_root = Rc::clone(&modal_root);
            wasm_bindgen_futures::spawn_local(async move {
                match lazy_query::<UpdateField>(variables).await {
                    QueryResponse {
                        data: Some(ResponseDataUpdateField { update_field }),
                        ..
                    } => {
                        number_of_updated_fields_handle.set(number_of_updated_fields + update_field)
                    }
                    QueryResponse { error: Some(e), .. } => {
                        error_handle.set(Some(e));
                        if let Some(ref modal_root) = *modal_root {
                            let _ = modal_root.show_modal();
                        }
                    }
                    _ => {}
                };
            });
        })
    };

    let handle_delete_entry = {
        let error_handle = error_handle.clone();
        let modal_root = Rc::clone(&modal_root);
        Callback::from(move |variables: VariablesDeleteEntry| {
            let number_of_updated_fields_handle = number_of_updated_fields_handle.clone();
            let error_handle = error_handle.clone();
            let modal_root = Rc::clone(&modal_root);
            wasm_bindgen_futures::spawn_local(async move {
                match lazy_query::<DeleteEntry>(variables).await {
                    QueryResponse {
                        data: Some(ResponseDataDeleteEntry { delete_entry }),
                        ..
                    } => {
                        number_of_updated_fields_handle.set(number_of_updated_fields + delete_entry)
                    }
                    QueryResponse { error: Some(e), .. } => {
                        error_handle.set(Some(e));
                        if let Some(ref modal_root) = *modal_root {
                            let _ = modal_root.show_modal();
                        }
                    }
                    _ => {}
                };
            });
        })
    };

    // use_effect_with_deps(
    //     move |u| {
    //         console_log!("number_of_updated_fields: {:#?}", u);
    //     },
    //     number_of_updated_fields,
    // );

    let view = match get_objects {
        QueryResponse {
            data:
                Some(ResponseData {
                    get_object:
                        GetObjectGetObject {
                            controllers: Some(controllers),
                            ..
                        },
                }),
            ..
        } => {
            let controllers_iter = controllers.into_iter().enumerate().map(|(mut row_num, controller)| {
                row_num = (row_num + 1) * 2;
                html! {
                    <ObjectRowComponent {row_num} object_data={ObjectDataProp::Controller(controller)} handle_update_field={handle_update_field.clone()} handle_delete_entry={handle_delete_entry.clone()} />
                }
            });

            html! {
                <div class={classes!("emitters", "controllers")}>
                    <div class={classes!("sticky")} style={gen_grid_style(1, 1)}></div>
                    <div class={classes!("sticky")} style={gen_grid_style(2, 1)}></div>
                    <div class={classes!("sticky")} style={gen_grid_style(3, 1)}>{ "Model" }</div>
                    <div class={classes!("sticky")} style={gen_grid_style(4, 1)}>{ "Serial Number" }</div>
                    <div class={classes!("sticky")} style={gen_grid_style(5, 1)}>{ "Manufacturer" }</div>
                    <div class={classes!("sticky")} style={gen_grid_style(6, 1)}>{ "Application" }</div>
                    <div class={classes!("sticky")} style={gen_grid_style(7, 1)}>{ "FDC ID" }</div>
                    <div class={classes!("sticky")} style={gen_grid_style(8, 1)}>{ "Created By" }</div>
                    <div class={classes!("sticky")} style={gen_grid_style(9, 1)}>{ "Created At" }</div>
                    <div class={classes!("sticky")} style={gen_grid_style(10, 1)}>{ "Updated By" }</div>
                    <div class={classes!("sticky")} style={gen_grid_style(11, 1)}>{ "Updated At" }</div>
                    <div class={classes!("sticky")} style={gen_grid_style(12, 1)}>{ "ID" }</div>
                    { for controllers_iter }
                </div>
            }
        }
        QueryResponse {
            data:
                Some(ResponseData {
                    get_object:
                        GetObjectGetObject {
                            compressors: Some(compressors),
                            ..
                        },
                }),
            ..
        } => {
            let compressors_iter = compressors.into_iter().enumerate().map(|(mut row_num, compressor)| {
                row_num = (row_num + 1) * 2;
                html! {
                    <ObjectRowComponent {row_num} object_data={ObjectDataProp::Compressor(compressor)} handle_update_field={handle_update_field.clone()} handle_delete_entry={handle_delete_entry.clone()} />
                }
            });

            html! {
                <div class={classes!("emitters", "compressors")}>
                    <div class={classes!("sticky")} style={gen_grid_style(1, 1)}></div>
                    <div class={classes!("sticky")} style={gen_grid_style(2, 1)}></div>
                    <div class={classes!("sticky")} style={gen_grid_style(3, 1)}>{ "Name" }</div>
                    <div class={classes!("sticky")} style={gen_grid_style(4, 1)}>{ "Serial Number" }</div>
                    <div class={classes!("sticky")} style={gen_grid_style(5, 1)}>{ "Install Date" }</div>
                    <div class={classes!("sticky")} style={gen_grid_style(6, 1)}>{ "Remove Date" }</div>
                    <div class={classes!("sticky")} style={gen_grid_style(7, 1)}>{ "FDC ID" }</div>
                    <div class={classes!("sticky")} style={gen_grid_style(8, 1)}>{ "Created By" }</div>
                    <div class={classes!("sticky")} style={gen_grid_style(9, 1)}>{ "Created At" }</div>
                    <div class={classes!("sticky")} style={gen_grid_style(10, 1)}>{ "Updated By" }</div>
                    <div class={classes!("sticky")} style={gen_grid_style(11, 1)}>{ "Updated At" }</div>
                    <div class={classes!("sticky")} style={gen_grid_style(12, 1)}>{ "ID" }</div>
                    { for compressors_iter }
                </div>
            }
        }
        QueryResponse {
            data:
                Some(ResponseData {
                    get_object:
                        GetObjectGetObject {
                            tank_farms: Some(tank_farms),
                            ..
                        },
                }),
            ..
        } => {
            let tank_farms_iter = tank_farms.into_iter().enumerate().map(|(mut row_num, tank_farm)| {
                row_num = (row_num + 1) * 2;
                html! {
                    <ObjectRowComponent {row_num} object_data={ObjectDataProp::TankFarm(tank_farm)} handle_update_field={handle_update_field.clone()} handle_delete_entry={handle_delete_entry.clone()} />
                }
            });

            html! {
                <div class={classes!("emitters", "tank-farms")}>
                    <div class={classes!("sticky")} style={gen_grid_style(1, 1)}></div>
                    <div class={classes!("sticky")} style={gen_grid_style(2, 1)}></div>
                    <div class={classes!("sticky")} style={gen_grid_style(3, 1)}>{ "ID" }</div>
                    <div class={classes!("sticky")} style={gen_grid_style(4, 1)}>{ "Created By" }</div>
                    <div class={classes!("sticky")} style={gen_grid_style(5, 1)}>{ "Created At" }</div>
                    <div class={classes!("sticky")} style={gen_grid_style(6, 1)}>{ "Updated By" }</div>
                    <div class={classes!("sticky")} style={gen_grid_style(7, 1)}>{ "Updated At" }</div>
                    { for tank_farms_iter }
                </div>
            }
        }
        QueryResponse {
            data:
                Some(ResponseData {
                    get_object:
                        GetObjectGetObject {
                            controller_changes: Some(controller_changes),
                            ..
                        },
                }),
            ..
        } => {
            let controller_changes_iter = controller_changes.into_iter().enumerate().map(|(mut row_num, controller_change)| {
                row_num = (row_num + 1) * 2;
                html! {
                    <ObjectRowComponent {row_num} object_data={ObjectDataProp::ControllerChange(controller_change)} handle_update_field={handle_update_field.clone()} handle_delete_entry={handle_delete_entry.clone()} />
                }
            });

            html! {
                <div class={classes!("emitters", "controller-changes")}>
                    <div class={classes!("sticky")} style={gen_grid_style(1, 1)}></div>
                    <div class={classes!("sticky")} style={gen_grid_style(2, 1)}>{ "Date" }</div>
                    <div class={classes!("sticky")} style={gen_grid_style(3, 1)}>{ "Rate" }</div>
                    <div class={classes!("sticky")} style={gen_grid_style(4, 1)}>{ "Created By" }</div>
                    <div class={classes!("sticky")} style={gen_grid_style(5, 1)}>{ "Created At" }</div>
                    <div class={classes!("sticky")} style={gen_grid_style(6, 1)}>{ "Updated By" }</div>
                    <div class={classes!("sticky")} style={gen_grid_style(7, 1)}>{ "Updated At" }</div>
                    <div class={classes!("sticky")} style={gen_grid_style(8, 1)}>{ "ID" }</div>
                    { for controller_changes_iter }
                </div>
            }
        }
        QueryResponse { error: Some(e), .. } => {
            error_handle.set(Some(e));
            if let Some(ref modal_root) = *modal_root {
                let _ = modal_root.show_modal();
            }
            html! {}
        }
        _ => {
            html! {}
        }
    };

    html! {
        <>
            <Error {on_error_dialog_button_click}>
                if let Some(error) = error {
                    <>{error}</>
                } else {
                    <></>
                }
            </Error>
            { view }
        </>
    }
}
