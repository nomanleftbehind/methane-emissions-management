use crate::{
    models::queries::pneumatic_instrument::{
        get_pneumatic_instruments::{
            self, GetPneumaticInstrumentsInput, PneumaticInstrumentsByVariant,
        },
        GetPneumaticInstruments,
    },
    routes::ExampleContext,
    utils::load_data,
};
use leptos::{logging::log, *};
use leptos_router::*;
use uuid::Uuid;

#[derive(Params, PartialEq, Clone, Debug)]
pub struct PneumaticInstrumentParams {
    id: Uuid,
}

#[component]
pub fn PneumaticInstruments() -> impl IntoView {
    log!("rendering <Pneumatic Instruments/>");

    log!(
        "ExampleContext should be Some(42). It is {:?}",
        use_context::<ExampleContext>()
    );

    on_cleanup(|| {
        log!("cleaning up <Pneumatic Instruments/>");
    });

    let params = use_params::<PneumaticInstrumentParams>();

    let pneumatil_device_list = create_resource(
        move || params().map(|params| params.id).ok(),
        |id| async move {
            load_data::<GetPneumaticInstruments>(get_pneumatic_instruments::Variables {
                get_pneumatic_instruments_input: GetPneumaticInstrumentsInput {
                    by: PneumaticInstrumentsByVariant::FACILITY_ID,
                    id: id.unwrap(),
                },
            })
            .await
        },
    );

    create_effect(move |_| {
        log!("params = {:#?}", params.get());
    });

    let contact_display = move || {
        pneumatil_device_list
            .get()
            .map(|response| {
                response.map(|data| {
                    view! {
                        <For
                        each=move || data.get_pneumatic_instruments.clone()
                        key=|pneumatic_instrument| pneumatic_instrument.id
                        let:pneumatic_instrument
                    >
                        <li class="sidebar-button-container">
                            <A href=pneumatic_instrument.id.to_string() class="sidebar-button">
                                // class=("active", "a" == "b")
                                {pneumatic_instrument.serial_number}
                            </A>
                        </li>
                    </For>
                    }
                })
            })
            .into_view()
    };

    view! {
        <div class="contact">
            <Transition fallback=move || {
                view! { <p>"Loading..."</p> }
            }>{contact_display}</Transition>
        </div>
    }
}
