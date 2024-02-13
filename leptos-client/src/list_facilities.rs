use crate::{
    models::queries::facility::{
        all_facilities::{self, AllFacilitiesAllFacilities},
        AllFacilities,
    },
    utils::load_data,
};
use leptos::*;

#[component]
pub fn list_facilities() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    let facilities = create_resource(
        || (),
        |_| async move { load_data::<AllFacilities>(all_facilities::Variables {}).await },
    );

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

    // the renderer can handle Option<_> and Result<_> states
    // by displaying nothing for None if the resource is still loading
    // and by using the ErrorBoundary fallback to catch Err(_)
    // so we'll just use `.and_then()` to map over the happy path
    let facilities_view = move || {
        facilities.and_then(|data| {
            data.all_facilities
                .iter()
                .map(
                    |AllFacilitiesAllFacilities {
                         id,
                         idpa,
                         name,
                         type_,
                     }| {
                        view! {
                            <button
                                class=("sidebar-button", move || count() % 2 == 1)
                                class:active=move || "a" == "b"
                                on:click=move |_| set_count.update(|n| *n += 1)
                            >
                                {name}
                            </button>
                        }
                    },
                )
                .collect_view()
        })
    };

    view! {
        <div>
            <Transition fallback=move || {
                view! { <div>"Loading (Suspense Fallback)..."</div> }
            }>
                <ErrorBoundary fallback>
                    <li class="sidebar-button-container">{facilities_view}</li>
                </ErrorBoundary>
            </Transition>
        </div>
    }
}
