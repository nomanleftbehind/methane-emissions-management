use std::rc::Rc;
use uuid::Uuid;
use yew::{classes, function_component, html, Html, Properties};

use crate::components::emitters_window::controller_changes::ControllerChangeComp;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: Uuid,
    pub row_num: usize,
    pub col_num: usize,
}

#[function_component(ControllerDataComponent)]
pub fn controller_data_component(
    Props {
        id,
        row_num,
        col_num,
    }: &Props,
) -> Html {
    let style = format!("grid-row: {}; grid-column: 3/{};", row_num, col_num + 1);

    let controller_id = Rc::new(*id);

    html! {
        <div class={classes!("emitter-data-wrapper")} {style}>
            <div class={classes!("emitter-data")}>
                <div class={classes!("emitter-data-side")}>
                    <button>{ "Changes" }</button>
                    <button>{ "Month Vent Overrides" }</button>
                </div>
                <ControllerChangeComp {controller_id} />
            </div>
        </div>
    }
}
