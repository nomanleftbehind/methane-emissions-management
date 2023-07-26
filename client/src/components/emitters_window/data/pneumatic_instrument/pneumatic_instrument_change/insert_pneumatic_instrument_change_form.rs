use crate::{
    models::{
        mutations::pneumatic_instrument::insert_pneumatic_instrument_change::{
            InsertPneumaticInstrumentChangeInput,
            Variables as VariablesInsertPneumaticInstrumentChange,
        },
        NaiveDateTime,
    },
    pages::ModalVariant,
};
use std::rc::Rc;
use uuid::Uuid;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::HtmlInputElement;
use yew::{
    classes, function_component, html, use_state_eq, Callback, Event, Html, Properties,
    SubmitEvent, TargetCast,
};

#[derive(PartialEq, Properties)]
pub struct Props {
    pub handle_insert_pneumatic_instrument_change:
        Callback<VariablesInsertPneumaticInstrumentChange>,
    pub close_insert_form: Callback<()>,
    pub modal_variant_handle: Callback<Option<ModalVariant>>,
    pub pneumatic_instrument_id: Rc<Uuid>,
}

#[function_component(InsertPneumaticInstrumentChangeForm)]
pub fn insert_pneumatic_instrument_change_form(
    Props {
        handle_insert_pneumatic_instrument_change,
        close_insert_form,
        modal_variant_handle,
        pneumatic_instrument_id,
    }: &Props,
) -> Html {
    let input_date_handle = use_state_eq(|| None);
    let input_rate_handle = use_state_eq(|| None);

    let date = *input_date_handle;
    let rate = *input_rate_handle;

    let onchange_date = Callback::from(move |e: Event| {
        let input: HtmlInputElement = e.target_unchecked_into();
        let input_value = input.value_as_number();
        let date = (!input_value.is_nan()).then(|| {
            NaiveDateTime::from_timestamp_millis(input_value as i64)
                .expect_throw("Unable to convert i64 to NaiveDateTime.")
                .date()
        });
        input_date_handle.set(date);
    });

    let onchange_rate = Callback::from(move |e: Event| {
        let input: HtmlInputElement = e.target_unchecked_into();
        let input_value = input.value_as_number();
        input_rate_handle.set(Some(input_value));
    });

    let disabled = !(date.is_some() && rate.is_some());

    let onsubmit = {
        let handle_insert_pneumatic_instrument_change =
            handle_insert_pneumatic_instrument_change.clone();
        let close_insert_form = close_insert_form.clone();
        let pneumatic_instrument_id = pneumatic_instrument_id.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            if let (Some(rate), Some(date)) = (rate, date) {
                let variables = VariablesInsertPneumaticInstrumentChange {
                    insert_pneumatic_instrument_change_input:
                        InsertPneumaticInstrumentChangeInput {
                            pneumatic_instrument_id: *pneumatic_instrument_id,
                            date,
                            rate,
                        },
                };
                handle_insert_pneumatic_instrument_change.emit(variables);
                close_insert_form.emit(());
            }
        })
    };

    html! {
        <form {onsubmit} class={classes!("insert-form", "emitter-cell")}>
            <fieldset class={classes!("pneumatic-instrument-form", "center")}>
                <button class={classes!("entry-button")} style="grid-row: 1; grid-column: 1;" type="submit" {disabled}>{"✓"}</button>
                <input type="date" style="grid-row: 1; grid-column: 3;" onchange={onchange_date}/>
                <input type="number" style="grid-row: 1; grid-column: 4;" onchange={onchange_rate}/>
            </fieldset>
        </form>
    }
}
