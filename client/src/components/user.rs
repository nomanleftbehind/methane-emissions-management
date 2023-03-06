use crate::hooks::{use_query, QueryResponse};
use crate::models::queries::user::{
    get_users::{ResponseData, Variables},
    GetUsers,
};
use crate::utils::console_log::console_log;
use yew::{function_component, html, virtual_dom::VNode, Html};

#[function_component(Users)]
pub fn users() -> Html {
    let get_users = use_query::<GetUsers>(Variables);

    let users = match get_users {
        Ok(QueryResponse {
            data: Some(ResponseData { all_users }),
            ..
        }) => {
            let user_iter = all_users.into_iter().map(|user| {
                console_log!("role: {:#?}", user.role);
                html! {
                    <tr>
                        <td> { user.id } </td>
                        <td> { user.email } </td>
                        <td> { user.first_name } </td>
                        <td> { user.last_name } </td>
                        <td> { user.role } </td>
                    </tr>
                }
            });

            html! { <>{ for user_iter }</> }
        }
        Ok(_) => VNode::default(),
        Err(e) => {
            html! {
                <tr>
                    <td>{ e }</td>
                </tr>
            }
        }
    };

    html! {
        <>
            <h1>{ "All Users" }</h1>
            <table class="table-test">
                <thead>
                    <tr>
                        <th> { "Id" } </th>
                        <th> { "Email" } </th>
                        <th> { "First Name" } </th>
                        <th> { "Last Name" } </th>
                        <th> { "Role" } </th>
                    </tr>
                </thead>
                <tbody>
                    { users }
                </tbody>
            </table>
        </>
    }
}
