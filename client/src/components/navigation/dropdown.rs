use crate::components::{contexts::user_context::UserContext, navigation::logout::Logout};
use gloo_timers::callback::Timeout;
use yew::{classes, function_component, html, use_context, use_state, Callback, Html};

#[function_component(Dropdown)]
pub fn dropdown() -> Html {
    let user_ctx = use_context::<UserContext>().unwrap();
    let user_email = user_ctx.0.as_ref().map(|u| &u.email);

    let timeout = use_state(|| None);
    let show_dropdown = use_state(|| (None, true));

    let show_dropdown_onmouseenter = show_dropdown.clone();
    let timeout_onmouseenter = timeout.clone();
    let onmouseenter = Callback::from(move |_| {
        show_dropdown_onmouseenter.set((Some("expanded"), false));
        timeout_onmouseenter.set(None);
    });

    let show_dropdown_onmouseleave = show_dropdown.clone();
    let timeout_onmouseleave = timeout.clone();
    let onmouseleave = Callback::from(move |_| {
        let show_dropdown_onmouseleave = show_dropdown_onmouseleave.clone();
        let timeout = Timeout::new(200, move || {
            show_dropdown_onmouseleave.set((None, true));
        });
        timeout_onmouseleave.set(Some(timeout));
    });

    let show_dropdown_on_logout = show_dropdown.clone();
    let on_logout = Callback::from(move |_| {
        show_dropdown_on_logout.set((None, true));
    });

    let (class_expanded, element_hidden) = *show_dropdown;

    html! {
        if user_email.is_some() {
            <div class="navbar-end">
                <div class={classes!("popover", class_expanded)} {onmouseenter} {onmouseleave} role="menuitem">
                    <div class={classes!("navbar-button", class_expanded)}>
                        { user_email }
                    </div>
                    <div hidden={element_hidden}>
                        <div class={classes!("dropdown")} style="transform: translateX(min(929.25px - 100% - var(--spacer-4), -1 * var(--spacer-1))); max-width: calc(1675px - 2 * var(--spacer-4));">
                            <div style="background-color: var(--yellow-dark);" role="menu" aria-label="Design">
                                <Logout {on_logout} />
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
