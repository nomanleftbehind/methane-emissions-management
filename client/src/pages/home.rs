use crate::{
    components::{
        emitters_window::emitters_window::EmittersWindow,
        modal::modal_variant::ModalVariantComponent, sidebar::sidebar::Sidebar,
    },
    utils::error::AppError,
};
use std::rc::Rc;
use uuid::Uuid;
use wasm_bindgen::JsCast;
use web_sys::HtmlDialogElement;
use yew::{
    classes, function_component, html, use_effect_with_deps, use_state, use_state_eq, Callback,
    Html,
};

#[function_component(Home)]
pub fn home() -> Html {
    let selected_facility_id = use_state(|| None);

    let facility_id = (*selected_facility_id).clone();

    let modal_root_handle = use_state_eq(|| None);
    let modal_root = (*modal_root_handle).clone();
    use_effect_with_deps(
        move |_| {
            let modal_root = gloo::utils::document()
                .get_element_by_id("modal-root")
                .expect("Expected to find a #modal-root element")
                .dyn_into::<HtmlDialogElement>()
                .expect("#modal-root is not a <dialog> element");

            modal_root_handle.set(Some(Rc::new(modal_root)));
        },
        (),
    );

    let modal_variant_handle = use_state_eq(|| None);
    let modal_variant = (*modal_variant_handle).clone();

    let modal_variant_handle_callback = {
        let modal_root = modal_root.clone();
        let modal_variant_handle = modal_variant_handle.clone();
        Callback::from(move |e: Option<ModalVariant>| {
            let modal_root = modal_root.clone();
            if let (Some(_), Some(modal_root)) = (&e, modal_root) {
                let _ = modal_root.show_modal();
            }
            modal_variant_handle.set(e);
        })
    };

    let on_facility_click = Callback::from(move |facility_id: Uuid| {
        selected_facility_id.set(Some(Rc::new(facility_id)))
    });

    html! {
        <div class={classes!("data-window")}>
            <Sidebar {on_facility_click} facility_id={facility_id.clone()} modal_variant_handle={modal_variant_handle_callback.clone()} />
            <EmittersWindow {facility_id} modal_variant_handle={modal_variant_handle_callback.clone()} />
            <ModalVariantComponent modal_variant={modal_variant} modal_variant_handle={modal_variant_handle_callback} />
        </div>
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum ModalVariant {
    Error(AppError),
    ConfirmDelete(Callback<()>),
}
