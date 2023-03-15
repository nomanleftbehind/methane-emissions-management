use crate::{
    hooks::{use_query, use_query_async, QueryResponse},
    models::queries::user::{
        me::{MeMe, ResponseData, Variables},
        Me,
    },
    utils::console_log,
};
use uuid::Uuid;
use yew::{
    function_component, html, use_context, use_effect_with_deps, use_reducer, Children,
    ContextProvider, Html, Properties, Reducible, UseReducerHandle,
};
use yew_hooks::{use_async, use_mount};

use crate::hooks::use_query_with_deps;

#[derive(Debug, PartialEq, Clone)]
pub struct User {
    pub inner: Option<MeMe>,
}

impl Reducible for User {
    type Action = Option<MeMe>;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        Self { inner: action }.into()
    }
}

pub type UserContext = UseReducerHandle<User>;

#[derive(Properties, Debug, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(UserProvider)]
pub fn user_provider(props: &Props) -> Html {
    let user_ctx = use_reducer(|| User { inner: None });

    let current_user = use_async(async move { use_query_async::<Me>(Variables).await });

    {
        let current_user = current_user.clone();
        use_mount(move || {
            current_user.run();
        });
    }

    {
        let user_ctx = user_ctx.clone();
        use_effect_with_deps(
            move |current_user| {
                if let Some(QueryResponse {
                    data: Some(ResponseData { me: Some(user) }),
                    ..
                }) = &current_user.data
                {
                    console_log!("user: {:#?}", &user);
                    user_ctx.dispatch(Some(user.clone()));
                } else {
                    console_log!("Not called");
                }
                || ()
            },
            current_user,
        );
    };

    html! {
        <ContextProvider<UserContext> context={user_ctx}>
            {props.children.clone()}
        </ContextProvider<UserContext>>
    }
}

#[function_component(Producer)]
pub fn producer() -> Html {
    let user_ctx = use_context::<UserContext>().unwrap();

    let user = user_ctx.clone();

    println!("producer: {:#?}", &user);

    let id = user.inner.as_ref().and_then(|v| Some(v.email.clone()));

    // let disp = user.as_ref().and_then(|s| Some(&s.email));

    // let user = use_query_with_deps::<Me, _>(Variables, ());

    // user.data.map_or_else(
    //     || (),
    //     |e| e.me.map_or_else(|| (), |me| {

    //         console_log!("me: {:#?}", &me);
    //         user_ctx.dispatch(Some(me.id))}),
    // );

    html! {
        <div>{ id }</div>
    }
}
