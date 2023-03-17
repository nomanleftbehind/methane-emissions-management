use yew::{classes, function_component, html, use_context, Html};

use crate::components::{contexts::user_context::UserContext, navigation::logout::Logout};

#[function_component(Dropdown)]
pub fn dropdown() -> Html {
    let user_ctx = use_context::<UserContext>().unwrap();
    let user_email = user_ctx.0.as_ref().map(|u| &u.email);

    html! {
        if user_email.is_some() {
            <div class="navbar-end">
                <div class={classes!("dropdown")}>
                    <div class={classes!("open-dropdown-button")}>
                        { user_email }
                    </div>
                    <div class={classes!("dropdown-list")}>
                        <Logout />
                    </div>
                </div>
            </div>
        }
    }
}
