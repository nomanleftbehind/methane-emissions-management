use crate::{
    components::emitters_window::{
        data::pneumatic_instruments::PneumaticInstrumentsComponent,
        emitter_navbar::EmitterNavbar,
        Emitter::{self, Compressor, Controller, TankFarm},
    },
    pages::ModalVariant,
};
use std::rc::Rc;
use uuid::Uuid;
use yew::{classes, function_component, html, use_state_eq, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub facility_id: Option<Rc<Uuid>>,
    pub modal_variant_handle: Callback<Option<ModalVariant>>,
}

#[function_component(EmittersWindow)]
pub fn emitters_window(
    Props {
        facility_id,
        modal_variant_handle,
    }: &Props,
) -> Html {
    let emitter_state = use_state_eq(|| Controller);

    let emitter = *emitter_state;

    let on_emitter_change = Callback::from(move |e: Emitter| {
        emitter_state.set(e);
    });

    // use_effect_with_deps(
    //     move |u| {
    //         console_log!("emitter type: {}", u);
    //     },
    //     emitter.clone(),
    // );

    html! {
        <div class={classes!("emitters-window")}>
            <EmitterNavbar {emitter} {on_emitter_change} />
            <div />
            if let Some(id) = facility_id {
                <PneumaticInstrumentsComponent {id} {modal_variant_handle} />
            }
        </div>
    }
}
