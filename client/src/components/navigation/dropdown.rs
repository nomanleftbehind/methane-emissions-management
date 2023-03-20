use gloo_timers::callback::Timeout;
use yew::{classes, function_component, html, use_context, use_state, Callback, Html};

use crate::components::{contexts::user_context::UserContext, navigation::logout::Logout};

#[function_component(Dropdown)]
pub fn dropdown() -> Html {
    let user_ctx = use_context::<UserContext>().unwrap();
    let user_email = user_ctx.0.as_ref().map(|u| &u.email);

    let timeout = use_state(|| None);
    let class_expanded = use_state(|| None);
    let element_hidden = use_state(|| true);

    let class_expanded_onmouseenter = class_expanded.clone();
    let element_hidden_onmouseenter = element_hidden.clone();
    let timeout_onmouseenter = timeout.clone();
    let onmouseenter = Callback::from(move |_| {
        class_expanded_onmouseenter.set(Some("expanded"));
        element_hidden_onmouseenter.set(false);
        timeout_onmouseenter.set(None);
    });

    let class_expanded_onmouseleave = class_expanded.clone();
    let element_hidden_onmouseleave = element_hidden.clone();
    let timeout_onmouseleave = timeout.clone();
    let onmouseleave = Callback::from(move |_| {
        let class_expanded_onmouseleave = class_expanded_onmouseleave.clone();
        let element_hidden_onmouseleave = element_hidden_onmouseleave.clone();
        let timeout = Timeout::new(200, move || {
            class_expanded_onmouseleave.set(None);
            element_hidden_onmouseleave.set(true);
        });
        timeout_onmouseleave.set(Some(timeout));
    });

    let class_expanded = *class_expanded;
    let element_hidden = *element_hidden;

    html! {
        if user_email.is_some() {
            <div class="navbar-end">
                <div class={classes!("popover", class_expanded)} {onmouseenter} {onmouseleave} role="menuitem" aria-expanded="true" aria-controls=":R36l:">
                    <div class={classes!("navbar-button")}>
                        { user_email }
                    </div>
                    <div id=":R36l:" hidden={element_hidden}>
                        <div class={classes!("dropdown")} style="transform: translateX(min(929.25px - 100% - var(--spacer-4), -1 * var(--spacer-1))); max-width: calc(1675px - 2 * var(--spacer-4));">
                            <div style="background-color: var(--yellow-dark);" role="menu" aria-label="Design">
                                <Logout />
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
