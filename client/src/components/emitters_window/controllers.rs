use crate::{
    components::emitters_window::entry::{EditFieldProp, Entry},
    hooks::{lazy_query, use_query_with_deps, QueryResponse},
    models::{
        mutations::update_field::{
            update_field::{
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
    utils::gen_style::gen_grid_style,
};
use common::UpdateFieldValue::{
    IntegerValue, NaiveDateTimeValue, NaiveDateValue, OptionStringValue, StringValue, UuidValue,
};
use std::rc::Rc;
use uuid::Uuid;
use yew::{classes, function_component, html, Callback, Html, Properties};

/// In an effort to avoid cloning large amounts of data to create props when re-rendering,
/// a smart pointer is passed in props to only clone a reference to the data instead of the data itself.
#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub facility_id: Rc<Uuid>,
}

#[function_component(ControllersComp)]
pub fn controllers_comp(Props { facility_id }: &Props) -> Html {
    let get_controllers = {
        let variables = Variables {
            by: EmittersByInput {
                facility_id: **facility_id,
            },
        };
        use_query_with_deps::<GetControllers, _>(variables, facility_id.clone())
    };

    let handle_update_field = Callback::from(move |variables: VariablesUpdateField| {
        wasm_bindgen_futures::spawn_local(async move {
            lazy_query::<UpdateField>(variables).await;
        });
    });

    let r = match get_controllers {
        QueryResponse {
            data: Some(ResponseData { controllers_by }),
            ..
        } => {
            let controllers_iter = controllers_by.into_iter().enumerate().map(|(row_num, c)| {
                let id = c.id;
                let manufacturer = c.manufacturer.map(|v| v.manufacturer);
                let application = c.application.map(|v| v.application);
                let created_by = c.created_by.map(|cb| cb.email);
                let row_num = row_num + 2;
                html! {
                    <>
                        // <Entry {id} col_num={1} {row_num} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: CONTROLLER_MODEL}} value={OptionStringValue(c.model)} />
                        <Entry {id} col_num={1} {row_num} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: CONTROLLER_MANUFACTURER_ID}} value={UuidValue(id)} />
                        <Entry {id} col_num={2} {row_num} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: CONTROLLER_SERIAL_NUMBER}} value={OptionStringValue(c.serial_number)} />
                        <Entry {id} col_num={3} {row_num} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: CONTROLLER_MANUFACTURER_ID}} value={OptionStringValue(manufacturer)} />
                        <Entry {id} col_num={4} {row_num} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: CONTROLLER_APPLICATION_ID}} value={OptionStringValue(application)} />
                        <Entry {id} col_num={5} {row_num} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: CONTROLLER_FDC_REC_ID}} value={StringValue(c.fdc_rec_id)} />
                        <Entry {id} col_num={6} {row_num} value={OptionStringValue(created_by)} />
                        <Entry {id} col_num={7} {row_num} value={NaiveDateTimeValue(c.created_at)} />
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
            { r }
        </div>
    }
}
