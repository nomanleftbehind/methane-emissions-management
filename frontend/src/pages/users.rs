use graphql_client::GraphQLQuery;
use serde_json::Value;
use std::fmt::Debug;

use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{Request, RequestInit, RequestMode, Response};
use yew::{html, Component, Context, Html};

use crate::util::common::gql_uri;
use crate::util::common::FetchError;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/all_users.graphql",
    response_derives = "Debug"
)]
struct AllUsers;

async fn fetch_users() -> Result<Vec<Value>, FetchError> {
    let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJlbWFpbCI6ImFzZmZhQGRzYWZhLmNvbSIsInVzZXJuYW1lIjoi5a-G56CBMTExIiwiZXhwIjoxMDAwMDAwMDAwMH0.NyEN13J5trkn9OlRqWv2xMHshysR9QPWclo_-q1cbF4y_9rbkpSI6ern-GgKIh_ED0Czk98M1fJ6tzLczbdptg";
    let build_query = AllUsers::build_query(all_users::Variables {
        token: token.to_string(),
    });
    let query = serde_json::json!(build_query);

    let mut req_opts = RequestInit::new();
    req_opts.method("POST");
    req_opts.body(Some(&JsValue::from_str(&query.to_string())));
    req_opts.mode(RequestMode::Cors); // Can not be written, the default is Cors

    let request = Request::new_with_str_and_init(&gql_uri().await, &req_opts)?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();
    let resp_text = JsFuture::from(resp.text()?).await?;

    let users_str = resp_text.as_string().unwrap();
    let users_value: Value = serde_json::from_str(&users_str).unwrap();
    let users_vec = users_value["data"]["allUsers"]
        .as_array()
        .unwrap()
        .to_owned();

    Ok(users_vec)
}

pub struct Users {
    list: Vec<Value>,
}

pub enum Msg {
    UpdateList(Vec<Value>),
}

impl Component for Users {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { list: Vec::new() }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        let link = ctx.link().clone();
        if first_render {
            spawn_local(async move {
                let res = fetch_users().await;
                link.send_message(Msg::UpdateList(res.unwrap()))
            });
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateList(res) => {
                self.list = res;
                true
            }
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let users = self.list.iter().map(|user| {
            html! {
                <div>
                    <li>
                        <strong>
                            { &user["username"].as_str().unwrap() }
                            { " - length: " }
                            { &user["username"].as_str().unwrap().len() }
                        </strong>
                    </li>
                    <ul>
                        <li>{ &user["id"].as_str().unwrap() }</li>
                        <li>{ &user["email"].as_str().unwrap() }</li>
                    </ul>
                </div>
            }
        });

        html! {
            <>
                <h1>{ "all users" }</h1>
                <ul>
                    { for users }
                </ul>
            </>
        }
    }
}
