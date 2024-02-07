use leptos::*;

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let double_count = move || count() * 2;
    view! {
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }

            // the class: syntax reactively updates a single class
            // here, we'll set the `red` class when `count` is odd
            class=("red", move || count() % 2 == 1)
        >
            "Click me"
        </button>
        // now we use our component!
        <ProgressBar progress=count/>
        <ProgressBar progress=double_count/>
    }
}

/// Shows progress toward a goal.
#[component]
fn ProgressBar<F: Fn() -> i32 + 'static>(
    /// The maximum value of the progress bar.
    #[prop(default = 100)]
    max: u16,
    /// How much progress should be displayed.
    progress: F,
) -> impl IntoView {
    view! {
        <progress
            max=max
            // now this works
            value=progress
        ></progress>
    }
}
