use leptos::{logging::log, *};

#[component]
pub fn Register() -> impl IntoView {
    log!("rendering <Register/>");

    on_cleanup(|| {
        log!("cleaning up <Register/>");
    });

    view! {
        <>
            <h1>"Register"</h1>
            <form>
                <fieldset>
                    <legend>"Name"</legend>
                    <input type="text" name="first_name" placeholder="First"/>
                    <input type="text" name="last_name" placeholder="Last"/>
                </fieldset>
                <pre>"This page is just a placeholder."</pre>
            </form>
        </>
    }
}
