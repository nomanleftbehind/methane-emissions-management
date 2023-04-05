use yew::{create_portal, function_component, html, Children, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Modal)]
pub fn modal(props: &Props) -> Html {
    let modal_host = gloo::utils::document()
        .get_element_by_id("modal_host")
        .expect("Expected to find a #modal_host element");

    create_portal(html! { {for props.children.iter()} }, modal_host.into())
}
