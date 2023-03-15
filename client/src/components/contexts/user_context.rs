use crate::models::mutations::user::login::LoginLogin;
use yew::{
    function_component, html, use_context, use_reducer, Children, ContextProvider, Html,
    Properties, Reducible, UseReducerHandle,
};

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct User(pub Option<LoginLogin>);

impl Reducible for User {
    type Action = Option<LoginLogin>;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        Self(action).into()
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
    let user_ctx = use_reducer(|| User(None));

    html! {
        <ContextProvider<UserContext> context={user_ctx}>
            {props.children.clone()}
        </ContextProvider<UserContext>>
    }
}

#[function_component(Producer)]
pub fn producer() -> Html {
    let user_ctx = use_context::<UserContext>().unwrap();
    let user = (&*user_ctx).0.as_ref().map(|x| &x.email);

    html! {
        <div>{ user }</div>
    }
}
