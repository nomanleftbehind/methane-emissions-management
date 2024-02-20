use crate::routes::ExampleContext;
use leptos::{logging::log, *};
use leptos_router::*;

#[component]
pub fn About() -> impl IntoView {
    log!("rendering <About/>");

    on_cleanup(|| {
        log!("cleaning up <About/>");
    });

    log!(
        "ExampleContext should be Some(0). It is {:?}",
        use_context::<ExampleContext>()
    );

    // use_navigate allows you to navigate programmatically by calling a function
    let navigate = use_navigate();

    view! {
        <>
            // note: this is just an illustration of how to use `use_navigate`
            // <button on:click> to navigate is an *anti-pattern*
            // you should ordinarily use a link instead,
            // both semantically and so your link will work before WASM loads
            <button on:click=move |_| navigate("/", Default::default())>"Home"</button>
            <h1>"About"</h1>
            <p>
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."
            </p>
        </>
    }
}
