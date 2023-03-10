use super::emitters_window::Emitters;
use yew::{classes, function_component, html, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub emitters: Emitters,
    pub on_emitters_change: Callback<Emitters>,
}

#[function_component(Navbar)]
pub fn navbar(
    Props {
        emitters,
        on_emitters_change,
    }: &Props,
) -> Html {
    let emitter_vec = vec![
        Emitters::Controllers,
        Emitters::Compressors,
        Emitters::TankFarms,
    ];

    let emitter_iter = emitter_vec.into_iter().enumerate().map(|(key, e)| {
        let on_emitters_change = on_emitters_change.clone();
        let onclick = move |_| {
            on_emitters_change.emit(e);
        };

        html! {
            <button {key} {onclick} class={classes!(
                "emitter-nav",
                (emitters == &e).then(|| "active")
            )}>{ e }</button>
        }
    });

    html! {
        <nav class={classes!("emitters-navbar")}>
            { for emitter_iter }
        </nav>
    }
}
