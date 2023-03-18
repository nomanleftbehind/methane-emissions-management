use gloo_timers::callback::Timeout;
use yew::{
    classes, function_component, html, use_context, use_effect_with_deps, use_state, Callback, Html,
};

use crate::{
    components::{contexts::user_context::UserContext, navigation::logout::Logout},
    utils::console_log,
};

#[function_component(Dropdown)]
pub fn dropdown() -> Html {
    let user_ctx = use_context::<UserContext>().unwrap();
    let user_email = user_ctx.0.as_ref().map(|u| &u.email);

    let call_onmouseleave = use_state(|| true);
    let class_expanded = use_state(|| None);
    let element_hidden = use_state(|| true);

    let t = (*call_onmouseleave).clone();

    let call_onmouseleave_onmouseover = call_onmouseleave.clone();
    let class_expanded_onmouseover = class_expanded.clone();
    let element_hidden_onmouseover = element_hidden.clone();
    let onmouseover = Callback::from(move |_| {
        // Prevent onmouseleave from hiding dropdown menu.
        call_onmouseleave_onmouseover.set(false);

        class_expanded_onmouseover.set(Some("expanded"));
        element_hidden_onmouseover.set(false);
    });

    let call_onmouseleave_onmouseleave = call_onmouseleave.clone();
    let class_expanded_onmouseleave = class_expanded.clone();
    let element_hidden_onmouseleave = element_hidden.clone();
    let onmouseleave = Callback::from(move |_| {
        // Prevent onmouseleave from hiding dropdown menu.
        call_onmouseleave_onmouseleave.set(true);
        let class_expanded_onmouseleave = class_expanded_onmouseleave.clone();
        let element_hidden_onmouseleave = element_hidden_onmouseleave.clone();

        let call_onmouseleave = call_onmouseleave.clone();
        let timeout = Timeout::new(500, move || {
            let w = *call_onmouseleave;
            console_log!("inside: {}", w);
            if w {
                class_expanded_onmouseleave.set(None);
                element_hidden_onmouseleave.set(true);
            }
        });
        timeout.forget();
    });

    use_effect_with_deps(
        move |t| {
            console_log!("Call: {}", t);
        },
        t,
    );

    let class_expanded = *class_expanded;
    let element_hidden = *element_hidden;

    html! {
        if user_email.is_some() {
            <div class="navbar-end">
                <div class={classes!("popover", class_expanded)} onmouseover={onmouseover.clone()} {onmouseleave} role="menuitem" aria-expanded="true" aria-controls=":R36l:">
                    <div class={classes!("navbar-button")}>
                        { user_email }
                    </div>
                    <div id=":R36l:" hidden={element_hidden}>
                        <div {onmouseover} class={classes!("dropdown")} style="transform: translateX(min(929.25px - 100% - var(--spacer-4), -1 * var(--spacer-1))); max-width: calc(1675px - 2 * var(--spacer-4));">
                            <div style="background-color: var(--yellow-dark);" role="menu" aria-label="Design">
                                <Logout />
                                <Logout />
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
