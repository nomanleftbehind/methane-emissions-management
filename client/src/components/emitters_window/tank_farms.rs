use crate::{
    hooks::{use_query_with_deps, QueryResponse},
    models::queries::tank_farm::{
        get_tank_farms::{EmittersByInput, ResponseData, Variables},
        GetTankFarms,
    },
    utils::gen_style::gen_grid_style,
};
use std::rc::Rc;
use uuid::Uuid;
use yew::{classes, function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub facility_id: Rc<Uuid>,
}

#[function_component(TankFarmsComp)]
pub fn tank_farms_comp(Props { facility_id }: &Props) -> Html {
    let get_tank_farms = {
        let variables = Variables {
            by: EmittersByInput {
                facility_id: **facility_id,
            },
        };

        use_query_with_deps::<GetTankFarms, _>(variables, facility_id.clone())
    };

    let r = match get_tank_farms {
        QueryResponse {
            data: Some(ResponseData { tank_farms_by }),
            ..
        } => {
            let tank_farms_iter = tank_farms_by.into_iter().enumerate().map(|(row_num, tf)| {
                let created_by = tf.created_by.map(|cb| cb.email);
                let updated_by = tf.updated_by.map(|ub| ub.email);

                html! {
                    <>
                        <div style={gen_grid_style(1, row_num + 2)}>{ tf.id }</div>
                        <div style={gen_grid_style(2, row_num + 2)}>{ created_by }</div>
                        <div style={gen_grid_style(3, row_num + 2)}>{ tf.created_at }</div>
                        <div style={gen_grid_style(4, row_num + 2)}>{ updated_by }</div>
                        <div style={gen_grid_style(5, row_num + 2)}>{ tf.updated_at }</div>
                    </>
                }
            });
            html! { for tank_farms_iter }
        }
        QueryResponse { error: Some(e), .. } => {
            html! {e}
        }
        _ => html! {},
    };

    html! {
       <div class={classes!("emitters")}>
            <div class={classes!("sticky")} style={gen_grid_style(1, 1)}>{ "ID" }</div>
            <div class={classes!("sticky")} style={gen_grid_style(2, 1)}>{ "Created By" }</div>
            <div class={classes!("sticky")} style={gen_grid_style(3, 1)}>{ "Created At" }</div>
            <div class={classes!("sticky")} style={gen_grid_style(4, 1)}>{ "Updated By" }</div>
            <div class={classes!("sticky")} style={gen_grid_style(5, 1)}>{ "Updated At" }</div>
            { r }
       </div>
    }
}
