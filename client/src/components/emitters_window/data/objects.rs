use crate::{
    components::emitters_window::data::object_row::{ObjectDataProp, ObjectRowComponent},
    hooks::{lazy_query, use_query_with_deps, QueryResponse},
    models::{
        mutations::manual_mutation::{
            delete_entry::{
                DeleteEntryInput, ResponseData as ResponseDataDeleteEntry,
                Variables as VariablesDeleteEntry,
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
    pages::ModalVariant,
    utils::{console_log, gen_style::gen_grid_style},
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
    pub id: Rc<Uuid>,
    pub object_variant: GetObjectVariant,
    pub modal_variant_handle: Callback<Option<ModalVariant>>,
}

#[function_component(ObjectsComponent)]
pub fn objects_component(
    Props {
        id,
        object_variant,
        modal_variant_handle,
    }: &Props,
) -> Html {
    let number_of_updated_fields_handle = use_state_eq(|| 0);
    let number_of_updated_fields = *number_of_updated_fields_handle;

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
                    QueryResponse { error: Some(e), .. } => {
                        modal_variant_handle.emit(Some(ModalVariant::Error(e)));
                    }
                    _ => (),
                };
            });
        })
    };

    // The reason for having callback within callback is to have access to both local and parent component's state.
    // Outer callback modifies parent component's state (opens modal to confirm deletion) and passes it the inner callback which modifies local state (updates number of modified fields once deletion has been confirmed and returned successfully).
    let handle_delete_entry = {
        let modal_variant_handle = modal_variant_handle.clone();
        let number_of_updated_fields_handle = number_of_updated_fields_handle.clone();

        Callback::from(
            move |VariablesDeleteEntry {
                      delete_entry_input:
                          DeleteEntryInput {
                              id,
                              delete_entry_variant,
                          },
                  }: VariablesDeleteEntry| {
                let delete_entry_callback = {
                    let number_of_updated_fields_handle = number_of_updated_fields_handle.clone();
                    let modal_variant_handle = modal_variant_handle.clone();

                    Callback::from(move |_| {
                        let delete_entry_variant = delete_entry_variant.clone();
                        let variables = VariablesDeleteEntry {
                            delete_entry_input: DeleteEntryInput {
                                id,
                                delete_entry_variant,
                            },
                        };
                        let number_of_updated_fields_handle =
                            number_of_updated_fields_handle.clone();
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
                                QueryResponse { error: Some(e), .. } => {
                                    modal_variant_handle.emit(Some(ModalVariant::Error(e)));
                                }
                                _ => (),
                            };
                        });
                    })
                };

                modal_variant_handle.emit(Some(ModalVariant::ConfirmDelete(delete_entry_callback)));
            },
        )
    };

    use_effect_with_deps(
        move |u| {
            console_log!("number_of_updated_fields: {:#?}", u);
        },
        number_of_updated_fields,
    );

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
                    <ObjectRowComponent {row_num} {modal_variant_handle} object_data={ObjectDataProp::Controller(controller)} handle_update_field={handle_update_field.clone()} handle_delete_entry={handle_delete_entry.clone()} />
                }
            });

            html! {
                <div class={classes!("emitters", "controllers")}>
                    <div class={classes!("sticky")} style={gen_grid_style(1, 1)}/>
                    <div class={classes!("sticky")} style={gen_grid_style(2, 1)}/>
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
                    <ObjectRowComponent {row_num} {modal_variant_handle} object_data={ObjectDataProp::Compressor(compressor)} handle_update_field={handle_update_field.clone()} handle_delete_entry={handle_delete_entry.clone()} />
                }
            });

            html! {
                <div class={classes!("emitters", "compressors")}>
                    <div class={classes!("sticky")} style={gen_grid_style(1, 1)}/>
                    <div class={classes!("sticky")} style={gen_grid_style(2, 1)}/>
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
                    <ObjectRowComponent {row_num} {modal_variant_handle} object_data={ObjectDataProp::TankFarm(tank_farm)} handle_update_field={handle_update_field.clone()} handle_delete_entry={handle_delete_entry.clone()} />
                }
            });

            html! {
                <div class={classes!("emitters", "tank-farms")}>
                    <div class={classes!("sticky")} style={gen_grid_style(1, 1)}/>
                    <div class={classes!("sticky")} style={gen_grid_style(2, 1)}/>
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
                    <ObjectRowComponent {row_num} {modal_variant_handle} object_data={ObjectDataProp::ControllerChange(controller_change)} handle_update_field={handle_update_field.clone()} handle_delete_entry={handle_delete_entry.clone()} />
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
                    { for controller_changes_iter }
                </div>
            }
        }
        QueryResponse {
            data:
                Some(ResponseData {
                    get_object:
                        GetObjectGetObject {
                            controller_month_hours: Some(controller_month_hours),
                            ..
                        },
                }),
            ..
        } => {
            let controller_month_hours_iter = controller_month_hours.into_iter().enumerate().map(|(mut row_num, controller_month_hours)| {
                row_num = (row_num + 1) * 2;
                html! {
                    <ObjectRowComponent {row_num} {modal_variant_handle} object_data={ObjectDataProp::ControllerMonthHours(controller_month_hours)} handle_update_field={handle_update_field.clone()} handle_delete_entry={handle_delete_entry.clone()} />
                }
            });

            html! {
                <div class={classes!("emitters", "controller-month-hours")}>
                    <div class={classes!("sticky")} style={gen_grid_style(1, 1)}/>
                    <div class={classes!("sticky")} style={gen_grid_style(2, 1)}>{ "Month" }</div>
                    <div class={classes!("sticky")} style={gen_grid_style(3, 1)}>{ "Hours On" }</div>
                    <div class={classes!("sticky")} style={gen_grid_style(4, 1)}>{ "Created By" }</div>
                    <div class={classes!("sticky")} style={gen_grid_style(5, 1)}>{ "Created At" }</div>
                    <div class={classes!("sticky")} style={gen_grid_style(6, 1)}>{ "Updated By" }</div>
                    <div class={classes!("sticky")} style={gen_grid_style(7, 1)}>{ "Updated At" }</div>
                    <div class={classes!("sticky")} style={gen_grid_style(8, 1)}>{ "ID" }</div>
                    { for controller_month_hours_iter }
                </div>
            }
        }
        QueryResponse {
            data:
                Some(ResponseData {
                    get_object:
                        GetObjectGetObject {
                            controller_month_vent_override: Some(controller_month_vent_overrides),
                            ..
                        },
                }),
            ..
        } => {
            let controller_month_vent_overrides_iter = controller_month_vent_overrides.into_iter().enumerate().map(|(mut row_num, controller_month_vent_override)| {
                row_num = (row_num + 1) * 2;
                html! {
                    <ObjectRowComponent {row_num} {modal_variant_handle} object_data={ObjectDataProp::ControllerMonthVentOverride(controller_month_vent_override)} handle_update_field={handle_update_field.clone()} handle_delete_entry={handle_delete_entry.clone()} />
                }
            });

            html! {
                <div class={classes!("emitters", "controller-month-vent-override")}>
                    <div class={classes!("sticky")} style={gen_grid_style(1, 1)}/>
                    <div class={classes!("sticky")} style={gen_grid_style(2, 1)}>{ "Month" }</div>
                    <div class={classes!("sticky")} style={gen_grid_style(3, 1)}>{ "Gas Volume (m³)" }</div>
                    <div class={classes!("sticky")} style={gen_grid_style(4, 1)}>{ "Comment" }</div>
                    <div class={classes!("sticky")} style={gen_grid_style(5, 1)}>{ "Created By" }</div>
                    <div class={classes!("sticky")} style={gen_grid_style(6, 1)}>{ "Created At" }</div>
                    <div class={classes!("sticky")} style={gen_grid_style(7, 1)}>{ "Updated By" }</div>
                    <div class={classes!("sticky")} style={gen_grid_style(8, 1)}>{ "Updated At" }</div>
                    <div class={classes!("sticky")} style={gen_grid_style(9, 1)}>{ "ID" }</div>
                    { for controller_month_vent_overrides_iter }
                </div>
            }
        }
        QueryResponse {
            data:
                Some(ResponseData {
                    get_object:
                        GetObjectGetObject {
                            controller_month_vent: Some(controller_month_vents),
                            ..
                        },
                }),
            ..
        } => {
            let controller_month_vents_iter = controller_month_vents.into_iter().enumerate().map(|(mut row_num, controller_month_vent)| {
                row_num = (row_num + 1) * 2;
                html! {
                    <ObjectRowComponent {row_num} {modal_variant_handle} object_data={ObjectDataProp::ControllerMonthVent(controller_month_vent)} handle_update_field={handle_update_field.clone()} handle_delete_entry={handle_delete_entry.clone()} />
                }
            });

            html! {
                <div class={classes!("emitters", "controller-month-vent")}>
                    <div class={classes!("sticky")} style={gen_grid_style(1, 1)}/>
                    <div class={classes!("sticky")} style={gen_grid_style(2, 1)}>{ "Month" }</div>
                    <div class={classes!("sticky")} style={gen_grid_style(3, 1)}>{ "Gas Volume (m³)" }</div>
                    <div class={classes!("sticky")} style={gen_grid_style(4, 1)}>{ "Methane Volume (m³)" }</div>
                    <div class={classes!("sticky")} style={gen_grid_style(5, 1)}>{ "CO₂ Volume (m³)" }</div>
                    <div class={classes!("sticky")} style={gen_grid_style(6, 1)}>{ "Created By" }</div>
                    <div class={classes!("sticky")} style={gen_grid_style(7, 1)}>{ "Created At" }</div>
                    <div class={classes!("sticky")} style={gen_grid_style(8, 1)}>{ "Updated By" }</div>
                    <div class={classes!("sticky")} style={gen_grid_style(9, 1)}>{ "Updated At" }</div>
                    <div class={classes!("sticky")} style={gen_grid_style(10, 1)}>{ "ID" }</div>
                    { for controller_month_vents_iter }
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
