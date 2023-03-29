use crate::{
    components::emitters_window::entry::Entry,
    hooks::{lazy_query, use_query_with_deps, QueryResponse},
    models::{
        mutations::manual_mutation::{
            update_field::{
                ResponseData as ResponseDataUpdateField, Variables as VariablesUpdateField,
            },
            UpdateField,
        },
        queries::tank_farm::{
            get_tank_farms::{EmittersByInput, ResponseData, Variables},
            GetTankFarms,
        },
    },
    utils::{console_log, gen_style::gen_grid_style},
};
use common::UpdateFieldValueEnum::{NaiveDateTimeValue, OptionStringValue, UuidValue};
use std::rc::Rc;
use uuid::Uuid;
use yew::{classes, function_component, html, use_state_eq, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub facility_id: Rc<Uuid>,
}

#[function_component(TankFarmsComp)]
pub fn tank_farms_comp(Props { facility_id }: &Props) -> Html {
    let number_of_updated_fields_handle = use_state_eq(|| 0);
    let number_of_updated_fields = *number_of_updated_fields_handle;

    let get_tank_farms = {
        let variables = Variables {
            by: EmittersByInput {
                facility_id: **facility_id,
            },
        };

        use_query_with_deps::<GetTankFarms, _>(
            variables,
            (facility_id.clone(), number_of_updated_fields),
        )
    };

    let _handle_update_field = Callback::from(move |variables: VariablesUpdateField| {
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

    let view = match get_tank_farms {
        QueryResponse {
            data: Some(ResponseData { tank_farms_by }),
            ..
        } => {
            let tank_farms_iter = tank_farms_by.into_iter().enumerate().map(|(row_num, tf)| {
                let id = tf.id;
                let created_by = tf.created_by.map(|cb| cb.email);
                let updated_by = tf.updated_by.map(|ub| ub.email);
                let row_num = row_num + 2;

                html! {
                    <>
                        <Entry {id} col_num={1} {row_num} value={UuidValue(id)} />
                        <Entry {id} col_num={2} {row_num} value={OptionStringValue(created_by)} />
                        <Entry {id} col_num={3} {row_num} value={NaiveDateTimeValue(tf.created_at)} />
                        <Entry {id} col_num={4} {row_num} value={OptionStringValue(updated_by)} />
                        <Entry {id} col_num={5} {row_num} value={NaiveDateTimeValue(tf.updated_at)} />
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
       <div class={classes!("emitters", "tank-farms")}>
            <div class={classes!("sticky")} style={gen_grid_style(1, 1)}>{ "ID" }</div>
            <div class={classes!("sticky")} style={gen_grid_style(2, 1)}>{ "Created By" }</div>
            <div class={classes!("sticky")} style={gen_grid_style(3, 1)}>{ "Created At" }</div>
            <div class={classes!("sticky")} style={gen_grid_style(4, 1)}>{ "Updated By" }</div>
            <div class={classes!("sticky")} style={gen_grid_style(5, 1)}>{ "Updated At" }</div>
            { view }
       </div>
    }
}
