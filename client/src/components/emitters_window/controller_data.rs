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
                <div class={classes!("emitter-data-side")} style="transform: translateX(min(1037.23px - 100% - var(--spacer-4), -1 * var(--spacer-1))); max-width: calc(1891px - 2 * var(--spacer-4));">
                </div>
            </div>
        </div>
    }
}
