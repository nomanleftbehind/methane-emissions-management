use yew::{classes, function_component, html, Callback, Html, MouseEvent, Properties};

use crate::utils::gen_style::gen_grid_style;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub toggle_insert_form_is_open: Callback<MouseEvent>,
    pub insert_form_is_open: bool,
}

#[function_component(InsertEntryButton)]
pub fn insert_entry_button(
    Props {
        toggle_insert_form_is_open,
        insert_form_is_open,
    }: &Props,
) -> Html {
    let display = if *insert_form_is_open { "x" } else { "+" };
    html! {
        <div class={classes!("sticky", "center")} style={gen_grid_style(1, 1)}>
            <button class={classes!("entry-button")} onclick={toggle_insert_form_is_open}>{ display }</button>
        </div>
    }
}
