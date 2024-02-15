mod api;
pub mod list_facilities;
pub mod models;
pub mod routes;
pub mod utils;

use crate::{api::*, routes::ExampleContext};
use leptos::{logging::log, *};
use leptos_router::*;

#[derive(Params, PartialEq, Clone, Debug)]
pub struct ContactParams {
    id: usize,
}

#[component]
pub fn Contact() -> impl IntoView {
    log!("rendering <Contact/>");

    log!(
        "ExampleContext should be Some(42). It is {:?}",
        use_context::<ExampleContext>()
    );

    on_cleanup(|| {
        log!("cleaning up <Contact/>");
    });

    let params = use_params::<ContactParams>();
    let contact = create_resource(
        move || params().map(|params| params.id).ok(),
        // any of the following would work (they're identical)
        // move |id| async move { get_contact(id).await }
        // move |id| get_contact(id),
        // get_contact
        get_contact,
    );

    create_effect(move |_| {
        log!("params = {:#?}", params.get());
    });

    let contact_display = move || match contact.get() {
        // None => loading, but will be caught by Suspense fallback
        // I'm only doing this explicitly for the example
        None => None,
        // Some(None) => has loaded and found no contact
        Some(None) => Some(view! { <p>"No contact with this ID was found."</p> }.into_any()),
        // Some(Some) => has loaded and found a contact
        Some(Some(contact)) => Some(
            view! {
                <section class="card">
                    <h1>{contact.first_name} " " {contact.last_name}</h1>
                    <p>{contact.address_1} <br/> {contact.address_2}</p>
                </section>
            }
            .into_any(),
        ),
    };

    view! {
        <div class="contact">
            <Transition fallback=move || {
                view! { <p>"Loading..."</p> }
            }>{contact_display}</Transition>
        </div>
    }
}
