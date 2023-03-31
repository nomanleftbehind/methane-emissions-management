use super::Emitter;
use yew::{classes, function_component, html, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub emitter: Emitter,
    pub on_emitter_change: Callback<Emitter>,
}

#[function_component(EmitterNavbar)]
pub fn emitter_navbar(
    Props {
        emitter,
        on_emitter_change,
    }: &Props,
) -> Html {
    let emitter_vec = vec![Emitter::Controller, Emitter::Compressor, Emitter::TankFarm];

    let emitter_iter = emitter_vec.into_iter().enumerate().map(|(key, e)| {
        let on_emitter_change = on_emitter_change.clone();
        let onclick = move |_| {
            on_emitter_change.emit(e);
        };

        html! {
            // <div class={classes!("navbar-button-wrapper")}>
                <button {key} {onclick} class={classes!(
                    "emitters-navigation-button",
                    (emitter == &e).then(|| "active")
                )}>{ e }
                </button>
            // </div>
        }
    });

    html! {
        <nav class={classes!("emitters-navbar")}>
            { for emitter_iter }
        </nav>
    }
}
