use uuid::Uuid;
use yew::{classes, function_component, html, Html, Properties};

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

    html! {
        <div class={classes!("emitter-data")} {style}>
            <div class={classes!("emitter-data-entry")}>
                <div class={classes!("emitter-data-side")}>
                    <button role="menuitem">{ "Changes" }</button>
                    <button role="menuitem">{ "Month Vent Overrides" }</button>
                </div>
            </div>
        </div>
    }
}
