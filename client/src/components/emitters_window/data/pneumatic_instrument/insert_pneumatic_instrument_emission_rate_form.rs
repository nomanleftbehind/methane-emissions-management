use crate::models::{
    mutations::pneumatic_instrument::insert_pneumatic_instrument_emission_rate::{
        InsertPneumaticInstrumentEmissionRateInput, Variables,
    },
    NaiveDateTime,
};
use common::InsertFieldValueEnum::{DateValue, FloatValue};
use std::rc::Rc;
use uuid::Uuid;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::HtmlInputElement;
use yew::{
    classes, function_component, html, use_state_eq, Callback, Event, Html, InputEvent, Properties,
    SubmitEvent, TargetCast,
};

#[derive(PartialEq, Properties)]
pub struct Props {
    pub handle_insert_pneumatic_instrument_emission_rate: Callback<Variables>,
    pub close_insert_form: Callback<()>,
    pub pneumatic_instrument_id: Rc<Uuid>,
}

#[function_component(InsertPneumaticInstrumentEmissionRateForm)]
pub fn insert_pneumatic_instrument_emission_rate_form(
    Props {
        handle_insert_pneumatic_instrument_emission_rate,
        close_insert_form,
        pneumatic_instrument_id,
    }: &Props,
) -> Html {
    let input_date_handle = use_state_eq(|| DateValue(None));
    let input_rate_handle = use_state_eq(|| FloatValue(None));

    let date = (*input_date_handle).clone();
    let rate = (*input_rate_handle).clone();

    let onchange_date = {
        let input_date_handle = input_date_handle.clone();
        let date = date.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            match date {
                DateValue(_) => {
                    let input_value = input.value_as_number();
                    input_date_handle.set(DateValue((!input_value.is_nan()).then(|| {
                        NaiveDateTime::from_timestamp_millis(input_value as i64)
                            .expect_throw("Unable to convert i64 to NaiveDateTime.")
                            .date()
                    })));
                }
                FloatValue(_) => {
                    let input_value = input.value_as_number();
                    input_date_handle.set(FloatValue((!input_value.is_nan()).then(|| input_value)));
                }
            }
        })
    };

    let oninput_rate = {
        let input_rate_handle = input_rate_handle.clone();
        let rate = rate.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            match rate {
                DateValue(_) => {
                    let input_value = input.value_as_number();
                    input_rate_handle.set(DateValue((!input_value.is_nan()).then(|| {
                        NaiveDateTime::from_timestamp_millis(input_value as i64)
                            .expect_throw("Unable to convert i64 to NaiveDateTime.")
                            .date()
                    })));
                }
                FloatValue(_) => {
                    let input_value = input.value_as_number();
                    input_rate_handle.set(FloatValue((!input_value.is_nan()).then(|| input_value)));
                }
            }
        })
    };

    let disabled = match (&date, &rate) {
        (DateValue(Some(_)), FloatValue(Some(_))) => false,
        _ => true,
    };

    // !(date.is_some() && rate.is_some());

    let onsubmit = {
        let handle_insert_pneumatic_instrument_emission_rate =
            handle_insert_pneumatic_instrument_emission_rate.clone();
        let close_insert_form = close_insert_form.clone();
        let pneumatic_instrument_id = pneumatic_instrument_id.clone();
        // let date = date.clone();
        // let rate = rate.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let date = date.clone();
            let rate = rate.clone();

            if let (DateValue(Some(date)), FloatValue(Some(rate))) = (date, rate) {
                let variables = Variables {
                    insert_pneumatic_instrument_emission_rate_input:
                        InsertPneumaticInstrumentEmissionRateInput {
                            pneumatic_instrument_id: *pneumatic_instrument_id,
                            date,
                            rate,
                        },
                };
                handle_insert_pneumatic_instrument_emission_rate.emit(variables);
                close_insert_form.emit(());
            }
        })
    };

    html! {
        <form {onsubmit} class={classes!("insert-form", "emitter-cell")}>
            <fieldset class={classes!("pneumatic-instrument-emission-rate-form", "center")}>
                <button class={classes!("entry-button")} style="grid-row: 1; grid-column: 1;" type="submit" {disabled}>{"✓"}</button>
                <input type="date" style="grid-row: 1; grid-column: 2;" onchange={onchange_date}/>
                // oninput event occurs immediately after the content has been changed, while onchange occurs when the element loses focus.
                // Using oninput here because we want button to stop being disabled as soon as date and rate are entered, and not having to click outide for onchange to occur.
                <input type="number" step="any" style="grid-row: 1; grid-column: 3;" oninput={oninput_rate}/>
            </fieldset>
        </form>
    }
}
