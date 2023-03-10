use yew::{function_component, html, Html, use_state_eq, Callback};

#[derive(PartialEq, Clone, Copy)]
pub enum Emitters {
    Controllers,
    Compressors,
    TankFarms,
}



#[function_component(EmittersWindow)]
pub fn emitters_window() -> Html {

    let state = use_state_eq(|| Emitters::Controllers);

    let on_emitter_change = Callback::from(move |e: Emitters| {
        
        state.set(e);
    });


    html! {}
}
