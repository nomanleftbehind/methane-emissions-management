use crate::build_request::build_request;
use crate::error::AppError;
use common::Role;
use graphql_client::{GraphQLQuery, Response};
use leptos::*;
use serde_json::json;

mod console_log;
pub(crate) use console_log::console_log;
pub mod build_request;
pub mod error;

/// `UUID` is a custom scalar type defined in schema, so we have to provide matching Rust type.
pub type UUID = uuid::Uuid;
// use serde::Deserialize;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries.graphql",
    variables_derives = "PartialEq, Clone, Debug",
    response_derives = "Debug, Clone, Serialize",
    extern_enums("Role")
)]
pub struct GetUsers;

// use gloo_timers::future::TimeoutFuture;

// use crate::models::queries::user::{
//     get_users::{ResponseData, Variables},
//     GetUsers,
// };

// Here we define an async function
// This could be anything: a network request, database read, etc.
// Here, we just multiply a number by 10
async fn load_data<Q>(variables: Q::Variables) -> Result<Q::ResponseData, AppError>
where
    Q: GraphQLQuery,
    Q::Variables: 'static,
    Q::ResponseData: Clone + std::fmt::Debug + 'static,
{
    let request_body = Q::build_query(variables);
    let request_json = &json!(request_body);
    let request = build_request(request_json).await;

    let response_data = match request {
        Ok(response) => {
            // json method cannot be called in build_request() function because response type has to implement Deserialize trait which compiler cannot infer.
            let json = response
                .json::<Response<Q::ResponseData>>()
                .await
                .map_err(AppError::from);
            match json {
                Ok(response) => response.data.ok_or_else(|| {
                    response
                        .errors
                        .map_or_else(|| "Unknown error.".into(), |e| e.into())
                }),
                Err(error) => Err(error),
            }
        }
        Err(error) => Err(error),
    };

    console_log!("response_body: {:?}", response_data);

    response_data
}

#[component]
fn App() -> impl IntoView {
    // this count is our synchronous, local state
    let (count, set_count) = create_signal(0);

    // create_resource takes two arguments after its scope
    // let async_data = create_resource(
    //     // the first is the "source signal"
    //     count,
    //     // the second is the loader
    //     // it takes the source signal's value as its argument
    //     // and does some async work
    //     |value| async move { load_data(value).await },
    // );
    // whenever the source signal changes, the loader reloads

    // you can also create resources that only load once
    // just return the unit type () from the source signal
    // that doesn't depend on anything: we just load it once
    let stable = create_resource(
        || (),
        |_| async move { load_data::<GetUsers>(get_users::Variables {}).await },
    );

    let y = stable.get();

    // we can access the resource values with .get()
    // this will reactively return None before the Future has resolved
    // and update to Some(T) when it has resolved
    // let async_result = move || {
    //     async_data
    //         .get()
    //         .map(|value| format!("Server returned {value:?}"))
    //         // This loading state will only show before the first load
    //         .unwrap_or_else(|| "Loading...".into())
    // };

    // the resource's loading() method gives us a
    // signal to indicate whether it's currently loading
    let loading = stable.loading();
    let is_loading = move || if loading() { "Loading..." } else { "Idle." };

    view! {
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
        >
            "Click me"
        </button>
        <p>
            // <code>"stable"</code>": " {move || stable.get()}
        </p>
        <p>
            <code>"count"</code>": " {count}
        </p>
        <p>
            <code>"async_value"</code>": "
            // {async_result}
            <br/>
            {is_loading}
        </p>
    }
}

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}

// #[component]
// fn App() -> impl IntoView {
//     let (count, set_count) = create_signal(0);
//     let double_count = move || count() * 2;
//     view! {
//         <button
//             on:click=move |_| {
//                 set_count.update(|n| *n += 1);
//             }

//             // the class: syntax reactively updates a single class
//             // here, we'll set the `red` class when `count` is odd
//             class=("red", move || count() % 2 == 1)
//         >
//             "Click me"
//         </button>
//         // now we use our component!
//         <ProgressBar progress=count/>
//         <ProgressBar progress=double_count/>
//     }
// }

// /// Shows progress toward a goal.
// #[component]
// fn ProgressBar<F: Fn() -> i32 + 'static>(
//     /// The maximum value of the progress bar.
//     #[prop(default = 100)]
//     max: u16,
//     /// How much progress should be displayed.
//     progress: F,
// ) -> impl IntoView {
//     view! {
//         <progress
//             max=max
//             // now this works
//             value=progress
//         ></progress>
//     }
// }
