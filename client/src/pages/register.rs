use crate::{
    components::contexts::user_context::UserContext,
    hooks::{lazy_query, QueryResponse},
    models::mutations::user::{
        login::{LoginUserInput, ResponseData, Variables},
        Login,
    },
};
use web_sys::HtmlInputElement;
use yew::{
    function_component, html, use_context, use_state, Callback, Html, InputEvent, SubmitEvent,
    TargetCast,
};

#[function_component(Register)]
pub fn register() -> Html {
    let user_ctx = use_context::<UserContext>().unwrap();
    let email = use_state(|| "dsucic@bonterraenergy.com".to_string());
    let password = use_state(|| "everythinghastostartsomewhere".to_string());

    let onsubmit = {
        let email = email.clone();
        let password = password.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let user_ctx = user_ctx.clone();
            let variables = Variables {
                login_user_input: LoginUserInput {
                    email: (*email).clone(),
                    password: (*password).clone(),
                },
            };
            wasm_bindgen_futures::spawn_local(async move {
                if let QueryResponse {
                    data: Some(ResponseData { login }),
                    ..
                } = lazy_query::<Login>(variables).await
                {
                    user_ctx.dispatch(Some(login));
                }
            });
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
        <div class="data-window">
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
