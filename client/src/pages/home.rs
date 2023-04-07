use crate::{
    components::{
        emitters_window::emitters_window::EmittersWindow, modal::error::Error,
        sidebar::sidebar::Sidebar,
    },
    utils::error::AppError,
};
use std::rc::Rc;
use uuid::Uuid;
use wasm_bindgen::JsCast;
use web_sys::HtmlDialogElement;
use yew::{classes, function_component, html, use_effect, use_state, use_state_eq, Callback, Html};

#[function_component(Home)]
pub fn home() -> Html {
    let selected_facility_id = use_state(|| Uuid::nil());

    let facility_id = Rc::new(*selected_facility_id);

    let error_handle = use_state_eq(|| None);
    let error = (*error_handle).clone();

    let modal_root_handle = use_state_eq(|| None);
    let modal_root = (*modal_root_handle).clone();
    use_effect(move || {
        let modal_root = gloo::utils::document()
            .get_element_by_id("modal-root")
            .expect("Expected to find a #modal-root element")
            .dyn_into::<HtmlDialogElement>()
            .expect("#modal-root is not a <dialog> element");

        modal_root_handle.set(Some(Rc::new(modal_root)));
    });

    let error_handle_callback = {
        let modal_root = modal_root.clone();
        let error_handle = error_handle.clone();
        Callback::from(move |e: Option<AppError>| {
            let modal_root = modal_root.clone();
            if let (Some(_), Some(modal_root)) = (&e, modal_root) {
                let _ = modal_root.show_modal();
            }
            error_handle.set(e);
        })
    };

    // use_effect_with_deps(
    //     move |u| {
    //         console_log!("prop id changed: {:#?}", u);
    //     },
    //     facility_id.clone(),
    // );

    let on_facility_click =
        Callback::from(move |facility_id: Uuid| selected_facility_id.set(facility_id));

    html! {
        <div class={classes!("data-window")}>
            <Sidebar {on_facility_click} facility_id={facility_id.clone()} />
            <EmittersWindow facility_id={facility_id.clone()} error_handle={error_handle_callback.clone()} />
            <Error error_handle={error_handle_callback}>
                if let Some(error) = error {
                    <>{error}</>
                } else {
                    <></>
                }
            </Error>
        </div>
    }
}
