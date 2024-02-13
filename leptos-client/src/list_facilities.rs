use crate::{
    models::queries::facility::{all_facilities, AllFacilities},
    utils::load_data,
};
use leptos::*;

#[component]
pub fn list_facilities() -> impl IntoView {
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

    #[allow(clippy::bind_instead_of_map)]
    let facilities_view = move || {
        facilities.get().and_then(|response| match response {
            Ok(all_facilities::ResponseData { all_facilities }) => Some(
                view! {
                    <li class="sidebar-button-container">
                        <For
                            each=move || all_facilities.clone()
                            key=|fac| fac.id
                            children=|fac| {
                                view! {
                                    <button
                                        class=("sidebar-button", "a" == "b")
                                        class:active=move || "a" == "b"
                                    >
                                        // on:click=move |_| set_count.update(|n| *n += 1)
                                        {fac.name}
                                    </button>
                                }
                            }
                        />

                    </li>
                }
                .into_any(),
            ),
            Err(error) => Some(view! { <p>Not Fallback: {error.to_string()}</p> }.into_any()),
        })
    };

    view! {
        <div>
            <Transition fallback=move || {
                view! { <div>"Loading (Suspense Fallback)..."</div> }
            }>
                <ErrorBoundary fallback>{facilities_view}</ErrorBoundary>
            </Transition>
        </div>
    }
}
