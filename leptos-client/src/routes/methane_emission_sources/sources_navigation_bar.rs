use common::Emitter;
use leptos::*;
use leptos_router::{AnimatedOutlet, A};
use strum::IntoEnumIterator;

use crate::console_log;

#[component]
pub fn sources_navigation_bar() -> impl IntoView {
    log::debug!("rendering <SourcesNavigationBar/>");

    let sources = Emitter::iter()
        .map(|s| {
            // let se = s
            //     .to_string()
            //     .chars()
            //     .enumerate()
            //     .map_windows(|&[(i, a), (j, b)]| {
            //         let v = if i == 0 {
            //             a.to_uppercase()
            //         } else if a == '_' {
            //             String::from(' ').pus + b.to_uppercase()
            //         } else {
            //             a.to_lowercase()
            //         };

            //         console_log!("{:#?}", v);
            //         v
            //     })
            //     .collect::<Vec<_>>();
            // let mut c = se.chars();
            // let a = match c.next() {
            //     None => String::new(),
            //     Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            // };
            let ns = s.to_string().split('_').collect::<Vec<_>>().join(" ");
            view! {
                <A href=s.to_string() class="emitters-navigation-button">
                    {s.to_string()}
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
