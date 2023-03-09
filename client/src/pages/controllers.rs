use crate::{
    hooks::{use_query_with_deps, QueryResponse},
    models::queries::controller::{
        get_controllers::{ControllersBy, ResponseData, Variables},
        GetControllers,
    },
    utils::console_log,
};
use uuid::Uuid;
use yew::{function_component, html, Html, Properties};

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub facility_id: Uuid,
}

fn gen_style(col: usize, row: usize) -> String {
    format!("grid-column: {}; grid-row: {};", col, row)
}

#[function_component(ControllersPage)]
pub fn controllers_page(Props { facility_id }: &Props) -> Html {
    console_log!("Ctr fac: {}", &facility_id);

    let get_controllers = {
        let facility_id = facility_id.clone();
        let deps = facility_id.clone();
        let var = Variables {
            by: ControllersBy { facility_id },
        };

        use_query_with_deps::<GetControllers, _>(var, deps)
    };

    let r = match get_controllers {
        QueryResponse {
            data: Some(ResponseData { controllers_by }),
            ..
        } => {
            // fn gen_style(col: usize, row: usize) -> String {
            //     format!("grid-column: {}; grid-row: {};", col, row)
            // }
            let controllers_iter = controllers_by.into_iter().enumerate().map(|(row_num, c)| {
                // let grid_col = gen_style(1, row_num + 1);
                // let style = format!("grid-column: {}; grid-row: {};", 1, row_num + 1);
                let m = c.manufacturer.map_or("".to_string(), |v| v.manufacturer);
                let a = c.application.map_or("".to_string(), |v| v.application);
                html! {
                    <>
                        <div style={gen_style(1, row_num + 2)}>{ c.id }</div>
                        <div style={gen_style(2, row_num + 2)}>{ c.model }</div>
                        <div style={gen_style(3, row_num + 2)}>{ c.serial_number }</div>
                        <div style={gen_style(4, row_num + 2)}>{ m }</div>
                        <div style={gen_style(5, row_num + 2)}>{ a }</div>
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
        <div class="data-window">
            <div class="sticky" style={gen_style(1, 1)} >{ "ID" }</div>
            <div class="sticky" style={gen_style(2, 1)} >{ "Model" }</div>
            <div class="sticky" style={gen_style(3, 1)} >{ "Serial Number" }</div>
            <div class="sticky" style={gen_style(4, 1)} >{ "Manufacturer" }</div>
            <div class="sticky" style={gen_style(5, 1)} >{ "Application" }</div>
            { r }
        </div>
    }
}
