use crate::{
    hooks::{use_query, QueryResponse},
    models::queries::controller::{
        user_controllers::{ResponseData, Variables},
        UserControllers,
    }, utils::console_log::console_log,
};
use yew::{function_component, html, Html};
use uuid::uuid;

#[function_component(ControllersPage)]
pub fn controllers_page() -> Html {

    // let var = Variables {
    //     by: ControllersBy {
    //         facility_id: Some(uuid!("c86b6414-83d0-4fab-8766-90aa8fa7c710")),
    //         created_by_id: None,
    //         updated_by_id: None,
    //     },
    // };
    // console_log!("Hi: {:#?}", &var);

    let get_controllers = use_query::<UserControllers>(Variables);

    let r = match get_controllers {
        Ok(QueryResponse {
            data: Some(ResponseData { user_controllers }),
            ..
        }) => {
            let controllers_iter = user_controllers.into_iter().map(|c| {

                console_log!("ctr: {:#?}", c);
                let m = if let Some(m) = c.manufacturer {
                    m.manufacturer
                } else {
                    "".to_string()
                };
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
        Ok(r) => {
            console_log!("resp: {:#?}", &r);
            html! {}},
        Err(e) => {
            html! {e}
        }
    };

    html! {r}
}
