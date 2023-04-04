use crate::{
    components::emitters_window::controller_change_row::ControllerChangeRowComponent,
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
        queries::controller_change::{
            get_controller_changes::{ControllerChangeInput, ResponseData, Variables},
            GetControllerChanges,
        },
    },
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
    pub controller_id: Rc<Uuid>,
}

#[function_component(ControllerChangeComp)]
pub fn controller_change_comp(Props { controller_id }: &Props) -> Html {
    let number_of_updated_fields_handle = use_state_eq(|| 0);
    let number_of_updated_fields = *number_of_updated_fields_handle;

    let get_controller_changes = {
        let variables = Variables {
            controller_change_input: ControllerChangeInput {
                controller_id: **controller_id,
            },
        };
        use_query_with_deps::<GetControllerChanges, _>(
            variables,
            (controller_id.clone(), number_of_updated_fields),
        )
    };

    let handle_update_field = {
        let number_of_updated_fields_handle = number_of_updated_fields_handle.clone();
        Callback::from(move |variables: VariablesUpdateField| {
            let number_of_updated_fields_handle = number_of_updated_fields_handle.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match lazy_query::<UpdateField>(variables).await {
                    QueryResponse {
                        data: Some(ResponseDataUpdateField { update_field }),
                        ..
                    } => {
                        number_of_updated_fields_handle.set(number_of_updated_fields + update_field)
                    }
                    QueryResponse { error: Some(e), .. } => {
                        console_log!("Update error: {}", e);
                    }
                    _ => {}
                };
            });
        })
    };

    let handle_delete_entry = Callback::from(move |variables: VariablesDeleteEntry| {
        let number_of_updated_fields_handle = number_of_updated_fields_handle.clone();
        wasm_bindgen_futures::spawn_local(async move {
            match lazy_query::<DeleteEntry>(variables).await {
                QueryResponse {
                    data: Some(ResponseDataDeleteEntry { delete_entry }),
                    ..
                } => number_of_updated_fields_handle.set(number_of_updated_fields + delete_entry),
                QueryResponse { error: Some(e), .. } => {
                    console_log!("Delete error: {}", e);
                }
                _ => {}
            };
        });
    });

    use_effect_with_deps(
        move |u| {
            console_log!("number_of_updated_fields: {:#?}", u);
        },
        number_of_updated_fields,
    );

    let controller_changes = match get_controller_changes {
        QueryResponse {
            data: Some(ResponseData {
                get_controller_changes,
            }),
            ..
        } => {
            let controllers_iter = get_controller_changes.into_iter().enumerate().map(|(mut row_num, controller_change)| {
                row_num = (row_num + 1) * 2;
                html! {
                    <ControllerChangeRowComponent {row_num} {controller_change} handle_update_field={handle_update_field.clone()} handle_delete_entry={handle_delete_entry.clone()} />
                }
            });

            html! { for controllers_iter }
        }
        QueryResponse { error: Some(e), .. } => {
            html! {e}
        }
        _ => {
            html! {}
        }
    };

    html! {
        <div class={classes!("emitters", "controllers")}>
            <div class={classes!("sticky")} style={gen_grid_style(1, 1)}></div>
            <div class={classes!("sticky")} style={gen_grid_style(2, 1)}>{ "Date" }</div>
            <div class={classes!("sticky")} style={gen_grid_style(3, 1)}>{ "Rate" }</div>
            <div class={classes!("sticky")} style={gen_grid_style(4, 1)}>{ "Created By" }</div>
            <div class={classes!("sticky")} style={gen_grid_style(5, 1)}>{ "Created At" }</div>
            <div class={classes!("sticky")} style={gen_grid_style(6, 1)}>{ "Updated By" }</div>
            <div class={classes!("sticky")} style={gen_grid_style(7, 1)}>{ "Updated At" }</div>
            <div class={classes!("sticky")} style={gen_grid_style(8, 1)}>{ "ID" }</div>
            { controller_changes }
        </div>
    }
}
