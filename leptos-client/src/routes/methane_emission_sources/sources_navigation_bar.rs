use common::Emitter;
use leptos::*;
use leptos_router::{AnimatedOutlet, A};
use strum::IntoEnumIterator;

use crate::console_log;

#[component]
pub fn sources_navigation_bar() -> impl IntoView {
    log::debug!("rendering <SourcesNavigationBar/>");

    let sources = Emitter::iter()
        .map(|source| {
            let ns = source.to_pretty_name();
            view! {
                <A href=source.to_string() class="emitters-navigation-button">
                    {ns}
                </A>
            }
        })
        .collect::<Vec<_>>();

    view! {
        <nav class="emitters-navbar">{sources}</nav>
        <div></div>
        <AnimatedOutlet class="emitters pneumatic-instruments" outro="fadeOut" intro="fadeIn"/>
    }
}
