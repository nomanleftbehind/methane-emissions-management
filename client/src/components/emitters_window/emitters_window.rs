use crate::{
    components::emitters_window::{
        data::objects::ObjectsComponent,
        emitter_navbar::EmitterNavbar,
        Emitter::{self, Compressor, Controller, TankFarm},
    },
    models::queries::get_object::get_object::GetObjectVariant::{
        COMPRESSOR_BY_FACILITY_ID, CONTROLLER_BY_FACILITY_ID, TANK_FARM_BY_FACILITY_ID,
    },
    utils::error::AppError,
};
use std::rc::Rc;
use uuid::Uuid;
use yew::{classes, function_component, html, use_state_eq, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub facility_id: Option<Rc<Uuid>>,
    pub error_handle: Callback<Option<AppError>>,
}

#[function_component(EmittersWindow)]
pub fn emitters_window(
    Props {
        facility_id,
        error_handle,
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
                <ObjectsComponent {id} {error_handle} object_variant={match emitter {
                    Controller => CONTROLLER_BY_FACILITY_ID,
                    Compressor => COMPRESSOR_BY_FACILITY_ID,
                    TankFarm => TANK_FARM_BY_FACILITY_ID,
                }} />
            }
        </div>
    }
}
