use crate::components::emitters_window::{
    compressors::CompressorsComp, controllers::ControllersComp, emitter_navbar::EmitterNavbar,
    tank_farms::TankFarmsComp, Emitter,
};
use std::rc::Rc;
use uuid::Uuid;
use yew::{classes, function_component, html, use_state_eq, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub facility_id: Rc<Uuid>,
}

#[function_component(EmittersWindow)]
pub fn emitters_window(Props { facility_id }: &Props) -> Html {
    let emitter_state = use_state_eq(|| Emitter::Controller);

    let emitter = *emitter_state;

    let on_emitter_change = Callback::from(move |e: Emitter| {
        emitter_state.set(e);
    });

    let view = match emitter {
        Emitter::Controller => html! {
            <ControllersComp {facility_id} />
        },
        Emitter::Compressor => html! {
            <CompressorsComp {facility_id} />
        },
        Emitter::TankFarm => html! {
            <TankFarmsComp {facility_id} />
        },
    };

    html! {
        <div class={classes!("emitters-window")}>
            <EmitterNavbar {emitter} on_emitter_change={on_emitter_change} />
            <div />
            { view }
        </div>
    }
}
