use yew::{classes, function_component, html, Callback, Html, MouseEvent, Properties};

use crate::utils::gen_style::gen_grid_style;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub handle_open_insert_form: Callback<MouseEvent>,
    pub open_insert_form: bool,
}

#[function_component(InsertEntryButton)]
pub fn insert_entry_button(
    Props {
        handle_open_insert_form,
        open_insert_form,
    }: &Props,
) -> Html {
    let display = if *open_insert_form { "x" } else { "+" };
    html! {
        <div class={classes!("sticky", "center")} style={gen_grid_style(1, 1)}>
            <button class={classes!("entry-button")} onclick={handle_open_insert_form}>{ display }</button>
        </div>
    }
}
