use crate::components::modal::modal::Modal;
use yew::{function_component, html, Callback, Children, Html, MouseEvent, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    pub on_error_dialog_button_click: Callback<MouseEvent>,
}

#[function_component(Error)]
pub fn error(
    Props {
        on_error_dialog_button_click,
        children,
    }: &Props,
) -> Html {
    html! {
        <Modal>
            <form method="dialog">
                <div>
                    { for children.iter() }
                </div>
                <button onclick={on_error_dialog_button_click}>{"Confirm"}</button>
            </form>
        </Modal>
    }
}
