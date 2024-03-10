use leptos::{component, view, CollectView, Errors, IntoView, RwSignal, SignalWith};

#[component]
pub fn ErrorTemplate(errors: RwSignal<Errors>) -> impl IntoView {
    let error_list = move || {
        errors.with(|errors| {
            errors
                .iter()
                .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                .collect_view()
        })
    };

    view! {
        <div>
            <p>"Error:"</p>
            <ul>{error_list}</ul>
        </div>
    }
}
