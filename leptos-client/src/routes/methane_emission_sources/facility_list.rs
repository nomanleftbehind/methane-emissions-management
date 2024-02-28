use crate::{
    // get_contacts,
    models::queries::facility::{all_facilities, AllFacilities},
    routes::ExampleContext,
    utils::load_data,
};
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

    let fallback = move |errors: RwSignal<Errors>| {
        let error_list = move || {
            errors.with(|errors| {
                errors
                    .iter()
                    .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                    .collect_view()
            })
        };

        view! {
            <div class="error">
                <h2>"Error"</h2>
                <ul>{error_list}</ul>
            </div>
        }
    };

    // let location = use_location();
    let facilities = create_resource(
        // move || location.search.get(),
        /* get_contacts */
        || (),
        |_| async move { load_data::<AllFacilities>(all_facilities::Variables {}).await },
    );
    let facility_list = move || {
        facilities
            .get()
            .map(|response| {
                response.map(|data| {
                    view! {
                        <nav class="sidebar" role="navigation">
                            <ul class="sidebar-list">
                                <For
                                    each=move || data.all_facilities.clone()
                                    key=|facility| facility.id
                                    let:facility
                                >
                                    <li class="sidebar-button-container">
                                        <A href=facility.id.to_string() class="sidebar-button">
                                            // class=("active", "a" == "b")
                                            {&facility.name}
                                        </A>
                                    </li>
                                </For>
                            </ul>
                        </nav>
                    }
                })
            })
            .into_view()
    };

    view! {
        <Suspense fallback=move || {
            view! { <p>"Loading facilities..."</p> }
        }>
            <ErrorBoundary fallback>{facility_list}</ErrorBoundary>
        </Suspense>
        <AnimatedOutlet class="emitters-window" outro="fadeOut" intro="fadeIn"/>
    }
}
