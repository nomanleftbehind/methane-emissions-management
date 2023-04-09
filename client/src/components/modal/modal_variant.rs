use crate::{
    components::modal::modal::Modal,
    pages::ModalVariant::{self, ConfirmDelete, Error},
};
use yew::{function_component, html, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub modal_variant: Option<ModalVariant>,
    pub modal_variant_handle: Callback<Option<ModalVariant>>,
}

#[function_component(ModalVariantComponent)]
pub fn modal_variant(
    Props {
        modal_variant,
        modal_variant_handle,
    }: &Props,
) -> Html {
    let modal_variant_handle = modal_variant_handle.clone();

    let view = match modal_variant {
        Some(modal_variant) => match modal_variant {
            Error(error) => html! { error },
            ConfirmDelete(_) => html! { "Are you sure you want to delete selected entry?" },
        },
        None => Html::default(),
    };

    let handle_delete_entry = match modal_variant {
        Some(ModalVariant::ConfirmDelete(delete_entry_callback)) => {
            Some(delete_entry_callback.clone())
        }
        _ => None,
    };

    let show_cancel_button = handle_delete_entry.is_some();

    let onclick_ok = {
        let modal_variant_handle = modal_variant_handle.clone();
        Callback::from(move |_| {
            let handle_delete_entry = handle_delete_entry.clone();
            if let Some(handle_delete_entry) = handle_delete_entry {
                handle_delete_entry.emit(());
            }
            modal_variant_handle.emit(None);
        })
    };

    let onclick_cancel = Callback::from(move |_| {
        modal_variant_handle.emit(None);
    });

    html! {
        <Modal>
        // <form> attribute method="dialog" is necessary to close the <dialog> on button click.
            <form method="dialog">
                <div>
                    { view }
                </div>
                <div>
                    <button onclick={onclick_ok}>{"Ok"}</button>
                    <button onclick={onclick_cancel} hidden={!show_cancel_button}>{"Cancel"}</button>
                </div>
            </form>
        </Modal>
    }
}
