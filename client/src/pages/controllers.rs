use crate::{
    hooks::{use_query, QueryResponse},
    models::queries::controller::{
        get_controllers::{ControllersBy, ResponseData, Variables},
        GetControllers,
    },
    utils::console_log::console_log,
};
use uuid::uuid;
use yew::{function_component, html, Html};

// #[derive(Clone, Debug, Eq, PartialEq, Properties)]
// pub struct Props {
//     pub id: Uuid,
// }

#[function_component(ControllersPage)]
pub fn controllers_page() -> Html {
    let var = Variables {
        by: ControllersBy {
            facility_id: uuid!("3f34bb1e-dc92-4985-869f-27784a3708f6"),
        },
    };

    let get_controllers = use_query::<GetControllers>(var);
    console_log!("Hi: {:#?}", &get_controllers);

    let r = match get_controllers {
        QueryResponse {
            data: Some(ResponseData { controllers_by }),
            ..
        } => {
            let controllers_iter = controllers_by.into_iter().map(|c| {
                console_log!("ctr: {:#?}", c);
                let m = c.manufacturer.map_or("".to_string(), |v| v.manufacturer);
                html! {
                    <>
                        <div>{ c.id }</div>
                        <div>{ c.model }</div>
                        <div>{ c.serial_number }</div>
                        <div>{ m }</div>
                    </>
                }
            });

            html! { for controllers_iter }
        }
        QueryResponse { error: Some(e), .. } => {
            html! {e}
        }
        _ => {
            html! {}
        }
    };

    html! {r}
}
