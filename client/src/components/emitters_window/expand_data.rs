use crate::components::emitters_window::expand_svg::ExpandSvgComponent;
use yew::{classes, function_component, html, Callback, Html, MouseEvent, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub row_num: usize,
    pub col_num: usize,
    pub expanded: bool,
    pub handle_expand_data: Callback<MouseEvent>,
}

#[function_component(ExpandDataComponent)]
pub fn expand_data_component(
    Props {
        row_num,
        col_num,
        handle_expand_data,
        expanded,
    }: &Props,
) -> Html {
    let style = format!("grid-row: {}; grid-column: {};", row_num, col_num);

    html! {
        <div class={classes!("emitter-cell", "center")} {style}>
            <button class={classes!("entry-button")} onclick={handle_expand_data}>
                <ExpandSvgComponent {expanded} />
            </button>
        </div>
    }
}
