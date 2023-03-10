use crate::{
    hooks::{use_query_with_deps, QueryResponse},
    models::queries::compressor::{
        get_compressors::{CompressorsByFacilityId, ResponseData, Variables},
        GetCompressors,
    },
    utils::gen_style::gen_grid_style,
};
use std::rc::Rc;
use uuid::Uuid;
use yew::{classes, function_component, html, html_nested, Html, Properties};

/// In an effort to avoid cloning large amounts of data to create props when re-rendering,
/// a smart pointer is passed in props to only clone a reference to the data instead of the data itself.
#[derive(Properties, PartialEq)]
pub struct Props {
    pub facility_id: Rc<Uuid>,
}

#[function_component(CompressorsComp)]
pub fn compressors_comp(Props { facility_id }: &Props) -> Html {
    let get_compressors = {
        let variables = Variables {
            by: CompressorsByFacilityId {
                facility_id: **facility_id,
            },
        };
        use_query_with_deps::<GetCompressors, _>(variables, facility_id.clone())
    };

    let compressors = match get_compressors {
        QueryResponse {
            data: Some(ResponseData { compressors_by }),
            ..
        } => {
            let compressors_iter = compressors_by.into_iter().enumerate().map(|(row_num, c)| {
                html! {
                    <>
                        <div style={gen_grid_style(1, row_num + 2)}>{ c.id }</div>
                        <div style={gen_grid_style(2, row_num + 2)}>{ c.name }</div>
                        <div style={gen_grid_style(3, row_num + 2)}>{ c.serial_number }</div>
                        <div style={gen_grid_style(4, row_num + 2)}>{ c.install_date }</div>
                        <div style={gen_grid_style(5, row_num + 2)}>{ c.remove_date }</div>
                        <div style={gen_grid_style(6, row_num + 2)}>{ c.fdc_rec_id }</div>
                    </>
                }
            });
            html! { for compressors_iter }
        }
        QueryResponse { error: Some(e), .. } => {
            html! {e}
        }
        _ => html! {},
    };

    html! {
        <div class={classes!("emitters")}>
            <div class={classes!("sticky")} style={gen_grid_style(1, 1)}>{ "ID" }</div>
            <div class={classes!("sticky")} style={gen_grid_style(2, 1)}>{ "Name" }</div>
            <div class={classes!("sticky")} style={gen_grid_style(3, 1)}>{ "Serial Number" }</div>
            <div class={classes!("sticky")} style={gen_grid_style(4, 1)}>{ "Install Date" }</div>
            <div class={classes!("sticky")} style={gen_grid_style(5, 1)}>{ "Remove Date" }</div>
            <div class={classes!("sticky")} style={gen_grid_style(6, 1)}>{ "FDC ID" }</div>
            { compressors }
        </div>
    }
}
