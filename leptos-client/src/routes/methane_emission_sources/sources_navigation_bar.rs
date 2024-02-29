use common::Emitter;
use leptos::*;
use leptos_router::{AnimatedOutlet, A};
use strum::IntoEnumIterator;

#[component]
pub fn sources_navigation_bar() -> impl IntoView {
    log::debug!("rendering <SourcesNavigationBar/>");

    let sources = Emitter::iter()
        .map(|s| {
            view! {
                <A href=s.to_string() class="emitters-navigation-button">
                    {s.to_string()}
                </A>
            }
        })
        .collect::<Vec<_>>();

    view! {
        <nav class="emitters-navbar">{sources}</nav>
        <div/>
        <AnimatedOutlet class="emitters pneumatic-instruments" outro="fadeOut" intro="fadeIn"/>
    }
}
