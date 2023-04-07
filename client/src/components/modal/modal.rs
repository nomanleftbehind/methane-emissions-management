use wasm_bindgen::JsCast;
use web_sys::HtmlDialogElement;
use yew::{
    create_portal, function_component, html, use_effect, use_state_eq, Children, Html, Properties,
};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Modal)]
pub fn modal(props: &Props) -> Html {
    let element_handle = use_state_eq(|| None);
    let element = (*element_handle).clone();

    use_effect(move || {
        let modal_root = gloo::utils::document()
            .get_element_by_id("modal-root")
            .expect("Expected to find a #modal-root element")
            .dyn_into::<HtmlDialogElement>()
            .expect("#modal-root is not a <dialog> element");

        element_handle.set(Some(modal_root));
    });

    if let Some(modal_root) = element {
        create_portal(html! { {for props.children.iter()} }, modal_root.into())
    } else {
        Html::default()
    }
}
