use crate::{models::queries::user::{me::Variables, Me}, utils::console_log};
use uuid::Uuid;
use yew::{
    function_component, html, use_context, use_reducer, Children, ContextProvider, Html,
    Properties, Reducible, UseReducerHandle,
};

use crate::hooks::use_query_with_deps;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct User {
    pub inner: Option<Uuid>,
}

impl Reducible for User {
    type Action = Option<Uuid>;

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
    let user = use_reducer(|| User { inner: None });

    html! {
        <ContextProvider<UserContext> context={user}>
            {props.children.clone()}
        </ContextProvider<UserContext>>
    }
}

#[function_component(Producer)]
pub fn producer() -> Html {
    let user_ctx = use_context::<UserContext>().unwrap();

    let user = user_ctx.inner;

    // let user = use_query_with_deps::<Me, _>(Variables, ());

    // user.data.map_or_else(
    //     || (),
    //     |e| e.me.map_or_else(|| (), |me| {
            
    //         console_log!("me: {:#?}", &me);
    //         user_ctx.dispatch(Some(me.id))}),
    // );

    html! {
        <div>{ user }</div>
    }
}
