use crate::{
    components::emitters_window::{
        data::pneumatic_instrument::pneumatic_instruments::PneumaticInstrumentsComponent,
        emitter_navbar::EmitterNavbar,
    },
    pages::ModalVariant,
};
use common::Emitter::{self, PneumaticInstrument};
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
    let emitter_state = use_state_eq(|| PneumaticInstrument);

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
            if let Some(facility_id) = facility_id {
                <PneumaticInstrumentsComponent {facility_id} {modal_variant_handle} />
            }
        </div>
    }
}
