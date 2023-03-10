use super::emitters_window::Emitters;
use std::rc::Rc;
use web_sys::MouseEvent;
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

    let emitters = emitters.clone();

    let onclick = {
        let emitters = emitters.clone();
        Callback::from(move |_: MouseEvent| {
            let emitters = emitters.clone();
            
            on_emitters_change.emit(emitters);
        })
    };


    // let emitters = emitters.as_ref();
    // let c1 = classes!("emitter-nav", (emitters == Emitters::Controllers).then(|| "active"));
    // let c2 = classes!("emitter-nav", (emitters == Emitters::Compressors).then(|| "active"));
    // let c3 = classes!("emitter-nav", (emitters == Emitters::TankFarms).then(|| "active"));

    html! {
        <nav>
            <button {onclick}>{ "Controllers" }</button>
            // <button class={c2}>{ "Compressors" }</button>
            // <button class={c3}>{ "Tank Farms" }</button>
        </nav>
    }
}
