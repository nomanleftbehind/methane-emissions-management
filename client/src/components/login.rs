use crate::{
    hooks::use_query_async,
    models::mutations::user::{
        login::{LoginUserInput, Variables},
        Login as LoginUser,
    },
};
use web_sys::HtmlInputElement;
use yew::{
    function_component, html, use_state, Callback, Html, InputEvent, SubmitEvent, TargetCast,
};
use yew_hooks::use_async;

#[function_component(Login)]
pub fn login() -> Html {
    let email = use_state(|| "dsucic@bonterraenergy.com".to_string());
    let password = use_state(|| "everythinghastostartsomewhere".to_string());

    let run_login = {
        let variables = Variables {
            login_user_input: LoginUserInput {
                email: (*email).clone(),
                password: (*password).clone(),
            },
        };
        use_async(async move { use_query_async::<LoginUser>(variables).await })
    };

    let onsubmit = {
        let run_async = run_login.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            run_async.run();
        })
    };

    let oninput_email = {
        let email = email.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let info = input.value();
            email.set(info)
        })
    };

    let oninput_password = {
        let password = password.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let info = input.value();
            password.set(info)
        })
    };

    html! {
        <div class="auth-page">
            <div class="container page">
                <div class="row">
                    <div class="col-md-6 offset-md-3 col-xs-12">
                        <h1 class="text-xs-center">{ "Sign In" }</h1>
                        // <p class="text-xs-center">
                        //     <Link<AppRoute> to={AppRoute::Register}>
                        //         { "Need an account?" }
                        //     </Link<AppRoute>>
                        // </p>
                        <form {onsubmit}>
                            <fieldset>
                                <fieldset class="form-group">
                                    <input
                                        class="form-control form-control-lg"
                                        type="email"
                                        placeholder="Email"
                                        value={(*email).clone()}
                                        oninput={oninput_email}
                                        />
                                </fieldset>
                                <fieldset class="form-group">
                                    <input
                                        class="form-control form-control-lg"
                                        type="password"
                                        placeholder="Password"
                                        value={(*password).clone()}
                                        oninput={oninput_password}
                                        />
                                </fieldset>
                                <button
                                    class="btn btn-lg btn-primary pull-xs-right"
                                    type="submit"
                                    disabled=false>
                                    { "Sign in" }
                                </button>
                            </fieldset>
                        </form>
                    </div>
                </div>
            </div>
        </div>
    }
}
