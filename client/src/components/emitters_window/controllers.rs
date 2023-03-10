use crate::{
    hooks::{use_query_with_deps, QueryResponse},
    models::queries::controller::{
        get_controllers::{ControllersBy, ResponseData, Variables},
        GetControllers,
    },
    utils::{console_log, gen_style::gen_grid_style},
};
use std::rc::Rc;
use uuid::Uuid;
use yew::{classes, function_component, html, Html, Properties};

/// In an effort to avoid cloning large amounts of data to create props when re-rendering,
/// a smart pointer is passed in props to only clone a reference to the data instead of the data itself.
#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub facility_id: Rc<Uuid>,
}

#[function_component(ControllersComp)]
pub fn controllers_comp(Props { facility_id }: &Props) -> Html {
    console_log!("Ctr fac: {}", facility_id);

    let get_controllers = {
        let variables = Variables {
            by: ControllersBy {
                facility_id: **facility_id,
            },
        };
        use_query_with_deps::<GetControllers, _>(variables, facility_id.clone())
    };

    let r = match get_controllers {
        QueryResponse {
            data: Some(ResponseData { controllers_by }),
            ..
        } => {
            let controllers_iter = controllers_by.into_iter().enumerate().map(|(row_num, c)| {
                let m = c.manufacturer.map_or("".to_string(), |v| v.manufacturer);
                let a = c.application.map_or("".to_string(), |v| v.application);
                html! {
                    <>
                        <div style={gen_grid_style(1, row_num + 2)}>{ c.id }</div>
                        <div style={gen_grid_style(2, row_num + 2)}>{ c.model }</div>
                        <div style={gen_grid_style(3, row_num + 2)}>{ c.serial_number }</div>
                        <div style={gen_grid_style(4, row_num + 2)}>{ m }</div>
                        <div style={gen_grid_style(5, row_num + 2)}>{ a }</div>
                        <div style={gen_grid_style(6, row_num + 2)}>{ c.fdc_rec_id }</div>
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
            <div class={classes!("sticky")} style={gen_grid_style(1, 1)}>{ "ID" }</div>
            <div class={classes!("sticky")} style={gen_grid_style(2, 1)}>{ "Model" }</div>
            <div class={classes!("sticky")} style={gen_grid_style(3, 1)}>{ "Serial Number" }</div>
            <div class={classes!("sticky")} style={gen_grid_style(4, 1)}>{ "Manufacturer" }</div>
            <div class={classes!("sticky")} style={gen_grid_style(5, 1)}>{ "Application" }</div>
            <div class={classes!("sticky")} style={gen_grid_style(6, 1)}>{ "FDC ID" }</div>
            { r }
        </div>
    }
}
