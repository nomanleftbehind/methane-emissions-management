use leptos::*;

use crate::{
    models::queries::facility::{all_facilities, AllFacilities},
    utils::load_data,
};

pub fn fetch_example() -> impl IntoView {
    let (cat_count, set_cat_count) = create_signal(0);

    // we use local_resource here because
    // 1) our error type isn't serializable/deserializable
    // 2) we're not doing server-side rendering in this example anyway
    //    (during SSR, create_resource will begin loading on the server and resolve on the client)
    let cats = create_resource(cat_count, |_| async move {
        load_data::<AllFacilities>(all_facilities::Variables {}).await
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

    // the renderer can handle Option<_> and Result<_> states
    // by displaying nothing for None if the resource is still loading
    // and by using the ErrorBoundary fallback to catch Err(_)
    // so we'll just use `.and_then()` to map over the happy path
    let cats_view = move || {
        cats.and_then(|data| {
            data.all_facilities
                .iter()
                .map(|s| view! { <p><img src={s}/></p> })
                .collect_view()
        })
    };

    view! {
        <div>
            <label>
                "How many cats would you like?"
                <input
                    type="number"
                    prop:value=move || cat_count.get().to_string()
                    on:input=move |ev| {
                        let val = event_target_value(&ev).parse::<CatCount>().unwrap_or(0);
                        set_cat_count(val);
                    }
                />
            </label>
            <Transition fallback=move || {
                view! { <div>"Loading (Suspense Fallback)..."</div> }
            }>
                <ErrorBoundary fallback>
                <div>
                    {cats_view}
                </div>
                </ErrorBoundary>
            </Transition>
        </div>
    }
}
