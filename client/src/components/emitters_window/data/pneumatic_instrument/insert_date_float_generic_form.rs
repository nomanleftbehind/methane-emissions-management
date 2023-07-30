use crate::{
    models::{
        mutations::pneumatic_instrument::insert_pneumatic_instrument_emission_rate::{
            InsertPneumaticInstrumentEmissionRateInput,
            Variables as VariablesInsertPneumaticInstrumentEmissionRate,
        },
        NaiveDateTime,
    },
    utils::console_log,
};
use graphql_client::GraphQLQuery;
use std::rc::Rc;
use uuid::Uuid;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::HtmlInputElement;
use yew::{
    classes, function_component, html, use_state_eq, Callback, Event, Html, InputEvent, Properties,
    SubmitEvent, TargetCast,
};

#[derive(PartialEq, Properties)]
pub struct Props<T>
where
    T: PartialEq + GraphQLQuery,
    T::Variables: PartialEq,
{
    pub variables: T::Variables,
    pub handle_insert: Callback<T::Variables>,
    pub close_insert_form: Callback<()>,
    pub pneumatic_instrument_id: Rc<Uuid>,
}

#[function_component(InsertGenericForm)]
pub fn insert_generic_form<T>(
    Props {
        variables,
        handle_insert,
        close_insert_form,
        pneumatic_instrument_id,
    }: &Props<T>,
) -> Html
where
    T: PartialEq + GraphQLQuery,
    T::Variables: PartialEq,
{
    let input_date_handle = use_state_eq(|| None);
    let input_float_handle = use_state_eq(|| None);

    let date = *input_date_handle;
    let float = *input_float_handle;

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

    let oninput_float = Callback::from(move |e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        let input_value = input.value_as_number();
        console_log!("value: {}", input_value);
        input_float_handle.set(Some(input_value));
    });

    let disabled = !(date.is_some() && float.is_some());

    let onsubmit = {
        let handle_insert = handle_insert.clone();
        let close_insert_form = close_insert_form.clone();
        let pneumatic_instrument_id = pneumatic_instrument_id.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            if let (Some(date), Some(float)) = (date, float) {
                let variables = VariablesInsertPneumaticInstrumentEmissionRate {
                    insert_pneumatic_instrument_emission_rate_input:
                        InsertPneumaticInstrumentEmissionRateInput {
                            pneumatic_instrument_id: *pneumatic_instrument_id,
                            date,
                            rate: float,
                        },
                };
                handle_insert.emit(variables);
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
                <input type="number" step="any" style="grid-row: 1; grid-column: 3;" oninput={oninput_float}/>
            </fieldset>
        </form>
    }
}
