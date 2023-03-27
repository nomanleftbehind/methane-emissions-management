use crate::{
    components::emitters_window::entry::{EditFieldProp, Entry},
    hooks::{lazy_query, use_query_with_deps, QueryResponse},
    models::{
        mutations::update_field::{
            update_field::{
                ResponseData as ResponseDataUpdateField,
                UpdateFieldVariant::{
                    CONTROLLER_APPLICATION_ID, CONTROLLER_FDC_REC_ID, CONTROLLER_MANUFACTURER_ID,
                    CONTROLLER_MODEL, CONTROLLER_SERIAL_NUMBER,
                },
                Variables as VariablesUpdateField,
            },
            UpdateField,
        },
        queries::controller::{
            get_controllers::{EmittersByInput, ResponseData, Variables},
            GetControllers,
        },
    },
    utils::{console_log, gen_style::gen_grid_style},
};
use common::UpdateFieldValueEnum::{
    IntegerValue, NaiveDateTimeValue, NaiveDateValue, OptionStringValue, OptionUuidValue,
    StringValue, UuidValue,
};
use std::rc::Rc;
use uuid::Uuid;
use yew::{
    classes, function_component, html, use_effect_with_deps, use_state_eq, Callback, Html,
    Properties,
};

/// In an effort to avoid cloning large amounts of data to create props when re-rendering,
/// a smart pointer is passed in props to only clone a reference to the data instead of the data itself.
#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub facility_id: Rc<Uuid>,
}

#[function_component(ControllersComp)]
pub fn controllers_comp(Props { facility_id }: &Props) -> Html {
    let number_of_updated_fields_handle = use_state_eq(|| 0);
    let number_of_updated_fields = *number_of_updated_fields_handle;

    let get_controllers = {
        let variables = Variables {
            by: EmittersByInput {
                facility_id: **facility_id,
            },
        };
        use_query_with_deps::<GetControllers, _>(
            variables,
            (facility_id.clone(), number_of_updated_fields),
        )
    };

    let handle_update_field = Callback::from(move |variables: VariablesUpdateField| {
        let updated_fields_handle = number_of_updated_fields_handle.clone();
        wasm_bindgen_futures::spawn_local(async move {
            match lazy_query::<UpdateField>(variables).await {
                QueryResponse {
                    data: Some(ResponseDataUpdateField { update_field }),
                    ..
                } => updated_fields_handle.set(number_of_updated_fields + update_field),
                QueryResponse { error: Some(e), .. } => {
                    console_log!("Update error: {}", e);
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

    let r = match get_controllers {
        QueryResponse {
            data: Some(ResponseData { controllers_by }),
            ..
        } => {
            let controllers_iter = controllers_by.into_iter().enumerate().map(|(row_num, c)| {
                let id = c.id;
                let manufacturer = c.manufacturer.map(|m| m.manufacturer);
                let application = c.application.map(|a| a.application);
                let created_by = c.created_by.map(|cb| cb.email);
                let row_num = row_num + 2;
                html! {
                    <>
                        <Entry {id} col_num={1} {row_num} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: CONTROLLER_MODEL}} value={OptionStringValue(c.model)} />
                        <Entry {id} col_num={2} {row_num} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: CONTROLLER_SERIAL_NUMBER}} value={OptionStringValue(c.serial_number)} />
                        <Entry {id} col_num={3} {row_num} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: CONTROLLER_MANUFACTURER_ID}} value={UuidValue(c.manufacturer_id)} display_value={OptionStringValue(manufacturer)} />
                        <Entry {id} col_num={4} {row_num} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: CONTROLLER_APPLICATION_ID}} value={OptionUuidValue(c.application_id)} display_value={OptionStringValue(application)} />
                        <Entry {id} col_num={5} {row_num} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: CONTROLLER_FDC_REC_ID}} value={StringValue(c.fdc_rec_id)} />
                        <Entry {id} col_num={6} {row_num} value={OptionStringValue(created_by)} />
                        <Entry {id} col_num={7} {row_num} value={NaiveDateTimeValue(c.created_at)} />
                        <Entry {id} col_num={8} {row_num} value={UuidValue(id)} />
                    </>
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
        <div class={classes!("emitters")}>
            <div class={classes!("sticky")} style={gen_grid_style(1, 1)}>{ "Model" }</div>
            <div class={classes!("sticky")} style={gen_grid_style(2, 1)}>{ "Serial Number" }</div>
            <div class={classes!("sticky")} style={gen_grid_style(3, 1)}>{ "Manufacturer" }</div>
            <div class={classes!("sticky")} style={gen_grid_style(4, 1)}>{ "Application" }</div>
            <div class={classes!("sticky")} style={gen_grid_style(5, 1)}>{ "FDC ID" }</div>
            <div class={classes!("sticky")} style={gen_grid_style(6, 1)}>{ "Created By" }</div>
            <div class={classes!("sticky")} style={gen_grid_style(7, 1)}>{ "Created At" }</div>
            <div class={classes!("sticky")} style={gen_grid_style(8, 1)}>{ "ID" }</div>
            { r }
        </div>
    }
}
