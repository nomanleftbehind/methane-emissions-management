use std::fmt::Display;

/// `FacilityType` is an externally defined enum inside schema, so we have to provide matching Rust type and `Display` trait implementation.
///
/// It is defined in common library so it can be used by both server and client.
#[cfg_attr(
    feature = "server",
    derive(async_graphql::Enum, sqlx::Type),
    sqlx(type_name = "facility_type")
)]
#[cfg_attr(
    feature = "client",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "UPPERCASE")
)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum FacilityType {
    TM,
    WT,
    CT,
    DS,
    GS,
    MS,
    GP,
    IF,
    PL,
    WP,
    WS,
    BT,
}

impl Display for FacilityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FacilityType::TM => write!(f, "TM"),
            FacilityType::WT => write!(f, "WT"),
            FacilityType::CT => write!(f, "CT"),
            FacilityType::DS => write!(f, "DS"),
            FacilityType::GS => write!(f, "GS"),
            FacilityType::MS => write!(f, "MS"),
            FacilityType::GP => write!(f, "GP"),
            FacilityType::IF => write!(f, "IF"),
            FacilityType::PL => write!(f, "PL"),
            FacilityType::WP => write!(f, "WP"),
            FacilityType::WS => write!(f, "WS"),
            FacilityType::BT => write!(f, "BT"),
        }
    }
}

/// `Role` is an externally defined enum inside schema, so we have to provide matching Rust type and `Display` trait implementation.
///
/// It is defined in common library so it can be used by both server and client.
#[cfg_attr(
    feature = "server",
    derive(async_graphql::Enum, sqlx::Type),
    sqlx(type_name = "user_role", rename_all = "UPPERCASE")
)]
#[cfg_attr(
    feature = "client",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "UPPERCASE")
)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Role {
    Admin,
    Engineer,
    Regulatory,
    Office,
    Operator,
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::Admin => write!(f, " Admin"),
            Role::Engineer => write!(f, " Engineer"),
            Role::Regulatory => write!(f, " Regulatory"),
            Role::Office => write!(f, " Office"),
            Role::Operator => write!(f, " Operator"),
        }
    }
}

// #[function_component(Comp)]
// fn comp(_props: &ChildrenProps) -> Html {
//     Html::default()
// }
// #[function_component(Provider)]
// fn provider(props: &ChildrenProps) -> Html {
//     let children = props.children.clone();
//     html! { <>{children}</> }
// }
// type Provider1 = Provider;
// type Provider2 = Provider;

// #[derive(Properties, PartialEq, Eq, Debug)]
// pub struct ServerAppProps {
//     pub url: AttrValue,
// }

// #[function_component(ServerApp)]
// pub fn server_app(props: &ServerAppProps) -> Html {
//     let history = AnyHistory::from(MemoryHistory::new());
//     history.push(&*props.url);

//     html! {
//         <Router history={history}>
//             <PhantomComponent<Provider1> />
//             <main class="main">
//                 <PhantomComponent<Provider2> />
//             </main>
//         </Router>
//     }
// }

// use crate::{components::logout::Logout, Route};
// use yew::{classes, function_component, html, Html};
// use yew_router::components::Link;
