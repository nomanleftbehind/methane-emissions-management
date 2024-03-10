use crate::{
    components::ErrorTemplate,
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

    let facilities = create_resource(
        || (),
        |_| async move { load_data::<AllFacilities>(all_facilities::Variables {}).await },
    );

    let facility_list = move || {
        facilities.get().map(|response| {
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
                                    <A
                                        href=format!("{}/pneumatic_instruments", facility.id)
                                        class="sidebar-button"
                                    >
                                        {&facility.name}
                                    </A>
                                </li>
                            </For>
                        </ul>
                    </nav>
                }
            })
        })
    };

    view! {
        <Transition fallback=move || {
            view! { <p>"Loading facilities..."</p> }
        }>
            <ErrorBoundary fallback=|errors| {
                view! { <ErrorTemplate errors=errors/> }
            }>{facility_list}</ErrorBoundary>
        </Transition>
        <AnimatedOutlet class="emitters-window" outro="fadeOut" intro="fadeIn"/>
    }
}
