use crate::{components::modal::modal::Modal, utils::error::AppError};
use yew::{function_component, html, Callback, Children, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    pub error_handle: Callback<Option<AppError>>,
}

#[function_component(Error)]
pub fn error(
    Props {
        children,
        error_handle,
    }: &Props,
) -> Html {
    let error_handle = error_handle.clone();

    let onclick = Callback::from(move |_| {
        error_handle.emit(None);
    });

    html! {
        <Modal>
            <form method="dialog">
                <div>
                    { for children.iter() }
                </div>
                <button {onclick}>{"Confirm"}</button>
            </form>
        </Modal>
    }
}
