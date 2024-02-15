use crate::{get_contacts, routes::ExampleContext};
use leptos::{logging::log, *};
use leptos_router::*;

#[component]
pub fn FacilityList() -> impl IntoView {
    log::debug!("rendering <FacilityList/>");

    // contexts are passed down through the route tree
    provide_context(ExampleContext(42));

    on_cleanup(|| {
        log!("cleaning up <FacilityList/>");
    });

    let location = use_location();
    let facilities = create_resource(move || location.search.get(), get_contacts);
    let facilities = move || {
        facilities.get().map(|facilities| {
            // this data doesn't change frequently so we can use .map().collect() instead of a keyed <For/>
            facilities
                .into_iter()
                .map(|contact| {
                    view! {
                        <li>
                            <A href=contact.id.to_string()>
                                <span>{&contact.first_name} " " {&contact.last_name}</span>
                            </A>
                        </li>
                    }
                })
                .collect_view()
        })
    };

    view! {
        <div class="contact-list">
            <h1>"Facilities"</h1>
            <Suspense fallback=move || {
                view! { <p>"Loading facilities..."</p> }
            }>
                {move || {
                    view! {
                        <nav class="sidebar" role="navigation">
                            <ul class="sidebar-list">{facilities}</ul>
                        </nav>
                    }
                }}
            </Suspense>
            <AnimatedOutlet class="outlet" outro="fadeOut" intro="fadeIn"/>
        </div>
    }
}
