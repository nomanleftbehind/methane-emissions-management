use crate::{
    list_facilities::ListFacilities,
    models::queries::user::{get_users, GetUsers},
    utils::{app_console_log, load_data},
};
use leptos::*;

pub mod list;
pub mod list_facilities;
pub mod models;
pub mod utils;

#[component]
fn App() -> impl IntoView {
    // this count is our synchronous, local state
    let (count, set_count) = create_signal(0);

    // create_resource takes two arguments after its scope
    let async_data = create_resource(
        // the first is the "source signal"
        count,
        // the second is the loader
        // it takes the source signal's value as its argument
        // and does some async work
        |value| async move {
            app_console_log!("count: {:?}", value);
        },
    );
    // whenever the source signal changes, the loader reloads

    // you can also create resources that only load once
    // just return the unit type () from the source signal
    // that doesn't depend on anything: we just load it once
    let user_data = create_resource(
        || (),
        |_| async move { load_data::<GetUsers>(get_users::Variables {}).await },
    );

    let y = move || match user_data.get() {
        None => view! { <p>"Loading..."</p> }.into_view(),
        Some(data) => match data {
            Ok(u) => view! { <div>u</div> }.into_view(),
            Err(e) => view! { <div>{e.to_string()}</div> }.into_view(),
        },
    };

    // we can access the resource values with .get()
    // this will reactively return None before the Future has resolved
    // and update to Some(T) when it has resolved
    let async_result = move || {
        async_data
            .get()
            .map(|value| format!("Server returned {value:?}"))
            // This loading state will only show before the first load
            .unwrap_or_else(|| "Loading...".into())
    };

    // the resource's loading() method gives us a
    // signal to indicate whether it's currently loading
    let loading = user_data.loading();
    let is_loading = move || if loading() { "Loading..." } else { "Idle." };

    view! { <ListFacilities/> }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App)
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
