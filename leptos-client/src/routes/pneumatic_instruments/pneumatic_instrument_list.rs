use crate::{
    models::{
        mutations::manual_mutation::{delete_entry, DeleteEntry},
        queries::pneumatic_instrument::{
            get_pneumatic_instruments::{
                self, GetPneumaticInstrumentsInput, PneumaticInstrumentsByVariant,
            },
            GetPneumaticInstruments,
        },
    },
    routes::{
        pneumatic_instruments::pneumatic_intrument_row::PneumaticInstrumentRow, ExampleContext,
    },
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

    let pneumatic_instrument_list = create_resource(
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

    // let pneumatic_instruments_iter = get_pneumatic_instruments.into_iter().enumerate().map(|(mut row_num, pneumatic_instrument)| {
    //     row_num = (row_num + 2) * 2 - 1;
    //     html! {
    //         <PneumaticInstrumentRowComponent {row_num} {modal_variant_handle} {pneumatic_instrument} handle_update_field={handle_update_field.clone()} handle_delete_entry={handle_delete_entry.clone()} />
    //     }
    // });

    // let handle_delete_entry = {
    //     let modal_variant_handle = modal_variant_handle.clone();
    //     let number_of_updated_fields_handle = number_of_updated_fields_handle.clone();

    //     Callback::from(move |variables: VariablesDeleteEntry| {
    //         let delete_entry_callback = {
    //             let number_of_updated_fields_handle = number_of_updated_fields_handle.clone();
    //             let modal_variant_handle = modal_variant_handle.clone();

    //             Callback::from(move |_| {
    //                 let variables = variables.clone();
    //                 let number_of_updated_fields_handle = number_of_updated_fields_handle.clone();
    //                 let modal_variant_handle = modal_variant_handle.clone();
    //                 wasm_bindgen_futures::spawn_local(async move {
    //                     match lazy_query::<DeleteEntry>(variables).await {
    //                         QueryResponse {
    //                             data: Some(ResponseDataDeleteEntry { delete_entry }),
    //                             ..
    //                         } => {
    //                             number_of_updated_fields_handle
    //                                 .set(number_of_updated_fields + delete_entry);
    //                         }
    //                         QueryResponse {
    //                             error: Some(error), ..
    //                         } => {
    //                             modal_variant_handle.emit(Some(ModalVariant::Error(error)));
    //                         }
    //                         _ => (),
    //                     };
    //                 });
    //             })
    //         };

    //         modal_variant_handle.emit(Some(ModalVariant::ConfirmDelete(delete_entry_callback)));
    //     })
    // };

    let handle_delete_entry = move |variables: delete_entry::Variables| {
        create_resource(
            move || variables.clone(),
            |variables| async move { load_data::<DeleteEntry>(variables).await },
        );
    };

    let pneumatic_instrument_display = move || {
        pneumatic_instrument_list
            .get()
            .map(|response| {
                response.map(|data| {
                    view! {
                        <div class:emitters=true class:pneumatic-instruments=true>
                            // <InsertEntryButton {insert_form_is_open} {toggle_insert_form_is_open}/>
                            <div class="sticky" style:grid-column=2 style:grid-row=1></div>
                            <div class="sticky" style:grid-column=3 style:grid-row=1>
                                "Type"
                            </div>
                            <div class="sticky" style:grid-column=4 style:grid-row=1>
                                "Manufacturer"
                            </div>
                            <div class="sticky" style:grid-column=5 style:grid-row=1>
                                "Model"
                            </div>
                            <div class="sticky" style:grid-column=6 style:grid-row=1>
                                "Serial Number"
                            </div>
                            <div class="sticky" style:grid-column=7 style:grid-row=1>
                                "Start Date"
                            </div>
                            <div class="sticky" style:grid-column=8 style:grid-row=1>
                                "End Date"
                            </div>
                            <div class="sticky" style:grid-column=9 style:grid-row=1>
                                "Site"
                            </div>
                            <div class="sticky" style:grid-column=10 style:grid-row=1>
                                "Created By"
                            </div>
                            <div class="sticky" style:grid-column=11 style:grid-row=1>
                                "Created At"
                            </div>
                            <div class="sticky" style:grid-column=12 style:grid-row=1>
                                "Updated By"
                            </div>
                            <div class="sticky" style:grid-column=13 style:grid-row=1>
                                "Updated At"
                            </div>
                            <div class="sticky" style:grid-column=14 style:grid-row=1>
                                "ID"
                            </div>
                            // if insert_form_is_open {
                            // <InsertPneumaticInstrumentForm {facility_id} {close_insert_form} {handle_insert} {modal_variant_handle} />
                            // }

                            <For
                                each=move || {
                                    data.get_pneumatic_instruments.clone().into_iter().enumerate()
                                }

                                key=|(_, pneumatic_instrument)| pneumatic_instrument.id
                                let:pneumatic_instrument
                            >
                                <PneumaticInstrumentRow
                                    row_num=Signal::derive(move || {
                                        (pneumatic_instrument.0 + 2) * 2 - 1
                                    })

                                    pneumatic_instrument=Signal::derive(move || {
                                        pneumatic_instrument.1.clone()
                                    })

                                    handle_delete_entry=handle_delete_entry
                                />
                            </For>
                        </div>
                    }
                })
            })
            .into_view()
    };

    view! {
        <Transition fallback=move || {
            view! { <p>"Loading..."</p> }
        }>{pneumatic_instrument_display}</Transition>
    }
}
